"""Conversion hooks, mirroring math_verify.parser.

Extraction always runs in Rust. ``latex2sympy(latex)`` converts an extracted (normalized)
LaTeX string and ``parse_expr(text)`` converts an extracted plain expression. Leave them as
they are for the Rust fast path; if you replace either one (custom converter, or
``unittest.mock.patch``), ``parse`` calls your function on the first extracted string,
with ``parsing_timeout`` enforced in a worker thread (no signals): on timeout ``parse``
returns ``[]``, like upstream.
"""

from . import _core


def latex2sympy(latex, *args, **kwargs):
    return _core.latex_to_expr(latex)


def parse_expr(text, *args, **kwargs):
    from . import ExprExtractionConfig, parse
    r = parse(text, [ExprExtractionConfig()], fallback_mode="no_fallback")
    if not r:
        raise ValueError(f"cannot parse {text!r}")
    return r[0]


_DEFAULT_LATEX = latex2sympy
_DEFAULT_EXPR = parse_expr


def parse(*args, **kwargs):
    from . import parse as _p
    return _p(*args, **kwargs)
