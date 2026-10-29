//! Expression tree mirroring the SymPy objects latex2sympy2_extended builds.
//!
//! Construction follows SymPy's `evaluate=False` behaviour where the upstream
//! converter uses it, so structure (and printing) stays close to upstream.

use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

pub type Rat = BigRational;

#[derive(Clone, Debug)]
pub enum Num {
    Int(BigInt),
    Rat(Rat),
    /// A SymPy Float: the value parsed from text is kept exactly as a rational,
    /// `f` is the f64 approximation used for transcendental arithmetic.
    Float { exact: Rat, f: f64 },
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Num::Float { exact: a, .. }, Num::Float { exact: b, .. }) => a == b,
            (Num::Float { .. }, _) | (_, Num::Float { .. }) => {
                // SymPy: Float(2.0) == Integer(2) is True
                self.to_rat() == other.to_rat()
            }
            _ => self.to_rat() == other.to_rat(),
        }
    }
}

impl Num {
    pub fn int(i: i64) -> Num {
        Num::Int(BigInt::from(i))
    }
    /// Reduced rational; denominator 1 collapses to an Integer (like sympy.Rational).
    pub fn rat(r: Rat) -> Num {
        if r.denom().is_one() {
            Num::Int(r.numer().clone())
        } else {
            Num::Rat(r)
        }
    }
    pub fn float_from_rat(r: Rat) -> Num {
        let f = rat_to_f64(&r);
        Num::Float { exact: r, f }
    }
    pub fn to_rat(&self) -> Rat {
        match self {
            Num::Int(i) => Rat::from_integer(i.clone()),
            Num::Rat(r) => r.clone(),
            Num::Float { exact, .. } => exact.clone(),
        }
    }
    pub fn to_f64(&self) -> f64 {
        match self {
            Num::Float { f, .. } => *f,
            _ => rat_to_f64(&self.to_rat()),
        }
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Num::Float { .. })
    }
    pub fn is_integer(&self) -> bool {
        matches!(self, Num::Int(_))
    }
    pub fn is_zero(&self) -> bool {
        self.to_rat().is_zero()
    }
    pub fn neg(&self) -> Num {
        match self {
            Num::Int(i) => Num::Int(-i),
            Num::Rat(r) => Num::Rat(-r),
            Num::Float { exact, f } => Num::Float { exact: -exact, f: -f },
        }
    }
}

