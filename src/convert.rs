//! Port of latex2sympy2_extended's `_Latex2Sympy` converter (parse tree -> Expr).

use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

use crate::eval;
use crate::expr::*;
use crate::gen::psparser::*;
use crate::symbols::get_symbol;
use crate::tree::{parse_math, Node, ParseOutput, RuleNode, Tok};

pub type CResult<T> = Result<T, String>;

fn err<T>(m: &str) -> CResult<T> {
    Err(m.to_string())
}

/// Value produced while converting: an expression, or the derivative operator
/// marker `[wrt]` that latex2sympy returns for `\frac{d}{dx}`.
enum Val {
    E(Expr),
    DiffOp(Expr),
}

pub struct Converter {
    pub is_real: bool,
}

// ---------- tree helpers ----------

impl RuleNode {
    fn rules(&self, rule: usize) -> Vec<&RuleNode> {
        self.children
            .iter()
            .filter_map(|c| match c {
                Node::Rule(r) if r.rule == rule => Some(r),
                _ => None,
            })
            .collect()
    }
    fn rule(&self, rule: usize) -> Option<&RuleNode> {
        self.children.iter().find_map(|c| match c {
            Node::Rule(r) if r.rule == rule => Some(r),
            _ => None,
        })
    }
    fn tok(&self, t: i32) -> Option<&Tok> {
        self.children.iter().find_map(|c| match c {
            Node::Leaf(k) if k.ttype == t => Some(k),
            _ => None,
        })
    }
    fn has(&self, t: i32) -> bool {
        self.tok(t).is_some()
    }
    /// ANTLR getText(): concatenation of leaf texts (skipped tokens excluded).
    pub fn text(&self) -> String {
        let mut s = String::new();
        self.text_into(&mut s);
        s
    }
    fn text_into(&self, s: &mut String) {
        for c in &self.children {
            match c {
                Node::Rule(r) => r.text_into(s),
                Node::Leaf(t) => s.push_str(&t.text),
            }
        }
    }
    fn leaves(&self) -> Vec<&Tok> {
        let mut v = Vec::new();
        self.leaves_into(&mut v);
        v
    }
    fn leaves_into<'a>(&'a self, v: &mut Vec<&'a Tok>) {
        for c in &self.children {
            match c {
                Node::Rule(r) => r.leaves_into(v),
                Node::Leaf(t) => v.push(t),
            }
        }
    }
}

fn source_text(r: &RuleNode, chars: &[char]) -> String {
    let l = r.leaves();
    match (l.first(), l.last()) {
        (Some(a), Some(b)) => chars[a.start..=b.stop.min(chars.len() - 1)].iter().collect(),
        _ => String::new(),
    }
}

/// `^\s*-?\d{1,3}(,\d{3})+(\.\d+)?\s*$`
fn is_comma_number(s: &str) -> bool {
    let t = s.trim();
    let t = t.strip_prefix('-').unwrap_or(t);
    let (int_part, dec) = match t.find('.') {
        Some(i) => (&t[..i], Some(&t[i + 1..])),
        None => (t, None),
    };
    if let Some(d) = dec {
        if d.is_empty() || !d.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
    }
    let groups: Vec<&str> = int_part.split(',').collect();
    if groups.len() < 2 {
        return false;
    }
    let first = groups[0];
    if first.is_empty() || first.len() > 3 || !first.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    groups[1..].iter().all(|g| g.len() == 3 && g.chars().all(|c| c.is_ascii_digit()))
}

/// sympy.Number(text) for decimal / integer / e-notation strings.
pub fn sympy_number(text: &str) -> CResult<Expr> {
    let t = text.trim();
    if t.is_empty() {
        return err("empty number");
    }
    if t.chars().all(|c| c.is_ascii_digit()) {
        return Ok(Expr::Num(Num::Int(t.parse::<BigInt>().map_err(|e| e.to_string())?)));
    }
    if let Some(r) = parse_decimal(t) {
        return Ok(Expr::Num(Num::float_from_rat(r)));
    }
    err("bad number")
}

/// Exact rational value of a decimal literal like "-1.25", ".5", "3E-2".
pub fn parse_decimal(t: &str) -> Option<Rat> {
    let (mant, exp) = match t.find(|c| c == 'e' || c == 'E') {
        Some(i) => (&t[..i], t[i + 1..].parse::<i64>().ok()?),
        None => (t, 0),
    };
    let (neg, mant) = match mant.strip_prefix('-') {
        Some(m) => (true, m),
        None => (false, mant.strip_prefix('+').unwrap_or(mant)),
    };
    let (ip, fp) = match mant.find('.') {
        Some(i) => (&mant[..i], &mant[i + 1..]),
        None => (mant, ""),
    };
    if ip.is_empty() && fp.is_empty() {
        return None;
    }
    if !ip.chars().all(|c| c.is_ascii_digit()) || !fp.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if exp.abs() > 400 {
        return None;
    }
    let digits = format!("{ip}{fp}");
    let n: BigInt = if digits.is_empty() { BigInt::zero() } else { digits.parse().ok()? };
    let scale = exp - fp.len() as i64;
    let ten = BigInt::from(10);
    let mut r = if scale >= 0 {
        Rat::from_integer(n * num_traits::pow(ten, scale as usize))
    } else {
        Rat::new(n, num_traits::pow(ten, (-scale) as usize))
    };
    if neg {
        r = -r;
    }
    Some(r)
}

/// latex2sympy `convert_number` for comma-grouped numbers like "1,234".
fn convert_number(number: &str) -> CResult<Expr> {
    let mut s = number.to_string();
    if s.contains(',') && s.starts_with('0') {
        s = s.replace(',', ".");
    }
    let s: String = s.chars().filter(|c| *c != ',' && *c != ' ').collect();
    let s = s.trim_start_matches('0');
    let s = if s.is_empty() { "0" } else { s };
    sympy_number(s)
}

fn parse_number(text: &str) -> CResult<Expr> {
    let mut t: String = text.replace(',', "");
    if !t.is_empty() && t.chars().all(|c| c.is_ascii_digit()) {
        while t.len() > 1 && t.starts_with('0') {
            t.remove(0);
        }
    }
    sympy_number(&t)
}

