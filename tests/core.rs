//! Rust-side tests (`cargo test`). The Python tests live in tests/test_fastmathverify.py and the
//! regression corpus in tests/data/regression.jsonl (verdicts recorded from upstream Math-Verify).
use std::time::{Duration, Instant};

use fastmathverify::{grader, item_str, parse_default, VerifyOpts};

fn v(gold: &str, pred: &str) -> bool {
    grader::verify(&parse_default(gold), &parse_default(pred), &VerifyOpts::default())
}

#[test]
fn basic_equivalences() {
    for (g, p) in [
        ("$\\frac{1}{2}$", "\\boxed{0.5}"),
        ("$\\sqrt{8}$", "The answer is $2\\sqrt{2}$."),
        ("$x^2+2x+1$", "$\\boxed{(x+1)^2}$"),
        ("$\\{1,2,3\\}$", "$\\boxed{\\{3,2,1\\}}$"),
        ("$(1, 2]$", "$\\boxed{(1,2]}$"),
        ("$9\\%$", "$\\boxed{9}$"),
        ("$x=3$", "$\\boxed{3}$"),
    ] {
        assert!(v(g, p), "{g} vs {p}");
    }
}

#[test]
fn rejects_wrong_answers() {
    for (g, p) in [
        ("$\\frac{1}{2}$", "\\boxed{\\frac{1}{3}}"),
        ("$2\\sqrt{2}$", "$\\boxed{2\\sqrt{3}}$"),
        ("$(1, 2]$", "$\\boxed{[1,2]}$"),
        ("$\\{1,2,3\\}$", "$\\boxed{\\{1,2\\}}$"),
        ("$x^2+2x+1$", "$\\boxed{(x-1)^2}$"),
        ("$0.333$", "$\\boxed{0.334}$"),
    ] {
        assert!(!v(g, p), "{g} vs {p}");
    }
}

#[test]
fn multiple_boxes_become_a_set_like_upstream() {
    let p = parse_default("First I thought \\boxed{3}, but actually the answer is \\boxed{4}.");
    assert_eq!(p.iter().map(item_str).collect::<Vec<_>>(), ["{3, 4}", "3,4"]);
}

#[test]
fn empty_and_garbage_inputs_dont_panic() {
    for s in ["", "$", "$$", "\\boxed{", "\\frac{", "}}}}{{{{", "\\left(\\right", "^^^", "\\sqrt[", "\u{0}\u{1}"] {
        let _ = v(s, s);
    }
}

#[test]
fn pathological_inputs_finish_quickly() {
    let big = format!("$\\boxed{{{}}}$", "9".repeat(5000));
    let tower = format!("$\\boxed{{{}2}}$", "2^{".repeat(60) + &"}".repeat(59));
    let fact = "$\\boxed{(10^{10})!}$".to_string();
    let nested = format!("$\\boxed{{{}x{}}}$", "\\frac{1}{".repeat(300), "}".repeat(300));
    // long syntax error: used to take seconds in ANTLR error recovery before bailing on first error
    let garbage = format!("${}$", ")n\\int".to_string() + &"oneofthegivenansiscorren".repeat(12));
    for s in [big, tower, fact, nested, garbage] {
        let t = Instant::now();
        let ok = grader::verify_with_timeout(&parse_default(&s), &parse_default(&s), &VerifyOpts::default(), Some(5.0));
        let _ = ok;
        assert!(t.elapsed() < Duration::from_secs(3), "took {:?}", t.elapsed());
    }
}

#[test]
fn threads_agree_with_serial() {
    let cases: Vec<(String, String)> = (1..200)
        .map(|i| (format!("$\\frac{{{i}}}{{7}}$"), format!("\\boxed{{{}}}", i as f64 / 7.0)))
        .collect();
    let serial: Vec<bool> = cases.iter().map(|(g, p)| v(g, p)).collect();
    let par: Vec<bool> = std::thread::scope(|s| {
        let hs: Vec<_> = cases.iter().map(|(g, p)| s.spawn(move || v(g, p))).collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    });
    assert_eq!(serial, par);
}

#[test]
fn regression_corpus_matches_upstream() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/regression.jsonl");
    let text = std::fs::read_to_string(path).unwrap();
    let mut bad = vec![];
    let mut n = 0;
    for line in text.lines() {
        let d: serde_json::Value = serde_json::from_str(line).unwrap();
        let (g, p) = (d["gold"].as_str().unwrap(), d["pred"].as_str().unwrap());
        n += 1;
        if v(g, p) != d["verify"].as_bool().unwrap() {
            bad.push(d["id"].as_str().unwrap().to_string());
        }
    }
    assert!(n > 100);
    assert!(bad.is_empty(), "{} / {n} disagree with upstream: {bad:?}", bad.len());
}

#[test]
fn exact_mode() {
    let ex = VerifyOpts { exact: true, ..VerifyOpts::default() };
    let e = |g: &str, p: &str| grader::verify(&parse_default(g), &parse_default(p), &ex);
    assert!(v(r"$9\%$", r"$\boxed{9}$") && !e(r"$9\%$", r"$\boxed{9}$"));
    assert!(e(r"$9\%$", r"$\boxed{0.09}$"));
    assert!(v(r"$\frac{1}{3}$", r"$\boxed{0.3333333}$") && !e(r"$\frac{1}{3}$", r"$\boxed{0.3333333}$"));
    assert!(e(r"$\frac{1}{2}$", r"$\boxed{0.5}$"));
    assert!(e(r"$2\sqrt{2}$", r"$\boxed{\sqrt{8}}$"));
}

#[test]
fn set_elements_match_in_any_order() {
    assert!(v(r"$\{n^2+1, \binom{n}{2}\}$", r"$\boxed{\{\frac{n(n-1)}{2}, n^2+1\}}$"));
    assert!(!v(r"$\{n^2+1, \binom{n}{2}\}$", r"$\boxed{\{\frac{n(n+1)}{2}, n^2+1\}}$"));
}

#[test]
fn sums_with_symbolic_bound() {
    assert!(v(r"$a_n = n! \sum_{k=2}^{n} \frac{(-1)^k}{k!}$", r"$\boxed{n! \sum_{k=0}^{n} \frac{(-1)^k}{k!}}$"));
    assert!(!v(r"$\sum_{k=1}^{n} k$", r"$\boxed{\sum_{k=0}^{n} (k+1)}$"));
}
