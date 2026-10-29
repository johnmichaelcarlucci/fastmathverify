//! Symbolic differentiation for the elementary functions latex2sympy produces.

use crate::expr::*;

fn has(e: &Expr, v: &str) -> bool {
    e.free_symbols().iter().any(|s| s == v)
}

pub fn diff(e: &Expr, v: &str) -> Option<Expr> {
    if !has(e, v) {
        return Some(int(0));
    }
    Some(match e {
        Expr::Sym(s) => if s == v { int(1) } else { int(0) },
        Expr::Uneval(x) => diff(x, v)?,
        Expr::Add(xs) => Expr::Add(xs.iter().map(|x| diff(x, v)).collect::<Option<Vec<_>>>()?),
        Expr::Mul(xs) => {
            let mut terms = Vec::new();
            for i in 0..xs.len() {
                if !has(&xs[i], v) {
                    continue;
                }
                let mut f = xs.clone();
                f[i] = diff(&xs[i], v)?;
                terms.push(Expr::Mul(f));
            }
            Expr::Add(terms)
        }
        Expr::Pow(b, x) => {
            if !has(x, v) {
                Expr::Mul(vec![(**x).clone(), pow((**b).clone(), Expr::Add(vec![(**x).clone(), int(-1)])), diff(b, v)?])
            } else {
                let lnb = Expr::Func(Func::Log, vec![(**b).clone()]);
                Expr::Mul(vec![
                    e.clone(),
                    Expr::Add(vec![
                        Expr::Mul(vec![diff(x, v)?, lnb]),
                        Expr::Mul(vec![(**x).clone(), diff(b, v)?, inv((**b).clone())]),
                    ]),
                ])
            }
        }
        Expr::Func(f, args) => {
            let a = args.first()?.clone();
            let da = diff(&a, v)?;
            let sq = |x: Expr| pow(x, int(2));
            let sqrt = |x: Expr| pow(x, half());
            let one_minus_sq = |x: Expr| Expr::Add(vec![int(1), Expr::Mul(vec![int(-1), sq(x)])]);
            let outer = match f {
                Func::Sin => Expr::Func(Func::Cos, vec![a]),
                Func::Cos => Expr::Mul(vec![int(-1), Expr::Func(Func::Sin, vec![a])]),
                Func::Tan => sq(Expr::Func(Func::Sec, vec![a])),
                Func::Exp => Expr::Func(Func::Exp, vec![a]),
                Func::Log => {
                    let d = inv(a);
                    if args.len() == 2 {
                        if has(&args[1], v) {
                            return None;
                        }
                        Expr::Mul(vec![d, inv(Expr::Func(Func::Log, vec![args[1].clone()]))])
                    } else {
                        d
                    }
                }
                Func::Asin => inv(sqrt(one_minus_sq(a))),
                Func::Acos => Expr::Mul(vec![int(-1), inv(sqrt(one_minus_sq(a)))]),
                Func::Atan => inv(Expr::Add(vec![int(1), sq(a)])),
                Func::Sinh => Expr::Func(Func::Cosh, vec![a]),
                Func::Cosh => Expr::Func(Func::Sinh, vec![a]),
                Func::Sec => Expr::Mul(vec![Expr::Func(Func::Sec, vec![a.clone()]), Expr::Func(Func::Tan, vec![a])]),
                Func::Csc => Expr::Mul(vec![int(-1), Expr::Func(Func::Csc, vec![a.clone()]), Expr::Func(Func::Cot, vec![a])]),
                Func::Cot => Expr::Mul(vec![int(-1), sq(Expr::Func(Func::Csc, vec![a]))]),
                _ => return None,
            };
            Expr::Mul(vec![outer, da])
        }
        Expr::Derivative(inner, w) => {
            let Expr::Sym(wn) = &**w else { return None };
            diff(&diff(inner, wn)?, v)?
        }
        _ => return None,
    })
}