pub fn is_expr_of_only_symbols(e: &Expr) -> bool {
    match e {
        Expr::Sym(_) => true,
        Expr::Pow(b, x) if matches!(&**x, Expr::Num(Num::Int(i)) if *i == BigInt::from(-1)) => {
            matches!(&**b, Expr::Sym(_)) || {
                let ch = b.children();
                !ch.is_empty() && ch.iter().all(|c| is_expr_of_only_symbols(c))
            }
        }
        _ => {
            let ch = e.children();
            !ch.is_empty() && ch.iter().all(|c| is_expr_of_only_symbols(c))
        }
    }
}

/// `x.func.is_Number`: Integer/Rational/Float and the infinities/NaN.
fn func_is_number(e: &Expr) -> bool {
    matches!(e, Expr::Num(_) | Expr::Infinity | Expr::NegInfinity | Expr::NaN)
}

/// Evaluated negation (`-x`) for Number-like atoms.
fn neg_number(e: Expr) -> Expr {
    match e {
        Expr::Num(n) => Expr::Num(n.neg()),
        Expr::Infinity => Expr::NegInfinity,
        Expr::NegInfinity => Expr::Infinity,
        other => other,
    }
}

fn unwrap_e(v: Val) -> CResult<Expr> {
    match v {
        Val::E(e) => Ok(e),
        Val::DiffOp(_) => err("derivative operator used as value"),
    }
}

impl Converter {
    pub fn new(is_real: bool) -> Self {
        Converter { is_real }
    }

    fn sym(&self, text: &str, enforce_case: bool) -> Expr {
        let name = if enforce_case { text.to_string() } else { text.to_lowercase() };
        crate::symbols::make_symbol(&name, self.is_real)
    }

    pub fn parse(&self, latex: &str) -> CResult<Expr> {
        let out = parse_math(latex)?;
        self.convert_math(&out, latex)
    }

    fn convert_math(&self, out: &ParseOutput, latex: &str) -> CResult<Expr> {
        let math = &out.root;
        let cx = Cx { out, conv: self };
        if let Some(sr) = math.rule(RULE_set_relation) {
            return cx.set_relation(sr);
        }
        if let Some(se) = math.rule(RULE_set_elements) {
            if is_comma_number(latex) {
                return convert_number(latex);
            }
            let elems = cx.semicolon_elements(se.rule(RULE_semicolon_elements).ok_or("x")?)?;
            return elements_to_set_or_tuple(elems);
        }
        if let Some(ser) = math.rule(RULE_set_elements_relation) {
            return cx.set_elements_relation(ser);
        }
        err("Nothing matched")
    }
}

/// Conversion context: the converter plus the parse output (for source text).
struct Cx<'a> {
    out: &'a ParseOutput,
    conv: &'a Converter,
}

fn elements_to_set_or_tuple(elements: Vec<Vec<Expr>>) -> CResult<Expr> {
    if elements.len() == 1 {
        let mut e = elements.into_iter().next().unwrap();
        if e.len() == 1 {
            return Ok(e.pop().unwrap());
        }
        return Ok(finite_set_or_empty(e));
    }
    if elements.iter().all(|e| e.len() == 1) {
        return Ok(finite_set_or_empty(elements.into_iter().map(|mut e| e.pop().unwrap()).collect()));
    }
    Ok(finite_set_or_empty(elements.into_iter().map(Expr::Tuple).collect()))
}

fn chain_rel(op: RelOp, lh: Expr, rh: Expr) -> Expr {
    match lh {
        Expr::And(mut v) => {
            let last_rhs = match v.last() {
                Some(Expr::Rel(_, _, r)) => (**r).clone(),
                _ => Expr::NaN,
            };
            v.push(Expr::Rel(op, Box::new(last_rhs), Box::new(rh)));
            Expr::And(v)
        }
        Expr::Rel(o, a, b) => {
            let r2 = Expr::Rel(op, b.clone(), Box::new(rh));
            Expr::And(vec![Expr::Rel(o, a, b), r2])
        }
        other => Expr::Rel(op, Box::new(other), Box::new(rh)),
    }
}

fn tuple_to_set(e: Expr) -> Expr {
    match e {
        Expr::Tuple(v) => finite_set_or_empty(v),
        o => o,
    }
}

impl<'a> Cx<'a> {
    fn chars(&self) -> &[char] {
        &self.out.chars
    }

    // ----- relations -----

    fn relation(&self, rel: &RuleNode) -> CResult<Expr> {
        if let Some(e) = rel.rule(RULE_expr) {
            return self.expr(e);
        }
        let rs = rel.rules(RULE_relation);
        if rs.len() != 2 {
            return err("bad relation");
        }
        let lh = self.relation(rs[0])?;
        let rh = self.relation(rs[1])?;
        if rel.has(PS_LT) {
            return Ok(chain_rel(RelOp::Lt, lh, rh));
        }
        if rel.has(PS_LTE) {
            return Ok(chain_rel(RelOp::Le, lh, rh));
        }
        if rel.has(PS_GT) {
            return Ok(chain_rel(RelOp::Gt, lh, rh));
        }
        if rel.has(PS_GTE) {
            return Ok(chain_rel(RelOp::Ge, lh, rh));
        }
        if rel.has(PS_EQUAL) || rel.has(PS_ASSIGNMENT) {
            return Ok(chain_rel(RelOp::Eq, lh, rh));
        }
        if rel.has(PS_APPROX) {
            return Ok(if is_expr_of_only_symbols(&lh) { rh } else { lh });
        }
        if rel.has(PS_IN) {
            if let Expr::Pow(_, e) = &rh {
                if matches!(&**e, Expr::Mul(_)) {
                    return Ok(lh);
                }
            }
            return err("Unrecognized relation");
        }
        if rel.has(PS_UNEQUAL) {
            return Ok(chain_rel(RelOp::Ne, lh, rh));
        }
        err("bad relation op")
    }

    fn set_relation(&self, sr: &RuleNode) -> CResult<Expr> {
        if let Some(ael) = sr.rule(RULE_atom_expr_list) {
            let left = self.atom_expr_list(ael)?;
            let right = self.set_relation(sr.rules(RULE_set_relation).first().ok_or("x")?)?;
            if sr.has(PS_IN) || sr.has(PS_ASSIGNMENT) {
                return Ok(Expr::Rel(RelOp::Eq, Box::new(left), Box::new(right)));
            }
            if sr.has(PS_NOTIN) {
                return Ok(Expr::Not(Box::new(Expr::Rel(RelOp::Eq, Box::new(left), Box::new(right)))));
            }
        }
        if !sr.rules(RULE_set_relation).is_empty() {
            return err("subset relations unsupported");
        }
        self.minus_expr(sr.rule(RULE_minus_expr).ok_or("x")?)
    }

