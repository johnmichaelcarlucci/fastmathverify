use std::io::BufRead;
fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        match fastmathverify::tree::parse_math(&line) {
            Ok(p) => println!("{}", p.root.dump()),
            Err(_) => println!("ERR"),
        }
    }
}
