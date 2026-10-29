//! Reads JSONL {id, gold, pred}; writes JSONL {id, verify, gold_parsed, pred_parsed, t}.
//! Usage: harness IN OUT [threads]
use std::io::{BufRead, Write};
use std::time::Instant;

use fastmathverify::{grader, item_str, parse_default, VerifyOpts};

fn run(line: &str) -> String {
    let d: serde_json::Value = serde_json::from_str(line).unwrap();
    let t0 = Instant::now();
    let g = parse_default(d["gold"].as_str().unwrap());
    let p = parse_default(d["pred"].as_str().unwrap());
    let ok = grader::verify(&g, &p, &VerifyOpts::default());
    let t = t0.elapsed().as_secs_f64();
    serde_json::json!({
        "id": d["id"], "verify": ok,
        "gold_parsed": g.iter().map(item_str).collect::<Vec<_>>(),
        "pred_parsed": p.iter().map(item_str).collect::<Vec<_>>(),
        "t": t,
    })
    .to_string()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let lines: Vec<String> = std::io::BufReader::new(std::fs::File::open(&a[1]).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let threads: usize = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
    let t0 = Instant::now();
    let mut out: Vec<String> = vec![String::new(); lines.len()];
    std::thread::scope(|s| {
        let chunk = lines.len().div_ceil(threads);
        for (li, oi) in lines.chunks(chunk.max(1)).zip(out.chunks_mut(chunk.max(1))) {
            s.spawn(move || {
                for (l, o) in li.iter().zip(oi.iter_mut()) {
                    *o = run(l);
                }
            });
        }
    });
    let wall = t0.elapsed().as_secs_f64();
    let mut f = std::io::BufWriter::new(std::fs::File::create(&a[2]).unwrap());
    for o in &out {
        writeln!(f, "{o}").unwrap();
    }
    eprintln!("done {} pairs in {:.2}s wall ({} threads)", lines.len(), wall, threads);
}