    fn set_elements_relation(&self, ser: &RuleNode) -> CResult<Expr> {
        let elems = self.semicolon_elements_no_relation(
            ser.rule(RULE_semicolon_elements_no_relation).ok_or("x")?,
        )?;
        let set_elements = elements_to_set_or_tuple(elems)?;
        let atoms = self.atom_expr_list(ser.rule(RULE_atom_expr_list).ok_or("x")?)?;
        if ser.has(PS_IN) || ser.has(PS_ASSIGNMENT) {
            return Ok(Expr::Rel(RelOp::Eq, Box::new(atoms), Box::new(set_elements)));
        }
        Ok(set_elements)
    }

    fn minus_expr(&self, e: &RuleNode) -> CResult<Expr> {
        if let Some(u) = e.rule(RULE_union_expr) {
            return self.union_expr(u);
        }
        let m = e.rules(RULE_minus_expr);
        let l = self.minus_expr(m[0])?;
        let r = self.minus_expr(m[1])?;
        Ok(Expr::Complement(Box::new(l), Box::new(r)))
    }

    fn union_expr(&self, e: &RuleNode) -> CResult<Expr> {
        if let Some(i) = e.rule(RULE_intersection_expr) {
            return self.intersection_expr(i);
        }
        let u = e.rules(RULE_union_expr);
        let l = tuple_to_set(self.union_expr(u[0])?);
        let r = tuple_to_set(self.union_expr(u[1])?);
        Ok(Expr::Union(vec![l, r]))
    }

    fn intersection_expr(&self, e: &RuleNode) -> CResult<Expr> {
        if let Some(g) = e.rule(RULE_set_group) {
            return self.set_group(g);
        }
        let u = e.rules(RULE_intersection_expr);
        let l = tuple_to_set(self.intersection_expr(u[0])?);
        let r = tuple_to_set(self.intersection_expr(u[1])?);
        Ok(Expr::Intersection(vec![l, r]))
    }

    fn set_group(&self, e: &RuleNode) -> CResult<Expr> {
        if let Some(a) = e.rule(RULE_set_atom) {
            return self.set_atom(a);
        }
        self.minus_expr(e.rule(RULE_minus_expr).ok_or("x")?)
    }

    fn set_atom(&self, e: &RuleNode) -> CResult<Expr> {
        if let Some(l) = e.rule(RULE_literal_set) {
            return self.literal_set(l);
        }
        if let Some(i) = e.rule(RULE_interval) {
            return self.interval(i);
        }
        if let Some(t) = e.rule(RULE_ordered_tuple) {
            let el = self.semicolon_elements(t.rule(RULE_semicolon_elements).ok_or("x")?)?;
            if el.len() == 1 && el[0].len() == 1 {
                return Ok(el.into_iter().next().unwrap().pop().unwrap());
            }
            return Ok(Expr::Tuple(el.into_iter().flatten().collect()));
        }
        if let Some(f) = e.rule(RULE_finite_set) {
            let content = self.semicolon_elements(f.rule(RULE_semicolon_elements).ok_or("x")?)?;
            if f.has(PS_BOXED_CMD) {
                return elements_to_set_or_tuple(content);
            }
            return Ok(finite_set_or_empty(content.into_iter().flatten().collect()));
        }
        err("Unrecognized set atom")
    }

    fn interval(&self, e: &RuleNode) -> CResult<Expr> {
        let lo = e.has(PS_L_PAREN) || e.has(PS_L_GROUP) || e.has(PS_L_PAREN_VISUAL);
        let ro = e.has(PS_R_PAREN) || e.has(PS_R_GROUP) || e.has(PS_R_PAREN_VISUAL);
        let ex = e.rules(RULE_expr);
        let left = self.expr(ex[0])?;
        let right = self.expr(ex[1])?;
        if let Some(ord) = eval::compare_real(&right, &left) {
            use std::cmp::Ordering::*;
            if (lo && ro && ord != Greater) || (!lo && !ro && ord == Less) {
                return Ok(Expr::Tuple(vec![left, right]));
            }
        }
        crate::sets::make_interval(left, right, lo, ro)
    }

    fn literal_set(&self, e: &RuleNode) -> CResult<Expr> {
        if e.has(PS_SET_NATURALS) {
            return Ok(Expr::SetConst(SetConst::Naturals));
        }
        if e.has(PS_SET_INTEGERS) {
            return Ok(Expr::SetConst(SetConst::Integers));
        }
        if e.has(PS_SET_RATIONALS) {
            return Ok(Expr::SetConst(SetConst::Rationals));
        }
        if e.has(PS_SET_REALS) {
            return Ok(Expr::SetConst(SetConst::Reals));
        }
        if e.has(PS_SET_COMPLEX) {
            return Ok(Expr::SetConst(SetConst::Complexes));
        }
        if e.has(PS_SET_EMPTY) || (e.has(PS_L_BRACE) && e.has(PS_R_BRACE)) {
            return Ok(Expr::EmptySet);
        }
        err("Unrecognized literal set")
    }

    fn semicolon_elements(&self, e: &RuleNode) -> CResult<Vec<Vec<Expr>>> {
        e.rules(RULE_comma_elements)
            .into_iter()
            .map(|c| {
                let mut out = Vec::new();
                for el in c.rules(RULE_element) {
                    out.extend(self.element(el)?);
                }
                Ok(out)
            })
            .collect()
    }

    fn semicolon_elements_no_relation(&self, e: &RuleNode) -> CResult<Vec<Vec<Expr>>> {
        e.rules(RULE_comma_elements_no_relation)
            .into_iter()
            .map(|c| {
                let mut out = Vec::new();
                for el in c.rules(RULE_element_no_relation) {
                    out.extend(self.element(el)?);
                }
                Ok(out)
            })
            .collect()
    }

