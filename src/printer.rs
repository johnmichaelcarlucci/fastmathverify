//! SymPy-style `str()` printing (best effort; used for display and the few
//! places upstream compares printed forms).

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};

use crate::expr::*;
use crate::symbols::display_name;

pub fn to_str(e: &Expr) -> String {
    p(e)
}

/// Format a SymPy Float from its exact decimal value.
pub fn float_str(r: &Rat) -> String {
    if r.is_zero() {
        return "0.0".into();
    }
    let neg = r.is_negative();
    let r = r.abs();
    // Decimal digits of r (r came from a terminating decimal literal).
    let (digits, exp10) = decimal_digits(&r, 60);
    let sig = digits.trim_end_matches('0').len().max(1);
    let prec = sig.max(15);
    let (digits, exp10) = round_digits(&digits, exp10, prec);
    let mut s = String::new();
    if neg {
        s.push('-');
    }
    if exp10 < -4 {
        s.push_str(&digits[..1]);
        s.push('.');
        s.push_str(&digits[1..]);
        s.push_str(&format!("e{}", exp10));
        return s;
    }
    if exp10 >= prec as i64 - 1 {
        s.push_str(&digits);
        for _ in 0..(exp10 - (prec as i64 - 1)) {
            s.push('0');
        }
        s.push('.');
        return s;
    }
    if exp10 < 0 {
        s.push_str("0.");
        for _ in 0..(-exp10 - 1) {
            s.push('0');
        }
        s.push_str(&digits);
    } else {
        let k = (exp10 + 1) as usize;
        s.push_str(&digits[..k]);
        s.push('.');
        s.push_str(&digits[k..]);
    }
    s
}

/// First `n` significant decimal digits of positive r, and the power of ten of the first digit.
fn decimal_digits(r: &Rat, n: usize) -> (String, i64) {
    let ten = BigInt::from(10);
    let mut num = r.numer().clone();
    let den = r.denom().clone();
    let mut exp10: i64 = 0;
    // normalise so that 1 <= num/den < 10
    while num >= &den * &ten {
        if (&num).is_multiple_of(&ten) {
            num /= &ten;
        } else {
            break;
        }
        exp10 += 1;
    }
    let mut den = den;
    while num >= &den * &ten {
        den *= &ten;
        exp10 += 1;
    }
    while num < den {
        num *= &ten;
        exp10 -= 1;
    }
    let mut out = String::new();
    for _ in 0..n {
        let (q, rem) = num.div_rem(&den);
        out.push_str(&q.to_string());
        num = rem * &ten;
    }
    (out, exp10)
}

fn round_digits(d: &str, exp10: i64, prec: usize) -> (String, i64) {
    let mut v: Vec<u8> = d.bytes().map(|b| b - b'0').collect();
    if v.len() <= prec {
        while v.len() < prec {
            v.push(0);
        }
        return (v.iter().map(|x| (x + b'0') as char).collect(), exp10);
    }
    let round_up = v[prec] >= 5;
    v.truncate(prec);
    let mut exp10 = exp10;
    if round_up {
        let mut i = prec;
        loop {
            if i == 0 {
                v.insert(0, 1);
                v.truncate(prec);
                exp10 += 1;
                break;
            }
            i -= 1;
            if v[i] == 9 {
                v[i] = 0;
            } else {
                v[i] += 1;
                break;
            }
        }
    }
    (v.iter().map(|x| (x + b'0') as char).collect(), exp10)
}

fn num_str(n: &Num) -> String {
    match n {
        Num::Int(i) => i.to_string(),
        Num::Rat(r) => format!("{}/{}", r.numer(), r.denom()),
        Num::Float { exact, .. } => float_str(exact),
    }
}

fn prec(e: &Expr) -> u8 {
    match e {
        Expr::Add(_) => 1,
        Expr::Mul(_) => 2,
        Expr::Num(Num::Rat(_)) => 2,
        Expr::Num(n) if n.to_rat().is_negative() => 1,
        Expr::Pow(..) => 3,
        _ => 4,
    }
}

fn paren(e: &Expr, level: u8) -> String {
    let s = p(e);
    if prec(e) < level { format!("({s})") } else { s }
}

fn is_neg_one(e: &Expr) -> bool {
    matches!(e, Expr::Num(Num::Int(i)) if *i == BigInt::from(-1))
}

fn neg_int_exp(e: &Expr) -> Option<BigInt> {
    if let Expr::Pow(_, x) = e {
        if let Expr::Num(Num::Int(i)) = &**x {
            if i.is_negative() {
                return Some(-i.clone());
            }
        }
    }
    None
}

