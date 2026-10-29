//! dbg latex <s> | dbg text <s>
use fastmathverify::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a[1].as_str() {
        "latex" => {
            match tree::parse_math(&a[2]) { Ok(p) => println!("tree: {}", p.root.dump()), Err(e) => println!("tree ERR {e}") }
            match extract::parse_latex(&a[2]) { Ok(e) => println!("expr: {e}  {:?}", e), Err(e) => println!("convert ERR {e}") }
        }
        "cmp" => {
            let (g, t) = (parse_default(&a[2]), parse_default(&a[3]));
            for x in &g {
                for y in &t {
                    println!("{:?} vs {:?} -> {}", item_str(x), item_str(y).chars().take(60).collect::<String>(), grader::compare_items(x, y, &VerifyOpts::default()));
                }
            }
        }
        _ => {
            for it in parse_default(&a[2]) { println!("{:?}", it); }
        }
    }
}
