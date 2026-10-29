//! PyO3 bindings: `fastmathverify._core`.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyString};

use crate::expr::Expr as RExpr;
use crate::extract::{
    ExprConfig, ExtractionMode, FallbackMode, Item, LatexConfig, StringConfig, Target,
};
use crate::grader::{verify, VerifyOpts};
use crate::normalize::{BoxedMode, NormalizationConfig};

/// A parsed mathematical expression (opaque, Rust-backed).
#[pyclass(name = "Expr", module = "fastmathverify._core", frozen)]
#[derive(Clone)]
pub struct PyExpr {
    pub inner: RExpr,
}

#[pymethods]
impl PyExpr {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.inner.to_string()
    }
    /// SymPy constructor string.
    fn srepr(&self) -> String {
        crate::printer::srepr(&self.inner)
    }
    /// Equivalent SymPy object (requires sympy).
    fn to_sympy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        py.import("fastmathverify")?.call_method1("_srepr_to_sympy", (crate::printer::srepr(&self.inner),))
    }
    fn _sympy_<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.to_sympy(py)
    }
    fn __eq__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> bool {
        if let Ok(o) = other.downcast::<PyExpr>() {
            return crate::grader::struct_eq(&self.inner, &o.get().inner);
        }
        if other.is_instance_of::<PyString>() {
            return false;
        }
        match self.to_sympy(py) {
            Ok(s) => s.eq(other).unwrap_or(false),
            Err(_) => false,
        }
    }
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        self.inner.to_string().hash(&mut h);
        h.finish()
    }
    /// Debug representation of the internal tree.
    fn tree(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Target spec passed from Python as a tuple:
/// ("latex", try_without_anchor, boxed_priority, basic, units, malformed, nits, boxed, equations)
/// ("expr", try_without_anchor)
/// ("string", strings, try_without_anchor, lowercase)
fn target_from_py(t: &Bound<'_, PyAny>) -> PyResult<Target> {
    let kind: String = t.get_item(0)?.extract()?;
    Ok(match kind.as_str() {
        "latex" => {
            let boxed: String = t.get_item(7)?.extract()?;
            Target::Latex(LatexConfig {
                try_extract_without_anchor: t.get_item(1)?.extract()?,
                boxed_match_priority: t.get_item(2)?.extract()?,
                normalization: NormalizationConfig {
                    basic_latex: t.get_item(3)?.extract()?,
                    units: t.get_item(4)?.extract()?,
                    malformed_operators: t.get_item(5)?.extract()?,
                    nits: t.get_item(6)?.extract()?,
                    boxed: match boxed.as_str() {
                        "all" => BoxedMode::All,
                        "last" => BoxedMode::Last,
                        _ => BoxedMode::None,
                    },
                    equations: t.get_item(8)?.extract()?,
                },
            })
        }
        "expr" => Target::Expr(ExprConfig { try_extract_without_anchor: t.get_item(1)?.extract()? }),
        "string" => Target::Str(StringConfig {
            strings: t.get_item(1)?.extract()?,
            try_extract_without_anchor: t.get_item(2)?.extract()?,
            lowercase: t.get_item(3)?.extract()?,
        }),
        other => return Err(PyValueError::new_err(format!("unknown target {other}"))),
    })
}

fn targets_from_py(ts: &Bound<'_, PyList>) -> PyResult<Vec<Target>> {
    ts.iter().map(|t| target_from_py(&t)).collect()
}

fn items_to_py<'py>(py: Python<'py>, items: Vec<Item>) -> PyResult<Bound<'py, PyList>> {
    let out = PyList::empty(py);
    for it in items {
        match it {
            Item::Expr(e) => out.append(Py::new(py, PyExpr { inner: e })?)?,
            Item::Str(s) => out.append(s)?,
        }
    }
    Ok(out)
}

fn item_from_py(o: &Bound<'_, PyAny>) -> PyResult<Option<Item>> {
    if let Ok(e) = o.downcast::<PyExpr>() {
        return Ok(Some(Item::Expr(e.get().inner.clone())));
    }
    if let Ok(s) = o.downcast::<PyString>() {
        return Ok(Some(Item::Str(s.to_str()?.to_string())));
    }
    Ok(None)
}

