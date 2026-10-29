use std::io::BufRead;
use std::time::Instant;
use fastmathverify::extract::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: usize = a[2].parse().unwrap();
    let preds: Vec<String> = std::io::BufReader::new(std::fs::File::open(&a[1]).unwrap())
        .lines().take(n).map(|l| { let v: serde_json::Value = serde_json::from_str(&l.unwrap()).unwrap(); v["pred"].as_str().unwrap().to_string() }).collect();
    for (name, pats) in [("latex", latex_regexes(&LatexConfig::default())), ("expr", expr_regexes(&ExprConfig::default()))] {
        for (i, pat) in pats.iter().enumerate() { let pr = pat.prio;
            let t = Instant::now();
            let mut cnt = 0;
            for p in &preds {
                cnt += pat.find_all(p).len();
            }
            println!("{name}[{i}] prio={pr} {:.3} ms/text matches={cnt}", t.elapsed().as_secs_f64() * 1e3 / n as f64);
        }
    }
}