    fn as_unary_minus(&self, e: Expr) -> Expr {
        match e {
            Expr::Num(Num::Int(_)) | Expr::Num(Num::Rat(_)) => neg_number(e),
            other => Expr::Mul(vec![int(-1), other]),
        }
    }

    fn element(&self, el: &RuleNode) -> CResult<Vec<Expr>> {
        if let Some(pm) = el.rule(RULE_plus_minus_expr) {
            let ex = pm.rules(RULE_expr);
            if ex.len() == 1 {
                let e = self.expr(ex[0])?;
                return Ok(vec![self.as_unary_minus(e.clone()), e]);
            }
            let l = self.expr(ex[0])?;
            let r = self.expr(ex[1])?;
            return Ok(vec![
                Expr::Add(vec![l.clone(), r.clone()]),
                Expr::Add(vec![l, self.as_unary_minus(r)]),
            ]);
        }
        if let Some(sa) = el.rule(RULE_set_atom) {
            return Ok(vec![self.set_atom(sa)?]);
        }
        if let Some(r) = el.rule(RULE_relation) {
            return Ok(vec![self.relation(r)?]);
        }
        if let Some(e) = el.rule(RULE_expr) {
            return Ok(vec![self.expr(e)?]);
        }
        err("Unrecognized comma element")
    }

    // ----- arithmetic -----

    fn expr(&self, e: &RuleNode) -> CResult<Expr> {
        match e.rule(RULE_additive) {
            Some(a) => self.additive(a),
            None => err("expr without additive"),
        }
    }

    fn additive(&self, a: &RuleNode) -> CResult<Expr> {
        let subs = a.rules(RULE_additive);
        if a.has(PS_ADD) && subs.len() == 2 {
            let lh = self.additive(subs[0])?;
            let rh = self.additive(subs[1])?;
            if lh.is_matrix() || rh.is_matrix() {
                return crate::matrix::mat_add(lh, rh);
            }
            return Ok(add_flat(lh, rh));
        }
        if a.has(PS_SUB) && subs.len() == 2 {
            let lh = self.additive(subs[0])?;
            let rh = self.additive(subs[1])?;
            if lh.is_matrix() || rh.is_matrix() {
                return crate::matrix::mat_add(lh, crate::matrix::mat_scale(int(-1), rh)?);
            }
            let rh = if func_is_number(&rh) { neg_number(rh) } else { mul_flat(int(-1), rh) };
            return Ok(add_flat(lh, rh));
        }
        self.mp(a.rule(RULE_mp).ok_or("x")?)
    }

    fn mp(&self, mp: &RuleNode) -> CResult<Expr> {
        let subs = {
            let v = mp.rules(RULE_mp);
            if v.is_empty() { mp.rules(RULE_mp_nofunc) } else { v }
        };
        if subs.len() == 2 {
            let lh = self.mp(subs[0])?;
            let rh = self.mp(subs[1])?;
            if mp.has(PS_MUL) || mp.has(PS_CMD_TIMES) || mp.has(PS_CMD_CDOT) {
                if lh.is_matrix() || rh.is_matrix() {
                    return crate::matrix::mat_mul(lh, rh);
                }
                return Ok(mul_flat(lh, rh));
            }
            if mp.has(PS_DIV) || mp.has(PS_CMD_DIV) || mp.has(PS_COLON) {
                if lh.is_matrix() || rh.is_matrix() {
                    return err("matrix division");
                }
                return Ok(div(lh, rh));
            }
            if mp.has(PS_CMD_MOD) {
                if rh.is_matrix() {
                    return err("Cannot perform modulo operation with a matrix as an operand");
                }
                return Ok(Expr::Func(Func::Mod, vec![lh, rh]));
            }
        }
        if let Some(u) = mp.rule(RULE_unary) {
            return self.unary(u);
        }
        if let Some(u) = mp.rule(RULE_unary_nofunc) {
            return self.unary(u);
        }
        err("bad mp")
    }

    fn unary(&self, u: &RuleNode) -> CResult<Expr> {
        let nested = u.rule(RULE_unary).or_else(|| u.rule(RULE_unary_nofunc));
        if u.has(PS_ADD) {
            if let Some(n) = nested {
                return self.unary(n);
            }
        }
        if u.has(PS_SUB) {
            if let Some(n) = nested {
                let x = self.unary(n)?;
                if x.is_matrix() {
                    return crate::matrix::mat_scale(int(-1), x);
                }
                if func_is_number(&x) {
                    return Ok(neg_number(x));
                }
                return Ok(mul_flat(int(-1), x));
            }
        }
        let mut postfix: Vec<&RuleNode> = u.rules(RULE_postfix);
        postfix.extend(u.rules(RULE_postfix_nofunc));
        if postfix.is_empty() {
            return err("empty unary");
        }
        unwrap_e(self.postfix_list(&postfix, 0)?)
    }

    fn postfix_list(&self, arr: &[&RuleNode], i: usize) -> CResult<Val> {
        if i >= arr.len() {
            return err("Index out of bounds");
        }
        let res = self.postfix(arr[i])?;
        match res {
            Val::E(res) => {
                if i == arr.len() - 1 {
                    return Ok(Val::E(res));
                }
                let rh = unwrap_e(self.postfix_list(arr, i + 1)?)?;
                if res.is_matrix() || rh.is_matrix() {
                    return Ok(Val::E(crate::matrix::mat_mul(res, rh)?));
                }
                if let (Expr::Num(Num::Int(n)), Some(r)) = (&res, rh.as_rational()) {
                    if r.numer().is_positive() && r.denom().is_positive() {
                        let q = r.denom().clone();
                        let p = r.numer().clone();
                        let num = if n.is_negative() { n * &q - p } else { n * &q + p };
                        return Ok(Val::E(rational(num, q)));
                    }
                }
                Ok(Val::E(mul_flat(res, rh)))
            }
            Val::DiffOp(wrt) => {
                if i == arr.len() - 1 {
                    return err("Expected expression for derivative");
                }
                let e = unwrap_e(self.postfix_list(arr, i + 1)?)?;
                Ok(Val::E(Expr::Derivative(Box::new(e), Box::new(wrt))))
            }
        }
    }

