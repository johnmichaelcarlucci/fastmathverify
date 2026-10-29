//! Port of latex2sympy2_extended.symbols (Greek letters, singleton constants).

use crate::expr::Expr;

/// Marker prefix for symbols created with `real=False` (complex context), so they
/// never compare equal to the real symbol of the same name, like SymPy.
pub const NONREAL: char = '\u{1}';

pub fn make_symbol(name: &str, is_real: bool) -> Expr {
    if is_real {
        Expr::Sym(name.to_string())
    } else {
        Expr::Sym(format!("{NONREAL}{name}"))
    }
}

pub fn display_name(s: &str) -> &str {
    s.strip_prefix(NONREAL).unwrap_or(s)
}

pub fn is_real_symbol(s: &str) -> bool {
    !s.starts_with(NONREAL)
}

fn greek(s: &str) -> Option<&'static str> {
    Some(match s {
        "α" | "\\alpha" | "\\char000391" => "alpha",
        "β" | "\\beta" | "\\char000392" => "beta",
        "γ" | "\\gamma" => "gamma",
        "Γ" | "\\Gamma" => "Gamma",
        "δ" | "\\delta" => "delta",
        "Δ" | "\\Delta" => "Delta",
        "ε" | "\\epsilon" | "\\char000190" => "epsilon",
        "ϵ" | "\\varepsilon" => "varepsilon",
        "ζ" | "\\zeta" | "\\char000396" => "zeta",
        "η" | "\\eta" | "\\char000397" => "eta",
        "θ" | "\\theta" => "theta",
        "Θ" | "\\Theta" => "Theta",
        "ϑ" | "\\vartheta" => "vartheta",
        "ι" | "\\iota" | "\\char000399" => "iota",
        "κ" | "\\kappa" | "\\char00039A" => "kappa",
        "λ" | "\\lambda" => "lambda",
        "Λ" | "\\Lambda" => "Lambda",
        "μ" | "\\mu" | "\\char00039C" => "mu",
        "ν" | "\\nu" | "\\char00039D" => "nu",
        "ξ" | "\\xi" => "xi",
        "Ξ" | "\\Xi" => "Xi",
        "ο" | "\\omicron" | "\\char00039F" => "omicron",
        "π" | "\\pi" => "pi",
        "Π" | "\\Pi" => "Pi",
        "ϖ" | "\\varpi" => "varpi",
        "ρ" | "\\rho" | "\\char0003A1" => "rho",
        "ϱ" | "\\varrho" => "varrho",
        "σ" | "\\sigma" => "sigma",
        "Σ" | "\\Sigma" => "Sigma",
        "ς" | "\\varsigma" => "varsigma",
        "τ" | "\\tau" | "\\char0003A4" => "tau",
        "υ" | "\\upsilon" => "upsilon",
        "Υ" | "\\Upsilon" => "Upsilon",
        "φ" | "\\phi" => "phi",
        "Φ" | "\\Phi" => "Phi",
        "ϕ" | "\\varphi" => "varphi",
        "χ" | "\\chi" | "\\char0003A7" => "chi",
        "ψ" | "\\psi" => "psi",
        "Ψ" | "\\Psi" => "Psi",
        "ω" | "\\omega" => "omega",
        "Ω" | "\\Omega" => "Omega",
        _ => return None,
    })
}

fn singleton(s: &str) -> Option<Expr> {
    match s {
        "pi" => Some(Expr::Pi),
        "gamma" | "Gamma" => Some(Expr::EulerGamma),
        _ => None,
    }
}

/// `get_symbol(latex_str, is_real, lowercase_symbols)`
pub fn get_symbol(latex: &str, is_real: bool, lowercase: bool) -> Expr {
    let s = latex.trim();
    let mut letter = match greek(&s.replace('"', "")) {
        Some(g) => g.to_string(),
        None => s.to_string(),
    };
    if let Some(rest) = letter.strip_prefix('\\') {
        letter = rest.to_string();
    }
    if let Some(c) = singleton(&letter) {
        return c;
    }
    if lowercase {
        letter = letter.to_lowercase();
    }
    if let Some(c) = singleton(&letter) {
        return c;
    }
    make_symbol(&letter, is_real)
}
