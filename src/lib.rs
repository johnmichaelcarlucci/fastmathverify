pub mod convert;
pub mod deadline;
pub mod diff;
pub mod eval;
pub mod expr;
pub mod functions;
pub mod gen;
pub mod matrix;
pub mod order;
pub mod printer;
pub mod sets;
pub mod symbols;
pub mod tree;
pub mod extract;
pub mod grader;
pub mod normalize;
pub mod poly;
pub mod pyexpr;
pub mod realset;
pub mod solve;

pub use extract::{default_targets, ExtractionMode, FallbackMode, Item, Target};
pub use grader::VerifyOpts;

/// parse() with upstream defaults: [Latex, Expr] targets, fallback first_match, any_match.
pub fn parse_default(text: &str) -> Vec<Item> {
    extract::parse(text, &default_targets(), FallbackMode::FirstMatch, ExtractionMode::AnyMatch)
}

pub fn item_str(i: &Item) -> String {
    match i {
        Item::Expr(e) => e.to_string(),
        Item::Str(s) => s.clone(),
    }
}

#[cfg(feature = "python")]
pub mod python;