    fn postfix(&self, p: &RuleNode) -> CResult<Val> {
        let en = p.rule(RULE_exp).or_else(|| p.rule(RULE_exp_nofunc)).ok_or("x")?;
        let mut v = self.exp(en)?;
        for op in p.rules(RULE_postfix_op) {
            if op.has(PS_BANG) {
                let x = unwrap_e(v).map_err(|_| "Cannot apply postfix to derivative".to_string())?;
                v = Val::E(Expr::Func(Func::Factorial, vec![x]));
            } else if let Some(ev) = op.rule(RULE_eval_at) {
                let x = unwrap_e(v)?;
                let at_b = match ev.rule(RULE_eval_at_sup) {
                    Some(s) => Some(self.do_subs(&x, s)?),
                    None => None,
                };
                let at_a = match ev.rule(RULE_eval_at_sub) {
                    Some(s) => Some(self.do_subs(&x, s)?),
                    None => None,
                };
                v = Val::E(match (at_b, at_a) {
                    (Some(b), Some(a)) => add_flat(b, mul_flat(a, int(-1))),
                    (Some(b), None) => b,
                    (None, Some(a)) => a,
                    (None, None) => x,
                });
            } else if op.rule(RULE_transpose).is_some() {
                if let Val::E(Expr::Matrix { rows, cols, data }) = &v {
                    v = Val::E(crate::matrix::transpose(*rows, *cols, data));
                }
            }
            // degree: convert_degrees is False upstream -> no-op
        }
        Ok(v)
    }

    fn do_subs(&self, x: &Expr, at: &RuleNode) -> CResult<Expr> {
        if let Some(e) = at.rule(RULE_expr) {
            let at_expr = self.expr(e)?;
            let syms = at_expr.free_symbols();
            if syms.is_empty() {
                return Ok(x.clone());
            }
            return Ok(x.subs_sym(&syms[0], &at_expr));
        }
        if let Some(eq) = at.rule(RULE_equality) {
            let ex = eq.rules(RULE_expr);
            let lh = self.expr(ex[0])?;
            let rh = self.expr(ex[1])?;
            if let Expr::Sym(s) = &lh {
                return Ok(x.subs_sym(s, &rh));
            }
            return Ok(x.clone());
        }
        err("bad eval_at")
    }

    fn exp(&self, e: &RuleNode) -> CResult<Val> {
        let nested = e.rule(RULE_exp).or_else(|| e.rule(RULE_exp_nofunc));
        if let Some(n) = nested {
            let base = match self.exp(n)? {
                Val::E(b) => b,
                Val::DiffOp(_) => return err("Cannot raise derivative to power"),
            };
            let exponent = if let Some(a) = e.rule(RULE_atom) {
                self.atom(a)?
            } else {
                self.expr(e.rule(RULE_expr).ok_or("x")?)?
            };
            return Ok(Val::E(pow(base, exponent)));
        }
        let c = e.rule(RULE_comp).or_else(|| e.rule(RULE_comp_nofunc)).ok_or("x")?;
        self.comp(c)
    }

    fn comp(&self, c: &RuleNode) -> CResult<Val> {
        if let Some(g) = c.rule(RULE_group) {
            return Ok(Val::E(self.expr(g.rule(RULE_expr).ok_or("x")?)?));
        }
        if let Some(g) = c.rule(RULE_formatting_group) {
            return Ok(Val::E(self.expr(g.rule(RULE_expr).ok_or("x")?)?));
        }
        if let Some(g) = c.rule(RULE_norm_group) {
            let x = self.expr(g.rule(RULE_expr).ok_or("x")?)?;
            return Ok(Val::E(crate::matrix::norm(x)?));
        }
        if let Some(g) = c.rule(RULE_abs_group) {
            let x = self.expr(g.rule(RULE_expr).ok_or("x")?)?;
            return Ok(Val::E(Expr::Func(Func::Abs, vec![x])));
        }
        if let Some(g) = c.rule(RULE_floor_group) {
            let x = self.expr(g.rule(RULE_expr).ok_or("x")?)?;
            return Ok(Val::E(Expr::Func(Func::Floor, vec![x])));
        }
        if let Some(g) = c.rule(RULE_ceil_group) {
            let x = self.expr(g.rule(RULE_expr).ok_or("x")?)?;
            return Ok(Val::E(Expr::Func(Func::Ceiling, vec![x])));
        }
        if let Some(a) = c.rule(RULE_atom) {
            return Ok(Val::E(self.atom(a)?));
        }
        if let Some(f) = c.rule(RULE_frac) {
            return self.frac(f);
        }
        if let Some(b) = c.rule(RULE_binom) {
            let ex = b.rules(RULE_expr);
            let top = self.expr(ex[0])?;
            let bot = self.expr(ex[1])?;
            return Ok(Val::E(crate::functions::binomial(top, bot)));
        }
        if let Some(m) = c.rule(RULE_matrix) {
            return Ok(Val::E(self.matrix(m)?));
        }
        if let Some(d) = c.rule(RULE_det) {
            let m = self.matrix(d)?;
            return Ok(Val::E(crate::matrix::det(&m)?));
        }
        if let Some(f) = c.rule(RULE_func) {
            return Ok(Val::E(self.func(f)?));
        }
        err("unsupported comp")
    }

    fn matrix(&self, m: &RuleNode) -> CResult<Expr> {
        if m.has(PS_MATRIX_XRIGHTARROW) {
            return err("elementary transforms unsupported");
        }
        let mut rows = Vec::new();
        for r in m.rules(RULE_matrix_row) {
            let mut row = Vec::new();
            for e in r.rules(RULE_expr) {
                row.push(self.expr(e)?);
            }
            rows.push(row);
        }
        crate::matrix::from_rows(rows)
    }

    // ----- atoms -----

    fn atom_expr_list(&self, l: &RuleNode) -> CResult<Expr> {
        let atoms = l.rules(RULE_atom_expr);
        let mut v = Vec::new();
        for a in atoms {
            v.push(self.atom_expr(a)?);
        }
        if v.len() == 1 {
            return Ok(v.pop().unwrap());
        }
        Ok(Expr::Tuple(v))
    }