fn p(e: &Expr) -> String {
    match e {
        Expr::Num(n) => num_str(n),
        Expr::Sym(s) => display_name(s).to_string(),
        Expr::Pi => "pi".into(),
        Expr::E => "E".into(),
        Expr::I => "I".into(),
        Expr::EulerGamma => "EulerGamma".into(),
        Expr::Infinity => "oo".into(),
        Expr::NegInfinity => "-oo".into(),
        Expr::ComplexInfinity => "zoo".into(),
        Expr::NaN => "nan".into(),
        Expr::Add(v) => {
            let mut s = String::new();
            for (i, t) in v.iter().enumerate() {
                let ts = paren(t, 1);
                if i == 0 {
                    s.push_str(&ts);
                } else if let Some(rest) = ts.strip_prefix('-') {
                    s.push_str(" - ");
                    s.push_str(rest);
                } else {
                    s.push_str(" + ");
                    s.push_str(&ts);
                }
            }
            s
        }
        Expr::Mul(v) => {
            let mut sign = "";
            let mut items: &[Expr] = v;
            if v.len() > 1 && is_neg_one(&v[0]) {
                sign = "-";
                items = &v[1..];
            }
            let mut num = Vec::new();
            let mut den = Vec::new();
            for t in items {
                if let Some(k) = neg_int_exp(t) {
                    let Expr::Pow(b, _) = t else { unreachable!() };
                    if k == BigInt::from(1) {
                        den.push(paren(b, 3));
                    } else {
                        den.push(format!("{}**{}", paren(b, 4), k));
                    }
                } else {
                    num.push(paren(t, 2));
                }
            }
            let ns = if num.is_empty() { "1".to_string() } else { num.join("*") };
            if den.is_empty() {
                format!("{sign}{ns}")
            } else if den.len() == 1 {
                format!("{sign}{ns}/{}", den[0])
            } else {
                format!("{sign}{ns}/({})", den.join("*"))
            }
        }
        Expr::Pow(b, x) => {
            if let Expr::Num(Num::Rat(r)) = &**x {
                if *r.numer() == BigInt::from(1) && *r.denom() == BigInt::from(2) {
                    return format!("sqrt({})", p(b));
                }
            }
            if is_neg_one(x) {
                return format!("1/{}", paren(b, 4));
            }
            format!("{}**{}", paren(b, 4), paren(x, 4))
        }
        Expr::Func(f, args) => {
            let a: Vec<String> = args.iter().map(p).collect();
            format!("{}({})", f.name(), a.join(", "))
        }
        Expr::Uneval(x) => p(x),
        Expr::Rel(op, a, b) => match op {
            RelOp::Eq | RelOp::Ne => format!("{}({}, {})", op.sym(), p(a), p(b)),
            _ => format!("{} {} {}", p(a), op.sym(), p(b)),
        },
        Expr::And(v) => v.iter().map(|x| format!("({})", p(x))).collect::<Vec<_>>().join(" & "),
        Expr::Not(x) => format!("~({})", p(x)),
        Expr::FiniteSet { args, .. } => format!("{{{}}}", args.iter().map(p).collect::<Vec<_>>().join(", ")),
        Expr::Tuple(v) => {
            if v.len() == 1 {
                format!("({},)", p(&v[0]))
            } else {
                format!("({})", v.iter().map(p).collect::<Vec<_>>().join(", "))
            }
        }
        Expr::Interval { start, end, lopen, ropen } => {
            let lopen = *lopen && !matches!(**start, Expr::NegInfinity);
            let ropen = *ropen && !matches!(**end, Expr::Infinity);
            let kind = match (lopen, ropen) {
                (false, false) => "Interval",
                (true, true) => "Interval.open",
                (true, false) => "Interval.Lopen",
                (false, true) => "Interval.Ropen",
            };
            format!("{kind}({}, {})", p(start), p(end))
        }
        Expr::Union(v) => format!("Union({})", v.iter().map(p).collect::<Vec<_>>().join(", ")),
        Expr::Intersection(v) => format!("Intersection({})", v.iter().map(p).collect::<Vec<_>>().join(", ")),
        Expr::Complement(a, b) => format!("Complement({}, {})", p(a), p(b)),
        Expr::SetConst(c) => format!("{c:?}"),
        Expr::EmptySet => "EmptySet".into(),
        Expr::Matrix { rows, cols, data } => {
            let rs: Vec<String> = (0..*rows)
                .map(|i| format!("[{}]", (0..*cols).map(|j| p(&data[i * cols + j])).collect::<Vec<_>>().join(", ")))
                .collect();
            format!("Matrix([{}])", rs.join(", "))
        }
        Expr::Derivative(f, v) => format!("Derivative({}, {})", p(f), p(v)),
        Expr::Integral { f, var, bounds } => match bounds {
            Some((a, b)) => format!("Integral({}, ({}, {}, {}))", p(f), p(var), p(a), p(b)),
            None => format!("Integral({}, {})", p(f), p(var)),
        },
        Expr::Sum { f, var, lo, hi, product } => {
            let n = if *product { "Product" } else { "Sum" };
            format!("{n}({}, ({}, {}, {}))", p(f), p(var), p(lo), p(hi))
        }
        Expr::Limit { f, var, to, dir_plus } => {
            format!("Limit({}, {}, {}, dir='{}')", p(f), p(var), p(to), if *dir_plus { "+" } else { "-" })
        }
    }
}

#[allow(dead_code)]
fn _u(b: &BigInt) -> Option<i64> {
    b.to_i64()
}
