<div align="center">

# ⚡ fastmathverify

**A Rust rewrite of Hugging Face's [Math-Verify](https://github.com/huggingface/Math-Verify): the math answer checker behind reasoning-model RL.**

**17× faster on one core · 67× on all cores · thread-safe · no hangs · zero Python dependencies**

[![CI](https://github.com/johnmichaelcarlucci/fastmathverify/actions/workflows/ci.yml/badge.svg)](https://github.com/johnmichaelcarlucci/fastmathverify/actions/workflows/ci.yml)
[![Upstream tests](https://img.shields.io/badge/upstream%20tests-291%2F291-brightgreen)](#correctness)
[![OpenR1 agreement](https://img.shields.io/badge/OpenR1%20agreement-99.91%25-brightgreen)](#correctness)
[![Speedup](https://img.shields.io/badge/speedup-67×-orange)](#benchmarks)
[![Rust](https://img.shields.io/badge/core-Rust-b7410e?logo=rust)](src/)
[![Python](https://img.shields.io/badge/python-3.10%2B-3776ab?logo=python&logoColor=white)](#install)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

</div>

---

Math-Verify is the reward function used in **TRL, verl, open-r1, SkyRL and Prime Intellect's verifiers**.
fastmathverify keeps the same `parse()` / `verify()` API and gives the same answers, but runs the whole
pipeline in Rust.

```python
import fastmathverify as mv        # drop-in for: import math_verify as mv

gold = mv.parse("$\\frac{1}{2}$")
pred = mv.parse("... so the answer is \\boxed{0.5}")
mv.verify(gold, pred)              # True

# Grade a whole RL batch on all cores, GIL released:
mv.verify_batch(gold_texts, completions)   # -> list[bool]
```

<table>
<tr>
<td align="center"><h3>67×</h3>faster with <code>verify_batch</code><br>on 8 threads</td>
<td align="center"><h3>291 / 291</h3>of upstream's own tests<br>pass unmodified</td>
<td align="center"><h3>99.91%</h3>agreement on 24,506<br>OpenR1-Math-220k pairs</td>
<td align="center"><h3>22×</h3>lower p99 latency<br>(12 ms vs 264 ms)</td>
</tr>
</table>

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
| **`fastmathverify`, 1 thread (Python loop)** | **3.81 s** | **788** | **17.3x** |
| **`fastmathverify.verify_batch`, 8 threads** | **0.97 s** | **3,081** | **67x** |

| latency per pair | math_verify | fastmathverify |
|---|---:|---:|
| median | 3.96 ms | 0.43 ms |
| p99 | 263.8 ms | 11.8 ms |

Reproduce: `python bench/bench.py data/openr1_pairs.jsonl 3000` (see *Reproducing* below).

## Correctness

The goal is to give the same answer as upstream, not a "better" one.

| check | result |
|---|---|
| `verify()` agreement with upstream on **24,506** OpenR1-Math-220k pairs (development set) | **99.914%** (24,485) |
| `verify()` agreement on **41,437 held-out pairs** never used during development (below) | **99.961%** (41,421) |
| extracted answer string identical to upstream (both sets, 65,943 pairs) | 65,943 / 65,943 |
| LaTeX parse trees identical to Python ANTLR (2,639 real parser inputs) | 2,639 / 2,639 |
| upstream's own test suite, run unmodified against fastmathverify | **291 / 291** |
| fastmathverify's own tests: Python (`tests/*.py`) and Rust (`cargo test`) | 26 / 26 and 7 / 7 |
| regression corpus: 2,701 pairs with recorded upstream verdicts, checked in CI | 2,701 / 2,701 |
| mutation fuzzing (`src/bin/fuzz.rs`): 860,000 mangled inputs | 0 panics; slowest pair 0.6 s |

**The held-out set** (`harness/build_holdout_set.py`) was built after development finished,
from data the port never saw: 13,437 OpenR1 traces from two other shards, 8,000 DeepScaleR
(answer, solution) pairs, 4,000 mismatched DeepScaleR pairs, 3,000 answers checked against
themselves in five answer formats, and **12,000 near-miss negatives** (each DeepScaleR /
MATH-500 answer against its neighbour in sorted order, so `\frac{1}{2}` meets
`\frac{1}{3}`). The near-miss pairs are there to catch a grader that's too lenient, which is
what matters for an RL reward. They found one real bug, now fixed: a number too large for a
float (`10^{400}`, `2005!`) overflowed to infinity and matched any other huge value. They
also turned up a false positive in upstream itself: Math-Verify accepts
`30!·1989!/2020!` for `30!/(10!)³` (1.1·10⁻⁷⁰ vs 5.6·10¹²) because SymPy's `evalf` loses
precision on the factorials. fastmathverify evaluates exactly and rejects it.

**Pluggable like upstream.** Upstream's tests patch its comparison and conversion
functions to inject delays and errors, so fastmathverify exposes the same hook points:
`fastmathverify.grader.sympy_expr_eq`, `fastmathverify.parser.latex2sympy` and
`fastmathverify.parser.parse_expr`. Left as they are, everything runs in Rust. Replace one
and `verify`/`parse` route through your function, with `timeout_seconds`/`parsing_timeout`
enforced in a worker thread (no signals) and `raise_on_error` honoured.

**About the disagreements (21 of 24,506 and 16 of 41,437).** 16 are cases where upstream returns
`False` but the answers are mathematically equal and SymPy's `simplify` can't prove it, for
example `cos(2π/7) + cos(4π/7) + cos(6π/7) = -1/2`, tan half-angle forms and `sqrt`/`log`
rewrites. fastmathverify uses randomized numeric identity testing instead of `simplify`, so
it accepts those. The rest are structural edge cases (nested integral limits, sets of sets
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

# held-out set (also needs shards 00002 + 00007, DeepScaleR and MATH-500; see the script)
python harness/build_holdout_set.py

# Rust tests, regression corpus and fuzzing
cargo test --release
cargo run --release --bin fuzz -- tests/data/regression.jsonl 200000 8

# benchmark
python bench/bench.py data/openr1_pairs.jsonl 3000
```

## 💬 Find this useful?

**Message me!** If you're using fastmathverify in a training run, a reward server or an eval
pipeline, I'd love to hear about it, and the same goes for bugs, mismatches with upstream or
feature requests.

- 💬 Say hi or share how you're using it in [Discussions](https://github.com/johnmichaelcarlucci/fastmathverify/discussions)
- 🐛 Found an input where it disagrees with Math-Verify? [Open an issue](https://github.com/johnmichaelcarlucci/fastmathverify/issues) with the gold and prediction text
- 👤 Or reach me through my profile: [@johnmichaelcarlucci](https://github.com/johnmichaelcarlucci)

If it saved you some GPU hours, a ⭐ helps other people find it.

## License

Apache-2.0. The LaTeX grammar comes from
[latex2sympy2_extended](https://github.com/huggingface/latex2sympy2_extended) (MIT), and
behaviour follows [Math-Verify](https://github.com/huggingface/Math-Verify) (Apache-2.0),
both by Hugging Face. A lightly patched copy of antlr4rust (BSD-3-Clause) is vendored in
`vendor/`. See [NOTICE](NOTICE).