    fn atom_expr(&self, a: &RuleNode) -> CResult<Expr> {
        let mut atom_text = String::new();
        if let Some(t) = a.tok(PS_LETTER_NO_E) {
            atom_text = t.text.clone();
            if atom_text == "I" {
                return Ok(Expr::I);
            }
        } else if let Some(t) = a.tok(PS_GREEK_CMD) {
            atom_text = t.text.clone();
        } else if let Some(t) = a.tok(PS_OTHER_SYMBOL_CMD) {
            atom_text = t.text.clone();
        } else if let Some(t) = a.tok(PS_ACCENT) {
            atom_text = accent_text(&t.text);
        }

        let mut subscript_text = String::new();
        if let Some(sub) = a.rule(RULE_subexpr) {
            let s = if let Some(e) = sub.rule(RULE_expr) {
                e.text().trim().to_string()
            } else if let Some(at) = sub.rule(RULE_atom) {
                at.text().trim().to_string()
            } else if let Some(ar) = sub.rule(RULE_args) {
                ar.text().trim().to_string()
            } else {
                String::new()
            };
            if s.chars().count() > 1 {
                subscript_text = format!("_{{{s}}}");
            } else {
                subscript_text = format!("_{s}");
            }
        }
        let symbol = get_symbol(&format!("{}{}", atom_text.trim(), subscript_text), self.conv.is_real, true);
        if let Some(sup) = a.rule(RULE_supexpr) {
            let p = self.supexpr(sup)?;
            return Ok(pow(symbol, p));
        }
        Ok(symbol)
    }

    fn supexpr(&self, sup: &RuleNode) -> CResult<Expr> {
        if let Some(e) = sup.rule(RULE_expr) {
            self.expr(e)
        } else {
            self.atom(sup.rule(RULE_atom).ok_or("x")?)
        }
    }

    fn subexpr_value(&self, sub: &RuleNode) -> CResult<Expr> {
        if let Some(a) = sub.rule(RULE_atom) {
            self.atom(a)
        } else {
            self.expr(sub.rule(RULE_expr).ok_or("x")?)
        }
    }

    fn atom(&self, a: &RuleNode) -> CResult<Expr> {
        if let Some(ae) = a.rule(RULE_atom_expr) {
            return self.atom_expr(ae);
        }
        if let Some(t) = a.tok(PS_SYMBOL) {
            let s = t.text.replace("\\$", "").replace("\\%", "");
            if s == "\\infty" {
                return Ok(Expr::Infinity);
            }
            return err("Unrecognized symbol");
        }
        if let Some(ns) = a.rule(RULE_number_subexpr) {
            let t = ns.tok(PS_NUMBER).ok_or("x")?;
            return parse_number(&t.text);
        }
        if let Some(t) = a.tok(PS_E_NOTATION) {
            return parse_number(&t.text);
        }
        if a.has(PS_E_NOTATION_E) {
            return Ok(self.conv.sym("E", false));
        }
        if let Some(t) = a.tok(PS_DIFFERENTIAL) {
            let v = differential_var_str(&t.text);
            return Ok(crate::symbols::make_symbol(&format!("d{v}"), self.conv.is_real));
        }
        if let Some(t) = a.tok(PS_VARIABLE) {
            let text = &t.text;
            let is_pct = text.ends_with("\\%");
            let trim = if is_pct { 3 } else { 1 };
            let chars: Vec<char> = text.chars().collect();
            let inner: String = chars[10..chars.len().saturating_sub(trim).max(10)].iter().collect();
            let s = self.conv.sym(&inner, false);
            return Ok(if is_pct { pct(s) } else { s });
        }
        if let Some(t) = a.tok(PS_FUNC_GAMMA) {
            return Ok(get_symbol(&t.text, self.conv.is_real, true));
        }
        if let Some(t) = a.tok(PS_PERCENT_NUMBER) {
            let txt = t.text.replace("\\%", "").replace('%', "").replace(',', "");
            let n = parse_number(txt.trim())?;
            return Ok(pct(n));
        }
        err("bad atom")
    }

    fn frac(&self, f: &RuleNode) -> CResult<Val> {
        let ex = f.rules(RULE_expr);
        if ex.len() != 2 {
            return err("bad frac");
        }
        let (upper, lower) = (ex[0], ex[1]);
        let ll = lower.leaves();
        let ul = upper.leaves();
        let mut diff_op = false;
        let mut partial_op = false;
        let mut wrt: Option<String> = None;
        if ll.len() == 1 && ll[0].ttype == PS_DIFFERENTIAL {
            wrt = Some(differential_var_str(&ll[0].text));
            diff_op = true;
        } else if ll.len() == 2
            && ll[0].ttype == PS_SYMBOL
            && ll[0].text == "\\partial"
            && (ll[1].ttype == PS_LETTER_NO_E || ll[1].ttype == PS_SYMBOL)
        {
            partial_op = true;
            let mut w = ll[1].text.clone();
            if ll[1].ttype == PS_SYMBOL {
                w = w.chars().skip(1).collect();
            }
            wrt = Some(w);
        }
        if diff_op || partial_op {
            let w = self.conv.sym(wrt.as_deref().unwrap_or("x"), true);
            if diff_op && ul.len() == 1 && ul[0].ttype == PS_LETTER_NO_E && ul[0].text == "d" {
                return Ok(Val::DiffOp(w));
            }
            if partial_op && ul.len() == 1 && ul[0].ttype == PS_SYMBOL && ul[0].text == "\\partial" {
                return Ok(Val::DiffOp(w));
            }
            let upper_text = source_text(upper, self.chars());
            let mut top: Option<Expr> = None;
            if diff_op && upper_text.starts_with('d') {
                top = Some(self.conv.parse(&upper_text[1..])?);
            } else if partial_op && ul.first().map(|t| t.text.as_str()) == Some("\\partial") {
                top = Some(self.conv.parse(&upper_text["\\partial".len()..])?);
            }
            if let Some(t) = top {
                if crate::eval::truthy(&t) {
                    return Ok(Val::E(Expr::Derivative(Box::new(t), Box::new(w))));
                }
            }
        }
        let top = self.expr(upper)?;
        let bot = self.expr(lower)?;
        if top.is_matrix() || bot.is_matrix() {
            return err("matrix fraction");
        }
        Ok(Val::E(div(top, bot)))
    }

    // ----- functions -----

