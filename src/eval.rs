//! Exact (rational) and numeric (complex f64) evaluation of expressions.

use std::cmp::Ordering;
use std::collections::HashMap;

use num_bigint::BigInt;
use num_complex::Complex64 as C;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};

use crate::expr::*;

const MAX_BITS: u64 = 1 << 16;

fn big_ok(r: &Rat) -> bool {
    r.numer().bits() <= MAX_BITS && r.denom().bits() <= MAX_BITS
}

/// Exact rational value, if the expression is rational-valued and contains no Floats.
pub fn exact(e: &Expr) -> Option<Rat> {
    let r = match e {
        Expr::Num(Num::Int(i)) => Rat::from_integer(i.clone()),
        Expr::Num(Num::Rat(r)) => r.clone(),
        Expr::Num(Num::Float { .. }) => return None,
        Expr::Uneval(x) => exact(x)?,
        Expr::Add(v) => {
            let mut acc = Rat::zero();
            for x in v {
                acc += exact(x)?;
            }
            acc
        }
        Expr::Mul(v) => {
            let mut acc = Rat::one();
            for x in v {
                acc *= exact(x)?;
                if !big_ok(&acc) {
                    return None;
                }
            }
            acc
        }
        Expr::Pow(b, x) => exact_pow(&exact(b)?, &exact(x)?)?,
        Expr::Func(f, args) => exact_func(f, args)?,
        Expr::Sum { f, var, lo, hi, product } => {
            let Expr::Sym(v) = &**var else { return None };
            let lo = exact(lo)?;
            let hi = exact(hi)?;
            if !lo.is_integer() || !hi.is_integer() {
                return None;
            }
            let (lo, hi) = (lo.to_integer().to_i64()?, hi.to_integer().to_i64()?);
            if hi - lo > 20_000 {
                return None;
            }
            let mut acc = if *product { Rat::one() } else { Rat::zero() };
            let mut i = lo;
            while i <= hi {
                if (i & 255) == 0 && crate::deadline::expired() {
                    return None;
                }
                let t = exact(&f.subs_sym(v, &int(i)))?;
                if *product { acc *= t } else { acc += t }
                if !big_ok(&acc) {
                    return None;
                }
                i += 1;
            }
            acc
        }
        _ => return None,
    };
    if !big_ok(&r) {
        return None;
    }
    Some(r)
}

fn exact_pow(b: &Rat, x: &Rat) -> Option<Rat> {
    if x.is_integer() {
        let n = x.to_integer().to_i64()?;
        if b.is_zero() {
            return if n > 0 { Some(Rat::zero()) } else if n == 0 { Some(Rat::one()) } else { None };
        }
        let bits = (b.numer().bits().max(b.denom().bits())) as i64;
        if bits * n.abs() > MAX_BITS as i64 {
            return None;
        }
        let p = num_traits::pow(b.clone(), n.unsigned_abs() as usize);
        return Some(if n < 0 { p.recip() } else { p });
    }
    // b^(p/q): exact only for perfect q-th powers of non-negative bases.
    if b.is_negative() {
        return None;
    }
    let q = x.denom().to_u32()?;
    if q > 64 {
        return None;
    }
    let rn = b.numer().nth_root(q);
    let rd = b.denom().nth_root(q);
    if num_traits::pow(rn.clone(), q as usize) != *b.numer() || num_traits::pow(rd.clone(), q as usize) != *b.denom() {
        return None;
    }
    exact_pow(&Rat::new(rn, rd), &Rat::from_integer(x.numer().clone()))
}

fn exact_func(f: &Func, args: &[Expr]) -> Option<Rat> {
    let a = |i: usize| args.get(i).and_then(exact);
    Some(match f {
        Func::Abs => a(0)?.abs(),
        Func::Floor => a(0)?.floor(),
        Func::Ceiling => a(0)?.ceil(),
        Func::Factorial => {
            let n = a(0)?;
            if !n.is_integer() || n.is_negative() {
                return None;
            }
            let n = n.to_integer().to_u64()?;
            if n > 3000 {
                return None;
            }
            let mut acc = BigInt::one();
            for i in 2..=n {
                acc *= i;
            }
            Rat::from_integer(acc)
        }
        Func::Binomial => {
            let r = crate::functions::binomial(Expr::Num(Num::rat(a(0)?)), Expr::Num(Num::rat(a(1)?)));
            return match r {
                Expr::Num(n) if !n.is_float() => Some(n.to_rat()),
                _ => None,
            };
        }
        Func::Gamma => {
            let n = a(0)?;
            if !n.is_integer() || !n.is_positive() {
                return None;
            }
            return exact_func(&Func::Factorial, &[Expr::Num(Num::rat(n - Rat::one()))]);
        }
        Func::Max | Func::Min => {
            let mut vals = Vec::new();
            for x in args {
                vals.push(exact(x)?);
            }
            let it = vals.into_iter();
            if *f == Func::Max { it.max()? } else { it.min()? }
        }
        Func::Mod => {
            let (x, m) = (a(0)?, a(1)?);
            if m.is_zero() {
                return None;
            }
            x.clone() - m.clone() * (x / m).floor()
        }
        Func::Gcd | Func::Lcm => {
            let mut it = args.iter().map(exact);
            let mut acc = it.next()??.abs();
            for r in it {
                let r = r?.abs();
                acc = if *f == Func::Gcd {
                    Rat::new(acc.numer().gcd(r.numer()), acc.denom().lcm(r.denom()))
                } else {
                    Rat::new(acc.numer().lcm(r.numer()), acc.denom().gcd(r.denom()))
                };
            }
            acc
        }
        _ => return None,
    })
}

