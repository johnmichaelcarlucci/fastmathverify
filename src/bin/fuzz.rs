//! Mutation fuzzer: mangles real answers with LaTeX fragments and checks that parse/verify
//! never panic and always return within the deadline.
//! Usage: fuzz CORPUS.jsonl ITERATIONS [threads] [seed]
use std::io::BufRead;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

use fastmathverify::{extract, grader, default_targets, ExtractionMode, FallbackMode, VerifyOpts};

const FRAGS: &[&str] = &[
    "\\frac{", "}", "{", "^", "_", "\\sqrt[", "]", "\\left(", "\\right)", "\\boxed{", "$", "\\(", "\\)",
    "\\infty", "\\pi", "!", "\\binom{", "\\int", "\\sum_{", "\\lim_{", "\\to", "=", "<", "\\le", ",",
    "\\cup", "\\{", "\\}", "[", "(", ")", "|", "\\%", "\\circ", "9999999999", "^{99}", "e^{", "\\log_",
    "\\sin", "\\cdot", "\\times", "x", "-", "+", "0", "0.", "1e308", "\\begin{pmatrix}", "\\end{pmatrix}",
    "&", "\\\\", "\\text{", "\\mathbf{", "\\pm", "\\mp", "\\neq", "\\in", "\\emptyset", "i", "\\overline{",
    "\\dot{", "'", "''", "\\prime", " ", "\\,", "\\!", "\\;", "\\quad",
];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn mutate(s: &str, r: &mut Rng) -> String {
    let mut c: Vec<char> = s.chars().collect();
    for _ in 0..1 + r.below(6) {
        let pos = if c.is_empty() { 0 } else { r.below(c.len() + 1) };
        match r.below(4) {
            0 if !c.is_empty() && pos < c.len() => {
                let end = (pos + 1 + r.below(4)).min(c.len());
                c.drain(pos..end);
            }
            1 if !c.is_empty() => {
                // duplicate a slice (builds deep nesting / long numbers)
                let a = r.below(c.len());
                let b = (a + 1 + r.below(30)).min(c.len());
                let dup: Vec<char> = c[a..b].to_vec();
                for _ in 0..1 + r.below(8) {
                    c.splice(pos.min(c.len())..pos.min(c.len()), dup.iter().cloned());
                }
            }
            _ => {
                let f = FRAGS[r.below(FRAGS.len())];
                c.splice(pos..pos, f.chars());
            }
        }
    }
    c.into_iter().collect()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let corpus: Vec<String> = std::io::BufReader::new(std::fs::File::open(&a[1]).unwrap())
        .lines()
        .flat_map(|l| {
            let d: serde_json::Value = serde_json::from_str(&l.unwrap()).unwrap();
            let p = d["pred"].as_str().unwrap();
            let tail: String = p.chars().rev().take(300).collect::<Vec<_>>().into_iter().rev().collect();
            [d["gold"].as_str().unwrap().to_string(), tail]
        })
        .collect();
    let iters: usize = a[2].parse().unwrap();
    let threads: usize = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
    let seed: u64 = a.get(4).and_then(|s| s.parse().ok()).unwrap_or(0x9e3779b97f4a7c15);
    let slow: f64 = a.get(5).and_then(|s| s.parse().ok()).unwrap_or(16.0);
    std::panic::set_hook(Box::new(|_| {}));
    let results: Vec<(usize, f64, Vec<String>)> = std::thread::scope(|s| {
        (0..threads)
            .map(|t| {
                let corpus = &corpus;
                s.spawn(move || {
                    let mut r = Rng(seed ^ (t as u64 + 1).wrapping_mul(0x2545F4914F6CDD1D));
                    let (mut panics, mut worst, mut bad) = (0, 0f64, vec![]);
                    for _ in 0..iters / threads {
                        let g = mutate(&corpus[r.below(corpus.len())], &mut r);
                        let p = mutate(&corpus[r.below(corpus.len())], &mut r);
                        let t0 = Instant::now();
                        let res = catch_unwind(AssertUnwindSafe(|| {
                            let tg = default_targets();
                            let gi = extract::parse_with_timeout(&g, &tg, FallbackMode::FirstMatch, ExtractionMode::AnyMatch, Some(5.0));
                            let pi = extract::parse_with_timeout(&p, &tg, FallbackMode::FirstMatch, ExtractionMode::AnyMatch, Some(5.0));
                            grader::verify_with_timeout(&gi, &pi, &VerifyOpts::default(), Some(5.0))
                        }));
                        let dt = t0.elapsed().as_secs_f64();
                        if dt > worst {
                            worst = dt;
                        }
                        if res.is_err() || dt > slow {
                            panics += res.is_err() as usize;
                            bad.push(serde_json::json!({"gold": g, "pred": p, "panic": res.is_err(), "secs": dt}).to_string());
                        }
                    }
                    (panics, worst, bad)
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect()
    });
    let panics: usize = results.iter().map(|r| r.0).sum();
    let worst = results.iter().map(|r| r.1).fold(0.0, f64::max);
    for (_, _, bad) in &results {
        for b in bad {
            println!("{b}");
        }
    }
    eprintln!("iterations={iters} panics={panics} slowest={worst:.3}s");
}
