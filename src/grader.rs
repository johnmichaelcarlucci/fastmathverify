//! Port of math_verify.grader.verify.

use std::cmp::Ordering;

use num_bigint::BigInt;
use num_complex::Complex64;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

use crate::eval::{exact, numeric, Env};
use crate::expr::*;
use crate::extract::Item;
use crate::poly::{evaluated_sub, solve_linear_dict, to_poly};
use crate::symbols::{display_name, is_real_symbol};

#[derive(Clone, Copy, Debug)]
pub struct VerifyOpts {
    pub float_rounding: u32,
    pub numeric_precision: u32,
    pub strict: bool,
    pub allow_set_relation_comp: bool,
    /// Compare numbers by exact value: no rounding to `float_rounding` places,
    /// and a percentage equals its value / 100 only (`9%` is not `9`).
    pub exact: bool,
}

impl Default for VerifyOpts {
    fn default() -> Self {
        VerifyOpts { float_rounding: 6, numeric_precision: 15, strict: true, allow_set_relation_comp: false, exact: false }
    }
}

/// Error inside a comparison: upstream catches every exception and returns False.
type R = Result<bool, ()>;

// ---------------- structural equality (sympy `==`) ----------------

pub fn struct_eq(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Num(x), Expr::Num(y)) => match (x, y) {
            (Num::Float { exact: p, .. }, Num::Float { exact: q, .. }) => p == q,
            (Num::Float { .. }, _) | (_, Num::Float { .. }) => false,
            _ => x.to_rat() == y.to_rat(),
        },
        (Expr::Sym(x), Expr::Sym(y)) => x == y,
        (Expr::Add(x), Expr::Add(y)) | (Expr::Mul(x), Expr::Mul(y)) => multiset_eq(x, y),
        (Expr::FiniteSet { args: x, .. }, Expr::FiniteSet { args: y, .. }) => multiset_eq(x, y),
        (Expr::Union(x), Expr::Union(y)) | (Expr::Intersection(x), Expr::Intersection(y)) => multiset_eq(x, y),
        (Expr::Tuple(x), Expr::Tuple(y)) | (Expr::And(x), Expr::And(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| struct_eq(p, q))
        }
        (Expr::Func(f, x), Expr::Func(g, y)) => {
            f == g && x.len() == y.len() && x.iter().zip(y).all(|(p, q)| struct_eq(p, q))
        }
        (Expr::Rel(o1, a1, b1), Expr::Rel(o2, a2, b2)) => o1 == o2 && struct_eq(a1, a2) && struct_eq(b1, b2),
        (
            Expr::Interval { start: s1, end: e1, lopen: l1, ropen: r1 },
            Expr::Interval { start: s2, end: e2, lopen: l2, ropen: r2 },
        ) => l1 == l2 && r1 == r2 && struct_eq(s1, s2) && struct_eq(e1, e2),
        (Expr::Matrix { rows: r1, cols: c1, data: d1 }, Expr::Matrix { rows: r2, cols: c2, data: d2 }) => {
            r1 == r2 && c1 == c2 && d1.iter().zip(d2).all(|(p, q)| struct_eq(p, q))
        }
        _ => {
            if std::mem::discriminant(a) != std::mem::discriminant(b) {
                return false;
            }
            let ca = a.children();
            let cb = b.children();
            if ca.is_empty() && cb.is_empty() {
                return a == b;
            }
            ca.len() == cb.len() && ca.iter().zip(&cb).all(|(p, q)| struct_eq(p, q))
        }
    }
}

fn multiset_eq(x: &[Expr], y: &[Expr]) -> bool {
    if x.len() != y.len() {
        return false;
    }
    let mut used = vec![false; y.len()];
    'outer: for p in x {
        for (j, q) in y.iter().enumerate() {
            if !used[j] && struct_eq(p, q) {
                used[j] = true;
                continue 'outer;
            }
        }
        return false;
    }
    true
}

// ---------------- semantic equality (replaces simplify(a - b) == 0) ----------------

fn finite(z: Complex64) -> bool {
    z.re.is_finite() && z.im.is_finite()
}

