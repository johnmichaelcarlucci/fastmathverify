//! The small part of sympy.solve / Relational.as_set that the grader relies on.

use std::cmp::Ordering;

use crate::eval::compare_real;
use crate::expr::*;
use crate::order::real_value;
use crate::poly::{coeffs_in, solve_linear_dict, to_poly};
use crate::realset::{to_realset, RealSet};
use crate::sets::make_interval;
use crate::symbols::is_real_symbol;

pub enum Sol {
    Val,
    Dict(String, Expr),
}

fn sub(a: &Expr, b: &Expr) -> Expr {
    Expr::Add(vec![a.clone(), Expr::Mul(vec![int(-1), b.clone()])])
}

/// Number of distinct real roots of a univariate polynomial of degree <= 2.
fn real_root_count(coeffs: &[Expr]) -> Option<usize> {
    let v: Vec<f64> = coeffs.iter().map(real_value).collect::<Option<Vec<_>>>()?;
    match v.len() {
        1 => Some(0),
        2 => Some(if v[1] != 0.0 { 1 } else { 0 }),
        3 => {
            if v[2] == 0.0 {
                return real_root_count(&coeffs[..2]);
            }
            let d = v[1] * v[1] - 4.0 * v[2] * v[0];
            let scale = (v[1] * v[1]).abs().max((4.0 * v[2] * v[0]).abs()).max(1e-300);
            Some(if d.abs() <= 1e-12 * scale { 1 } else if d > 0.0 { 2 } else { 0 })
        }
        _ => None,
    }
}

/// `solve(Eq(l, r), free_symbols)` shape: None when we cannot tell.
pub fn solve_eq(l: &Expr, r: &Expr) -> Option<Vec<Sol>> {
    let d = sub(l, r);
    let syms = d.free_symbols();
    match syms.len() {
        0 => Some(vec![]),
        1 => {
            let p = to_poly(&d)?;
            let c = coeffs_in(&p, &syms[0])?;
            if !is_real_symbol(&syms[0]) {
                return None;
            }
            let n = real_root_count(&c)?;
            Some((0..n).map(|_| Sol::Val).collect())
        }
        _ => {
            let sol = solve_linear_dict(l, r)?;
            Some(sol.into_iter().map(|(k, v)| Sol::Dict(k, v)).collect())
        }
    }
}

fn reals() -> Expr {
    Expr::SetConst(SetConst::Reals)
}

fn iv(a: Expr, b: Expr, lo: bool, ro: bool) -> Option<Expr> {
    make_interval(a, b, lo, ro).ok()
}