// ---------------- numeric evaluation ----------------

pub type Env = HashMap<String, C>;

fn ci(x: f64) -> C {
    C::new(x, 0.0)
}

fn is_realish(z: C) -> bool {
    z.im.abs() <= 1e-12 * (1.0 + z.re.abs())
}

/// Numeric value with symbols looked up in `env`. None if undefined/unsupported.
pub fn numeric(e: &Expr, env: &Env) -> Option<C> {
    let v = match e {
        Expr::Num(n) => ci(n.to_f64()),
        Expr::Sym(s) => *env.get(s)?,
        Expr::Pi => ci(std::f64::consts::PI),
        Expr::E => ci(std::f64::consts::E),
        Expr::I => C::new(0.0, 1.0),
        Expr::EulerGamma => ci(0.577_215_664_901_532_9),
        Expr::Uneval(x) => numeric(x, env)?,
        Expr::Add(v) => {
            let mut acc = ci(0.0);
            for x in v {
                acc += numeric(x, env)?;
            }
            acc
        }
        Expr::Mul(v) => {
            let mut acc = ci(1.0);
            for x in v {
                acc *= numeric(x, env)?;
            }
            acc
        }
        Expr::Pow(b, x) => {
            if let (Some(br), Some(xr)) = (exact(b), exact(x)) {
                if let Some(r) = exact_pow(&br, &xr) {
                    return Some(ci(rat_to_f64(&r)));
                }
            }
            let bv = numeric(b, env)?;
            let xv = numeric(x, env)?;
            cpow(bv, xv)?
        }
        Expr::Func(f, args) => {
            if let Some(r) = exact_func(f, args) {
                return Some(ci(rat_to_f64(&r)));
            }
            numeric_func(f, args, env)?
        }
        Expr::Derivative(f, var) => {
            let Expr::Sym(v) = &**var else { return None };
            let d = crate::diff::diff(f, v)?;
            numeric(&d, env)?
        }
        Expr::Integral { f, var, bounds: Some((a, b)) } => {
            let Expr::Sym(v) = &**var else { return None };
            let a = numeric(a, env)?;
            let b = numeric(b, env)?;
            if !is_realish(a) || !is_realish(b) {
                return None;
            }
            integrate(f, v, a.re, b.re, env)?
        }
        Expr::Sum { f, var, lo, hi, product } => {
            if let Some(r) = exact(e) {
                return Some(ci(rat_to_f64(&r)));
            }
            let Expr::Sym(v) = &**var else { return None };
            let (lo, hi) = (int_bound(lo, env)?, int_bound(hi, env)?);
            if hi - lo > 100_000 {
                return None;
            }
            let mut acc = if *product { ci(1.0) } else { ci(0.0) };
            let mut env2 = env.clone();
            for i in lo..=hi {
                if (i & 255) == 0 && crate::deadline::expired() {
                    return None;
                }
                env2.insert(v.clone(), ci(i as f64));
                let t = numeric(f, &env2)?;
                if *product { acc *= t } else { acc += t }
            }
            acc
        }
        _ => return None,
    };
    if v.re.is_finite() && v.im.is_finite() { Some(v) } else { None }
}

// A sum bound: exact when constant, else numeric under `env` (a bound symbol
// sampled at an integer) when the value is an integer.
fn int_bound(e: &Expr, env: &Env) -> Option<i64> {
    if let Some(r) = exact(e) {
        return r.to_integer().to_i64();
    }
    let z = numeric(e, env)?;
    if z.im != 0.0 || z.re.fract() != 0.0 || z.re.abs() > 1e9 {
        return None;
    }
    Some(z.re as i64)
}

