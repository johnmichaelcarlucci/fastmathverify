//! Canonical multivariate polynomial form over "atoms" (symbols and opaque
//! subexpressions), used to mimic SymPy's automatic evaluation of `a - b` and
//! to solve linear equations.

use std::collections::BTreeMap;

use num_traits::{One, Signed, ToPrimitive, Zero};

use crate::expr::*;

/// Monomial: sorted (atom key, exponent) pairs.
pub type Mono = Vec<(String, u32)>;

#[derive(Clone, Debug, Default)]
pub struct Poly {
    pub terms: BTreeMap<Mono, Rat>,
    pub atoms: BTreeMap<String, Expr>,
    pub has_float: bool,
}

fn key_of(e: &Expr) -> String {
    match e {
        Expr::Sym(s) => s.clone(),
        other => format!("\u{2}{}", other),
    }
}

impl Poly {
    fn constant(r: Rat) -> Poly {
        let mut p = Poly::default();
        if !r.is_zero() {
            p.terms.insert(vec![], r);
        }
        p
    }
    fn atom(e: &Expr) -> Poly {
        let mut p = Poly::default();
        let k = key_of(e);
        p.atoms.insert(k.clone(), e.clone());
        p.terms.insert(vec![(k, 1)], Rat::one());
        p
    }
    fn add(mut self, o: Poly) -> Poly {
        self.has_float |= o.has_float;
        self.atoms.extend(o.atoms);
        for (m, c) in o.terms {
            let e = self.terms.entry(m.clone()).or_insert_with(Rat::zero);
            *e += c;
            if e.is_zero() {
                self.terms.remove(&m);
            }
        }
        self
    }
    fn mul(self, o: &Poly) -> Option<Poly> {
        if self.terms.len() * o.terms.len() > 400 {
            return None;
        }
        let mut out = Poly { has_float: self.has_float || o.has_float, ..Default::default() };
        out.atoms = self.atoms.clone();
        out.atoms.extend(o.atoms.clone());
        for (m1, c1) in &self.terms {
            for (m2, c2) in &o.terms {
                let mut m: BTreeMap<String, u32> = m1.iter().cloned().collect();
                for (k, e) in m2 {
                    *m.entry(k.clone()).or_insert(0) += e;
                }
                let mono: Mono = m.into_iter().collect();
                let e = out.terms.entry(mono.clone()).or_insert_with(Rat::zero);
                *e += c1 * c2;
                if e.is_zero() {
                    out.terms.remove(&mono);
                }
            }
        }
        Some(out)
    }
    fn scale(mut self, k: &Rat) -> Poly {
        if k.is_zero() {
            self.terms.clear();
            return self;
        }
        for c in self.terms.values_mut() {
            *c *= k;
        }
        self
    }

    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn as_constant(&self) -> Option<Rat> {
        match self.terms.len() {
            0 => Some(Rat::zero()),
            1 => self.terms.get(&vec![]).cloned(),
            _ => None,
        }
    }

    pub fn degree_in(&self, k: &str) -> u32 {
        self.terms.keys().map(|m| m.iter().find(|(a, _)| a == k).map(|x| x.1).unwrap_or(0)).max().unwrap_or(0)
    }

    /// Split as coef*atom + rest for an atom of degree exactly 1.
    pub fn linear_in(&self, k: &str) -> Option<(Poly, Poly)> {
        if self.degree_in(k) != 1 {
            return None;
        }
        let mut coef = Poly { atoms: self.atoms.clone(), has_float: self.has_float, ..Default::default() };
        let mut rest = Poly { atoms: self.atoms.clone(), has_float: self.has_float, ..Default::default() };
        for (m, c) in &self.terms {
            if m.iter().any(|(a, _)| a == k) {
                let m2: Mono = m.iter().filter(|(a, _)| a != k).cloned().collect();
                coef.terms.insert(m2, c.clone());
            } else {
                rest.terms.insert(m.clone(), c.clone());
            }
        }
        Some((coef, rest))
    }

    pub fn to_expr(&self) -> Expr {
        let mk_num = |r: &Rat| {
            if self.has_float {
                Expr::Num(Num::float_from_rat(r.clone()))
            } else {
                Expr::Num(Num::rat(r.clone()))
            }
        };
        let mut terms = Vec::new();
        for (m, c) in &self.terms {
            let mut factors = Vec::new();
            if m.is_empty() || !c.is_one() {
                factors.push(mk_num(c));
            }
            for (k, e) in m {
                let a = self.atoms.get(k).cloned().unwrap_or(Expr::Sym(k.clone()));
                factors.push(if *e == 1 { a } else { pow(a, int(*e as i64)) });
            }
            terms.push(if factors.len() == 1 { factors.pop().unwrap() } else { Expr::Mul(factors) });
        }
        match terms.len() {
            0 => if self.has_float { Expr::Num(Num::float_from_rat(Rat::zero())) } else { int(0) },
            1 => terms.pop().unwrap(),
            _ => Expr::Add(terms),
        }
    }
}