    fn func(&self, f: &RuleNode) -> CResult<Expr> {
        if let Some(fns) = f.rule(RULE_func_normal_single_arg) {
            let arg = if let Some(a) = f.rule(RULE_func_single_arg) {
                self.expr(a.rule(RULE_expr).ok_or("x")?)?
            } else {
                let a = f.rule(RULE_func_single_arg_noparens).ok_or("x")?;
                self.mp(a.rule(RULE_mp_nofunc).ok_or("x")?)?
            };
            let first = fns.leaves().first().map(|t| t.text.clone()).unwrap_or_default();
            let mut name: String = first.chars().skip(1).collect();
            let mut func_pow = match f.rule(RULE_supexpr) {
                Some(s) => Some(self.supexpr(s)?),
                None => None,
            };
            let trig = |n: &str| -> Option<Func> {
                Some(match n {
                    "sin" => Func::Sin, "cos" => Func::Cos, "tan" => Func::Tan,
                    "csc" => Func::Csc, "sec" => Func::Sec, "cot" => Func::Cot,
                    "asin" => Func::Asin, "acos" => Func::Acos, "atan" => Func::Atan,
                    "acsc" => Func::Acsc, "asec" => Func::Asec, "acot" => Func::Acot,
                    "sinh" => Func::Sinh, "cosh" => Func::Cosh, "tanh" => Func::Tanh,
                    "asinh" => Func::Asinh, "acosh" => Func::Acosh, "atanh" => Func::Atanh,
                    _ => return None,
                })
            };
            let is_neg_one = |p: &Option<Expr>| matches!(p, Some(Expr::Num(n)) if *n == Num::int(-1));
            let e = if ["arcsin", "arccos", "arctan", "arccsc", "arcsec", "arccot"].contains(&name.as_str()) {
                Expr::Func(trig(&format!("a{}", &name[3..])).unwrap(), vec![arg])
            } else if ["arsinh", "arcosh", "artanh"].contains(&name.as_str()) {
                Expr::Func(trig(&format!("a{}", &name[2..])).unwrap(), vec![arg])
            } else if ["arcsinh", "arccosh", "arctanh"].contains(&name.as_str()) {
                Expr::Func(trig(&format!("a{}", &name[3..])).unwrap(), vec![arg])
            } else if name == "operatorname" {
                let op = fns.rules(RULE_func_operator_names_single_arg);
                let opname = op.first().map(|r| r.text()).unwrap_or_default();
                match opname.as_str() {
                    "arsinh" | "arcosh" | "artanh" => {
                        Expr::Func(trig(&format!("a{}", &opname[2..])).unwrap(), vec![arg])
                    }
                    "arcsinh" | "arccosh" | "arctanh" => {
                        Expr::Func(trig(&format!("a{}", &opname[3..])).unwrap(), vec![arg])
                    }
                    "floor" => Expr::Func(Func::Floor, vec![arg]),
                    "ceil" => Expr::Func(Func::Ceiling, vec![arg]),
                    "trace" | "tr" => crate::matrix::trace(&arg)?,
                    "norm" => crate::matrix::norm(arg)?,
                    "eye" | "rank" | "rref" | "nullspace" | "cols" | "rows" | "eig" | "eigen"
                    | "diagonalize" | "eigenvals" | "eigenvalues" | "eigenvects" | "eigenvectors"
                    | "svd" | "SVD" => return err("unsupported matrix operator"),
                    other => Expr::Func(Func::Undefined(other.to_string()), vec![arg]),
                }
            } else if name == "log" || name == "ln" {
                let base = if let Some(sub) = f.rule(RULE_subexpr) {
                    self.subexpr_value(sub)?
                } else if name == "log" {
                    int(10)
                } else {
                    Expr::E
                };
                crate::functions::log_with_base(arg, base)
            } else if name == "exp" || name == "exponentialE" {
                Expr::Func(Func::Exp, vec![arg])
            } else if name == "floor" {
                Expr::Func(Func::Floor, vec![arg])
            } else if name == "ceil" {
                Expr::Func(Func::Ceiling, vec![arg])
            } else if name == "det" {
                crate::matrix::det(&arg)?
            } else if name == "Gamma" || name == "gamma" {
                crate::functions::gamma(arg)
            } else if ["sin", "cos", "tan", "csc", "sec", "cot", "sinh", "cosh", "tanh"].contains(&name.as_str()) {
                if is_neg_one(&func_pow) {
                    name = format!("a{name}");
                    func_pow = None;
                }
                Expr::Func(trig(&name).unwrap(), vec![arg])
            } else {
                Expr::Func(Func::Undefined(name.clone()), vec![arg])
            };
            if let Some(p) = func_pow {
                if eval::truthy(&p) {
                    return Ok(pow(e, p));
                }
            }
            return Ok(e);
        }

        if let Some(fnm) = f.rule(RULE_func_normal_multi_arg) {
            let texts: Vec<String> = if let Some(a) = f.rule(RULE_func_multi_arg) {
                a.text().split(',').map(|s| s.to_string()).collect()
            } else {
                return err("multi-arg function without parentheses");
            };
            let mut args = Vec::new();
            for t in &texts {
                args.push(self.conv.parse(t)?);
            }
            let first = fnm.leaves().first().map(|t| t.text.clone()).unwrap_or_default();
            let name: String = first.chars().skip(1).collect();
            let e = if name == "operatorname" {
                let opname = fnm.rules(RULE_func_operator_names_multi_arg).first().map(|r| r.text()).unwrap_or_default();
                match opname.as_str() {
                    "gcd" | "lcm" => crate::functions::gcd_lcm(&opname, args)?,
                    "zeros" | "ones" | "diag" | "hstack" | "vstack" | "orth" | "ortho" | "orthogonal"
                    | "orthogonalize" => return err("unsupported matrix operator"),
                    other => Expr::Func(Func::Undefined(other.to_string()), args),
                }
            } else if name == "gcd" || name == "lcm" {
                crate::functions::gcd_lcm(&name, args)?
            } else if name == "max" {
                Expr::Func(Func::Max, args)
            } else if name == "min" {
                Expr::Func(Func::Min, args)
            } else {
                Expr::Func(Func::Undefined(name), args)
            };
            if let Some(s) = f.rule(RULE_supexpr) {
                let p = self.supexpr(s)?;
                if eval::truthy(&p) {
                    return Ok(pow(e, p));
                }
            }
            return Ok(e);
        }

        if let Some(ae) = f.rule(RULE_atom_expr_no_supexpr) {
            let fname = ae.text();
            let ca = f.rule(RULE_func_common_args).ok_or("x")?;
            let mut texts: Vec<String> = ca.text().split(',').map(|s| s.to_string()).collect();
            if texts.last().map(|s| s.is_empty()).unwrap_or(false) {
                texts.pop();
            }
            let mut args = Vec::new();
            for t in &texts {
                args.push(self.conv.parse(t)?);
            }
            let app = Expr::Func(Func::Undefined(fname), args);
            if let Some(s) = f.rule(RULE_supexpr) {
                let p = self.supexpr(s)?;
                return Ok(pow(app, p));
            }
            return Ok(app);
        }

        if f.has(PS_FUNC_INT) {
            return self.integral(f);
        }
        if f.has(PS_FUNC_SQRT) {
            let ex = f.rules(RULE_expr);
            if ex.len() == 2 {
                let root = self.expr(ex[0])?;
                let base = self.expr(ex[1])?;
                return Ok(pow(base, crate::functions::reciprocal(root)));
            }
            let base = self.expr(ex[0])?;
            return Ok(pow(base, half()));
        }
        if f.has(PS_FUNC_SUM) || f.has(PS_FUNC_PROD) {
            let val = self.mp(f.rule(RULE_mp).ok_or("x")?)?;
            let subeq = f.rule(RULE_subeq).ok_or("x")?;
            let eq = subeq.rule(RULE_equality).ok_or("x")?;
            let ex = eq.rules(RULE_expr);
            let var = self.expr(ex[0])?;
            let lo = self.expr(ex[1])?;
            let hi = self.supexpr(f.rule(RULE_supexpr).ok_or("x")?)?;
            return Ok(Expr::Sum {
                f: Box::new(val),
                var: Box::new(var),
                lo: Box::new(lo),
                hi: Box::new(hi),
                product: f.has(PS_FUNC_PROD),
            });
        }
        if f.has(PS_FUNC_LIM) {
            let sub = f.rule(RULE_limit_sub).ok_or("x")?;
            let var = if let Some(t) = sub.tok(PS_LETTER_NO_E) {
                self.conv.sym(&t.text, true)
            } else if let Some(t) = sub.tok(PS_GREEK_CMD) {
                get_symbol(t.text.trim(), self.conv.is_real, false)
            } else if let Some(t) = sub.tok(PS_OTHER_SYMBOL_CMD) {
                get_symbol(t.text.trim(), self.conv.is_real, false)
            } else {
                self.conv.sym("x", true)
            };
            let dir_plus = !sub.has(PS_SUB);
            let to = self.expr(sub.rule(RULE_expr).ok_or("x")?)?;
            let content = self.mp(f.rule(RULE_mp).ok_or("x")?)?;
            return Ok(Expr::Limit {
                f: Box::new(content),
                var: Box::new(var),
                to: Box::new(to),
                dir_plus,
            });
        }
        if f.has(PS_EXP_E) {
            let arg = match f.rule(RULE_supexpr) {
                Some(s) => self.supexpr(s)?,
                None => int(1),
            };
            return Ok(crate::functions::exp_eval(arg));
        }
        err("unknown func")
    }

