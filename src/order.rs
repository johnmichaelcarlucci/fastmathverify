//! Orderings approximating SymPy's FiniteSet canonical order and sort keys.

use std::cmp::Ordering;

use crate::eval::{exact_or_float_rat, numeric, Env};
use crate::expr::*;

/// Real numeric value of a closed expression (for ordering only).
pub fn real_value(e: &Expr) -> Option<f64> {
    match e {
        Expr::Infinity => return Some(f64::INFINITY),
        Expr::NegInfinity => return Some(f64::NEG_INFINITY),
        _ => {}
    }
    if e.has_free_symbols() || e.is_set() || e.is_relational() || e.is_matrix() {
        return None;
    }
    if let Expr::Tuple(_) = e {
        return None;
    }
    let z = numeric(e, &Env::new())?;
    if z.im.abs() <= 1e-12 * (1.0 + z.re.abs()) { Some(z.re) } else { None }
}

/// Order used for FiniteSet args: plain numbers by value first, then the rest
/// by printed form (SymPy's default_sort_key puts Numbers first).
pub fn set_order(a: &Expr, b: &Expr) -> Ordering {
    match (a, b) {
        (Expr::Num(x), Expr::Num(y)) => x.to_rat().cmp(&y.to_rat()),
        (Expr::Num(_), _) => Ordering::Less,
        (_, Expr::Num(_)) => Ordering::Greater,
        _ => a.to_string().cmp(&b.to_string()),
    }
}

/// SymPy `class_key()` approximation: (rank, sub-rank, class name).
fn class_key(e: &Expr) -> (u8, i32, String) {
    match e {
        Expr::Num(_) | Expr::Infinity | Expr::NegInfinity | Expr::NaN | Expr::ComplexInfinity => (1, 0, "Number".into()),
        Expr::Sym(_) => (2, 0, "Symbol".into()),
        Expr::Pi => (2, 0, "Pi".into()),
        Expr::E => (2, 0, "Exp1".into()),
        Expr::I => (2, 0, "ImaginaryUnit".into()),
        Expr::EulerGamma => (2, 0, "EulerGamma".into()),
        Expr::Mul(_) => (3, 0, "Mul".into()),
        Expr::Add(_) => (3, 1, "Add".into()),
        Expr::Pow(..) => (3, 2, "Pow".into()),
        Expr::Func(f, _) => (4, 0, f.name().to_string()),
        Expr::Tuple(_) => (5, 0, "Tuple".into()),
        Expr::Interval { .. } => (5, 0, "Interval".into()),
        Expr::FiniteSet { .. } => (5, 0, "FiniteSet".into()),
        Expr::Union(_) => (5, 0, "Union".into()),
        Expr::Rel(op, ..) => (5, 0, match op {
            RelOp::Eq => "Equality", RelOp::Ne => "Unequality", RelOp::Lt => "StrictLessThan",
            RelOp::Le => "LessThan", RelOp::Gt => "StrictGreaterThan", RelOp::Ge => "GreaterThan",
        }.into()),
        Expr::And(_) => (5, 0, "And".into()),
        _ => (5, 0, "Basic".into()),
    }
}

/// Grader `sort_key`: default_sort_key(x.evalf()) -- numbers by value first,
/// then by SymPy class key, then structurally (approximated by printed form).
pub fn value_order(a: &Expr, b: &Expr) -> Ordering {
    let ka = key(a);
    let kb = key(b);
    match (&ka, &kb) {
        (Some(x), Some(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => {
            let ua = unwrap_assign(a);
            let ub = unwrap_assign(b);
            class_key(ua).cmp(&class_key(ub)).then_with(|| ua.to_string().cmp(&ub.to_string()))
        }
    }
}

fn unwrap_assign(e: &Expr) -> &Expr {
    match e {
        Expr::Rel(RelOp::Eq, l, r) if crate::convert::is_expr_of_only_symbols(l) => r,
        _ => e,
    }
}

fn key(e: &Expr) -> Option<f64> {
    let e = match e {
        Expr::Rel(RelOp::Eq, l, r) if crate::convert::is_expr_of_only_symbols(l) => &**r,
        _ => e,
    };
    if let Some(r) = exact_or_float_rat(e) {
        return Some(rat_to_f64(&r));
    }
    real_value(e)
}
