# Known differences from upstream Math-Verify

Measured on 24,506 (gold, completion) pairs from OpenR1-Math-220k
(`harness/compare.py data/openr1_ref.jsonl data/openr1_ours.jsonl`): **24,485 agree
(99.914%)**, 21 differ. The extracted answer strings are identical on all 24,506 pairs; every
difference is in the comparison step.

## Upstream `False`, fastmathverify `True` (16)

Upstream decides symbolic equality with `sympy.simplify(a - b) == 0`. fastmathverify
evaluates both sides at several random points, so it accepts true identities that
`simplify` can't prove:

| gold | completion | note |
|---|---|---|
| `-1/2` | `cos(2π/7) + cos(4π/7) + cos(6π/7)` | classic identity SymPy can't simplify |
| `tan((α-γ)/2) / (2 tan((α+γ)/2))` (x2) | `(sin α - sin γ) / (2 (sin α + sin γ))` | sum-to-product identity |
| `acos(2/sqrt(sin²2α + 8))` | `acos(1/sqrt((sin²α+1)(cos²α+1)))` | trig identity |
| `C - 2 sqrt((x^{1/3}+1)^3/x)` (x2) | `C - 2 (x^{1/3}+1)^{3/2}/sqrt(x)` | power/root rewrite |
| `3 sqrt((e^{3x}-1)/(e^{3x}+1))` (x2) | `3(e^{3x}-1)/sqrt(e^{6x}-1)` | root rewrite |
| `x_n = sqrt(2)/sqrt(5^{2^{n-1}} - 3)` (x2) | `sqrt(2/(5^{2^{n-1}} - 3))` | root rewrite |
| `C - log|(sqrt(x²+1)+1)/x|` | `C + log|x/(sqrt(x²+1)+1)|` | log rewrite |
| `b·sqrt(...)`-form side length | equivalent closed form | algebraic rewrite |
| `F(p) = sqrt(π/2) e^{-ip-|p|}` | same expression | upstream builds the symbol `p` with different (real vs. complex) assumptions on each side |
| `{x_a = asin(c), x_b = π/2 - asin(c)}` (x2) | `{acos(c), asin(c)}` | `π/2 - asin c = acos c` |
| `{79, 56 - 23 = 33}` | `{56 - 23 = 33, 56 + 23 = 79}` | element pairing for sets of numeric equations |

Some root/log rewrites only hold on part of the domain (e.g. `sqrt(a)/sqrt(b) = sqrt(a/b)`
needs `b > 0`). fastmathverify samples real symbols at both signs but accepts the pair when
both sides agree wherever they're defined.

## Upstream `True`, fastmathverify `False` (5)

| gold | completion | cause |
|---|---|---|
| `∫₀¹ dy ∫_{√y}^{√(2-y²)} f dx` | same | upstream merges nested integral limits into one `Integral`; we keep them nested |
| `{(0,1), (1,0), ((-1±√5)/2, ...)}` | set with `(0,1)` parsed as an open interval | element ordering inside sets of mixed tuples/intervals |
| `{{5,12,13}, {6,8,10}, ...}` (x2) | `{(5,12,13), (6,8,10), ...}` | upstream compares set-of-sets with set-of-tuples element-wise |
| `{α²/2, α(β - α/2)}` | set of `x₁ = x₂ = x₃ = ...` chains | ordering of chained assignments inside sets |

## API-level differences

- `parse()` returns `fastmathverify.Expr` objects instead of SymPy objects. They print like
  SymPy (`str()`), compare equal to the equivalent SymPy object, and convert with
  `.to_sympy()`. Code that calls SymPy methods directly on parse results needs `.to_sympy()`.
- Printed forms can order terms differently from SymPy (`x + 1` vs `1 + x`).
- Timeouts are cooperative deadlines, not `signal.alarm`, so they work in threads. A stage
  that is already inside one big-integer operation finishes it before the deadline is seen.
  Those operations are size-capped, so this is bounded.
- `raise_on_error=True` has nothing to re-raise: internal errors are handled like upstream
  handles them by default (`parse` returns `[]`, `verify` returns `False`).
- Matrix operators upstream evaluates through SymPy (`\operatorname{rref}`, `eig`, `svd`,
  elementary row transforms) are not supported; those expressions fail to parse.