fn cpow(b: C, x: C) -> Option<C> {
    if b == ci(0.0) {
        return if x.re > 0.0 { Some(ci(0.0)) } else if x == ci(0.0) { Some(ci(1.0)) } else { None };
    }
    if x.im == 0.0 && x.re.fract() == 0.0 && x.re.abs() <= 1e6 {
        return Some(b.powi(x.re as i32));
    }
    if b.im == 0.0 && b.re > 0.0 && x.im == 0.0 {
        return Some(ci(b.re.powf(x.re)));
    }
    Some((x * b.ln()).exp())
}

fn real_arg(args: &[Expr], i: usize, env: &Env) -> Option<f64> {
    let z = numeric(args.get(i)?, env)?;
    if is_realish(z) { Some(z.re) } else { None }
}

fn snap(x: f64) -> f64 {
    let r = x.round();
    if (x - r).abs() <= 1e-9 * (1.0 + x.abs()) { r } else { x }
}

fn numeric_func(f: &Func, args: &[Expr], env: &Env) -> Option<C> {
    let z = || numeric(args.first()?, env);
    Some(match f {
        Func::Sin => z()?.sin(),
        Func::Cos => z()?.cos(),
        Func::Tan => z()?.tan(),
        Func::Csc => z()?.sin().inv(),
        Func::Sec => z()?.cos().inv(),
        Func::Cot => z()?.tan().inv(),
        Func::Asin => z()?.asin(),
        Func::Acos => z()?.acos(),
        Func::Atan => z()?.atan(),
        Func::Acsc => z()?.inv().asin(),
        Func::Asec => z()?.inv().acos(),
        Func::Acot => {
            let v = z()?;
            if v == ci(0.0) { ci(std::f64::consts::FRAC_PI_2) } else { v.inv().atan() }
        }
        Func::Sinh => z()?.sinh(),
        Func::Cosh => z()?.cosh(),
        Func::Tanh => z()?.tanh(),
        Func::Asinh => z()?.asinh(),
        Func::Acosh => z()?.acosh(),
        Func::Atanh => z()?.atanh(),
        Func::Exp => z()?.exp(),
        Func::Log => {
            let x = z()?;
            if x == ci(0.0) {
                return None;
            }
            let l = x.ln();
            if args.len() == 2 {
                let b = numeric(&args[1], env)?;
                let lb = b.ln();
                if lb == ci(0.0) {
                    return None;
                }
                l / lb
            } else {
                l
            }
        }
        Func::Abs => ci(z()?.norm()),
        Func::Floor => ci(snap(real_arg(args, 0, env)?).floor()),
        Func::Ceiling => ci(snap(real_arg(args, 0, env)?).ceil()),
        Func::Factorial => {
            let x = real_arg(args, 0, env)?;
            if x < 0.0 && x.fract() == 0.0 {
                return None;
            }
            ci(gamma_f(x + 1.0)?)
        }
        Func::Gamma => ci(gamma_f(real_arg(args, 0, env)?)?),
        Func::Binomial => {
            let n = real_arg(args, 0, env)?;
            let k = real_arg(args, 1, env)?;
            ci(gamma_f(n + 1.0)? / (gamma_f(k + 1.0)? * gamma_f(n - k + 1.0)?))
        }
        Func::Max | Func::Min => {
            let mut best: Option<f64> = None;
            for i in 0..args.len() {
                let v = real_arg(args, i, env)?;
                best = Some(match best {
                    None => v,
                    Some(b) => if (*f == Func::Max) == (v > b) { v } else { b },
                });
            }
            ci(best?)
        }
        Func::Mod => {
            let a = real_arg(args, 0, env)?;
            let m = real_arg(args, 1, env)?;
            if m == 0.0 {
                return None;
            }
            ci(a - m * snap(a / m).floor())
        }
        Func::Gcd | Func::Lcm => return None,
        Func::Undefined(name) => {
            // Opaque function: deterministic pseudo-random value of (name, args).
            let mut h: u64 = 1469598103934665603;
            for b in name.bytes() {
                h = (h ^ b as u64).wrapping_mul(1099511628211);
            }
            for a in args {
                let v = numeric(a, env)?;
                for x in [v.re, v.im] {
                    let q = (x * 1e9).round() as i64 as u64;
                    h = (h ^ q).wrapping_mul(1099511628211);
                }
            }
            ci(((h >> 11) as f64 / (1u64 << 53) as f64) * 4.0 - 2.0)
        }
    })
}