fn close(a: Complex64, b: Complex64) -> bool {
    // f64 overflow gives inf, and inf <= 1e-10 * inf would accept any two huge values
    if !finite(a) || !finite(b) {
        return false;
    }
    let scale = a.norm().max(b.norm());
    let d = (a - b).norm();
    d <= 1e-10 * scale || d <= 1e-14
}

const SAMPLE_REAL: [f64; 6] = [0.7316, -1.3779, 2.2153, -0.4417, 1.6191, -2.8933];
const SAMPLE_IM: [f64; 6] = [0.5311, -0.8761, 1.2207, 0.3779, -1.4143, 0.9187];

pub fn semantic_eq(a: &Expr, b: &Expr) -> bool {
    if let (Expr::Matrix { rows: r1, cols: c1, data: d1 }, Expr::Matrix { rows: r2, cols: c2, data: d2 }) = (a, b) {
        return r1 == r2 && c1 == c2 && d1.iter().zip(d2).all(|(p, q)| semantic_eq(p, q));
    }
    if a.is_matrix() || b.is_matrix() {
        return false;
    }
    if let (Some(x), Some(y)) = (exact(a), exact(b)) {
        return x == y;
    }
    let mut syms = a.free_symbols();
    syms.extend(b.free_symbols());
    syms.sort();
    syms.dedup();
    if syms.is_empty() {
        return match (numeric(a, &Env::new()), numeric(b, &Env::new())) {
            (Some(x), Some(y)) if finite(x) || finite(y) => close(x, y),
            _ => struct_eq(a, b),
        };
    }
    let mut ok = 0;
    for t in 0..6 {
        if crate::deadline::expired() {
            return false;
        }
        let mut env = Env::new();
        for (i, s) in syms.iter().enumerate() {
            let k = (t + 2 * i) % 6;
            let v = if is_real_symbol(s) {
                Complex64::new(SAMPLE_REAL[k] + 0.0371 * i as f64, 0.0)
            } else {
                Complex64::new(SAMPLE_REAL[k], SAMPLE_IM[(k + 1) % 6])
            };
            env.insert(s.clone(), v);
        }
        match (numeric(a, &env), numeric(b, &env)) {
            (Some(x), Some(y)) if !finite(x) && !finite(y) => {} // both overflow: no information
            (Some(x), Some(y)) => {
                if !close(x, y) {
                    return false;
                }
                ok += 1;
            }
            (None, None) => {}
            _ => return false,
        }
    }
    if ok >= 2 {
        return true;
    }
    struct_eq(a, b)
}

// ---------------- helpers mirroring grader.py ----------------

fn is_uneval_pct(e: &Expr) -> bool {
    matches!(e, Expr::Uneval(x) if matches!(&**x, Expr::Num(Num::Rat(r)) if *r.numer() == BigInt::one() && *r.denom() == BigInt::from(100)))
}

fn pct_inner(e: &Expr) -> Option<&Expr> {
    if let Expr::Mul(v) = e {
        if v.len() == 2 && is_uneval_pct(&v[1]) {
            return Some(&v[0]);
        }
    }
    None
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Number,
    Float,
    Integer,
}

fn is_kind(e: &Expr, k: Kind) -> bool {
    match (e, k) {
        (Expr::Num(_), Kind::Number) => true,
        (Expr::Infinity | Expr::NegInfinity | Expr::NaN, Kind::Number) => true,
        (Expr::Num(n), Kind::Float) => n.is_float(),
        (Expr::Num(n), Kind::Integer) => n.is_integer(),
        _ => false,
    }
}

fn atomic_or_pct(e: &Expr, k: Kind) -> bool {
    is_kind(e, k) || pct_inner(e).map(|x| is_kind(x, k)).unwrap_or(false)
}