fn items_from_py(o: &Bound<'_, PyAny>) -> PyResult<Vec<Item>> {
    if let Ok(l) = o.downcast::<PyList>() {
        let mut v = Vec::new();
        for x in l.iter() {
            if let Some(i) = item_from_py(&x)? {
                v.push(i);
            }
        }
        return Ok(v);
    }
    Ok(item_from_py(o)?.into_iter().collect())
}

fn modes(fallback: &str, extraction: &str) -> (FallbackMode, ExtractionMode) {
    (
        if fallback == "first_match" { FallbackMode::FirstMatch } else { FallbackMode::NoFallback },
        if extraction == "first_match" { ExtractionMode::FirstMatch } else { ExtractionMode::AnyMatch },
    )
}

#[pyfunction]
#[pyo3(signature = (pred, targets, fallback_mode, extraction_mode, timeout=None))]
fn parse<'py>(
    py: Python<'py>,
    pred: &str,
    targets: &Bound<'py, PyList>,
    fallback_mode: &str,
    extraction_mode: &str,
    timeout: Option<f64>,
) -> PyResult<Bound<'py, PyList>> {
    let ts = targets_from_py(targets)?;
    let (f, m) = modes(fallback_mode, extraction_mode);
    let pred = pred.to_string();
    let items = py.detach(move || crate::extract::parse_with_timeout(&pred, &ts, f, m, timeout));
    items_to_py(py, items)
}

#[pyfunction]
#[pyo3(signature = (gold, target, float_rounding, numeric_precision, strict, allow_set_relation_comp, timeout=None))]
#[allow(clippy::too_many_arguments)]
fn verify_items(
    py: Python<'_>,
    gold: &Bound<'_, PyAny>,
    target: &Bound<'_, PyAny>,
    float_rounding: u32,
    numeric_precision: u32,
    strict: bool,
    allow_set_relation_comp: bool,
    timeout: Option<f64>,
) -> PyResult<bool> {
    let g = items_from_py(gold)?;
    let t = items_from_py(target)?;
    let o = VerifyOpts { float_rounding, numeric_precision, strict, allow_set_relation_comp };
    Ok(py.detach(move || crate::grader::verify_with_timeout(&g, &t, &o, timeout)))
}

fn run_parallel<T: Send, R: Send>(inputs: Vec<T>, threads: usize, f: impl Fn(T) -> R + Sync) -> Vec<R> {
    let n = inputs.len();
    let threads = threads.max(1).min(n.max(1));
    if threads == 1 {
        return inputs.into_iter().map(f).collect();
    }
    let work = std::sync::Mutex::new(inputs.into_iter().enumerate().collect::<Vec<_>>().into_iter());
    let results = std::sync::Mutex::new(Vec::with_capacity(n));
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let next = work.lock().unwrap().next();
                let Some((i, x)) = next else { break };
                let r = f(x);
                results.lock().unwrap().push((i, r));
            });
        }
    });
    let mut r = results.into_inner().unwrap();
    r.sort_by_key(|(i, _)| *i);
    r.into_iter().map(|(_, x)| x).collect()
}

fn default_threads() -> usize {
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
}