/// Set of real x with `poly(x) op 0`, polynomial degree 1 or 2.
fn poly_rel_set(c: &[Expr], op: RelOp) -> Option<Expr> {
    let neg = |e: &Expr| Expr::Mul(vec![int(-1), e.clone()]);
    match c.len() {
        2 => {
            let a = &c[1];
            let av = real_value(a)?;
            if av == 0.0 {
                return None;
            }
            let root = simplify_closed(&Expr::Mul(vec![neg(&c[0]), inv(a.clone())]));
            let op = if av < 0.0 { op.inverse() } else { op };
            Some(match op {
                RelOp::Eq => finite_set(vec![root]),
                RelOp::Ne => Expr::Union(vec![iv(Expr::NegInfinity, root.clone(), true, true)?, iv(root, Expr::Infinity, true, true)?]),
                RelOp::Lt => iv(Expr::NegInfinity, root, true, true)?,
                RelOp::Le => iv(Expr::NegInfinity, root, true, false)?,
                RelOp::Gt => iv(root, Expr::Infinity, true, true)?,
                RelOp::Ge => iv(root, Expr::Infinity, false, true)?,
            })
        }
        3 => {
            let (a, b, cc) = (&c[2], &c[1], &c[0]);
            let av = real_value(a)?;
            if av == 0.0 {
                return poly_rel_set(&c[..2], op);
            }
            let disc = Expr::Add(vec![pow(b.clone(), int(2)), Expr::Mul(vec![int(-4), a.clone(), cc.clone()])]);
            let dv = real_value(&disc)?;
            let op = if av < 0.0 { op.inverse() } else { op };
            let two_a = Expr::Mul(vec![int(2), a.clone()]);
            let mk = |sign: i64| {
                simplify_closed(&Expr::Mul(vec![
                    Expr::Add(vec![neg(b), Expr::Mul(vec![int(sign), pow(disc.clone(), half())])]),
                    inv(two_a.clone()),
                ]))
            };
            let scale = real_value(&pow(b.clone(), int(2)))?.abs().max(1e-300);
            let (mut r1, mut r2) = (mk(-1), mk(1));
            if compare_real(&r1, &r2) == Some(Ordering::Greater) {
                std::mem::swap(&mut r1, &mut r2);
            }
            if dv.abs() <= 1e-12 * scale {
                let r = simplify_closed(&Expr::Mul(vec![neg(b), inv(two_a)]));
                return Some(match op {
                    RelOp::Eq | RelOp::Le => finite_set(vec![r]),
                    RelOp::Lt => Expr::EmptySet,
                    RelOp::Ge => reals(),
                    RelOp::Gt | RelOp::Ne => {
                        Expr::Union(vec![iv(Expr::NegInfinity, r.clone(), true, true)?, iv(r, Expr::Infinity, true, true)?])
                    }
                });
            }
            if dv < 0.0 {
                return Some(match op {
                    RelOp::Eq | RelOp::Lt | RelOp::Le => Expr::EmptySet,
                    _ => reals(),
                });
            }
            Some(match op {
                RelOp::Eq => finite_set(vec![r1, r2]),
                RelOp::Lt => iv(r1, r2, true, true)?,
                RelOp::Le => iv(r1, r2, false, false)?,
                RelOp::Gt => Expr::Union(vec![iv(Expr::NegInfinity, r1, true, true)?, iv(r2, Expr::Infinity, true, true)?]),
                RelOp::Ge => Expr::Union(vec![iv(Expr::NegInfinity, r1, true, false)?, iv(r2, Expr::Infinity, false, true)?]),
                RelOp::Ne => Expr::Union(vec![
                    iv(Expr::NegInfinity, r1.clone(), true, true)?,
                    iv(r1, r2.clone(), true, true)?,
                    iv(r2, Expr::Infinity, true, true)?,
                ]),
            })
        }
        _ => None,
    }
}

/// Collapse closed rational-valued expressions to numbers (keeps others as-is).
fn simplify_closed(e: &Expr) -> Expr {
    match crate::eval::exact(e) {
        Some(r) => Expr::Num(Num::rat(r)),
        None => e.clone(),
    }
}

fn rel_set(op: RelOp, l: &Expr, r: &Expr, var: &str) -> Option<Expr> {
    let inf = |e: &Expr| matches!(e, Expr::Infinity | Expr::NegInfinity);
    if inf(l) || inf(r) {
        // x compared with an infinity: true for all reals or for none
        let (x_on_left, other) = if inf(r) { (true, r) } else { (false, l) };
        let pos = matches!(other, Expr::Infinity);
        let op = if x_on_left { op } else { op.inverse() };
        let all = match op {
            RelOp::Lt | RelOp::Le => pos,
            RelOp::Gt | RelOp::Ge => !pos,
            RelOp::Ne => true,
            RelOp::Eq => false,
        };
        return Some(if all { reals() } else { Expr::EmptySet });
    }
    let p = to_poly(&sub(l, r))?;
    let c = coeffs_in(&p, var)?;
    poly_rel_set(&c, op)
}

fn from_realset(rs: RealSet) -> Expr {
    rs.to_expr()
}

/// `Relational.as_set()` / `And.as_set()` for one real variable.
pub fn as_set(e: &Expr) -> Option<Expr> {
    let syms = e.free_symbols();
    if syms.len() != 1 || !is_real_symbol(&syms[0]) {
        return None;
    }
    let v = &syms[0];
    match e {
        Expr::Rel(op, l, r) => rel_set(*op, l, r, v),
        Expr::And(rels) => {
            let mut acc: Option<RealSet> = None;
            for x in rels {
                let Expr::Rel(op, l, r) = x else { return None };
                let s = to_realset(&rel_set(*op, l, r, v)?)?;
                acc = Some(match acc {
                    None => s,
                    Some(a) => a.intersect(s)?,
                });
            }
            Some(from_realset(acc?))
        }
        _ => None,
    }
}