/// Value of `x.round(p)` as (decimal value, is_integer_type). None when sympy raises.
fn round_value(x: &Expr, p: u32) -> Option<(Rat, bool)> {
    let ten = BigInt::from(10).pow(p);
    let round_half_even = |r: &Rat| -> Rat {
        let scaled = r * Rat::from_integer(ten.clone());
        let fl = scaled.floor();
        let diff = &scaled - &fl;
        let half = Rat::new(BigInt::one(), BigInt::from(2));
        let n = match diff.cmp(&half) {
            Ordering::Less => fl,
            Ordering::Greater => fl + Rat::one(),
            Ordering::Equal => {
                if fl.to_integer().is_even() { fl } else { fl + Rat::one() }
            }
        };
        n / Rat::from_integer(ten.clone())
    };
    match x {
        Expr::Num(Num::Int(i)) => return Some((Rat::from_integer(i.clone()), true)),
        Expr::Num(Num::Float { f, .. }) => {
            let r = Rat::from_float(*f)?;
            return Some((round_half_even(&r), false));
        }
        _ => {}
    }
    if let Some(r) = exact(x) {
        if r.is_integer() {
            return Some((r, true));
        }
        return Some((round_half_even(&r), false));
    }
    if x.has_free_symbols() {
        return None;
    }
    // doit() of pct/expressions containing Floats: evaluate numerically.
    let z = numeric(x, &Env::new())?;
    if z.im.abs() > 1e-12 * (1.0 + z.re.abs()) {
        return None;
    }
    let r = Rat::from_float(z.re)?;
    Some((round_half_even(&r), false))
}

fn doit_pct(e: &Expr) -> Expr {
    // UnevaluatedExpr(1/100).doit() -> x/100 evaluated
    if let Some(x) = pct_inner(e) {
        if let Expr::Num(n) = x {
            let r = n.to_rat() / Rat::from_integer(BigInt::from(100));
            return Expr::Num(if n.is_float() { Num::float_from_rat(r) } else { Num::rat(r) });
        }
    }
    e.clone()
}

fn numeric_eq(a: &Expr, b: &Expr, opts: &VerifyOpts) -> bool {
    if a.is_matrix() && b.is_matrix() {
        if let (Expr::Matrix { rows: r1, cols: c1, data: d1 }, Expr::Matrix { rows: r2, cols: c2, data: d2 }) = (a, b) {
            if r1 == r2 && c1 == c2 {
                return d1.iter().zip(d2).all(|(x, y)| numeric_eq(x, y, opts));
            }
        }
        return false;
    }
    if opts.exact && (atomic_or_pct(a, Kind::Number) || atomic_or_pct(b, Kind::Number)) {
        let val = |e: &Expr| match doit_pct(e) {
            Expr::Num(n) => Some(n.to_rat()),
            x => exact(&x),
        };
        return match (val(a), val(b)) {
            (Some(x), Some(y)) => x == y,
            _ => semantic_eq(&doit_pct(a), &doit_pct(b)),
        };
    }
    if atomic_or_pct(a, Kind::Number) || atomic_or_pct(b, Kind::Number) {
        if atomic_or_pct(a, Kind::Float) || atomic_or_pct(b, Kind::Float) {
            let (ra, rb) = (round_value(&doit_pct(a), opts.float_rounding), round_value(&doit_pct(b), opts.float_rounding));
            return match (ra, rb) {
                (Some((x, xi)), Some((y, yi))) => xi == yi && x == y,
                _ => false,
            };
        }
        if atomic_or_pct(a, Kind::Integer) && atomic_or_pct(b, Kind::Integer) {
            let x = pct_inner(a).unwrap_or(a);
            let y = pct_inner(b).unwrap_or(b);
            return struct_eq(x, y);
        }
        return doit_eq(a, b);
    }
    // (a - b).evalf(chop=True) == 0: only closed numeric differences can vanish here
    if a.has_free_symbols() || b.has_free_symbols() {
        return match to_poly(&Expr::Add(vec![a.clone(), Expr::Mul(vec![int(-1), b.clone()])])) {
            Some(p) => p.is_zero(),
            None => false,
        };
    }
    match (numeric(a, &Env::new()), numeric(b, &Env::new())) {
        (Some(x), Some(y)) if finite(x) && finite(y) => {
            // (a - b).evalf(chop=True) == 0: zero only up to working precision, relative
            let tol = 10f64.powi(-(opts.numeric_precision as i32 - 2)).max(1e-13);
            let d = (x - y).norm();
            d <= tol * x.norm().max(y.norm()) || d <= 1e-300
        }
        _ => false,
    }
}

