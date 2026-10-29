//! The small part of sympy.solve / Relational.as_set that the grader relies on.

use std::cmp::Ordering;

use crate::eval::compare_real;
use crate::expr::*;
use crate::order::real_value;
use crate::poly::{coeffs_in, coeffs_in_ex, to_poly};
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
            // sympy solves for the first symbol (sorted) it can isolate.
            let p = to_poly(&d)?;
            for s in &syms {
                let Some(c) = coeffs_in_ex(&p, s, true) else { continue };
                let zero = |e: &Expr| crate::poly::to_poly(e).map(|q| q.is_zero()).unwrap_or(false);
                match c.len() {
                    2 if !zero(&c[1]) => {
                        let v = Expr::Mul(vec![int(-1), c[0].clone(), inv(c[1].clone())]);
                        return Some(vec![Sol::Dict(s.clone(), v)]);
                    }
                    3 if !zero(&c[2]) => {
                        let (a, b, cc) = (&c[2], &c[1], &c[0]);
                        let disc = Expr::Add(vec![pow(b.clone(), int(2)), Expr::Mul(vec![int(-4), a.clone(), cc.clone()])]);
                        let mk = |sg: i64| {
                            Expr::Mul(vec![
                                Expr::Add(vec![Expr::Mul(vec![int(-1), b.clone()]), Expr::Mul(vec![int(sg), pow(disc.clone(), half())])]),
                                inv(Expr::Mul(vec![int(2), a.clone()])),
                            ])
                        };
                        return Some(vec![Sol::Dict(s.clone(), mk(-1)), Sol::Dict(s.clone(), mk(1))]);
                    }
                    _ => continue,
                }
            }
            None
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
    let d = sub(l, r);
    if let Some(g) = find_abs_linear(&d, var) {
        // |g| with g linear in var: split at the root of g.
        let p = to_poly(&g)?;
        let c = coeffs_in(&p, var)?;
        let t = simplify_closed(&Expr::Mul(vec![int(-1), c[0].clone(), inv(c[1].clone())]));
        let up = real_value(&c[1])? > 0.0;
        let (dom_pos, dom_neg) = if up {
            (iv(t.clone(), Expr::Infinity, false, true)?, iv(Expr::NegInfinity, t, true, true)?)
        } else {
            (iv(Expr::NegInfinity, t.clone(), true, false)?, iv(t, Expr::Infinity, true, true)?)
        };
        let abs = Expr::Func(Func::Abs, vec![g.clone()]);
        let d_pos = replace(&d, &abs, &g);
        let d_neg = replace(&d, &abs, &Expr::Mul(vec![int(-1), g.clone()]));
        let a = to_realset(&rel_set(op, &d_pos, &int(0), var)?)?.intersect(to_realset(&dom_pos)?)?;
        let b = to_realset(&rel_set(op, &d_neg, &int(0), var)?)?.intersect(to_realset(&dom_neg)?)?;
        let u = to_realset(&Expr::Union(vec![a.to_expr(), b.to_expr()]))?;
        return Some(u.to_expr());
    }
    let p = to_poly(&d)?;
    let c = coeffs_in(&p, var)?;
    poly_rel_set(&c, op)
}

fn find_abs_linear(e: &Expr, var: &str) -> Option<Expr> {
    if let Expr::Func(Func::Abs, args) = e {
        let g = &args[0];
        if let Some(p) = to_poly(g) {
            if let Some(c) = coeffs_in(&p, var) {
                if c.len() == 2 {
                    return Some(g.clone());
                }
            }
        }
    }
    e.children().into_iter().find_map(|c| find_abs_linear(c, var))
}

fn replace(e: &Expr, from: &Expr, to: &Expr) -> Expr {
    if crate::grader::struct_eq(e, from) {
        return to.clone();
    }
    e.map_children(|c| replace(c, from, to))
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
