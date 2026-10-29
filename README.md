# fastmathverify

A Rust rewrite of Hugging Face's [Math-Verify](https://github.com/huggingface/Math-Verify): the
answer checker used as the math reward function in TRL, verl, open-r1, SkyRL and Prime
Intellect's verifiers.

Same `parse()` / `verify()` API. **~16x faster on one core, ~60x with `verify_batch` on all
cores, thread-safe, no `signal.alarm`, and zero Python dependencies** (no SymPy, no
antlr4 runtime).

```python
import fastmathverify as mv        # drop-in for: import math_verify as mv

gold = mv.parse("$\\frac{1}{2}$")
pred = mv.parse("... so the answer is \\boxed{0.5}")
mv.verify(gold, pred)              # True

# Grade a whole RL batch on all cores, GIL released:
mv.verify_batch(gold_texts, completions)   # -> list[bool]
```

## Why

Math-Verify is the de facto reward function for reasoning-model RL, but:

- **It breaks in threads.** Timeouts use `signal.alarm`, so `verify` returns `False` or raises
  `signal only works in main thread` from worker threads (upstream issues
  [#42](https://github.com/huggingface/Math-Verify/issues/42),
  [#22](https://github.com/huggingface/Math-Verify/issues/22),
  [#50](https://github.com/huggingface/Math-Verify/issues/50),
  [#55](https://github.com/huggingface/Math-Verify/issues/55)). Reward servers end up forking processes.
- **It's slow for reward computation.** Every call goes through regexes, an ANTLR parser in
  pure Python, and SymPy.
- **Install pain.** Its `latex2sympy2_extended` dependency pins specific antlr4 runtimes
  ([#12](https://github.com/huggingface/Math-Verify/issues/12)).

fastmathverify keeps upstream's behaviour (same extraction regexes, same LaTeX grammar,
same comparison rules) and replaces the engine.

## Install

```bash
pip install .            # from a checkout; needs a Rust toolchain (rustup.rs)
```

Python >= 3.10. Built with [maturin](https://www.maturin.rs/) as an abi3 wheel. SymPy is
optional and only used for `Expr.to_sympy()` interop.

## Benchmarks

3,000 (gold, completion) pairs from
[OpenR1-Math-220k](https://huggingface.co/datasets/open-r1/OpenR1-Math-220k) (real reasoning
traces, median ~13k characters), each scored as `verify(parse(gold), parse(completion))`.
Same machine (8 cores, 3 GB RAM), same pairs, `math-verify` 0.9.0 / SymPy 1.14 / Python 3.13.

| configuration | time | pairs/s | vs. math_verify |
|---|---:|---:|---:|
| `math_verify`, 1 process | 61.9 s | 48 | 1x |
| `math_verify`, `multiprocessing.Pool(8)` | 12.5 s | 240 | 5x |
| **`fastmathverify`, 1 thread (Python loop)** | **3.95 s** | **760** | **15.7x** |
| **`fastmathverify.verify_batch`, 8 threads** | **1.03 s** | **2,913** | **60x** |

| latency per pair | math_verify | fastmathverify |
|---|---:|---:|
| median | 3.90 ms | 0.43 ms |
| p99 | 244.6 ms | 12.2 ms |

Reproduce: `python bench/bench.py data/openr1_pairs.jsonl 3000` (see *Reproducing* below).

## Correctness

The goal is to give the same answer as upstream, not a "better" one.

| check | result |
|---|---|
| `verify()` agreement with upstream on **24,506** OpenR1-Math-220k pairs | **99.914%** (24,485) |
| extracted answer string identical to upstream (same pairs) | 24,506 / 24,506 |
| LaTeX parse trees identical to Python ANTLR (2,639 real parser inputs) | 2,639 / 2,639 |
| upstream's own test suite, run unmodified against fastmathverify | **291 / 291** |
| fastmathverify's own tests (`tests/`) | 26 / 26 |

**Pluggable like upstream.** Upstream's tests patch its comparison and conversion
functions to inject delays and errors, so fastmathverify exposes the same hook points:
`fastmathverify.grader.sympy_expr_eq`, `fastmathverify.parser.latex2sympy` and
`fastmathverify.parser.parse_expr`. Left as they are, everything runs in Rust. Replace one
and `verify`/`parse` route through your function, with `timeout_seconds`/`parsing_timeout`
enforced in a worker thread (no signals) and `raise_on_error` honoured.

**About the 21 OpenR1 disagreements (0.086%).** 16 are cases where upstream returns
`False` but the answers are mathematically equal and SymPy's `simplify` can't prove it, for
example `cos(2π/7) + cos(4π/7) + cos(6π/7) = -1/2`, tan half-angle forms and `sqrt`/`log`
rewrites. fastmathverify uses randomized numeric identity testing instead of `simplify`, so
it accepts those. The other 5 are structural edge cases (nested integral limits, sets of sets
vs. sets of tuples). Full list: [DIFFERENCES.md](DIFFERENCES.md).

## API

Everything upstream exposes, with the same signatures and defaults:

- `parse(pred, extraction_config=[LatexExtractionConfig(), ExprExtractionConfig()], fallback_mode="first_match", extraction_mode="any_match", parsing_timeout=5, raise_on_error=False)`
- `verify(gold, target, float_rounding=6, numeric_precision=15, strict=True, allow_set_relation_comp=False, timeout_seconds=5, raise_on_error=False)`
- `LatexExtractionConfig`, `ExprExtractionConfig`, `StringExtractionConfig`, `NormalizationConfig` / `LatexNormalizationConfig`, `math_metric`

Upstream's config objects are accepted too, so `parse(text, [math_verify.LatexExtractionConfig()])` works.

Additions:

- `verify_batch(golds, preds, ..., threads=None) -> list[bool]` and `parse_batch(texts, ...)`:
  parallel, GIL released.
- `parse()` returns `fastmathverify.Expr` objects (plus the fallback string, like upstream).
  `str(expr)` prints SymPy-style; `expr.to_sympy()` returns the equivalent SymPy object, and
  `expr == sympy_obj` / `verify(sympy_obj, ...)` work when SymPy is installed.
- `grader.sympy_expr_eq`, `parser.latex2sympy` and `parser.parse_expr` are replaceable
  hooks (see above); the Rust fast path is used when they're left as the defaults.
- Timeouts are cooperative, thread-local deadlines, so they work from any thread. As upstream,
  a timed-out `parse` returns `[]` and a timed-out comparison counts as `False`.

Drop-in in TRL/verl-style reward functions:

```python
from fastmathverify import parse, verify, verify_batch

def accuracy_reward(completions, solution, **kwargs):
    return [float(x) for x in verify_batch(solution, [c[0]["content"] for c in completions])]
```

## How it works

```
text ─► extraction regexes ─► normalize_latex ─► ANTLR LaTeX parser ─► expression tree ─► grader
        (upstream's, DFA       (port of          (upstream's PS.g4,      (mirrors the        (port of grader.py;
         prefilter)             latex2sympy2)     generated for Rust)     SymPy objects)       numeric identity tests)
```

- **Extraction**: upstream's regexes, verbatim, run with `fancy-regex`. A lookaround-free
  copy of each pattern runs first on the `regex` crate's DFA to jump to candidate positions.
  Every real match is also a match of the relaxed pattern, so results are identical, and it's
  20-150x faster than running the backtracking VM at every position.
- **Parsing**: the *same* `PS.g4` grammar latex2sympy2_extended uses, compiled to Rust with
  [antlr4rust](https://github.com/antlr4rust/antlr4). Two-stage prediction (SLL, then full LL
  on a syntax error) gives identical trees to Python's ANTLR on every input tested.
- **Conversion**: a function-by-function port of latex2sympy2_extended's converter into an
  expression tree that mirrors SymPy's unevaluated objects (`Mul(-1, x)`, `UnevaluatedExpr`
  percentages, the unsorted `FiniteSet`, `And` chains...).
- **Grading**: a port of `grader.py` with the same dispatch order and quirks (rounding
  `Float`s to 6 places, `9% == 9`, assignment unwrapping, interval/tuple handling, even
  `solve`-based equation comparison). Exact rational arithmetic decides whenever it can;
  where upstream calls `simplify(a - b) == 0`, fastmathverify evaluates both sides at
  several random real (or complex) points with principal branches.
- **Safety**: big-number arithmetic is size-capped, sums and products are bounded, regex
  backtracking is limited, and every stage checks a per-thread deadline.

## Reproducing

```bash
uv venv .venv && . .venv/bin/activate
uv pip install maturin math-verify pytest datasets pyarrow
maturin develop --release

# upstream's test suite against fastmathverify
git clone --depth 1 https://github.com/huggingface/Math-Verify ref/upstream
./harness/run_upstream_tests.sh

# OpenR1-Math-220k agreement (downloads 2 parquet shards, ~430 MB)
mkdir -p data && for i in 00000 00005; do curl -sL -o data/openr1-$i.parquet \
  https://huggingface.co/datasets/open-r1/OpenR1-Math-220k/resolve/main/data/train-$i-of-00010.parquet; done
python harness/build_openr1_set.py                                        # 24,506 pairs
python harness/ref_upstream.py data/openr1_pairs.jsonl data/openr1_ref.jsonl 6
cargo run --release --bin harness -- data/openr1_pairs.jsonl data/openr1_ours.jsonl 8
python harness/compare.py data/openr1_ref.jsonl data/openr1_ours.jsonl

# benchmark
python bench/bench.py data/openr1_pairs.jsonl 3000
```

## License

Apache-2.0. The LaTeX grammar comes from
[latex2sympy2_extended](https://github.com/huggingface/latex2sympy2_extended) (MIT), and
behaviour follows [Math-Verify](https://github.com/huggingface/Math-Verify) (Apache-2.0),
both by Hugging Face. A lightly patched copy of antlr4rust (BSD-3-Clause) is vendored in
`vendor/`. See [NOTICE](NOTICE).