pub fn rat_to_f64(r: &Rat) -> f64 {
    match (r.numer().to_f64(), r.denom().to_f64()) {
        (Some(n), Some(d)) if n.is_finite() && d.is_finite() && d != 0.0 => n / d,
        _ => {
            // Scale down huge numerators/denominators before dividing.
            let nb = r.numer().bits() as i64;
            let db = r.denom().bits() as i64;
            let shift = (nb.max(db) - 1000).max(0) as usize;
            let n = (r.numer() >> shift).to_f64().unwrap_or(f64::NAN);
            let d = (r.denom() >> shift).to_f64().unwrap_or(f64::NAN);
            if d == 0.0 {
                if n.is_sign_negative() { f64::NEG_INFINITY } else { f64::INFINITY }
            } else {
                n / d
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RelOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl RelOp {
    pub fn inverse(self) -> RelOp {
        match self {
            RelOp::Lt => RelOp::Gt,
            RelOp::Gt => RelOp::Lt,
            RelOp::Le => RelOp::Ge,
            RelOp::Ge => RelOp::Le,
            o => o,
        }
    }
    pub fn sym(self) -> &'static str {
        match self {
            RelOp::Eq => "Eq",
            RelOp::Ne => "Ne",
            RelOp::Lt => "<",
            RelOp::Le => "<=",
            RelOp::Gt => ">",
            RelOp::Ge => ">=",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetConst {
    Naturals,
    Integers,
    Rationals,
    Reals,
    Complexes,
    Primes,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Func {
    Sin, Cos, Tan, Csc, Sec, Cot,
    Asin, Acos, Atan, Acsc, Asec, Acot,
    Sinh, Cosh, Tanh, Asinh, Acosh, Atanh,
    Log,       // natural log, one argument (sympy.log(x, b) builds log(x)/log(b))
    Exp,
    Abs,
    Floor,
    Ceiling,
    Factorial,
    Binomial,
    Gamma,
    Max,
    Min,
    Mod,
    Gcd,
    Lcm,
    /// Unknown function application, e.g. f(x) or \operatorname{foo}(x)
    Undefined(String),
}

impl Func {
    pub fn name(&self) -> &str {
        match self {
            Func::Sin => "sin", Func::Cos => "cos", Func::Tan => "tan",
            Func::Csc => "csc", Func::Sec => "sec", Func::Cot => "cot",
            Func::Asin => "asin", Func::Acos => "acos", Func::Atan => "atan",
            Func::Acsc => "acsc", Func::Asec => "asec", Func::Acot => "acot",
            Func::Sinh => "sinh", Func::Cosh => "cosh", Func::Tanh => "tanh",
            Func::Asinh => "asinh", Func::Acosh => "acosh", Func::Atanh => "atanh",
            Func::Log => "log", Func::Exp => "exp", Func::Abs => "Abs",
            Func::Floor => "floor", Func::Ceiling => "ceiling",
            Func::Factorial => "factorial", Func::Binomial => "binomial",
            Func::Gamma => "gamma", Func::Max => "Max", Func::Min => "Min",
            Func::Mod => "Mod", Func::Gcd => "gcd", Func::Lcm => "lcm",
            Func::Undefined(n) => n,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Num(Num),
    Sym(String),
    Pi,
    E,
    I,
    EulerGamma,
    Infinity,
    NegInfinity,
    ComplexInfinity,
    NaN,
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Func(Func, Vec<Expr>),
    /// sympy.UnevaluatedExpr, used for percentages: x * Uneval(1/100)
    Uneval(Box<Expr>),
    Rel(RelOp, Box<Expr>, Box<Expr>),
    And(Vec<Expr>),
    Not(Box<Expr>),
    /// latex2sympy FiniteSet: `args` sorted+deduplicated, `unsorted` as written.
    FiniteSet { args: Vec<Expr>, unsorted: Vec<Expr> },
    Tuple(Vec<Expr>),
    Interval { start: Box<Expr>, end: Box<Expr>, lopen: bool, ropen: bool },
    Union(Vec<Expr>),
    Intersection(Vec<Expr>),
    Complement(Box<Expr>, Box<Expr>),
    SetConst(SetConst),
    EmptySet,
    Matrix { rows: usize, cols: usize, data: Vec<Expr> },
    Derivative(Box<Expr>, Box<Expr>),
    Integral { f: Box<Expr>, var: Box<Expr>, bounds: Option<(Box<Expr>, Box<Expr>)> },
    Sum { f: Box<Expr>, var: Box<Expr>, lo: Box<Expr>, hi: Box<Expr>, product: bool },
    Limit { f: Box<Expr>, var: Box<Expr>, to: Box<Expr>, dir_plus: bool },
}

pub fn int(i: i64) -> Expr {
    Expr::Num(Num::int(i))
}

pub fn half() -> Expr {
    Expr::Num(Num::Rat(Rat::new(BigInt::one(), BigInt::from(2))))
}

pub fn rational(n: BigInt, d: BigInt) -> Expr {
    if d.is_zero() {
        if n.is_zero() {
            return Expr::NaN;
        }
        return Expr::ComplexInfinity;
    }
    Expr::Num(Num::rat(Rat::new(n, d)))
}

/// The percent marker: Mul(x, UnevaluatedExpr(Rational(1, 100)))
pub fn pct(x: Expr) -> Expr {
    Expr::Mul(vec![
        x,
        Expr::Uneval(Box::new(Expr::Num(Num::Rat(Rat::new(BigInt::one(), BigInt::from(100)))))),
    ])
}

impl Expr {
    pub fn num(&self) -> Option<&Num> {
        match self {
            Expr::Num(n) => Some(n),
            _ => None,
        }
    }
    pub fn is_number_atom(&self) -> bool {
        matches!(self, Expr::Num(_))
    }
    pub fn is_integer_atom(&self) -> bool {
        matches!(self, Expr::Num(Num::Int(_)))
    }
    /// sympy `is_Rational` (Integer or Rational, not Float)
    pub fn as_rational(&self) -> Option<Rat> {
        match self {
            Expr::Num(Num::Int(i)) => Some(Rat::from_integer(i.clone())),
            Expr::Num(Num::Rat(r)) => Some(r.clone()),
            _ => None,
        }
    }
    pub fn is_set(&self) -> bool {
        matches!(
            self,
            Expr::FiniteSet { .. }
                | Expr::Interval { .. }
                | Expr::Union(_)
                | Expr::Intersection(_)
                | Expr::Complement(..)
                | Expr::SetConst(_)
                | Expr::EmptySet
        )
    }
    pub fn is_matrix(&self) -> bool {
        matches!(self, Expr::Matrix { .. })
    }
    pub fn is_relational(&self) -> bool {
        matches!(self, Expr::Rel(..))
    }

    /// Negation as latex2sympy does it: numbers are negated, everything else
    /// becomes Mul(-1, x) (unevaluated, flattened).
    pub fn neg_number_or_mul(self) -> Expr {
        match self {
            Expr::Num(n) => Expr::Num(n.neg()),
            other => mul_flat(int(-1), other),
        }
    }

    pub fn children(&self) -> Vec<&Expr> {
        match self {
            Expr::Add(v) | Expr::Mul(v) | Expr::Func(_, v) | Expr::And(v) | Expr::Tuple(v)
            | Expr::Union(v) | Expr::Intersection(v) => v.iter().collect(),
            Expr::FiniteSet { args, .. } => args.iter().collect(),
            Expr::Matrix { data, .. } => data.iter().collect(),
            Expr::Pow(a, b) | Expr::Rel(_, a, b) | Expr::Complement(a, b) | Expr::Derivative(a, b) => {
                vec![a, b]
            }
            Expr::Uneval(a) | Expr::Not(a) => vec![a],
            Expr::Interval { start, end, .. } => vec![start, end],
            Expr::Integral { f, var, bounds } => {
                let mut v = vec![&**f, &**var];
                if let Some((a, b)) = bounds {
                    v.push(a);
                    v.push(b);
                }
                v
            }
            Expr::Sum { f, var, lo, hi, .. } => vec![f, var, lo, hi],
            Expr::Limit { f, var, to, .. } => vec![f, var, to],
            _ => vec![],
        }
    }

    pub fn free_symbols(&self) -> Vec<String> {
        let mut out = Vec::new();
        self.collect_symbols(&mut out);
        out.sort();
        out.dedup();
        out
    }

    fn collect_symbols(&self, out: &mut Vec<String>) {
        match self {
            Expr::Sym(s) => out.push(s.clone()),
            // Bound variables are not free.
            Expr::Sum { f, var, lo, hi, .. } => {
                let mut inner = Vec::new();
                f.collect_symbols(&mut inner);
                if let Expr::Sym(v) = &**var {
                    inner.retain(|s| s != v);
                }
                out.extend(inner);
                lo.collect_symbols(out);
                hi.collect_symbols(out);
            }
            Expr::Integral { f, var, bounds: Some((a, b)) } => {
                let mut inner = Vec::new();
                f.collect_symbols(&mut inner);
                if let Expr::Sym(v) = &**var {
                    inner.retain(|s| s != v);
                }
                out.extend(inner);
                a.collect_symbols(out);
                b.collect_symbols(out);
            }
            Expr::Limit { f, var, to, .. } => {
                let mut inner = Vec::new();
                f.collect_symbols(&mut inner);
                if let Expr::Sym(v) = &**var {
                    inner.retain(|s| s != v);
                }
                out.extend(inner);
                to.collect_symbols(out);
            }
            Expr::Func(Func::Undefined(_), args) => {
                for a in args {
                    a.collect_symbols(out);
                }
            }
            _ => {
                for c in self.children() {
                    c.collect_symbols(out);
                }
            }
        }
    }

    pub fn has_free_symbols(&self) -> bool {
        !self.free_symbols().is_empty()
    }

    pub fn contains_undefined_func(&self) -> bool {
        match self {
            Expr::Func(Func::Undefined(_), _) => true,
            _ => self.children().iter().any(|c| c.contains_undefined_func()),
        }
    }

    pub fn subs_sym(&self, from: &str, to: &Expr) -> Expr {
        match self {
            Expr::Sym(s) if s == from => to.clone(),
            _ => self.map_children(|c| c.subs_sym(from, to)),
        }
    }

    pub fn map_children(&self, f: impl Fn(&Expr) -> Expr) -> Expr {
        let b = |e: &Expr| Box::new(f(e));
        match self {
            Expr::Add(v) => Expr::Add(v.iter().map(&f).collect()),
            Expr::Mul(v) => Expr::Mul(v.iter().map(&f).collect()),
            Expr::Func(n, v) => Expr::Func(n.clone(), v.iter().map(&f).collect()),
            Expr::And(v) => Expr::And(v.iter().map(&f).collect()),
            Expr::Tuple(v) => Expr::Tuple(v.iter().map(&f).collect()),
            Expr::Union(v) => Expr::Union(v.iter().map(&f).collect()),
            Expr::Intersection(v) => Expr::Intersection(v.iter().map(&f).collect()),
            Expr::FiniteSet { unsorted, .. } => finite_set(unsorted.iter().map(&f).collect()),
            Expr::Matrix { rows, cols, data } => {
                Expr::Matrix { rows: *rows, cols: *cols, data: data.iter().map(&f).collect() }
            }
            Expr::Pow(a, c) => Expr::Pow(b(a), b(c)),
            Expr::Rel(o, a, c) => Expr::Rel(*o, b(a), b(c)),
            Expr::Complement(a, c) => Expr::Complement(b(a), b(c)),
            Expr::Derivative(a, c) => Expr::Derivative(b(a), b(c)),
            Expr::Uneval(a) => Expr::Uneval(b(a)),
            Expr::Not(a) => Expr::Not(b(a)),
            Expr::Interval { start, end, lopen, ropen } => {
                Expr::Interval { start: b(start), end: b(end), lopen: *lopen, ropen: *ropen }
            }
            Expr::Integral { f: g, var, bounds } => Expr::Integral {
                f: b(g),
                var: var.clone(),
                bounds: bounds.as_ref().map(|(x, y)| (b(x), b(y))),
            },
            Expr::Sum { f: g, var, lo, hi, product } => Expr::Sum {
                f: b(g),
                var: var.clone(),
                lo: b(lo),
                hi: b(hi),
                product: *product,
            },
            Expr::Limit { f: g, var, to, dir_plus } => {
                Expr::Limit { f: b(g), var: var.clone(), to: b(to), dir_plus: *dir_plus }
            }
            other => other.clone(),
        }
    }
}

pub fn add_flat(lh: Expr, rh: Expr) -> Expr {
    let mut args = Vec::new();
    match lh {
        Expr::Add(v) => args.extend(v),
        x => args.push(x),
    }
    match rh {
        Expr::Add(v) => args.extend(v),
        x => args.push(x),
    }
    Expr::Add(args)
}

pub fn mul_flat(lh: Expr, rh: Expr) -> Expr {
    let mut args = Vec::new();
    match lh {
        Expr::Mul(v) => args.extend(v),
        x => args.push(x),
    }
    match rh {
        Expr::Mul(v) => args.extend(v),
        x => args.push(x),
    }
    Expr::Mul(args)
}

pub fn pow(b: Expr, e: Expr) -> Expr {
    Expr::Pow(Box::new(b), Box::new(e))
}

pub fn inv(x: Expr) -> Expr {
    pow(x, int(-1))
}

/// Mul(a, Pow(b, -1)) or Rational if both are integers (latex2sympy's division).
pub fn div(a: Expr, b: Expr) -> Expr {
    match (&a, &b) {
        (Expr::Num(Num::Int(x)), Expr::Num(Num::Int(y))) => rational(x.clone(), y.clone()),
        _ => Expr::Mul(vec![a, inv(b)]),
    }
}

/// latex2sympy FiniteSet: deduplicate (by structural equality) and sort.
pub fn finite_set(unsorted: Vec<Expr>) -> Expr {
    let mut args: Vec<Expr> = Vec::new();
    for a in &unsorted {
        if !args.iter().any(|b| b == a) {
            args.push(a.clone());
        }
    }
    args.sort_by(|x, y| crate::order::set_order(x, y));
    Expr::FiniteSet { args, unsorted }
}

pub fn finite_set_or_empty(unsorted: Vec<Expr>) -> Expr {
    if unsorted.is_empty() {
        Expr::EmptySet
    } else {
        finite_set(unsorted)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&crate::printer::to_str(self))
    }
}

pub fn big_to_i64(b: &BigInt) -> Option<i64> {
    b.to_i64()
}

pub fn rat_abs(r: &Rat) -> Rat {
    r.abs()
}
