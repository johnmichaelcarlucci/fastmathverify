"""Record every latex string upstream hands to latex2sympy over OpenR1 pairs."""
import json, sys
import math_verify.parser as P
seen = set()
orig = P.latex2sympy
def wrap(latex, *a, **k):
    seen.add(latex)
    return orig(latex, *a, **k)
P.latex2sympy = wrap
P.parse_latex_cached.cache_clear()
from math_verify import parse
for i, line in enumerate(open(sys.argv[1], encoding="utf-8")):
    if i >= int(sys.argv[3]): break
    d = json.loads(line)
    parse(d["gold"]); parse(d["pred"])
json.dump(sorted(seen), open(sys.argv[2], "w", encoding="utf-8"))
print(len(seen))
