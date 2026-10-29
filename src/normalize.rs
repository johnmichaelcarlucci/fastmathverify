//! Port of latex2sympy2_extended.math_normalization.normalize_latex.

use std::sync::LazyLock;

use fancy_regex::{Captures, Regex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoxedMode {
    All,
    Last,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NormalizationConfig {
    pub basic_latex: bool,
    pub units: bool,
    pub malformed_operators: bool,
    pub nits: bool,
    pub boxed: BoxedMode,
    pub equations: bool,
}

impl Default for NormalizationConfig {
    fn default() -> Self {
        NormalizationConfig {
            basic_latex: true,
            units: false,
            malformed_operators: false,
            nits: false,
            boxed: BoxedMode::All,
            equations: false,
        }
    }
}

fn re(p: &str) -> Regex {
    let t = py_escapes(p);
    Regex::new(&t).expect(p)
}

/// Python `re` treats `\` + any ASCII punctuation as a literal. Rust's regex
/// gives some of those meaning (`\<`, `\>` are word boundaries), so drop the
/// backslash unless the character is a Rust regex metacharacter.
pub fn py_escapes(p: &str) -> String {
    const META: &str = "\\.+*?()|[]{}^$#&-~";
    let mut out = String::with_capacity(p.len());
    let mut it = p.chars();
    while let Some(c) = it.next() {
        if c == '\\' {
            match it.next() {
                Some(n) if n.is_ascii_punctuation() && !META.contains(n) => out.push(n),
                Some(n) if !n.is_ascii() => out.push(n),
                Some(n) => {
                    out.push('\\');
                    out.push(n);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Python `pattern.sub(f, text)` over all non-overlapping matches.
pub fn sub_with(r: &Regex, text: &str, mut f: impl FnMut(&Captures) -> String) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    let mut pos = 0;
    while pos <= text.len() {
        let caps = match r.captures_from_pos(text, pos) {
            Ok(Some(c)) => c,
            _ => break,
        };
        let m = caps.get(0).unwrap();
        out.push_str(&text[last..m.start()]);
        out.push_str(&f(&caps));
        last = m.end();
        if m.end() == m.start() {
            // empty match: advance one char
            match text[m.end()..].chars().next() {
                Some(c) => {
                    out.push(c);
                    last = m.end() + c.len_utf8();
                    pos = last;
                }
                None => break,
            }
        } else {
            pos = m.end();
        }
    }
    out.push_str(&text[last.min(text.len())..]);
    out
}

fn g<'a>(c: &'a Captures, i: usize) -> &'a str {
    c.get(i).map(|m| m.as_str()).unwrap_or("")
}

static R_LEFT: LazyLock<Regex> = LazyLock::new(|| re(r"\\m?left(\\\{|\{|\\\||\||\[|\(|\\rbracl|\\lgroup|\\lbrace|\\lbrack|\\vert|\\lvert|\\lceil|\\lfloor|\\vert|\\lvert|\\langle|\\llcorner|\\ulcorner)"));
static R_RIGHT: LazyLock<Regex> = LazyLock::new(|| re(r"\\m?right(\\\}|\}|\\\||\||\]|\)|\\rbrack|\\rgroup|\\rbrace|\\rbrack|\\vert|\\rvert|\\rceil|\\rfloor|\\vert|\\rvert|\\rangle|\\lrcorner|\\urcorner)"));

const UNITS: &[&str] = &[
    "integerpoint", "feet", "sue", "digit", "pound", "meal", "edge", "student", "children ticket",
    "multiple", "east", "degree", "mph", "kmph", "ft", "m square", " m east", "sq m", "deg", "mile",
    "q .", "monkey", "prime", "ratio", "profit of rs", "rd", "o", "gm", "p . m", "lb", "tile", "per",
    "dm", "lt", "gain", "ab", "way", "west", "a .", "b .", "c .", "d .", "e .", "f .", "g .", "h .",
    "t", "h", "no change", "men", "soldier", "pie", "bc", "excess", "st", "inches", "noon", "cent",
    "by", "gal", "kmh", "c", "acre", "rise", "a . m", "th", "π r 2", "sq", "mark", "l", "toy", "coin",
    "sq . m", "gallon", "° f", "profit", "minw", "yr", "women", "am", "pm", "hr", "cu cm", "square",
    "v â € ™", "are", "rupee", "rounds", "cubic", "cc", "mtr", "s", "ohm", "number", "kmph", "day",
    "hour", "minute", "min", "second", "man", "woman", "sec", "cube", "mt", "sq inch", "mp",
    "∏ cm ³", "hectare", "more", "sec", "unit", "cu . m", "cm 2", "rs .", "rs", "kg", "g", "month",
    "km", "m", "cm", "mm", "apple", "liter", "loss", "yard", "pure", "year", "increase", "decrease",
    "d", "less", "Surface", "litre", "pi sq m", "s .", "metre", "meter", "inch",
];

static UNITS_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Units are joined verbatim (unescaped), exactly like upstream.
    let pat = format!(r"(\d|\}}|\s)\s*(?:(?:{})(?:s|es)?)\s*$", UNITS.join("|"));
    re(&pat)
});

static TO_REMOVE: LazyLock<Regex> = LazyLock::new(|| {
    re(concat!(
        r"\\mathrm\{th\}|",
        r"\\!\s*|",
        r"\\text\s*\{\s*\}|",
        r"\\text\s*\{\s*\}|",
        r"\\\$|\$|",
        r#"(?<!\\)["']|"#,
        r"\\displaystyle"
    ))
});

const REPLACE: &[(&str, &str, &str)] = &[
    ("math", r"\\math(?:rm|it|bf)", r"\text"),
    ("text", r"\\text(?:normal|bf|it|rm)", r"\text"),
    ("frac", r"\\(?:d|t|c)frac", r"\frac"),
    ("decimal_space", r"\s\.", r" 0."),
    ("decimal_brace", r"\{\.", r"{0."),
    ("approx", r"\~\=", r"\approx"),
    ("comma", r"\s*\{\s*,\s*\}", r","),
    ("and_or", r"(?<![a-zA-Z])(,?\s*(?:and|or))(?![a-zA-Z])", r","),
    ("and_or_text", r"(,?\s*\\text{\s*(?:and|or)\s*})", r","),
    ("backslash_space", r"(?<!\\)\\\s", r" "),
    ("infinity", r"infinity", r"\infty"),
    ("dot", r",?(\\ldots)", r" "),
    ("percentage", r"\s*\bpercentage\b", r"\%"),
    ("percentage_in_text", r"\\text{percentage}", r"%"),
    ("percent", r"\s*\bpercent\b", r"\%"),
    ("percent_in_text", r"\\text{percent}", r"%"),
    ("pct", r"\s*\bpct\b", r"\%"),
    ("pct_in_text", r"\\text{pct}", r"%"),
    ("inf", r"((?<!\\)inf(?!inity))", r"\infty"),
    ("sqrt", r" sqrt", r"\sqrt"),
];

static REPLACE_RE: LazyLock<Regex> = LazyLock::new(|| {
    let pat: Vec<String> = REPLACE.iter().map(|(n, p, _)| format!("(?P<{n}>{p})")).collect();
    re(&pat.join("|"))
});

static COMMAND_SLASH_FIX: LazyLock<Regex> = LazyLock::new(|| re(r"\\\\(?=[a-zA-Z])"));
static PERMUTATION: LazyLock<Regex> =
    LazyLock::new(|| re(r"\(([a-zA-Z0-9+\-*/\\ ]+?)\)_{([a-zA-Z0-9+\-*/\\ ]+?)}"));
pub static EQUATION_SPLIT: LazyLock<Regex> = LazyLock::new(|| re(r"(?<!\\|\<|\!|\>)="));
static UNIT_SUPERSCRIPT: LazyLock<Regex> = LazyLock::new(|| re(r"(\\(?:text|mbox){.*?})(\^\d|\{\^\d\})?$"));

static MALFORMED: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    vec![
        (re(r"\^\s?\((.*?)\)"), "^{$1}"),
        (re(r"sqrt\s?\((.*?)\)"), "\\sqrt{$1}"),
        (re(r"\\frac\s?(\d)\s?(\d+)"), "\\frac{$1}{$2}"),
        (re(r"\\log_\s?(\d)\s?(\d+)"), "\\log_{$1}{$2}"),
        (re(r"\\frac\s?{(.*?)}\s?(\d)"), "\\frac{$1}{$2}"),
        (re(r"\\frac\s?(\d)\s?{(.*?)}"), "\\frac{$1}{$2}"),
        (re(r"\\sqrt\s?(\d)"), "\\sqrt{$1}"),
    ]
});

