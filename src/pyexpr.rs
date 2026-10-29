//! Emulates `sympy.parse_expr(s, evaluate=False)` for the plain arithmetic
//! strings math_verify feeds it (digits, '.', + - * / ** // and parentheses).

use crate::convert::parse_decimal;
use crate::eval::exact;
use crate::expr::*;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
enum Tk {
    Num(String),
    Op(&'static str),
    LP,
    RP,
}

fn tokenize(s: &str) -> Result<Vec<Tk>, String> {
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < cs.len() {
        let c = cs[i];
        if c == ' ' || c == '\t' || c == '\x0c' {
            i += 1;
            continue;
        }
        if c.is_ascii_digit() || (c == '.' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit()) {
            let st = i;
            while i < cs.len() && cs[i].is_ascii_digit() {
                i += 1;
            }
            if i < cs.len() && cs[i] == '.' {
                i += 1;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    i += 1;
                }
            }
            if i < cs.len() && (cs[i] == '.' || cs[i].is_alphanumeric()) {
                return Err("bad number".into());
            }
            out.push(Tk::Num(cs[st..i].iter().collect()));
            continue;
        }
        let two: String = cs[i..(i + 2).min(cs.len())].iter().collect();
        if two == "**" {
            out.push(Tk::Op("**"));
            i += 2;
            continue;
        }
        if two == "//" {
            out.push(Tk::Op("//"));
            i += 2;
            continue;
        }
        match c {
            '+' => out.push(Tk::Op("+")),
            '-' => out.push(Tk::Op("-")),
            '*' => out.push(Tk::Op("*")),
            '/' => out.push(Tk::Op("/")),
            '(' => out.push(Tk::LP),
            ')' => out.push(Tk::RP),
            _ => return Err(format!("unexpected {c}")),
        }
        i += 1;
    }
    Ok(out)
}

struct P {
    t: Vec<Tk>,
    i: usize,
}

fn number(s: &str) -> Result<Expr, String> {
    if s.chars().all(|c| c.is_ascii_digit()) {
        if s.len() > 1 && s.starts_with('0') && !s.chars().all(|c| c == '0') {
            return Err("leading zeros".into());
        }
        return Ok(Expr::Num(Num::Int(s.parse::<BigInt>().map_err(|e| e.to_string())?)));
    }
    let r = parse_decimal(s).ok_or("bad float")?;
    Ok(Expr::Num(Num::float_from_rat(r)))
}

fn all_numeric(e: &Expr) -> bool {
    match e {
        Expr::Num(_) => true,
        Expr::Add(v) | Expr::Mul(v) => v.iter().all(all_numeric),
        _ => false,
    }
}

fn has_float(e: &Expr) -> bool {
    match e {
        Expr::Num(n) => n.is_float(),
        _ => e.children().iter().any(|c| has_float(c)),
    }
}

/// Python-level `-x` on a SymPy object (an evaluated Mul(-1, x)).
fn py_neg(x: Expr) -> Expr {
    match x {
        Expr::Num(n) => Expr::Num(n.neg()),
        other if all_numeric(&other) => {
            if has_float(&other) {
                let v = crate::eval::numeric(&other, &Default::default()).map(|z| z.re).unwrap_or(0.0);
                let r = Rat::from_float(-v).unwrap_or_default();
                Expr::Num(Num::Float { exact: r, f: -v })
            } else {
                match exact(&other) {
                    Some(r) => Expr::Num(Num::rat(-r)),
                    None => Expr::Mul(vec![int(-1), other]),
                }
            }
        }
        other => Expr::Mul(vec![int(-1), other]),
    }
}

impl P {
    fn peek(&self) -> Option<&Tk> {
        self.t.get(self.i)
    }
    fn expr(&mut self) -> Result<Expr, String> {
        let mut l = self.term()?;
        while let Some(Tk::Op(o)) = self.peek() {
            let o = *o;
            if o != "+" && o != "-" {
                break;
            }
            self.i += 1;
            let r = self.term()?;
            l = if o == "+" { Expr::Add(vec![l, r]) } else { Expr::Add(vec![l, Expr::Mul(vec![int(-1), r])]) };
        }
        Ok(l)
    }
    fn term(&mut self) -> Result<Expr, String> {
        let mut l = self.factor()?;
        while let Some(Tk::Op(o)) = self.peek() {
            let o = *o;
            if o != "*" && o != "/" && o != "//" {
                break;
            }
            self.i += 1;
            let r = self.factor()?;
            l = match o {
                "*" => Expr::Mul(vec![l, r]),
                "/" => Expr::Mul(vec![l, pow(r, int(-1))]),
                _ => Expr::Func(Func::Floor, vec![Expr::Mul(vec![l, pow(r, int(-1))])]),
            };
        }
        Ok(l)
    }
    fn factor(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Tk::Op("-")) => {
                self.i += 1;
                Ok(py_neg(self.factor()?))
            }
            Some(Tk::Op("+")) => {
                self.i += 1;
                self.factor()
            }
            _ => self.power(),
        }
    }
    fn power(&mut self) -> Result<Expr, String> {
        let base = self.atom()?;
        if let Some(Tk::Op("**")) = self.peek() {
            self.i += 1;
            let e = self.factor()?;
            return Ok(pow(base, e));
        }
        Ok(base)
    }
    fn atom(&mut self) -> Result<Expr, String> {
        match self.peek().cloned() {
            Some(Tk::Num(s)) => {
                self.i += 1;
                number(&s)
            }
            Some(Tk::LP) => {
                self.i += 1;
                let e = self.expr()?;
                if self.peek() != Some(&Tk::RP) {
                    return Err("expected )".into());
                }
                self.i += 1;
                Ok(e)
            }
            _ => Err("unexpected token".into()),
        }
    }
}

pub fn parse_expr(s: &str) -> Result<Expr, String> {
    let t = tokenize(s)?;
    if t.is_empty() {
        return Err("empty".into());
    }
    let mut p = P { t, i: 0 };
    let e = p.expr()?;
    if p.i != p.t.len() {
        return Err("trailing tokens".into());
    }
    Ok(e)
}
