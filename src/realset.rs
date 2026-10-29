//! Canonical subsets of the real line (unions of intervals and points) with
//! closed-form endpoints, used to decide set equality like SymPy's
//! `symmetric_difference(...).is_empty`.

use std::cmp::Ordering;

use crate::eval::compare_real;
use crate::expr::*;

#[derive(Clone, Debug)]
struct Iv {
    a: Expr,
    b: Expr,
    lo: bool, // open at a
    ro: bool, // open at b
}

#[derive(Clone, Debug, Default)]
pub struct RealSet {
    ivs: Vec<Iv>,
}

fn cmp(a: &Expr, b: &Expr) -> Option<Ordering> {
    compare_real(a, b)
}

impl RealSet {
    fn normalize(mut self) -> Option<RealSet> {
        // drop empty intervals
        let mut v = Vec::new();
        for iv in self.ivs.drain(..) {
            match cmp(&iv.a, &iv.b)? {
                Ordering::Greater => {}
                Ordering::Equal => {
                    if !iv.lo && !iv.ro {
                        v.push(iv);
                    }
                }
                Ordering::Less => v.push(iv),
            }
        }
        // sort by start (closed before open at equal start)
        let mut err = false;
        v.sort_by(|x, y| match cmp(&x.a, &y.a) {
            Some(Ordering::Equal) => x.lo.cmp(&y.lo),
            Some(o) => o,
            None => {
                err = true;
                Ordering::Equal
            }
        });
        if err {
            return None;
        }
        let mut out: Vec<Iv> = Vec::new();
        for iv in v {
            if let Some(last) = out.last_mut() {
                let c = cmp(&iv.a, &last.b)?;
                let touches = c == Ordering::Less || (c == Ordering::Equal && !(iv.lo && last.ro));
                if touches {
                    match cmp(&iv.b, &last.b)? {
                        Ordering::Greater => {
                            last.b = iv.b;
                            last.ro = iv.ro;
                        }
                        Ordering::Equal => last.ro = last.ro && iv.ro,
                        Ordering::Less => {}
                    }
                    continue;
                }
            }
            out.push(iv);
        }
        Some(RealSet { ivs: out })
    }

    fn union(mut self, o: RealSet) -> Option<RealSet> {
        self.ivs.extend(o.ivs);
        self.normalize()
    }

    fn complement(self) -> Option<RealSet> {
        let mut out = Vec::new();
        let mut cur = Expr::NegInfinity;
        let mut cur_open = true;
        for iv in &self.ivs {
            out.push(Iv { a: cur.clone(), b: iv.a.clone(), lo: cur_open, ro: !iv.lo });
            cur = iv.b.clone();
            cur_open = !iv.ro;
        }
        out.push(Iv { a: cur, b: Expr::Infinity, lo: cur_open, ro: true });
        RealSet { ivs: out }.normalize()
    }

    fn intersection(self, o: RealSet) -> Option<RealSet> {
        self.complement()?.union(o.complement()?)?.complement()
    }

    pub fn intersect(self, o: RealSet) -> Option<RealSet> {
        self.intersection(o)
    }

    /// Back to a SymPy-style set expression.
    pub fn to_expr(&self) -> Expr {
        let mut points = Vec::new();
        let mut pieces = Vec::new();
        for iv in &self.ivs {
            if !iv.lo && !iv.ro && cmp(&iv.a, &iv.b) == Some(Ordering::Equal) {
                points.push(iv.a.clone());
            } else if matches!(iv.a, Expr::NegInfinity) && matches!(iv.b, Expr::Infinity) {
                pieces.push(Expr::SetConst(SetConst::Reals));
            } else {
                pieces.push(Expr::Interval {
                    start: Box::new(iv.a.clone()),
                    end: Box::new(iv.b.clone()),
                    lopen: iv.lo,
                    ropen: iv.ro,
                });
            }
        }
        if !points.is_empty() {
            pieces.push(finite_set(points));
        }
        match pieces.len() {
            0 => Expr::EmptySet,
            1 => pieces.pop().unwrap(),
            _ => Expr::Union(pieces),
        }
    }

    pub fn equals(&self, o: &RealSet) -> Option<bool> {
        if self.ivs.len() != o.ivs.len() {
            return Some(false);
        }
        for (x, y) in self.ivs.iter().zip(&o.ivs) {
            if x.lo != y.lo || x.ro != y.ro {
                return Some(false);
            }
            if cmp(&x.a, &y.a)? != Ordering::Equal || cmp(&x.b, &y.b)? != Ordering::Equal {
                return Some(false);
            }
        }
        Some(true)
    }
}

/// Convert a set expression with closed-form real endpoints into a RealSet.
pub fn to_realset(e: &Expr) -> Option<RealSet> {
    let rs = match e {
        Expr::EmptySet => RealSet::default(),
        Expr::SetConst(SetConst::Reals) => {
            RealSet { ivs: vec![Iv { a: Expr::NegInfinity, b: Expr::Infinity, lo: true, ro: true }] }
        }
        Expr::Interval { start, end, lopen, ropen } => RealSet {
            ivs: vec![Iv { a: (**start).clone(), b: (**end).clone(), lo: *lopen, ro: *ropen }],
        },
        Expr::FiniteSet { args, .. } => {
            let mut ivs = Vec::new();
            for a in args {
                if a.has_free_symbols() || crate::order::real_value(a).is_none() {
                    return None;
                }
                ivs.push(Iv { a: a.clone(), b: a.clone(), lo: false, ro: false });
            }
            RealSet { ivs }
        }
        Expr::Union(v) => {
            let mut acc = RealSet::default();
            for x in v {
                acc = acc.union(to_realset(x)?)?;
            }
            return Some(acc);
        }
        Expr::Intersection(v) => {
            let mut it = v.iter();
            let mut acc = to_realset(it.next()?)?;
            for x in it {
                acc = acc.intersection(to_realset(x)?)?;
            }
            return Some(acc);
        }
        Expr::Complement(a, b) => {
            return to_realset(a)?.intersection(to_realset(b)?.complement()?);
        }
        _ => return None,
    };
    rs.normalize()
}
