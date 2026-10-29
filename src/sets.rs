//! sympy.Interval constructor semantics.

use std::cmp::Ordering;

use crate::eval::compare_real;
use crate::expr::*;

pub fn make_interval(start: Expr, end: Expr, lopen: bool, ropen: bool) -> Result<Expr, String> {
    if !crate::eval::maybe_real(&start) || !crate::eval::maybe_real(&end) {
        return Err("Only real intervals are supported".into());
    }
    let mut lopen = lopen;
    let mut ropen = ropen;
    match compare_real(&end, &start) {
        Some(Ordering::Less) => return Ok(Expr::EmptySet),
        Some(Ordering::Equal) => {
            if matches!(start, Expr::Infinity | Expr::NegInfinity) {
                return Ok(Expr::EmptySet);
            }
            if lopen || ropen {
                return Ok(Expr::EmptySet);
            }
            return Ok(finite_set(vec![start]));
        }
        _ => {}
    }
    if matches!(start, Expr::NegInfinity) {
        lopen = true;
    }
    if matches!(end, Expr::Infinity) {
        ropen = true;
    }
    if matches!(start, Expr::Infinity) || matches!(end, Expr::NegInfinity) {
        return Ok(Expr::EmptySet);
    }
    Ok(Expr::Interval { start: Box::new(start), end: Box::new(end), lopen, ropen })
}