/// `safe_sympy_doit(a) == safe_sympy_doit(b)` for number-vs-expression comparisons.
fn doit_eq(a: &Expr, b: &Expr) -> bool {
    let a = doit_pct(a);
    let b = doit_pct(b);
    match (&a, &b) {
        (Expr::Num(x), Expr::Num(y)) => {
            if x.is_float() != y.is_float() {
                return false;
            }
            return x.to_rat() == y.to_rat();
        }
        _ => {}
    }
    // doit() evaluates b*0 -> 0 (sympy's Mul drops to zero when a factor is exactly zero)
    let zero_mul = |e: &Expr| -> Option<Rat> {
        match e {
            Expr::Mul(v)
                if v.iter().any(|x| exact(x).is_some_and(|r| r.is_zero()))
                    && v.iter().all(|x| !matches!(x, Expr::Infinity | Expr::NegInfinity | Expr::NaN | Expr::ComplexInfinity)) =>
            {
                Some(Rat::zero())
            }
            _ => exact(e),
        }
    };
    if let (Some(x), Some(y)) = (zero_mul(&a), zero_mul(&b)) {
        return x == y;
    }
    false
}

fn symbolic_eq(a: &Expr, b: &Expr) -> bool {
    semantic_eq(a, b)
}

fn is_relation(e: &Expr) -> bool {
    match e {
        Expr::Rel(..) => true,
        Expr::And(v) => !v.is_empty() && v.iter().all(|x| matches!(x, Expr::Rel(..))),
        _ => false,
    }
}

fn is_equation(e: &Expr) -> bool {
    match e {
        Expr::Rel(RelOp::Eq, ..) => true,
        Expr::And(v) => !v.is_empty() && v.iter().all(|x| matches!(x, Expr::Rel(RelOp::Eq, ..))),
        _ => false,
    }
}

fn is_assignment_relation(e: &Expr) -> bool {
    use crate::convert::is_expr_of_only_symbols as only;
    match e {
        Expr::Rel(RelOp::Eq, l, _) => only(l),
        Expr::And(v) if !v.is_empty() => {
            v.iter().all(|x| matches!(x, Expr::Rel(RelOp::Eq, ..)))
                && matches!(&v[0], Expr::Rel(_, l, _) if only(l))
        }
        _ => false,
    }
}

fn take_last_relation(e: &Expr) -> &Expr {
    match e {
        Expr::And(v) => take_last_relation(v.last().unwrap()),
        x => x,
    }
}

fn take_first_relation(e: &Expr) -> &Expr {
    match e {
        Expr::And(v) => &v[0],
        x => x,
    }
}

fn lhs(e: &Expr) -> Expr {
    match e {
        Expr::Rel(_, l, _) => (**l).clone(),
        _ => e.clone(),
    }
}

fn rhs(e: &Expr) -> Expr {
    match e {
        Expr::Rel(_, _, r) => (**r).clone(),
        _ => e.clone(),
    }
}

fn is_sympy_set(e: &Expr) -> bool {
    e.is_set()
}

fn unwrap_fcs(e: &Expr) -> Expr {
    match e {
        Expr::Func(Func::Undefined(name), args) => {
            let parts: Vec<String> = args.iter().map(|a| unwrap_fcs(a).to_string()).collect();
            Expr::Sym(format!("{}_{}", name, parts.join("_")))
        }
        _ => e.map_children(unwrap_fcs),
    }
}

fn as_set(e: &Expr) -> Option<Expr> {
    crate::solve::as_set(e)
}

// ---------------- comparisons ----------------

