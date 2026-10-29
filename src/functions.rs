//! Evaluating constructors that latex2sympy calls (binomial, gamma, exp, gcd/lcm...).

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};

use crate::expr::*;

/// sympy.binomial(n, k) (evaluated for integer k >= 0 and rational n).
pub fn binomial(n: Expr, k: Expr) -> Expr {
    if let (Some(nr), Expr::Num(Num::Int(kk))) = (n.as_rational(), &k) {
        if kk.is_negative() {
            if nr.is_integer() && !nr.is_negative() {
                return int(0);
            }
        } else if let Some(kv) = kk.to_u64() {
            if nr.is_integer() && !nr.is_negative() && Rat::from_integer(kk.clone()) > nr {
                return int(0);
            }
            if kv <= 100_000 {
                let mut acc = Rat::one();
                for i in 0..kv {
                    acc = acc * (nr.clone() - Rat::from_integer(BigInt::from(i)))
                        / Rat::from_integer(BigInt::from(i + 1));
                }
                return Expr::Num(Num::rat(acc));
            }
        }
    }
    Expr::Func(Func::Binomial, vec![n, k])
}

/// sympy.log(x, base, evaluate=False) keeps both args.
pub fn log_with_base(arg: Expr, base: Expr) -> Expr {
    Expr::Func(Func::Log, vec![arg, base])
}

/// sympy.gamma(x): evaluated for positive integers and half-integers.
pub fn gamma(x: Expr) -> Expr {
    if let Some(r) = x.as_rational() {
        if r.is_integer() && r.is_positive() {
            if let Some(n) = r.to_integer().to_u64() {
                if n <= 2000 {
                    let mut acc = BigInt::one();
                    for i in 2..n {
                        acc *= i;
                    }
                    return Expr::Num(Num::Int(acc));
                }
            }
        }
    }
    Expr::Func(Func::Gamma, vec![x])
}

/// sympy.exp(x) evaluated: exp(0) = 1, exp(1) = E.
pub fn exp_eval(x: Expr) -> Expr {
    match &x {
        Expr::Num(n) if n.is_integer() && n.is_zero() => int(1),
        Expr::Num(Num::Int(i)) if i.is_one() => Expr::E,
        _ => Expr::Func(Func::Exp, vec![x]),
    }
}

/// `1 / r` evaluated, for \sqrt[r]{x}.
pub fn reciprocal(r: Expr) -> Expr {
    match &r {
        Expr::Num(Num::Int(i)) => rational(BigInt::one(), i.clone()),
        Expr::Num(Num::Rat(q)) => Expr::Num(Num::rat(q.recip())),
        _ => inv(r),
    }
}

/// UnevaluatedExpr(gcd(args)) / lcm, computed for rational arguments.
pub fn gcd_lcm(name: &str, args: Vec<Expr>) -> Result<Expr, String> {
    let rats: Option<Vec<Rat>> = args.iter().map(|a| a.as_rational()).collect();
    let Some(rats) = rats else {
        let f = if name == "gcd" { Func::Gcd } else { Func::Lcm };
        return Ok(Expr::Uneval(Box::new(Expr::Func(f, args))));
    };
    if rats.is_empty() {
        return Err("empty gcd".into());
    }
    // gcd/lcm of rationals: gcd(nums)/lcm(dens), lcm(nums)/gcd(dens)
    let mut acc = rats[0].abs();
    for r in &rats[1..] {
        let r = r.abs();
        acc = if name == "gcd" {
            Rat::new(acc.numer().gcd(r.numer()), acc.denom().lcm(r.denom()))
        } else {
            Rat::new(acc.numer().lcm(r.numer()), acc.denom().gcd(r.denom()))
        };
    }
    Ok(Expr::Uneval(Box::new(Expr::Num(Num::rat(acc)))))
}

pub fn is_zero_int(e: &Expr) -> bool {
    matches!(e, Expr::Num(Num::Int(i)) if i.is_zero())
}
