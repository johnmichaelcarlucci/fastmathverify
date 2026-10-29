# Loaded via -p to record every latex string passed to latex2sympy during upstream tests.
import atexit, json, os
import math_verify.parser as P
_orig = P.latex2sympy
_seen = []
def _wrap(latex, *a, **k):
    _seen.append(latex)
    return _orig(latex, *a, **k)
P.latex2sympy = _wrap
@atexit.register
def _dump():
    out = os.environ.get("CAPTURE_OUT", "data/latex_inputs.json")
    with open(out, "w") as f:
        json.dump(sorted(set(_seen)), f)