fn compare_symbols(gold: &Expr, pred: &Expr) -> bool {
    let name_of = |e: &Expr| match e {
        Expr::Sym(s) => Some(display_name(s).to_string()),
        _ => None,
    };
    if let Some(g) = name_of(gold) {
        if g.to_lowercase() == "e" && matches!(pred, Expr::E) {
            return true;
        }
    }
    if let Some(p) = name_of(pred) {
        if p.to_lowercase() == "e" && matches!(gold, Expr::E) {
            return true;
        }
    }
    let concat = |v: &[Expr]| -> Option<String> {
        if !v.iter().all(|a| matches!(a, Expr::E | Expr::Sym(_))) {
            return None;
        }
        let names: Vec<String> =
            v.iter().map(|a| match a { Expr::Sym(s) => display_name(s).to_string(), _ => "e".to_string() }).collect();
        Some(names.concat())
    };
    if let (Some(g), Expr::Mul(v)) = (name_of(gold), pred) {
        if let Some(c) = concat(v) {
            return g.to_lowercase() == c.to_lowercase();
        }
    }
    if let (Some(p), Expr::Mul(v)) = (name_of(pred), gold) {
        if let Some(c) = concat(v) {
            return p.to_lowercase() == c.to_lowercase();
        }
    }
    if let (Some(mut g), Some(mut p)) = (name_of(gold), name_of(pred)) {
        if g.chars().count() > 1 {
            g = g.to_lowercase();
        }
        if p.chars().count() > 1 {
            p = p.to_lowercase();
        }
        return g == p;
    }
    gold.to_string() == pred.to_string()
}

fn compare_interval(a: &Expr, b: &Expr, o: &VerifyOpts) -> R {
    if let (
        Expr::Interval { start: s1, end: e1, lopen: l1, ropen: r1 },
        Expr::Interval { start: s2, end: e2, lopen: l2, ropen: r2 },
    ) = (a, b)
    {
        if l1 != l2 || r1 != r2 {
            return Ok(false);
        }
        return Ok(expr_eq(s1, s2, o, false, true)? && expr_eq(e1, e2, o, false, true)?);
    }
    Ok(false)
}

fn set_len(e: &Expr) -> usize {
    match e {
        Expr::FiniteSet { args, .. } => args.len(),
        Expr::Tuple(v) => v.len(),
        _ => 0,
    }
}

fn deep_compare(gold: &Expr, pred: &Expr, o: &VerifyOpts) -> R {
    if set_len(gold) != set_len(pred) {
        return Ok(false);
    }
    let sorted = |v: &[Expr]| {
        let mut v = v.to_vec();
        v.sort_by(crate::order::value_order);
        v
    };
    let args = |e: &Expr| -> Vec<Expr> {
        match e {
            Expr::FiniteSet { args, .. } => args.clone(),
            Expr::Tuple(v) => v.clone(),
            _ => vec![],
        }
    };
    let (g, p): (Vec<Expr>, Vec<Expr>) = match (gold, pred) {
        (Expr::FiniteSet { .. }, _) => (sorted(&args(gold)), sorted(&args(pred))),
        (Expr::Tuple(ga), Expr::FiniteSet { unsorted, .. }) => (ga.clone(), unsorted.clone()),
        (_, Expr::FiniteSet { .. }) => (args(gold), sorted(&args(pred))),
        _ => (args(gold), args(pred)),
    };
    if g.len() != p.len() {
        return Err(());
    }
    let mut straight = Ok(true);
    for (x, y) in g.iter().zip(&p) {
        match expr_eq(x, y, o, false, true) {
            Ok(true) => {}
            r => {
                straight = r;
                break;
            }
        }
    }
    if matches!(straight, Ok(true)) || !matches!((gold, pred), (Expr::FiniteSet { .. }, Expr::FiniteSet { .. })) {
        return straight;
    }
    // Sets are unordered: the value sort above can pair symbolic elements wrongly,
    // so match each gold element to any unused equal prediction element.
    if g.len() > 64 {
        return straight;
    }
    let mut used = vec![false; p.len()];
    for x in &g {
        let hit = (0..p.len()).find(|&j| !used[j] && matches!(expr_eq(x, &p[j], o, false, true), Ok(true)));
        match hit {
            Some(j) => used[j] = true,
            None => return straight,
        }
    }
    Ok(true)
}

