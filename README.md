# fastmathverify

A fast math answer checker for Python, written in Rust. It pulls the final answer out of a model's output, parses LaTeX or plain expressions, and decides whether it matches the gold answer. Built for reward functions in reasoning-model RL.

[![CI](https://github.com/johnmichaelcarlucci/fastmathverify/actions/workflows/ci.yml/badge.svg)](https://github.com/johnmichaelcarlucci/fastmathverify/actions/workflows/ci.yml)

Accepts the same `parse()` / `verify()` calls as Math-Verify and returns the same verdicts. 17x faster on one core,
67x with `verify_batch` on 8 threads, and it works from any thread.

```python
import fastmathverify as mv

gold = mv.parse("$\\frac{1}{2}$")
pred = mv.parse("... so the answer is \\boxed{0.5}")
mv.verify(gold, pred)              # True

mv.verify_batch(gold_texts, completions)   # -> list[bool]
```

## Features

- Answer extraction from `\boxed{}`, `$...$`, "the answer is" and plain expressions.
- LaTeX parsing: fractions, roots, powers, trig and log functions, sets, intervals, tuples, matrices, equations.
- Equivalence checking: exact rational arithmetic, numeric identity tests, set and interval comparison, equation solving.
- Timeouts that work in any thread; no signals, no subprocesses.
- Batch grading across all cores with the GIL released; each distinct gold is parsed once.
- `explain()` says why a pair failed: no answer found, mismatch or timeout.
- `exact=True` grading: no float rounding, and `9%` is not `9`.
- No SymPy or ANTLR runtime needed at run time.

## Install

```bash
pip install git+https://github.com/johnmichaelcarlucci/fastmathverify
# or, from a checkout:
pip install .
```

Building needs a Rust toolchain ([rustup.rs](https://rustup.rs)). Python >= 3.10. Built with [maturin](https://www.maturin.rs/) as an abi3 wheel. SymPy is
optional and only used for `Expr.to_sympy()` interop.

## Benchmarks

3,000 (gold, completion) pairs from
[OpenR1-Math-220k](https://huggingface.co/datasets/open-r1/OpenR1-Math-220k) (real reasoning
traces, median ~13k characters), each scored as `verify(parse(gold), parse(completion))`.
Same machine (8 cores, 3 GB RAM), same pairs, `math-verify` 0.9.0 / SymPy 1.14 / Python 3.13.

| configuration | time | pairs/s | vs. math_verify |
|---|---:|---:|---:|
| `math_verify`, 1 process | 65.9 s | 46 | 1x |
| `math_verify`, `multiprocessing.Pool(8)` | 12.4 s | 242 | 5x |
| `fastmathverify`, 1 thread (Python loop) | 3.81 s | 788 | 17.3x |
| `fastmathverify.verify_batch`, 8 threads | 0.97 s | 3,081 | 67x |

| latency per pair | math_verify | fastmathverify |
|---|---:|---:|
| median | 3.96 ms | 0.43 ms |
| p99 | 263.8 ms | 11.8 ms |

Reproduce: `python bench/bench.py data/openr1_pairs.jsonl 3000` (see *Reproducing* below).

## Correctness

Target: the same answer as upstream on every input.

| check | result |
|---|---|
| `verify()` agreement with upstream on 24,506 OpenR1-Math-220k pairs (development set) | 99.906% (24,483) |
| `verify()` agreement on 41,437 held-out pairs never used during development (below) | 99.961% (41,421) |
| extracted answer string identical to upstream (both sets, 65,943 pairs) | 65,943 / 65,943 |
| LaTeX parse trees identical to Python ANTLR (2,639 real parser inputs) | 2,639 / 2,639 |
| upstream's own test suite, run unmodified against fastmathverify | 291 / 291 |
| fastmathverify's own tests: Python (`tests/*.py`) and Rust (`cargo test`) | 30 / 30 and 11 / 11 |
| regression corpus: 2,701 pairs with recorded upstream verdicts, checked in CI | 2,701 / 2,701 |
| mutation fuzzing (`src/bin/fuzz.rs`): 860,000 mangled inputs | 0 panics; slowest pair 0.6 s |

The held-out set (`harness/build_holdout_set.py`) was built after development finished,
from data the port never saw: 13,437 OpenR1 traces from two other shards, 8,000 DeepScaleR
(answer, solution) pairs, 4,000 mismatched DeepScaleR pairs, 3,000 answers checked against
themselves in five answer formats, and 12,000 near-miss negatives (each DeepScaleR /
MATH-500 answer against its neighbour in sorted order, so `\frac{1}{2}` meets
`\frac{1}{3}`). The near-miss pairs are there to catch a grader that's too lenient, which is
what matters for an RL reward. They found one bug, now fixed: a number too large for a
float (`10^{400}`, `2005!`) overflowed to infinity and matched any other huge value. They
also turned up a false positive in upstream itself: Math-Verify accepts
`30!·1989!/2020!` for `30!/(10!)³` (1.1·10⁻⁷⁰ vs 5.6·10¹²) because SymPy's `evalf` loses
precision on the factorials. fastmathverify evaluates exactly and rejects it.

Upstream's tests patch its comparison and conversion
functions to inject delays and errors, so fastmathverify exposes the same hook points:
`fastmathverify.grader.sympy_expr_eq`, `fastmathverify.parser.latex2sympy` and
`fastmathverify.parser.parse_expr`. Left as they are, everything runs in Rust. Replace one
and `verify`/`parse` route through your function, with `timeout_seconds`/`parsing_timeout`
enforced in a worker thread (no signals) and `raise_on_error` honoured.

Disagreements (23 of 24,506 and 16 of 41,437): 36 of the 39 are correct answers upstream
rejects. Some are identities SymPy's `simplify` can't prove, for example
`cos(2π/7) + cos(4π/7) + cos(6π/7) = -1/2`, tan half-angle forms and `sqrt`/`log` rewrites;
fastmathverify uses randomized numeric identity testing instead. Others are sets of points
listed in a different order, which fastmathverify matches element by element. One is an
upstream false positive (above). The remaining two are structural edge cases (nested integral
limits, chained assignments inside sets). Full list: [DIFFERENCES.md](DIFFERENCES.md).

## API

Everything upstream exposes, with the same signatures and defaults:

- `parse(pred, extraction_config=[LatexExtractionConfig(), ExprExtractionConfig()], fallback_mode="first_match", extraction_mode="any_match", parsing_timeout=5, raise_on_error=False)`
- `verify(gold, target, float_rounding=6, numeric_precision=15, strict=True, allow_set_relation_comp=False, timeout_seconds=5, raise_on_error=False)`
- `LatexExtractionConfig`, `ExprExtractionConfig`, `StringExtractionConfig`, `NormalizationConfig` / `LatexNormalizationConfig`, `math_metric`

Upstream's config objects are accepted too, so `parse(text, [math_verify.LatexExtractionConfig()])` works.

Additions:

- `verify_batch(golds, preds, ..., threads=None, exact=False, parsing_timeout=5, timeout_seconds=5) -> list[bool]`
  and `parse_batch(texts, ...)`: parallel, GIL released. Repeated golds are parsed once.
- `explain(gold, pred, ...) -> dict`: `{"correct", "reason", "gold", "pred"}`, where `reason` is
  `"match"`, `"mismatch"`, `"no answer in gold"`, `"no answer in prediction"` or `"timeout"`, and
  `gold`/`pred` are the extracted answers.

  ```python
  >>> mv.explain("$\\frac{1}{2}$", "so the answer is 0.6")
  {'correct': False, 'gold': ['1/2'], 'pred': ['0.600000000000000'], 'reason': 'mismatch'}
  ```
- `python -m fastmathverify GOLD PRED [--exact]` prints the `explain()` result as JSON and exits 0 on
  a match, 1 otherwise. Pass `-` as PRED to read it from stdin.
- `exact=True` on `verify` and `verify_batch`: numbers compare by exact value. No rounding to
  `float_rounding` places (`0.3333333` is not `1/3`), and a percentage equals its value / 100
  (`9%` matches `0.09`, not `9`). Symbolic answers are graded the same in both modes.
- `parse()` returns `fastmathverify.Expr` objects (plus the fallback string, like upstream).
  `str(expr)` prints SymPy-style; `expr.to_sympy()` returns the equivalent SymPy object, and
  `expr == sympy_obj` / `verify(sympy_obj, ...)` work when SymPy is installed.
- `grader.sympy_expr_eq`, `parser.latex2sympy` and `parser.parse_expr` are replaceable
  hooks (see above); the Rust fast path is used when they're left as the defaults.
- Timeouts are cooperative, thread-local deadlines, so they work from any thread. As upstream,
  a timed-out `parse` returns `[]` and a timed-out comparison counts as `False`.

In a TRL/verl-style reward function:

```python
from fastmathverify import parse, verify, verify_batch

def accuracy_reward(completions, solution, **kwargs):
    return [float(x) for x in verify_batch(solution, [c[0]["content"] for c in completions])]
```

## How it works

Pipeline: extraction regexes, LaTeX normalization, ANTLR parse, expression tree, grader.

- Extraction: upstream's regexes, verbatim, run with `fancy-regex`. A lookaround-free
  copy of each pattern runs first on the `regex` crate's DFA to jump to candidate positions.
  Every real match is also a match of the relaxed pattern, so results are identical, and it's
  20-150x faster than running the backtracking VM at every position.
- Parsing: the same `PS.g4` grammar latex2sympy2_extended uses, compiled to Rust with
  [antlr4rust](https://github.com/antlr4rust/antlr4). Two-stage prediction (SLL, then full LL
  on a syntax error) gives identical trees to Python's ANTLR on every input tested.
- Conversion: a function-by-function port of latex2sympy2_extended's converter into an
  expression tree that mirrors SymPy's unevaluated objects (`Mul(-1, x)`, `UnevaluatedExpr`
  percentages, the unsorted `FiniteSet`, `And` chains...).
- Grading: a port of `grader.py` with the same dispatch order and quirks (rounding
  `Float`s to 6 places, `9% == 9`, assignment unwrapping, interval/tuple handling, even
  `solve`-based equation comparison). Exact rational arithmetic decides whenever it can;
  where upstream calls `simplify(a - b) == 0`, fastmathverify evaluates both sides at
  several random real (or complex) points with principal branches.
- Safety: big-number arithmetic is size-capped, sums and products are bounded, regex
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

# held-out set (also needs shards 00002 + 00007, DeepScaleR and MATH-500; see the script)
python harness/build_holdout_set.py

# Rust tests, regression corpus and fuzzing
cargo test --release
cargo run --release --bin fuzz -- tests/data/regression.jsonl 200000 8

# benchmark
python bench/bench.py data/openr1_pairs.jsonl 3000
```

Found a mismatch with Math-Verify? Open an issue with the gold and prediction text.

## License

Apache-2.0. The LaTeX grammar comes from
[latex2sympy2_extended](https://github.com/huggingface/latex2sympy2_extended) (MIT), and
behaviour follows [Math-Verify](https://github.com/huggingface/Math-Verify) (Apache-2.0),
A lightly patched copy of antlr4rust (BSD-3-Clause) is vendored in
`vendor/`. See [NOTICE](NOTICE).
