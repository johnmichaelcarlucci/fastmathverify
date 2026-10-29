"""Run upstream Math-Verify's unmodified test files against fastmathverify.

The `math_verify` module names are aliased to fastmathverify so `from math_verify
import parse, verify` (and .parser/.grader) resolve to the Rust implementation.
"""
import sys, types
import fastmathverify as F

def sympy_expr_eq(gold, pred, float_rounding=6, numeric_precision=15, allow_set_relation_comp=False, strict=True):
    return F.verify(gold, pred, float_rounding=float_rounding, numeric_precision=numeric_precision,
                    strict=strict, allow_set_relation_comp=allow_set_relation_comp)

mv = types.ModuleType("math_verify")
for name in F.__all__:
    setattr(mv, name, getattr(F, name))
parser = types.ModuleType("math_verify.parser")
for name in ["parse", "LatexExtractionConfig", "ExprExtractionConfig", "StringExtractionConfig"]:
    setattr(parser, name, getattr(F, name))
grader = types.ModuleType("math_verify.grader")
grader.verify = F.verify
grader.sympy_expr_eq = sympy_expr_eq
errors = types.ModuleType("math_verify.errors")
class TimeoutException(Exception):
    pass
errors.TimeoutException = TimeoutException
mv.parser, mv.grader, mv.errors = parser, grader, errors
sys.modules.update({"math_verify": mv, "math_verify.parser": parser, "math_verify.grader": grader,
                    "math_verify.errors": errors})