/// Real gamma function (Lanczos), None at poles.
pub fn gamma_f(x: f64) -> Option<f64> {
    if x <= 0.0 && x.fract() == 0.0 {
        return None;
    }
    if x < 0.5 {
        return Some(std::f64::consts::PI / ((std::f64::consts::PI * x).sin() * gamma_f(1.0 - x)?));
    }
    if x.fract() == 0.0 && x <= 171.0 {
        let mut acc = 1.0f64;
        let mut i = 2.0;
        while i < x {
            acc *= i;
            i += 1.0;
        }
        return Some(acc);
    }
    const G: f64 = 7.0;
    const COEF: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    let x = x - 1.0;
    let mut a = COEF[0];
    let t = x + G + 0.5;
    for (i, c) in COEF.iter().enumerate().skip(1) {
        a += c / (x + i as f64);
    }
    Some((2.0 * std::f64::consts::PI).sqrt() * t.powf(x + 0.5) * (-t).exp() * a)
}

fn integrate(f: &Expr, v: &str, a: f64, b: f64, env: &Env) -> Option<C> {
    // Composite Gauss-Legendre (5 points) on 64 panels.
    const X: [f64; 5] = [0.0, -0.538_469_310_105_683, 0.538_469_310_105_683, -0.906_179_845_938_664, 0.906_179_845_938_664];
    const W: [f64; 5] = [0.568_888_888_888_889, 0.478_628_670_499_366, 0.478_628_670_499_366, 0.236_926_885_056_189, 0.236_926_885_056_189];
    let n = 64;
    let h = (b - a) / n as f64;
    let mut env2 = env.clone();
    let mut acc = ci(0.0);
    for p in 0..n {
        let mid = a + h * (p as f64 + 0.5);
        for k in 0..5 {
            env2.insert(v.to_string(), ci(mid + 0.5 * h * X[k]));
            acc += numeric(f, &env2)? * (W[k] * 0.5 * h);
        }
    }
    Some(acc)
}

// ---------------- comparisons ----------------

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
enum Ext {
    NegInf,
    Fin(f64),
    PosInf,
}

/// Compare two closed (symbol-free) real expressions, like SymPy's `<`/`<=` on
/// numbers. None if either side is symbolic, non-real or not evaluable.
pub fn compare_real(a: &Expr, b: &Expr) -> Option<Ordering> {
    if let (Some(x), Some(y)) = (exact_or_float_rat(a), exact_or_float_rat(b)) {
        return Some(x.cmp(&y));
    }
    let x = ext_value(a)?;
    let y = ext_value(b)?;
    match (x, y) {
        (Ext::Fin(p), Ext::Fin(q)) => {
            if (p - q).abs() <= 1e-12 * (1.0 + p.abs().max(q.abs())) {
                Some(Ordering::Equal)
            } else {
                p.partial_cmp(&q)
            }
        }
        _ => x.partial_cmp(&y),
    }
}

/// Exact rational value allowing Floats (their decimal value).
pub fn exact_or_float_rat(e: &Expr) -> Option<Rat> {
    match e {
        Expr::Num(n) => Some(n.to_rat()),
        _ => exact(e),
    }
}

fn ext_value(e: &Expr) -> Option<Ext> {
    match e {
        Expr::Infinity => Some(Ext::PosInf),
        Expr::NegInfinity => Some(Ext::NegInf),
        Expr::Mul(v) if v.len() == 2 && matches!(v[1], Expr::Infinity | Expr::NegInfinity) => {
            let s = ext_value(&v[0])?;
            let pos = matches!(v[1], Expr::Infinity);
            match s {
                Ext::Fin(x) if x > 0.0 => Some(if pos { Ext::PosInf } else { Ext::NegInf }),
                Ext::Fin(x) if x < 0.0 => Some(if pos { Ext::NegInf } else { Ext::PosInf }),
                _ => None,
            }
        }
        _ => {
            if e.has_free_symbols() {
                return None;
            }
            let z = numeric(e, &Env::new())?;
            if is_realish(z) { Some(Ext::Fin(z.re)) } else { None }
        }
    }
}

/// `is_real is not False` (used by Interval): false only for provably non-real values.
pub fn maybe_real(e: &Expr) -> bool {
    match e {
        Expr::Sym(s) => crate::symbols::is_real_symbol(s),
        Expr::Infinity | Expr::NegInfinity => true,
        Expr::ComplexInfinity | Expr::NaN | Expr::I => false,
        _ if e.is_set() || e.is_relational() || e.is_matrix() => false,
        _ => {
            if e.has_free_symbols() {
                return true;
            }
            match numeric(e, &Env::new()) {
                Some(z) => is_realish(z),
                None => true,
            }
        }
    }
}

/// Python truthiness of a SymPy expression: numeric zero is False.
pub fn truthy(e: &Expr) -> bool {
    !matches!(e, Expr::Num(n) if n.is_zero())
}

#[allow(dead_code)]
fn _int_check(b: &BigInt) -> bool {
    b.is_even()
}