static VALID_SEPARATOR: LazyLock<Regex> = LazyLock::new(|| re(r"\b(and|or)\b|,|;"));

fn template(tpl: &str, c: &Captures) -> String {
    let mut out = String::new();
    let b = tpl.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'$' && i + 1 < b.len() && b[i + 1].is_ascii_digit() {
            out.push_str(g(c, (b[i + 1] - b'0') as usize));
            i += 2;
        } else {
            let ch = tpl[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

fn fix_malformed_operators(text: &str) -> String {
    let mut s = text.to_string();
    for (r, tpl) in MALFORMED.iter() {
        s = sub_with(r, &s, |c| template(tpl, c));
    }
    s.replace(" sqrt", "\\sqrt")
}

fn replace_in_latex(text: &str) -> String {
    sub_with(&REPLACE_RE, text, |c| {
        for (name, _, rep) in REPLACE {
            if c.name(name).is_some() {
                return rep.to_string();
            }
        }
        String::new()
    })
}

fn chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn s_of(v: &[char]) -> String {
    v.iter().collect()
}

/// Python str.rfind(sub, 0, end) over code points.
fn rfind(t: &[char], sub: &[char], end: usize) -> Option<usize> {
    if sub.len() > end {
        return None;
    }
    (0..=end - sub.len()).rev().find(|&i| t[i..i + sub.len()] == *sub)
}

pub fn extract_boxed_content(text: &str, mode: BoxedMode) -> String {
    let t = chars(text);
    let boxed: Vec<char> = "\\boxed".chars().collect();
    let fbox: Vec<char> = "\\fbox".chars().collect();
    let mut results: Vec<String> = Vec::new();
    let mut current_pos = t.len();
    let mut last_boxed_start: Option<usize> = None;
    let mut max_pos = t.len();

    let find_bounds = |open: usize, max_pos: usize| -> Option<(usize, usize)> {
        let mut i = open;
        let mut depth = 1i64;
        while i + 1 < max_pos {
            i += 1;
            if t[i] == '{' {
                depth += 1;
            } else if t[i] == '}' {
                depth -= 1;
                if depth == 0 {
                    return Some((open, i));
                }
            }
        }
        None
    };

    loop {
        let bi = rfind(&t, &boxed, current_pos);
        let fi = rfind(&t, &fbox, current_pos);
        if bi.is_none() && fi.is_none() {
            break;
        }
        let bi_v = bi.map(|x| x as i64).unwrap_or(-1);
        let fi_v = fi.map(|x| x as i64).unwrap_or(-1);
        let start_idx = bi_v.max(fi_v) as usize;
        let command_end = start_idx + if bi_v > fi_v { 6 } else { 5 };
        let mut next = command_end;
        while next < max_pos && t[next].is_whitespace() {
            next += 1;
        }
        if next >= max_pos {
            break;
        }
        if t[next] == '{' {
            match find_bounds(next, max_pos) {
                None => {
                    if results.is_empty() {
                        results.push(s_of(&t[next..]));
                    }
                    break;
                }
                Some((cs, ce)) => {
                    let content = s_of(&t[cs + 1..ce]).trim().to_string();
                    if mode == BoxedMode::Last {
                        if let Some(lbs) = last_boxed_start {
                            let prev = results.last().cloned().unwrap_or_default();
                            let between: String = if ce + 1 <= lbs { s_of(&t[ce + 1..lbs]) } else { String::new() };
                            let ok = between.chars().count() < 10
                                || (prev != content
                                    && between.chars().count() < 70
                                    && VALID_SEPARATOR.is_match(&between).unwrap_or(false));
                            if !ok {
                                break;
                            }
                        }
                    }
                    if results.last() != Some(&content) {
                        results.push(content);
                    }
                    last_boxed_start = Some(start_idx);
                    max_pos = start_idx;
                }
            }
        } else {
            if results.is_empty() {
                results.push(s_of(&t[next..]));
            }
            break;
        }
        current_pos = start_idx;
    }
    if results.is_empty() {
        return text.to_string();
    }
    results.reverse();
    results.join(",")
}

fn fix_fracs(text: &str) -> String {
    let parts: Vec<&str> = text.split("\\frac").collect();
    let mut new_str = parts[0].to_string();
    for sub in &parts[1..] {
        let sub = sub.trim_start();
        let cs = chars(sub);
        new_str.push_str("\\frac");
        if !cs.is_empty() && cs[0] == '{' {
            new_str.push_str(sub);
        } else if cs.len() < 2 {
            return text.to_string();
        } else {
            let a = cs[0];
            let b = cs[1];
            let post = s_of(&cs[2..]);
            if b != '{' {
                new_str.push_str(&format!("{{{a}}}{{{b}}}{post}"));
            } else {
                new_str.push_str(&format!("{{{a}}}{b}{post}"));
            }
        }
    }
    new_str
}

fn canonical_int(s: &str) -> bool {
    let d = s.strip_prefix('-').unwrap_or(s);
    if d.is_empty() || !d.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if d == "0" {
        return !s.starts_with('-');
    }
    !d.starts_with('0')
}

fn fix_a_slash_b(text: &str) -> String {
    let parts: Vec<&str> = text.split('/').collect();
    if parts.len() != 2 {
        return text.to_string();
    }
    if canonical_int(parts[0]) && canonical_int(parts[1]) {
        return format!("\\frac{{{}}}{{{}}}", parts[0], parts[1]);
    }
    text.to_string()
}

fn fix_sqrt(text: &str) -> String {
    if !text.contains("\\sqrt") {
        return text.to_string();
    }
    let parts: Vec<&str> = text.split("\\sqrt").collect();
    let mut s = parts[0].to_string();
    for sp in &parts[1..] {
        let sp = sp.trim_start();
        let cs = chars(sp);
        if !cs.is_empty() && cs[0] != '{' && cs[0] != '[' {
            s.push_str(&format!("\\sqrt{{{}}}{}", cs[0], s_of(&cs[1..])));
        } else {
            s.push_str("\\sqrt");
            s.push_str(sp);
        }
    }
    s
}

/// Python str.strip() whitespace set (Unicode whitespace).
pub fn py_strip(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_whitespace() || c == '\u{1c}' || c == '\u{1d}' || c == '\u{1e}' || c == '\u{1f}')
}

pub fn normalize_latex(text: &str, config: &NormalizationConfig) -> String {
    let mut text = text.to_string();
    if config.boxed != BoxedMode::None {
        text = extract_boxed_content(&text, config.boxed);
    }
    if config.basic_latex {
        text = text.replace("\\mathrm{T}", "T");
        text = text.replace("\\mathrm{d}", "d").replace("{\\rm d}", "d");
        text = text
            .replace("\\left[\\begin{matrix}", "\\begin{bmatrix}")
            .replace("\\end{matrix}\\right]", "\\end{bmatrix}");
        text = sub_with(&R_LEFT, &text, |c| g(c, 1).to_string());
        text = sub_with(&R_RIGHT, &text, |c| g(c, 1).to_string());
        text = sub_with(&PERMUTATION, &text, |c| {
            format!("\\frac{{({})!}}{{(({})-({}))!}}", g(c, 1), g(c, 1), g(c, 2))
        });
        text = sub_with(&TO_REMOVE, &text, |_| String::new());
        text = replace_in_latex(&text);
        text = text.replace('\n', " ").replace('\t', " ");
        if !text.contains("matrix") {
            text = sub_with(&COMMAND_SLASH_FIX, &text, |_| "\\".to_string());
        }
    }
    if config.equations && !text.contains(',') && !text.contains(';') {
        let parts = split_by(&EQUATION_SPLIT, &text);
        if parts.len() > 2 {
            text = parts.last().unwrap().clone();
        }
    }
    if config.units {
        let t2 = sub_with(&UNIT_SUPERSCRIPT, &text, |_| String::new());
        let t2 = py_strip(&t2).to_string();
        if !t2.is_empty() && t2 != text {
            text = t2;
        }
        for _ in 0..2 {
            let t2 = sub_with(&UNITS_RE, &text, |c| g(c, 1).to_string());
            if !t2.is_empty() && t2 != text {
                text = t2;
            }
        }
    }
    if config.nits {
        if text.starts_with('.') {
            text = format!("0{text}");
        }
        if text == "0.5" {
            text = "\\frac{1}{2}".to_string();
        }
    }
    if config.malformed_operators {
        text = fix_malformed_operators(&text);
        text = fix_sqrt(&text);
        text = fix_fracs(&text);
        text = fix_a_slash_b(&text);
    }
    py_strip(&text).to_string()
}

/// Python `regex.split(text)` (no capture groups in pattern).
pub fn split_by(r: &Regex, text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut last = 0;
    let mut pos = 0;
    while pos <= text.len() {
        let m = match r.find_from_pos(text, pos) {
            Ok(Some(m)) => m,
            _ => break,
        };
        if m.start() == m.end() {
            pos = m.end() + text[m.end()..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            continue;
        }
        out.push(text[last..m.start()].to_string());
        last = m.end();
        pos = m.end();
    }
    out.push(text[last..].to_string());
    out
}
