//! Stage timing on a JSONL sample: regex matching, normalization, ANTLR parsing.
use std::io::BufRead;
use std::time::Instant;
use fastmathverify::*;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: usize = a[2].parse().unwrap();
    let lines: Vec<serde_json::Value> = std::io::BufReader::new(std::fs::File::open(&a[1]).unwrap())
        .lines().take(n).map(|l| serde_json::from_str(&l.unwrap()).unwrap()).collect();
    let preds: Vec<String> = lines.iter().map(|d| d["pred"].as_str().unwrap().to_string()).collect();
    let golds: Vec<String> = lines.iter().map(|d| d["gold"].as_str().unwrap().to_string()).collect();

    let t = Instant::now();
    for p in &preds { let _ = parse_default(p); }
    println!("full parse(pred): {:.3} ms/item", t.elapsed().as_secs_f64() * 1e3 / n as f64);

    let t = Instant::now();
    for g in &golds { let _ = parse_default(g); }
    println!("full parse(gold): {:.3} ms/item", t.elapsed().as_secs_f64() * 1e3 / n as f64);

    let latex = extract::Target::Latex(extract::LatexConfig::default());
    let t = Instant::now();
    for p in &preds { let _ = extract::parse(p, &[latex.clone()], FallbackMode::FirstMatch, ExtractionMode::AnyMatch); }
    println!("latex-only parse(pred): {:.3} ms/item", t.elapsed().as_secs_f64() * 1e3 / n as f64);

    let ex = extract::Target::Expr(extract::ExprConfig::default());
    let t = Instant::now();
    for p in &preds { let _ = extract::parse(p, &[ex.clone()], FallbackMode::FirstMatch, ExtractionMode::AnyMatch); }
    println!("expr-only parse(pred): {:.3} ms/item", t.elapsed().as_secs_f64() * 1e3 / n as f64);

    let samples = ["\\frac{1}{2}", "x^2+2x+1", "(x+1)^2", "[1,2)", "3\\sqrt{2}", "\\{1,2,3\\}"];
    let t = Instant::now();
    for _ in 0..200 { for s in &samples { let _ = extract::parse_latex(s); } }
    println!("antlr parse+convert: {:.3} ms/expr", t.elapsed().as_secs_f64() * 1e3 / (200.0 * samples.len() as f64));

    let t = Instant::now();
    for _ in 0..200 { for s in &samples { let _ = tree::parse_math(s); } }
    println!("antlr parse only: {:.3} ms/expr", t.elapsed().as_secs_f64() * 1e3 / (200.0 * samples.len() as f64));
}