pub fn to_poly(e: &Expr) -> Option<Poly> {
    Some(match e {
        Expr::Num(Num::Float { exact, .. }) => {
            let mut p = Poly::constant(exact.clone());
            p.has_float = true;
            p
        }
        Expr::Num(n) => Poly::constant(n.to_rat()),
        Expr::Uneval(x) => to_poly(x)?,
        Expr::Add(v) => {
            let mut acc = Poly::default();
            for x in v {
                acc = acc.add(to_poly(x)?);
            }
            acc
        }
        Expr::Mul(v) => {
            let mut acc = Poly::constant(Rat::one());
            for x in v {
                acc = acc.mul(&to_poly(x)?)?;
            }
            acc
        }
        Expr::Pow(b, x) => {
            if let Expr::Num(Num::Int(n)) = &**x {
                let n = n.to_i64()?;
                if (0..=12).contains(&n) {
                    let base = to_poly(b)?;
                    let mut acc = Poly::constant(Rat::one());
                    for _ in 0..n {
                        acc = acc.mul(&base)?;
                    }
                    return Some(acc);
                }
                if n < 0 {
                    if let Some(r) = crate::eval::exact(b) {
                        if !r.is_zero() && n >= -64 {
                            let v = num_traits::pow(r.recip(), n.unsigned_abs() as usize);
                            return Some(Poly::constant(v));
                        }
                    }
                }
            }
            Poly::atom(e)
        }
        Expr::Sym(_) => Poly::atom(e),
        Expr::Pi | Expr::E | Expr::I | Expr::EulerGamma | Expr::Func(..) | Expr::Derivative(..)
        | Expr::Integral { .. } | Expr::Sum { .. } | Expr::Limit { .. } => Poly::atom(e),
        _ => return None,
    })
}

/// SymPy-like evaluated `a - b`: collapses to a number or single symbol when the
/// terms cancel that far; otherwise an (unevaluated) Add.
pub fn evaluated_sub(a: &Expr, b: &Expr) -> Expr {
    let d = Expr::Add(vec![a.clone(), Expr::Mul(vec![int(-1), b.clone()])]);
    match to_poly(&d) {
        Some(p) => {
            if let Some(c) = p.as_constant() {
                if p.has_float {
                    return Expr::Num(Num::float_from_rat(c));
                }
                return Expr::Num(Num::rat(c));
            }
            if p.terms.len() == 1 {
                let (m, c) = p.terms.iter().next().unwrap();
                if c.is_one() && m.len() == 1 && m[0].1 == 1 {
                    if let Some(Expr::Sym(s)) = p.atoms.get(&m[0].0) {
                        return Expr::Sym(s.clone());
                    }
                }
            }
            d
        }
        None => d,
    }
}

/// Solve a single equation `lhs = rhs` the way upstream's sympy.solve call is
/// used: only multi-symbol linear equations yield dict solutions {sym: value}.
pub fn solve_linear_dict(lhs: &Expr, rhs: &Expr) -> Option<Vec<(String, Expr)>> {
    let syms = {
        let mut s = lhs.free_symbols();
        s.extend(rhs.free_symbols());
        s.sort();
        s.dedup();
        s
    };
    if syms.len() < 2 {
        return None;
    }
    let d = to_poly(&Expr::Add(vec![lhs.clone(), Expr::Mul(vec![int(-1), rhs.clone()])]))?;
    for s in &syms {
        if let Some((coef, rest)) = d.linear_in(s) {
            if let Some(c) = coef.as_constant() {
                if c.is_zero() {
                    continue;
                }
                let sol = rest.scale(&(-c.recip()));
                return Some(vec![(s.clone(), sol.to_expr())]);
            }
        }
    }
    None
}

#[allow(dead_code)]
fn _s(r: &Rat) -> bool {
    r.is_negative()
}

/// Coefficients (as symbol-free expressions) of `p` viewed as a polynomial in
/// the atom `var`; index i holds the coefficient of var^i.
pub fn coeffs_in(p: &Poly, var: &str) -> Option<Vec<Expr>> {
    let deg = p.degree_in(var) as usize;
    let mut groups: Vec<Vec<Expr>> = vec![Vec::new(); deg + 1];
    for (m, c) in &p.terms {
        let mut k = 0usize;
        let mut factors = vec![if p.has_float {
            Expr::Num(Num::float_from_rat(c.clone()))
        } else {
            Expr::Num(Num::rat(c.clone()))
        }];
        for (a, e) in m {
            if a == var {
                k = *e as usize;
            } else {
                let at = p.atoms.get(a)?.clone();
                if at.has_free_symbols() {
                    return None;
                }
                factors.push(if *e == 1 { at } else { pow(at, int(*e as i64)) });
            }
        }
        groups[k].push(if factors.len() == 1 { factors.pop().unwrap() } else { Expr::Mul(factors) });
    }
    Some(
        groups
            .into_iter()
            .map(|g| match g.len() {
                0 => int(0),
                1 => g.into_iter().next().unwrap(),
                _ => Expr::Add(g),
            })
            .collect(),
    )
}