    fn integral(&self, f: &RuleNode) -> CResult<Expr> {
        let mut integrand = if let Some(a) = f.rule(RULE_additive) {
            self.additive(a)?
        } else if let Some(fr) = f.rule(RULE_frac) {
            unwrap_e(self.frac(fr)?)?
        } else {
            int(1)
        };
        let var = if let Some(t) = f.tok(PS_DIFFERENTIAL) {
            self.conv.sym(&differential_var_str(&t.text), true)
        } else {
            let mut found: Option<(String, Expr)> = None;
            for s in integrand.free_symbols() {
                let plain = crate::symbols::display_name(&s);
                let cs: Vec<char> = plain.chars().collect();
                if cs.len() > 1 && cs[0] == 'd' {
                    let rest: String = if cs[1] == '\\' { cs[2..].iter().collect() } else { cs[1..].iter().collect() };
                    found = Some((s.clone(), self.conv.sym(&rest, true)));
                }
            }
            match found {
                Some((s, v)) => {
                    integrand = integrand.subs_sym(&s, &int(1));
                    v
                }
                None => self.conv.sym("x", true),
            }
        };
        let bounds = if let Some(sub) = f.rule(RULE_subexpr) {
            let lower = self.subexpr_value(sub)?;
            let upper = self.supexpr(f.rule(RULE_supexpr).ok_or("x")?)?;
            Some((Box::new(lower), Box::new(upper)))
        } else {
            None
        };
        Ok(Expr::Integral { f: Box::new(integrand), var: Box::new(var), bounds })
    }
}

fn differential_var_str(text: &str) -> String {
    let cs: Vec<char> = text.chars().collect();
    let mut idx = 1;
    for (i, c) in cs.iter().enumerate().skip(1) {
        if !(*c == ' ' || *c == '\r' || *c == '\n' || *c == '\t') {
            idx = i;
            break;
        }
    }
    let mut s: String = cs[idx..].iter().collect();
    if s.starts_with('\\') {
        s.remove(0);
    }
    s
}

fn accent_text(raw: &str) -> String {
    let start = match raw.find('{') {
        Some(i) => i,
        None => return raw.to_string(),
    };
    let mut accent_name = raw[1..start].to_string();
    let inner = &raw[start + 1..raw.len() - 1];
    let mut accent_text: String = inner.replace(' ', "");
    if accent_name == "bar" || accent_name == "overline" {
        accent_name = "bar".into();
    } else if accent_name == "vec" || accent_name == "overrightarrow" {
        accent_name = "vec".into();
    } else if accent_name == "tilde" || accent_name == "widetilde" {
        accent_name = "tilde".into();
    } else if accent_name.contains("text") || accent_name.contains("mbox") {
        accent_name = String::new();
        accent_text = accent_text.replace(['(', ')'], "");
    } else if accent_name.contains("math") {
        accent_name = "math".into();
    }
    if accent_name.is_empty() {
        accent_text
    } else {
        format!("{accent_name}{{{accent_text}}}")
    }
}

#[allow(dead_code)]
fn _unused(_: BigInt) -> bool {
    BigInt::one().is_one()
}
