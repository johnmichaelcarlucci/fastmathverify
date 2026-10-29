"""fastmathverify: a Rust rewrite of Hugging Face Math-Verify.

Drop-in replacement for ``math_verify``: same ``parse()`` / ``verify()`` API and
extraction configs, but thread-safe, hang-free (no ``signal.alarm``), GIL-free,
and much faster. Use ``verify_batch`` to grade many answers on all cores.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal, Sequence

from . import _core
from ._core import Expr
from . import grader

def _srepr_to_sympy(srepr: str):
    """Rebuild a SymPy object from the constructor string our printer produces."""
    import sympy

    ns = {k: getattr(sympy, k) for k in dir(sympy) if not k.startswith("_")}
    return eval(srepr, {"__builtins__": {}}, ns)


__all__ = [
    "parse",
    "verify",
    "verify_batch",
    "parse_batch",
    "math_metric",
    "Expr",
    "LatexExtractionConfig",
    "ExprExtractionConfig",
    "StringExtractionConfig",
    "LatexNormalizationConfig",
    "NormalizationConfig",
    "normalize_latex",
    "latex_to_expr",
]


@dataclass(frozen=True)
class NormalizationConfig:
    """LaTeX normalization switches (same fields as latex2sympy2_extended.NormalizationConfig)."""

    basic_latex: bool = True
    units: bool = False
    malformed_operators: bool = False
    nits: bool = False
    boxed: Literal["all", "last", "none"] = "all"
    equations: bool = False


LatexNormalizationConfig = NormalizationConfig


@dataclass(frozen=True)
class LatexExtractionConfig:
    try_extract_without_anchor: bool = True
    boxed_match_priority: int = 50
    normalization_config: NormalizationConfig = field(
        default_factory=lambda: NormalizationConfig(
            basic_latex=True, units=True, malformed_operators=True, nits=True, boxed="all", equations=False
        )
    )


@dataclass(frozen=True)
class ExprExtractionConfig:
    try_extract_without_anchor: bool = True


@dataclass(frozen=True)
class StringExtractionConfig:
    strings: tuple[str, ...] = field(default_factory=lambda: ("A", "B", "C", "D"))
    try_extract_without_anchor: bool = True
    lowercase: bool = True


ExtractionTarget = LatexExtractionConfig | ExprExtractionConfig | StringExtractionConfig

_DEFAULT_TARGETS = (LatexExtractionConfig(), ExprExtractionConfig())


def _spec(t) -> tuple:
    # Accept upstream math_verify config objects too (duck-typed by class name).
    name = type(t).__name__
    if name == "LatexExtractionConfig":
        n = t.normalization_config
        return ("latex", bool(t.try_extract_without_anchor), int(t.boxed_match_priority), bool(n.basic_latex),
                bool(n.units), bool(n.malformed_operators), bool(n.nits), str(n.boxed), bool(n.equations))
    if name == "ExprExtractionConfig":
        return ("expr", bool(t.try_extract_without_anchor))
    if name == "StringExtractionConfig":
        return ("string", list(t.strings), bool(t.try_extract_without_anchor), bool(t.lowercase))
    raise TypeError(f"unsupported extraction config: {t!r}")


def _specs(targets: Sequence) -> list:
    return [_spec(t) for t in targets]


def parse(
    pred: str,
    extraction_config: Sequence[ExtractionTarget] = _DEFAULT_TARGETS,
    fallback_mode: Literal["no_fallback", "first_match"] = "first_match",
    extraction_mode: Literal["first_match", "any_match"] = "any_match",
    parsing_timeout: int | None = 5,
    raise_on_error: bool = False,
) -> list:
    """Extract and parse math answers from ``pred``.

    Returns a list of ``Expr`` objects and/or the extracted fallback string, exactly
    like ``math_verify.parse``. ``parsing_timeout`` is enforced with a cooperative,
    thread-local deadline (no signals), so it works from any thread; on timeout the
    result is ``[]`` like upstream.
    """
    timeout = float(parsing_timeout) if parsing_timeout else None
    return _core.parse(str(pred), _specs(extraction_config), fallback_mode, extraction_mode, timeout)


def _coerce(x):
    if isinstance(x, (Expr, str)):
        return x
    if isinstance(x, list):
        return [_coerce(i) for i in x]
    if isinstance(x, bool):
        return str(x)
    if isinstance(x, (int, float)):
        return _core.latex_to_expr(repr(x))
    # SymPy objects (or anything with a LaTeX form): best-effort conversion.
    try:
        import sympy

        if isinstance(x, sympy.Basic) or hasattr(x, "_sympy_"):
            return _core.latex_to_expr(sympy.latex(x))
    except Exception:
        pass
    return str(x)


def verify(
    gold,
    target,
    float_rounding: int = 6,
    numeric_precision: int = 15,
    strict: bool = True,
    allow_set_relation_comp: bool = False,
    timeout_seconds: int | None = 5,
    raise_on_error: bool = False,
) -> bool:
    """Return True if any parsed ``target`` matches any parsed ``gold`` (math_verify.verify)."""
    timeout = float(timeout_seconds) if timeout_seconds else None
    if grader.sympy_expr_eq is not grader._DEFAULT:
        return _verify_with_hook(_coerce(gold), _coerce(target), float_rounding, numeric_precision, strict,
                                 allow_set_relation_comp, timeout, raise_on_error)
    return _core.verify_items(
        _coerce(gold), _coerce(target), int(float_rounding), int(numeric_precision), bool(strict),
        bool(allow_set_relation_comp), timeout,
    )


def _verify_with_hook(gold, target, float_rounding, numeric_precision, strict, allow_set_relation_comp,
                      timeout, raise_on_error):
    """Slow path: a user-supplied grader.sympy_expr_eq, per (gold, target) pair."""
    import threading

    golds = gold if isinstance(gold, list) else [gold]
    targets = target if isinstance(target, list) else [target]
    for g in golds:
        for t in targets:
            if isinstance(g, str) and isinstance(t, str):
                if g.strip() and t.strip() and g.strip() == t.strip():
                    return True
                continue
            if isinstance(g, str) or isinstance(t, str):
                continue
            box = {}

            def run():
                try:
                    box["r"] = grader.sympy_expr_eq(g, t, float_rounding, numeric_precision,
                                                    allow_set_relation_comp, strict)
                except BaseException as e:  # noqa: BLE001
                    box["e"] = e

            th = threading.Thread(target=run, daemon=True)
            th.start()
            th.join(timeout)
            if th.is_alive():
                continue  # timed out -> this comparison counts as False
            if "e" in box:
                if raise_on_error:
                    raise box["e"]
                continue
            if box.get("r"):
                return True
    return False


def verify_batch(
    golds: Sequence[str],
    preds: Sequence[str],
    gold_extraction_config: Sequence[ExtractionTarget] = _DEFAULT_TARGETS,
    pred_extraction_config: Sequence[ExtractionTarget] = _DEFAULT_TARGETS,
    float_rounding: int = 6,
    numeric_precision: int = 15,
    strict: bool = True,
    allow_set_relation_comp: bool = False,
    threads: int | None = None,
) -> list[bool]:
    """``[verify(parse(g), parse(p)) for g, p in zip(golds, preds)]`` on all cores, GIL released."""
    return _core.verify_batch(
        list(map(str, golds)), list(map(str, preds)), _specs(gold_extraction_config),
        _specs(pred_extraction_config), int(float_rounding), int(numeric_precision), bool(strict),
        bool(allow_set_relation_comp), threads,
    )


def parse_batch(
    texts: Sequence[str],
    extraction_config: Sequence[ExtractionTarget] = _DEFAULT_TARGETS,
    fallback_mode: Literal["no_fallback", "first_match"] = "first_match",
    extraction_mode: Literal["first_match", "any_match"] = "any_match",
    threads: int | None = None,
) -> list[list]:
    """Parse many texts in parallel (GIL released)."""
    return _core.parse_batch(list(map(str, texts)), _specs(extraction_config), fallback_mode, extraction_mode, threads)


def math_metric(
    gold_extraction_target: Sequence[ExtractionTarget] = (ExprExtractionConfig(),),
    pred_extraction_target: Sequence[ExtractionTarget] = (ExprExtractionConfig(),),
    aggregation_function=max,
    precision: int = 6,
):
    """Same shape as math_verify.math_metric: returns fn(golds, predictions) -> (score, extracted)."""

    def sample_level_fn(golds: list[str], predictions: list[str]):
        extracted_golds = [parse(g, gold_extraction_target) for g in golds]
        extracted_preds = [parse(p, pred_extraction_target) for p in predictions]
        if all(len(g) == 0 for g in extracted_golds):
            raise ValueError(f"No gold targets found for at least one gold. Gold: {golds}")
        score = aggregation_function(
            [1.0 if any(verify(g, p, float_rounding=precision) for g in extracted_golds) else 0.0 for p in extracted_preds]
        )
        return score, ([str(x) for g in extracted_golds for x in g], [str(x) for p in extracted_preds for x in p])

    return sample_level_fn


def normalize_latex(text: str, config: NormalizationConfig = NormalizationConfig()) -> str:
    return _core.normalize_latex(text, config.basic_latex, config.units, config.malformed_operators, config.nits,
                                 config.boxed, config.equations)


def latex_to_expr(latex: str) -> Expr:
    """Convert one LaTeX string to an ``Expr`` (raises ValueError if it does not parse)."""
    return _core.latex_to_expr(latex)
