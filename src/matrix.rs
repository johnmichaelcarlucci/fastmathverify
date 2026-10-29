//! Minimal explicit-matrix support (sympy.Matrix built from \begin{pmatrix}...).

use crate::expr::*;

type R<T> = Result<T, String>;

pub fn from_rows(rows: Vec<Vec<Expr>>) -> R<Expr> {
    let r = rows.len();
    let c = rows.first().map(|x| x.len()).unwrap_or(0);
    if rows.iter().any(|x| x.len() != c) {
        return Err("mismatched matrix rows".into());
    }
    Ok(Expr::Matrix { rows: r, cols: c, data: rows.into_iter().flatten().collect() })
}

pub fn mat_add(a: Expr, b: Expr) -> R<Expr> {
    match (a, b) {
        (Expr::Matrix { rows, cols, data }, Expr::Matrix { rows: r2, cols: c2, data: d2 })
            if rows == r2 && cols == c2 =>
        {
            let data = data.into_iter().zip(d2).map(|(x, y)| Expr::Add(vec![x, y])).collect();
            Ok(Expr::Matrix { rows, cols, data })
        }
        _ => Err("matrix addition shape mismatch".into()),
    }
}

pub fn mat_scale(k: Expr, m: Expr) -> R<Expr> {
    match m {
        Expr::Matrix { rows, cols, data } => Ok(Expr::Matrix {
            rows,
            cols,
            data: data.into_iter().map(|x| Expr::Mul(vec![k.clone(), x])).collect(),
        }),
        _ => Err("not a matrix".into()),
    }
}

pub fn mat_mul(a: Expr, b: Expr) -> R<Expr> {
    match (a, b) {
        (Expr::Matrix { rows, cols, data }, Expr::Matrix { rows: r2, cols: c2, data: d2 }) => {
            if cols != r2 {
                return Err("matrix product shape mismatch".into());
            }
            let mut out = Vec::with_capacity(rows * c2);
            for i in 0..rows {
                for j in 0..c2 {
                    let terms = (0..cols)
                        .map(|k| Expr::Mul(vec![data[i * cols + k].clone(), d2[k * c2 + j].clone()]))
                        .collect();
                    out.push(Expr::Add(terms));
                }
            }
            Ok(Expr::Matrix { rows, cols: c2, data: out })
        }
        (m @ Expr::Matrix { .. }, k) | (k, m @ Expr::Matrix { .. }) => mat_scale(k, m),
        _ => Err("no matrix".into()),
    }
}

pub fn transpose(rows: usize, cols: usize, data: &[Expr]) -> Expr {
    let mut out = Vec::with_capacity(data.len());
    for j in 0..cols {
        for i in 0..rows {
            out.push(data[i * cols + j].clone());
        }
    }
    Expr::Matrix { rows: cols, cols: rows, data: out }
}

pub fn det(m: &Expr) -> R<Expr> {
    match m {
        Expr::Matrix { rows, cols, data } if rows == cols && *rows > 0 && *rows <= 6 => {
            Ok(det_rec(*rows, data))
        }
        _ => Err("det of non-square matrix".into()),
    }
}

fn det_rec(n: usize, d: &[Expr]) -> Expr {
    if n == 1 {
        return d[0].clone();
    }
    let mut terms = Vec::new();
    for j in 0..n {
        let mut minor = Vec::with_capacity((n - 1) * (n - 1));
        for i in 1..n {
            for k in 0..n {
                if k != j {
                    minor.push(d[i * n + k].clone());
                }
            }
        }
        let sign = if j % 2 == 0 { int(1) } else { int(-1) };
        terms.push(Expr::Mul(vec![sign, d[j].clone(), det_rec(n - 1, &minor)]));
    }
    Expr::Add(terms)
}

pub fn trace(m: &Expr) -> R<Expr> {
    match m {
        Expr::Matrix { rows, cols, data } if rows == cols => {
            Ok(Expr::Add((0..*rows).map(|i| data[i * cols + i].clone()).collect()))
        }
        _ => Err("trace of non-square".into()),
    }
}

pub fn norm(m: Expr) -> R<Expr> {
    match m {
        Expr::Matrix { data, .. } => {
            let s = Expr::Add(data.into_iter().map(|x| pow(Expr::Func(Func::Abs, vec![x]), int(2))).collect());
            Ok(pow(s, half()))
        }
        _ => Err("norm of scalar".into()),
    }
}