/// Parse and verify many (gold, prediction) text pairs in parallel without the GIL.
#[pyfunction]
#[pyo3(signature = (golds, preds, gold_targets, pred_targets, float_rounding=6, numeric_precision=15, strict=true, allow_set_relation_comp=false, threads=None))]
#[allow(clippy::too_many_arguments)]
fn verify_batch(
    py: Python<'_>,
    golds: Vec<String>,
    preds: Vec<String>,
    gold_targets: &Bound<'_, PyList>,
    pred_targets: &Bound<'_, PyList>,
    float_rounding: u32,
    numeric_precision: u32,
    strict: bool,
    allow_set_relation_comp: bool,
    threads: Option<usize>,
) -> PyResult<Vec<bool>> {
    if golds.len() != preds.len() {
        return Err(PyValueError::new_err("golds and preds must have the same length"));
    }
    let gt = targets_from_py(gold_targets)?;
    let pt = targets_from_py(pred_targets)?;
    let o = VerifyOpts { float_rounding, numeric_precision, strict, allow_set_relation_comp };
    let threads = threads.unwrap_or_else(default_threads);
    let pairs: Vec<(String, String)> = golds.into_iter().zip(preds).collect();
    Ok(py.detach(move || {
        run_parallel(pairs, threads, |(g, p)| {
            let gi = crate::extract::parse_with_timeout(&g, &gt, FallbackMode::FirstMatch, ExtractionMode::AnyMatch, Some(5.0));
            let pi = crate::extract::parse_with_timeout(&p, &pt, FallbackMode::FirstMatch, ExtractionMode::AnyMatch, Some(5.0));
            verify(&gi, &pi, &o)
        })
    }))
}

/// Parse many texts in parallel without the GIL.
#[pyfunction]
#[pyo3(signature = (texts, targets, fallback_mode="first_match", extraction_mode="any_match", threads=None))]
fn parse_batch<'py>(
    py: Python<'py>,
    texts: Vec<String>,
    targets: &Bound<'py, PyList>,
    fallback_mode: &str,
    extraction_mode: &str,
    threads: Option<usize>,
) -> PyResult<Bound<'py, PyList>> {
    let ts = targets_from_py(targets)?;
    let (f, m) = modes(fallback_mode, extraction_mode);
    let threads = threads.unwrap_or_else(default_threads);
    let res = py.detach(move || run_parallel(texts, threads, |t| crate::extract::parse(&t, &ts, f, m)));
    let out = PyList::empty(py);
    for items in res {
        out.append(items_to_py(py, items)?)?;
    }
    Ok(out)
}

/// Like `parse`, also returning (kind, extracted string) of the first regex match.
#[pyfunction]
#[pyo3(signature = (pred, targets, fallback_mode, extraction_mode))]
fn parse_with_source<'py>(
    py: Python<'py>,
    pred: &str,
    targets: &Bound<'py, PyList>,
    fallback_mode: &str,
    extraction_mode: &str,
) -> PyResult<(Bound<'py, PyList>, Option<(String, String)>)> {
    let ts = targets_from_py(targets)?;
    let (f, m) = modes(fallback_mode, extraction_mode);
    let items = crate::extract::parse(pred, &ts, f, m);
    let src = crate::extract::FIRST_MATCH.with(|x| x.borrow().clone()).map(|(k, s)| (k.to_string(), s));
    Ok((items_to_py(py, items)?, src))
}

/// Normalize a LaTeX string (latex2sympy2_extended.normalize_latex).
#[pyfunction]
#[pyo3(signature = (text, basic_latex=true, units=false, malformed_operators=false, nits=false, boxed="all", equations=false))]
fn normalize_latex(
    text: &str,
    basic_latex: bool,
    units: bool,
    malformed_operators: bool,
    nits: bool,
    boxed: &str,
    equations: bool,
) -> String {
    let cfg = NormalizationConfig {
        basic_latex,
        units,
        malformed_operators,
        nits,
        boxed: match boxed {
            "all" => BoxedMode::All,
            "last" => BoxedMode::Last,
            _ => BoxedMode::None,
        },
        equations,
    };
    crate::normalize::normalize_latex(text, &cfg)
}

/// Convert a single (already normalized) LaTeX string, like latex2sympy.
#[pyfunction]
fn latex_to_expr(latex: &str) -> PyResult<PyExpr> {
    crate::extract::parse_latex(latex)
        .map(|e| PyExpr { inner: e })
        .map_err(PyValueError::new_err)
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyExpr>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(verify_items, m)?)?;
    m.add_function(wrap_pyfunction!(verify_batch, m)?)?;
    m.add_function(wrap_pyfunction!(parse_batch, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_latex, m)?)?;
    m.add_function(wrap_pyfunction!(parse_with_source, m)?)?;
    m.add_function(wrap_pyfunction!(latex_to_expr, m)?)?;
    Ok(())
}