fn compare_sets(gold: &Expr, pred: &Expr, o: &VerifyOpts) -> R {
    let wrap = |e: &Expr| -> Expr {
        if e.is_set() || matches!(e, Expr::Tuple(_)) {
            e.clone()
        } else {
            finite_set(vec![e.clone()])
        }
    };
    let a = wrap(gold);
    let b = wrap(pred);
    if matches!(a, Expr::Interval { .. }) && matches!(b, Expr::Interval { .. }) {
        return compare_interval(&a, &b, o);
    }
    if struct_eq(&a, &b) {
        return Ok(true);
    }
    if is_sympy_set(&a) && is_sympy_set(&b) {
        if let (Some(x), Some(y)) = (crate::realset::to_realset(&a), crate::realset::to_realset(&b)) {
            if x.equals(&y) == Some(true) {
                return Ok(true);
            }
        } else if let (Expr::FiniteSet { args: x, .. }, Expr::FiniteSet { args: y, .. }) = (&a, &b) {
            // symbolic finite sets: each element provably contained in the other
            let contains = |s: &[Expr], e: &Expr| s.iter().any(|t| struct_eq(t, e) || doit_eq(t, e));
            if x.iter().all(|e| contains(y, e)) && y.iter().all(|e| contains(x, e)) {
                return Ok(true);
            }
        }
    }
    let fin_or_tuple = |e: &Expr| matches!(e, Expr::FiniteSet { .. } | Expr::Tuple(_));
    if fin_or_tuple(&a) && fin_or_tuple(&b) {
        return deep_compare(&a, &b, o);
    }
    if let Expr::Interval { start, end, lopen: true, ropen: true } = &a {
        if fin_or_tuple(&b) && set_len(&b) == 2 {
            return deep_compare(&Expr::Tuple(vec![(**start).clone(), (**end).clone()]), &b, o);
        }
    }
    if let Expr::Interval { start, end, lopen: true, ropen: true } = &b {
        if fin_or_tuple(&a) && set_len(&a) == 2 {
            return deep_compare(&a, &Expr::Tuple(vec![(**start).clone(), (**end).clone()]), o);
        }
    }
    Ok(false)
}

fn compare_relational(gold: &Expr, pred: &Expr, o: &VerifyOpts) -> R {
    if let (Expr::And(g), Expr::And(p)) = (gold, pred) {
        if g.len() != p.len() {
            return Err(());
        }
        for (x, y) in g.iter().zip(p) {
            if !compare_relational(x, y, o)? {
                return Ok(false);
            }
        }
        return Ok(true);
    }
    let (Expr::Rel(gop, gl, gr), Expr::Rel(pop, pl, pr)) = (gold, pred) else {
        return Ok(false);
    };
    if gop == pop {
        if let Ok(true) = expr_eq(&evaluated_sub(gl, gr), &evaluated_sub(pl, pr), o, false, true) {
            return Ok(true);
        }
    }
    if *gop == RelOp::Ne {
        return Err(()); // INVERSE_RELATIONS[Ne] raises KeyError upstream
    }
    if gop.inverse() == *pop || (*gop == RelOp::Eq && *pop == RelOp::Eq) {
        if let Ok(true) = expr_eq(&evaluated_sub(gl, gr), &evaluated_sub(pr, pl), o, false, true) {
            return Ok(true);
        }
    }
    // sympy_solve_and_compare: symbol-free equations both solve to [] (-> True);
    // single-symbol solutions are plain values (upstream's `.items()` raises);
    // only multi-symbol linear equations yield comparable dict solutions.
    if *gop == RelOp::Eq && *pop == RelOp::Eq {
        match (crate::solve::solve_eq(gl, gr), crate::solve::solve_eq(pl, pr)) {
            (Some(gs), Some(ps)) => {
                if gs.len() != ps.len() {
                    return Ok(false);
                }
                if gs.is_empty() {
                    return Ok(true);
                }
                use crate::solve::Sol;
                let pair_ok = |g: &Sol, p: &Sol| match (g, p) {
                    (Sol::Dict(gk, gv), Sol::Dict(pk, pv)) => {
                        gk == pk && matches!(expr_eq(gv, pv, o, false, true), Ok(true))
                    }
                    _ => false,
                };
                // sympy orders both solution lists canonically; try both pairings for two roots.
                let straight = gs.iter().zip(&ps).all(|(g, p)| pair_ok(g, p));
                let swapped = gs.len() == 2 && pair_ok(&gs[0], &ps[1]) && pair_ok(&gs[1], &ps[0]);
                return Ok(straight || swapped);
            }
            _ => return Ok(false),
        }
    }
    Ok(false)
}

