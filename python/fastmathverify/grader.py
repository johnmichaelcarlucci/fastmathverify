"""Comparison hook, mirroring math_verify.grader.

``sympy_expr_eq(gold, pred, float_rounding, numeric_precision, allow_set_relation_comp, strict)``
compares two single parsed items. ``verify`` uses the Rust implementation directly unless
this attribute is replaced (e.g. a custom comparator, or ``unittest.mock.patch``); then each
comparison goes through the replacement, with ``timeout_seconds`` enforced in a worker thread
(no signals) and exceptions handled per ``raise_on_error``.
"""

from . import _core


def sympy_expr_eq(gold, pred, float_rounding=6, numeric_precision=15, allow_set_relation_comp=False, strict=True):
    from . import _coerce
    return _core.verify_items(_coerce(gold), _coerce(pred), int(float_rounding), int(numeric_precision), bool(strict),
                              bool(allow_set_relation_comp), None)


_DEFAULT = sympy_expr_eq


def verify(*args, **kwargs):
    from . import verify as _v
    return _v(*args, **kwargs)
