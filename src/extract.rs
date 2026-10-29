//! Port of math_verify.parser: regex-based answer extraction and parsing.

use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

use fancy_regex::{Captures, Regex, RegexBuilder};

use crate::convert::{sympy_number, Converter};
use crate::expr::*;
use crate::normalize::{normalize_latex, split_by, BoxedMode, NormalizationConfig, EQUATION_SPLIT};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LatexConfig {
    pub try_extract_without_anchor: bool,
    pub boxed_match_priority: i32,
    pub normalization: NormalizationConfig,
}

impl Default for LatexConfig {
    fn default() -> Self {
        LatexConfig {
            try_extract_without_anchor: true,
            boxed_match_priority: 50,
            normalization: NormalizationConfig {
                basic_latex: true,
                units: true,
                malformed_operators: true,
                nits: true,
                boxed: BoxedMode::All,
                equations: false,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExprConfig {
    pub try_extract_without_anchor: bool,
}

impl Default for ExprConfig {
    fn default() -> Self {
        ExprConfig { try_extract_without_anchor: true }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StringConfig {
    pub strings: Vec<String>,
    pub try_extract_without_anchor: bool,
    pub lowercase: bool,
}

impl Default for StringConfig {
    fn default() -> Self {
        StringConfig {
            strings: vec!["A".into(), "B".into(), "C".into(), "D".into()],
            try_extract_without_anchor: true,
            lowercase: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Target {
    Latex(LatexConfig),
    Expr(ExprConfig),
    Str(StringConfig),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    Expr(Expr),
    Str(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FallbackMode {
    NoFallback,
    FirstMatch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtractionMode {
    FirstMatch,
    AnyMatch,
}

/// Python `re` syntax -> fancy-regex syntax for the constructs used upstream.
fn translate(p: &str, dotall: bool) -> String {
    let t = crate::normalize::py_escapes(p);
    if dotall { format!("(?s){t}") } else { t }
}

/// Remove lookaround groups. Every match of the original pattern is a match of
/// the result at the same position, so the result is a sound prefilter.
fn strip_lookarounds(p: &str) -> String {
    let b: Vec<char> = p.chars().collect();
    let mut out = String::with_capacity(p.len());
    let mut i = 0;
    let mut in_class = false;
    while i < b.len() {
        let c = b[i];
        if c == '\\' && i + 1 < b.len() {
            out.push(c);
            out.push(b[i + 1]);
            i += 2;
            continue;
        }
        if in_class {
            if c == ']' {
                in_class = false;
            }
            out.push(c);
            i += 1;
            continue;
        }
        if c == '[' {
            in_class = true;
            out.push(c);
            i += 1;
            // a leading ']' or '^]' is literal
            if i < b.len() && b[i] == '^' {
                out.push('^');
                i += 1;
            }
            if i < b.len() && b[i] == ']' {
                out.push(']');
                i += 1;
            }
            continue;
        }
        let rest: String = b[i..(i + 4).min(b.len())].iter().collect();
        if rest.starts_with("(?<!") || rest.starts_with("(?<=") || rest.starts_with("(?!") || rest.starts_with("(?=") {
            // skip to the matching ')'
            let mut depth = 0;
            let mut j = i;
            let mut cls = false;
            while j < b.len() {
                let d = b[j];
                if d == '\\' {
                    j += 2;
                    continue;
                }
                if cls {
                    if d == ']' {
                        cls = false;
                    }
                } else if d == '[' {
                    cls = true;
                } else if d == '(' {
                    depth += 1;
                } else if d == ')' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                j += 1;
            }
            i = j + 1;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Escape `{`/`}` that Python treats literally (not a `{m,n}` quantifier).
fn py_braces(p: &str) -> String {
    let b: Vec<char> = p.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut in_class = false;
    while i < b.len() {
        let c = b[i];
        if c == '\\' && i + 1 < b.len() {
            out.push(c);
            out.push(b[i + 1]);
            i += 2;
            continue;
        }
        if in_class {
            if c == ']' {
                in_class = false;
            }
            out.push(c);
            i += 1;
            continue;
        }
        if c == '[' {
            in_class = true;
            out.push(c);
            i += 1;
            if i < b.len() && b[i] == '^' {
                out.push('^');
                i += 1;
            }
            if i < b.len() && b[i] == ']' {
                out.push(']');
                i += 1;
            }
            continue;
        }
        if c == '{' {
            // valid quantifier: {digits} {digits,} {digits,digits}
            let rest: String = b[i..].iter().take(32).collect();
            let close = rest.find('}');
            let ok = close
                .map(|k| {
                    let inner = &rest[1..k];
                    let mut parts = inner.splitn(2, ',');
                    let a = parts.next().unwrap_or("");
                    let bpart = parts.next();
                    !a.is_empty() && a.chars().all(|c| c.is_ascii_digit())
                        && bpart.map(|x| x.chars().all(|c| c.is_ascii_digit())).unwrap_or(true)
                })
                .unwrap_or(false);
            if ok {
                let k = close.unwrap();
                out.push_str(&rest[..=k]);
                i += rest[..=k].chars().count();
                continue;
            }
            out.push_str("\\{");
            i += 1;
            continue;
        }
        if c == '}' {
            out.push_str("\\}");
            i += 1;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

pub struct Pat {
    pub re: Regex,
    pre: Option<regex::Regex>,
    pub prio: i32,
}

impl Pat {
    /// All non-overlapping matches, like Python's `finditer`.
    pub fn find_all<'t>(&self, text: &'t str) -> Vec<Captures<'t>> {
        let mut out = Vec::new();
        let mut pos = 0;
        while pos <= text.len() {
            let start = match &self.pre {
                Some(pre) => match pre.find_at(text, pos) {
                    Some(m) => m.start(),
                    None => break,
                },
                None => pos,
            };
            let c = match self.re.captures_from_pos(text, start) {
                Ok(Some(c)) => c,
                _ => break,
            };
            let m = c.get(0).unwrap();
            pos = if m.end() > m.start() {
                m.end()
            } else {
                m.end() + text[m.end()..].chars().next().map(|c| c.len_utf8()).unwrap_or(1)
            };
            out.push(c);
        }
        out
    }
}

fn make_pat(p: &str, dotall: bool, prio: i32) -> Pat {
    let t = translate(p, dotall);
    let re = RegexBuilder::new(&t)
        .backtrack_limit(2_000_000)
        .build()
        .unwrap_or_else(|e| panic!("regex {t}: {e}"));
    let pre = match regex::RegexBuilder::new(&py_braces(&strip_lookarounds(&t))).size_limit(1 << 28).dfa_size_limit(1 << 28).build() {
        Ok(r) => Some(r),
        Err(e) => {
            if std::env::var("FMV_DEBUG").is_ok() {
                eprintln!("prefilter compile failed: {e}");
            }
            None
        }
    };
    Pat { re, pre, prio }
}

fn escape(s: &str) -> String {
    let mut o = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || !c.is_ascii() {
            o.push(c);
        } else {
            o.push('\\');
            o.push(c);
        }
    }
    o
}

pub type Pats = Vec<Pat>;

fn string_regexes(cfg: &StringConfig) -> Pats {
    let keys: Vec<String> = cfg.strings.iter().map(|s| escape(s)).collect();
    let string_keys = format!("(?P<string_keys>{})", keys.join("|"));
    let full_stop_re = r"\.";
    let comma_re = ",";
    let colon_re = ":";
    let space_re = r"\s";
    let answer_prefix_re = format!(r"(^|{space_re})(?:\*\*)?");
    let answer_suffix_re = format!(r"(?:\*\*)?(?:{full_stop_re}|{comma_re}|{colon_re}|{space_re}|$)");
    let answer_re = format!("{answer_prefix_re}{string_keys}{answer_suffix_re}");
    let answer_re_start = format!(r"^(?:\*\*)?{string_keys}{answer_suffix_re}");
    let answer_word = "(?i:answer)";
    let mut v: Vec<(String, i32)> = vec![
        (format!(r"(?i:final answer is)\:?\s*{string_keys}\.?\s?I hope"), 0),
        (format!(r"(?i:final answer.{{0,100}}?)\s+is\:?\s*{string_keys}"), 50),
        (format!("{answer_word}{colon_re}.{{0,50}}?{answer_re}"), 100),
        (format!("{answer_word}.{{0,50}}?{answer_re}"), 200),
    ];
    if cfg.try_extract_without_anchor {
        v.push((answer_re_start, 250));
        v.push((answer_re, 300));
    }
    v.into_iter().map(|(p, pr)| make_pat(&p, false, pr)).collect()
}

pub fn expr_regexes(cfg: &ExprConfig) -> Pats {
    let number_re = concat!(
        r"(?<!\d)(?:",
        r"(?P<integer1>-?[1-9]\d{0,2}(?:[ ,]\d{3})+)(?P<decimal1>\.\d+)?|",
        r"(?P<integer2>-?\d+)(?P<decimal2>[.,]\d+)|",
        r"(?P<decimal3>\.\d+)|",
        r"(?P<integer3>-?\d+)",
        r")(?P<percent>\s*(?:%|[Pp]ercent|\s*[Pp]ercentage|\s*[Pp]ct))?"
    );
    let operators = [r"\+", r"\-", r"\*", r"\×", r"\/", r"\^", r"\(", r"\)", r"\÷"];
    let operators_re = operators.join("");
    let all_expr_chars = format!(r"[\d\.\s{operators_re}]");
    let expr_re = format!(r"(?P<expr>-?\(?-?\d{all_expr_chars}*[{operators_re}]{all_expr_chars}+\)?)");
    let currency_units = "$€£¥₹₽₪₩₫฿₡₢₣₤₥₦₧₨₩₪₫₭₮₯₰₱₲₳₴₵₶₷₸₹₺₻₼₽₾₿".replace('$', r"\$");
    let expr_prefix_re = r"(?:^|\s|\=)(?:\*\*)?";
    let expr_suffix_re = r"(?:\*\*)?(?:\.|,|:|\s|\)|\$|$)";
    let expr_with_anchors = format!("(?:{expr_prefix_re}{expr_re}{expr_suffix_re})");
    let number_with_anchors = format!("(?:{expr_prefix_re}[{currency_units}]?{number_re})");
    let expr_or_number = format!("(?:{expr_with_anchors}|{number_with_anchors})");
    let answer_prefix_re = "(?i:answer)";
    let mut v: Vec<(String, i32)> = vec![
        (format!(r"(?i:final answer is)\:?\s*{expr_or_number}\.?\s?I hope"), 0),
        (format!(r"(?i:final answer.{{0,100}}?)\s+is\:?{expr_or_number}"), 50),
        (format!(r"{answer_prefix_re}:(?:.{{0,100}}=\s*|.{{0,50}}?){expr_or_number}(?!\s*=)"), 100),
        (format!(r"{answer_prefix_re}(?:.{{0,100}}=\s*|.{{0,50}}?){expr_or_number}(?!\s*=)"), 200),
    ];
    if cfg.try_extract_without_anchor {
        v.push((expr_with_anchors, 300));
        v.push((number_with_anchors, 300));
    }
    v.into_iter().map(|(p, pr)| make_pat(&p, false, pr)).collect()
}

fn make_latex_env_pattern(prefix: &str, boxed: bool) -> String {
    let percent = format!(r"(?P<{prefix}percent>(?:\\?%|[Pp]ercent|[Pp]ercentage|[Pp]ct))");
    let mut dd = r"(?:[^$]|\$(?!\$))".to_string();
    let mut db = r"(?:[^\\]|\\(?!\]))".to_string();
    let mut id = r"(?:\\[$]|[^\n$])".to_string();
    let mut ip = r"(?:[^\\\n]|\\(?!\)))".to_string();
    let mut ib = r"[^\n\]\[]".to_string();
    if boxed {
        dd = format!(r"{dd}*?\\boxed{{{dd}+?}}{dd}*?");
        db = format!(r"{db}*?\\boxed{{{db}+?}}{db}*?");
        id = format!(r"{id}*?\\boxed{{{id}+?}}{id}*?");
        ip = format!(r"{ip}*?\\boxed{{{ip}+?}}{ip}*?");
        ib = format!(r"{ib}*?\\boxed{{{ib}+?}}{ib}*?");
    } else {
        dd = format!("{dd}+?");
        db = format!("{db}+?");
        id = format!("{id}+?");
        ip = format!("{ip}+?");
        ib = format!("{ib}+?");
    }
    let mut pats = vec![
        format!(r"(?<!\\)\$\$(?P<{prefix}latexDisplayDollar>{dd})(?<!\\)\$\$"),
        format!(r"(?<!\\)\\\[(?P<{prefix}latexDisplayBracket>{db})(?<!\\)\\\]"),
        format!(r"(?<!\\|\d)\$(?P<{prefix}latexInlineDollar>{id})(?<!\\)\$"),
        format!(r"(?<!\\)\\\((?P<{prefix}latexInlineParenthesis>{ip})(?<!\\)\\\)"),
        format!(r"\s\[(?P<{prefix}latexInlineBracket>{ib})\]\s"),
    ];
    if !boxed {
        let simple_number = r"-?\d+(?:[.,]\d+)?";
        pats.push(format!(r"(?P<{prefix}latexFraction>-?\\frac{{{simple_number}}}{{{simple_number}}})"));
    }
    format!(r"(?:(?:{})\s*{percent}?)", pats.join("|"))
}

pub fn latex_regexes(cfg: &LatexConfig) -> Pats {
    let first = make_latex_env_pattern("first_", false);
    let next: String = (1..6)
        .map(|i| format!(r"(?:\s*(?:,?and|,?or|,)\s*{})?", make_latex_env_pattern(&format!("next{i}_"), false)))
        .collect();
    let latex_re = format!("(?:{first}{next})");
    let answer_prefix_re = "(?i:answer)";
    let mut v: Vec<(String, i32)> = vec![
        (format!(r"(?i:final answer is)\:?\s*{latex_re}\.?\s?I hope"), 0),
        (format!(r"(?i:final answer.{{0,100}}?)\s+is\:?\s*{latex_re}"), 50),
        (format!("{answer_prefix_re}:.{{0,50}}?{latex_re}"), 100),
        (format!("{answer_prefix_re}.{{0,50}}?{latex_re}"), 200),
    ];
    if cfg.try_extract_without_anchor {
        v.push((latex_re.clone(), 300));
    }
    if cfg.boxed_match_priority >= 0 {
        let fb = make_latex_env_pattern("first_", true);
        let nb: String = (1..6)
            .map(|i| format!(r"\s*(?:\s*(?:,?and|,?or|,)\s*{})?", make_latex_env_pattern(&format!("next{i}_"), true)))
            .collect();
        v.push((format!("{fb}{nb}"), cfg.boxed_match_priority));
        v.push((r"(?P<first_latexBoxed>\\boxed{.+})".to_string(), cfg.boxed_match_priority));
    }
    v.into_iter().map(|(p, pr)| make_pat(&p, true, pr)).collect()
}

static REGEX_CACHE: LazyLock<Mutex<HashMap<Target, Arc<Pats>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn regexes_for(t: &Target) -> Arc<Pats> {
    if let Some(p) = REGEX_CACHE.lock().unwrap().get(t) {
        return p.clone();
    }
    let p = Arc::new(match t {
        Target::Latex(c) => latex_regexes(c),
        Target::Expr(c) => expr_regexes(c),
        Target::Str(c) => string_regexes(c),
    });
    REGEX_CACHE.lock().unwrap().insert(t.clone(), p.clone());
    p
}

// ---------------- extraction ----------------

pub fn should_treat_as_complex(latex: &str) -> bool {
    static R: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(concat!(
            r"\\mathbb\{C\}|\\i\b|\bi\b|\\text\{i\}|\\mathrm\{i\}|\\imath\b|",
            r"\\det|\\operatorname\{tr\}|\\operatorname\{rank\}|\\text\{rank\}|\\arg\{|\\Re\{|\\Im\{|",
            r"\\operatorname\{Re\}|\\operatorname\{Im\}|\\text\{Re\}|\\text\{Im\}"
        ))
        .unwrap()
    });
    R.is_match(latex).unwrap_or(false)
}

pub fn get_last_eq(latex: &str) -> String {
    if !latex.contains(',') && !latex.contains(';') {
        let parts = split_by(&EQUATION_SPLIT, latex);
        if parts.len() > 2 {
            return parts.last().unwrap().clone();
        }
    }
    latex.to_string()
}

pub fn parse_latex(latex: &str) -> Result<Expr, String> {
    if crate::deadline::expired() {
        return Err("timeout".into());
    }
    let conv = Converter::new(!should_treat_as_complex(latex));
    match conv.parse(latex) {
        Ok(e) => Ok(e),
        Err(e) => {
            let last = get_last_eq(latex);
            if last != latex {
                Converter::new(!should_treat_as_complex(&last)).parse(&last)
            } else {
                Err(e)
            }
        }
    }
}

/// Values of named groups in definition order (like Python's groupdict()).
fn groupdict<'a>(r: &'a Regex, c: &Captures<'a>) -> Vec<(&'a str, Option<String>)> {
    r.capture_names()
        .enumerate()
        .filter_map(|(i, n)| n.map(|n| (n, c.get(i).map(|m| m.as_str().to_string()))))
        .collect()
}

fn convert_to_pct(e: Expr) -> Expr {
    pct(e)
}

fn extract_latex(r: &Regex, c: &Captures, cfg: &LatexConfig) -> (Option<Item>, String) {
    let gd = groupdict(r, c);
    let get = |name: &str| gd.iter().find(|(n, _)| *n == name).and_then(|(_, v)| v.clone());
    let pick = |prefix: &str| -> Option<(String, String)> {
        gd.iter()
            .find(|(n, v)| n.starts_with(prefix) && v.as_deref().map(|s| !s.is_empty()).unwrap_or(false))
            .map(|(n, v)| (v.clone().unwrap(), n.to_string()))
    };
    let mut all = Vec::new();
    if let Some(f) = pick("first_latex") {
        all.push(f);
    }
    for i in 1..6 {
        if let Some(f) = pick(&format!("next{i}_latex")) {
            all.push(f);
        }
    }
    let mut exprs: Vec<Option<Expr>> = Vec::new();
    let mut strs: Vec<String> = Vec::new();
    for (latex, name) in &all {
        let parts: Vec<&str> = name.split('_').collect();
        let prefix = parts[0];
        let group_name = parts.get(1).copied();
        let is_pct = get(&format!("{prefix}_percent")).map(|s| !s.is_empty()).unwrap_or(false);
        let mut ncfg = cfg.normalization;
        if group_name == Some("latexBoxed") {
            ncfg.boxed = BoxedMode::Last;
        }
        let normalized = normalize_latex(latex, &ncfg);
        strs.push(normalized.clone());
        match parse_latex(&normalized) {
            Ok(e) => exprs.push(Some(if is_pct { convert_to_pct(e) } else { e })),
            Err(_) => exprs.push(None),
        }
    }
    if exprs.is_empty() {
        return (None, String::new());
    }
    if exprs.len() > 1 && exprs.iter().all(|e| e.is_some()) {
        let mut elements = Vec::new();
        for e in exprs.into_iter().flatten() {
            match e {
                Expr::FiniteSet { args, .. } => elements.extend(args),
                other => elements.push(other),
            }
        }
        return (Some(Item::Expr(finite_set_or_empty(elements))), strs.join(" and "));
    }
    let first = exprs.into_iter().next().unwrap();
    (first.map(Item::Expr), strs.into_iter().next().unwrap())
}

fn extract_expr(r: &Regex, c: &Captures) -> (Option<Item>, String) {
    let gd = groupdict(r, c);
    let expr = gd.iter().find(|(n, _)| *n == "expr").and_then(|(_, v)| v.clone()).unwrap_or_default();
    let first_nonempty = |prefix: &str| {
        gd.iter()
            .find(|(n, v)| n.starts_with(prefix) && v.as_deref().map(|s| !s.is_empty()).unwrap_or(false))
            .and_then(|(_, v)| v.clone())
            .unwrap_or_default()
    };
    let integer = first_nonempty("integer");
    let decimal = first_nonempty("decimal");
    let is_pct = gd.iter().any(|(n, v)| *n == "percent" && v.as_deref().map(|s| !s.is_empty()).unwrap_or(false));
    if !integer.is_empty() || !decimal.is_empty() {
        let int_clean: String = integer.chars().filter(|c| *c != ',' && *c != ' ').collect();
        let int_clean = int_clean.trim_start_matches('0');
        let int_clean = if int_clean.is_empty() { "0" } else { int_clean };
        let dec = decimal.replace(',', ".");
        let number_str = format!("{int_clean}{dec}");
        return match sympy_number(&number_str) {
            Ok(n) => {
                let n = if is_pct { convert_to_pct(n) } else { n };
                (Some(Item::Expr(n)), number_str)
            }
            Err(_) => (None, number_str),
        };
    }
    if !expr.is_empty() {
        let src = expr.replace('\n', " ").replace('^', "**");
        if let Ok(e) = crate::pyexpr::parse_expr(&src) {
            return (Some(Item::Expr(e)), expr);
        }
    }
    (None, expr)
}

fn extract_string(c: &Captures, cfg: &StringConfig) -> (Option<Item>, String) {
    let s = c.name("string_keys").map(|m| m.as_str().to_string()).unwrap_or_default();
    let parsed = if cfg.lowercase { s.to_lowercase() } else { s.clone() };
    (Some(Item::Str(parsed)), s)
}

pub fn extract_target_from_pred(
    pred: &str,
    targets: &[Target],
    fallback_mode: FallbackMode,
    extraction_mode: ExtractionMode,
) -> Vec<Item> {
    let mut extracted = Vec::new();
    let mut fallbacks: Vec<String> = Vec::new();

    let regs: Vec<(Arc<Pats>, &Target)> = targets.iter().map(|t| (regexes_for(t), t)).collect();
    let mut all: Vec<(&Pat, &Target, i32)> = Vec::new();
    for (pats, t) in &regs {
        for p in pats.iter() {
            all.push((p, t, p.prio));
        }
    }
    all.sort_by_key(|x| x.2); // stable, like Python's sorted
    let mut match_found = false;
    let mut i = 0;
    while i < all.len() {
        let pr = all[i].2;
        let mut j = i;
        while j < all.len() && all[j].2 == pr {
            j += 1;
        }
        let group = &all[i..j];
        i = j;

        let mut matches: Vec<(Captures, usize, usize, &Regex, &Target)> = Vec::new();
        for (p, t, _) in group {
            for c in p.find_all(pred) {
                let m = c.get(0).unwrap();
                let (s, e) = (m.start(), m.end());
                matches.push((c, s, e, &p.re, t));
            }
        }
        // sorted(key=(end, -start), reverse=True), stable
        matches.sort_by(|a, b| (b.2, a.1).cmp(&(a.2, b.1)));

        for (c, _, _, r, t) in &matches {
            if crate::deadline::expired() {
                return Vec::new();
            }
            let (item, fb) = match t {
                Target::Latex(cfg) => extract_latex(r, c, cfg),
                Target::Expr(_) => extract_expr(r, c),
                Target::Str(cfg) => extract_string(c, cfg),
            };
            match_found = true;
            if !fb.is_empty() {
                fallbacks.push(fb);
            }
            if let Some(it) = item {
                extracted.push(it);
                break;
            }
            if extraction_mode == ExtractionMode::FirstMatch {
                break;
            }
        }
        if !extracted.is_empty() || (match_found && extraction_mode == ExtractionMode::FirstMatch) {
            break;
        }
    }
    if fallback_mode == FallbackMode::FirstMatch {
        if let Some(f) = fallbacks.into_iter().next() {
            extracted.push(Item::Str(f));
        }
    }
    extracted
}

pub fn default_targets() -> Vec<Target> {
    vec![Target::Latex(LatexConfig::default()), Target::Expr(ExprConfig::default())]
}

pub fn parse(pred: &str, targets: &[Target], fallback: FallbackMode, mode: ExtractionMode) -> Vec<Item> {
    extract_target_from_pred(pred, targets, fallback, mode)
}

/// `parse` with upstream's `parsing_timeout` semantics: on timeout the result is [].
pub fn parse_with_timeout(
    pred: &str,
    targets: &[Target],
    fallback: FallbackMode,
    mode: ExtractionMode,
    timeout: Option<f64>,
) -> Vec<Item> {
    let (items, hit) = crate::deadline::run(timeout, || extract_target_from_pred(pred, targets, fallback, mode));
    if hit { Vec::new() } else { items }
}