pub fn expr_eq(gold: &Expr, pred: &Expr, o: &VerifyOpts, allow_set_rel: bool, strict: bool) -> R {
    let mut gold = gold.clone();
    let mut pred = pred.clone();
    if !strict {
        let gv = gold.free_symbols();
        let pv = pred.free_symbols();
        if gv.len() == pv.len() {
            for (p, g) in pv.iter().zip(&gv) {
                pred = pred.subs_sym(p, &Expr::Sym(format!("\u{3}{g}")));
            }
            for g in &gv {
                pred = pred.subs_sym(&format!("\u{3}{g}"), &Expr::Sym(g.clone()));
            }
        }
    }
    let ga = is_assignment_relation(&gold);
    let pa = is_assignment_relation(&pred);
    let ge = is_equation(&gold);
    let pe = is_equation(&pred);
    if ga {
        gold = Expr::Rel(RelOp::Eq, Box::new(lhs(take_first_relation(&gold))), Box::new(rhs(take_last_relation(&gold))));
    }
    if pa {
        pred = Expr::Rel(RelOp::Eq, Box::new(lhs(take_first_relation(&pred))), Box::new(rhs(take_last_relation(&pred))));
    }
    if pe && !ge {
        pred = rhs(take_last_relation(&pred));
    } else if ga && !pe {
        gold = rhs(take_last_relation(&gold));
    }
    if is_relation(&gold) && is_sympy_set(&pred) {
        if let Some(s) = as_set(&unwrap_fcs(&gold)) {
            gold = s;
        }
    }
    if allow_set_rel && is_relation(&pred) && is_sympy_set(&gold) {
        if let Some(s) = as_set(&unwrap_fcs(&pred)) {
            pred = s;
        }
    }
    // sympy_str_eq
    if matches!(gold, Expr::NaN | Expr::ComplexInfinity) {
        return Err(());
    }
    if struct_eq(&gold, &pred) {
        return Ok(true);
    }
    if is_relation(&gold) && is_relation(&pred) {
        return compare_relational(&gold, &pred, o);
    }
    let set_or_tuple = |e: &Expr| e.is_set() || matches!(e, Expr::Tuple(_));
    if set_or_tuple(&gold) || set_or_tuple(&pred) {
        return compare_sets(&gold, &pred, o);
    }
    if matches!(gold, Expr::Sym(_)) || matches!(pred, Expr::Sym(_)) {
        return Ok(compare_symbols(&gold, &pred));
    }
    if is_relation(&gold) || is_relation(&pred) || matches!(gold, Expr::Not(_)) || matches!(pred, Expr::Not(_)) {
        return Ok(false);
    }
    if numeric_eq(&gold, &pred, o) {
        return Ok(true);
    }
    Ok(symbolic_eq(&gold, &pred))
}

pub fn compare_items(g: &Item, t: &Item, o: &VerifyOpts) -> bool {
    match (g, t) {
        (Item::Expr(a), Item::Expr(b)) => expr_eq(a, b, o, o.allow_set_relation_comp, o.strict).unwrap_or(false),
        (Item::Str(a), Item::Str(b)) => {
            let a = crate::normalize::py_strip(a);
            let b = crate::normalize::py_strip(b);
            !a.is_empty() && !b.is_empty() && a == b
        }
        _ => false,
    }
}

pub fn verify(gold: &[Item], target: &[Item], o: &VerifyOpts) -> bool {
    verify_with_timeout(gold, target, o, Some(5.0))
}

/// Upstream semantics: each (gold, target) comparison gets its own timeout and
/// counts as False when it expires.
pub fn verify_with_timeout(gold: &[Item], target: &[Item], o: &VerifyOpts, timeout: Option<f64>) -> bool {
    verify_detail(gold, target, o, timeout).0
}

/// `(matched, any comparison timed out)`.
pub fn verify_detail(gold: &[Item], target: &[Item], o: &VerifyOpts, timeout: Option<f64>) -> (bool, bool) {
    let mut timed_out = false;
    for g in gold {
        for t in target {
            let (r, hit) = crate::deadline::run(timeout, || compare_items(g, t, o));
            if r && !hit {
                return (true, timed_out);
            }
            timed_out |= hit;
        }
    }
    (false, timed_out)
}

#[allow(dead_code)]
fn _z(b: &BigInt) -> bool {
    b.is_zero()
}
