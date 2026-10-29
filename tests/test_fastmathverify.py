"""fastmathverify's own tests: API parity, thread safety, timeouts, batch APIs.

Upstream's full test suite is run separately via harness/run_upstream_tests.sh.
"""
import time
from concurrent.futures import ThreadPoolExecutor

import pytest

import fastmathverify as F

CASES = [
    ("$\\frac{1}{2}$", "The answer is $0.5$", True),
    ("$x^2+2x+1$", "so \\boxed{(x+1)^2}", True),
    ("$[1,2)$", "answer: $[1,2)$", True),
    ("$3$", "final answer is 4", False),
    ("$\\{1,2,3\\}$", "\\boxed{3,2,1}", True),
    ("$50\\%$", "\\boxed{50}", True),
    ("$\\sqrt{8}$", "\\boxed{2\\sqrt{2}}", True),
    ("$x=2$", "\\boxed{2}", True),
    ("$\\frac{63}{400}$", "$0.1575$", True),
    ("$\\pi$", "\\boxed{3.14}", False),
]


@pytest.mark.parametrize("gold,pred,expected", CASES)
def test_basic(gold, pred, expected):
    assert F.verify(F.parse(gold), F.parse(pred)) is expected


def test_parse_returns_expr_and_fallback_string():
    out = F.parse("$\\frac{1}{2}$")
    assert len(out) == 2 and isinstance(out[0], F.Expr) and out[1] == "\\frac{1}{2}"
    assert str(out[0]) == "1/2"


def test_issue_22_same_result_in_threads():
    # upstream: returns only the string when called from a worker thread
    kw = dict(extraction_mode="first_match", extraction_config=[F.LatexExtractionConfig()])
    main = F.parse(r"\frac{63}{400}", **kw)
    with ThreadPoolExecutor(max_workers=4) as ex:
        threaded = ex.submit(F.parse, r"\frac{63}{400}", **kw).result()
    assert [str(x) for x in main] == [str(x) for x in threaded] == ["63/400", "\\frac{63}{400}"]


def test_issue_42_50_verify_in_threads():
    # upstream: always False (or raises "signal only works in main thread") in threads
    def job(_):
        return F.verify(F.parse("$7.41\\%$"), F.parse("$7.41\\%$"))

    with ThreadPoolExecutor(max_workers=16) as ex:
        assert all(ex.map(job, range(200)))


def test_threaded_results_match_sequential():
    pairs = [(g, p) for g, p, _ in CASES] * 20
    seq = [F.verify(F.parse(g), F.parse(p)) for g, p in pairs]
    with ThreadPoolExecutor(max_workers=8) as ex:
        par = list(ex.map(lambda gp: F.verify(F.parse(gp[0]), F.parse(gp[1])), pairs))
    assert seq == par


def test_verify_batch_matches_loop():
    golds = [g for g, _, _ in CASES]
    preds = [p for _, p, _ in CASES]
    loop = [F.verify(F.parse(g), F.parse(p)) for g, p in zip(golds, preds)]
    assert F.verify_batch(golds, preds) == loop == [e for _, _, e in CASES]
    assert F.verify_batch(golds, preds, threads=1) == loop


def test_parse_batch_matches_loop():
    texts = [p for _, p, _ in CASES]
    assert [[str(x) for x in r] for r in F.parse_batch(texts)] == [[str(x) for x in F.parse(t)] for t in texts]


@pytest.mark.parametrize(
    "text",
    [
        # upstream issue #65 (reported to hang forever)
        r"\boxed{\dfrac{5^{\left(5^{\left(5^{\left(5^5\right)}\right)} - 4\right)} - 5}{16}}",
        # upstream issue #63 (timeout fails, memory blow-up)
        r"\boxed{2^{2^{2^{49}}}}",
        r"\boxed{10^{10^{10^{10}}}!}",
        r"\boxed{\sum_{i=1}^{10^{12}} i^{2}}",
    ],
)
def test_pathological_inputs_finish_fast(text):
    t = time.perf_counter()
    p = F.parse(text)
    assert F.verify(p, F.parse(text)) is True
    assert time.perf_counter() - t < 2.0


def test_parsing_timeout_is_enforced():
    long_text = ("Answer: $x$ and " * 20000) + "\\boxed{1}"
    t = time.perf_counter()
    assert F.parse(long_text, parsing_timeout=1e-4) == []
    assert time.perf_counter() - t < 1.0


def test_timeout_none_and_zero_disable_timeout():
    assert str(F.parse("$1/2$", parsing_timeout=None)[0]) == "1/2"
    assert str(F.parse("$1/2$", parsing_timeout=0)[0]) == "1/2"


def test_string_extraction():
    assert F.parse("The answer is B.", [F.StringExtractionConfig()]) == ["b", "B"]


def test_accepts_upstream_config_objects():
    mv = pytest.importorskip("math_verify")
    out = F.parse("$\\frac12$", [mv.LatexExtractionConfig()])
    assert str(out[0]) == "1/2"


def test_sympy_interop():
    sympy = pytest.importorskip("sympy")
    x = F.parse("$\\frac{1}{2}$")[0]
    assert x == sympy.Rational(1, 2)
    assert x.to_sympy() == sympy.Rational(1, 2)
    g = F.parse(r"\(\Gamma(\frac{\pi}{12})\)", fallback_mode="no_fallback")[0]
    assert sympy.simplify(g - sympy.gamma(sympy.pi / 12)) == 0
    assert F.verify(sympy.Rational(1, 3), F.parse("$0.333333$"))


def test_math_metric():
    metric = F.math_metric(gold_extraction_target=(F.LatexExtractionConfig(),),
                           pred_extraction_target=(F.ExprExtractionConfig(), F.LatexExtractionConfig()))
    score, _ = metric(["$\\frac{1}{2}$"], ["The answer is 0.5"])
    assert score == 1.0


def test_explain():
    r = F.explain(r"$\frac{1}{2}$", r"so \boxed{0.5}")
    assert r["correct"] and r["reason"] == "match" and r["gold"] == ["1/2"]
    assert F.explain("$2$", r"\boxed{3}")["reason"] == "mismatch"
    assert F.explain("$2$", "no idea")["reason"] == "no answer in prediction"


def test_exact_mode():
    g, p = F.parse(r"$9\%$"), F.parse(r"\boxed{9}")
    assert F.verify(g, p) and not F.verify(g, p, exact=True)
    assert F.verify_batch([r"$9\%$"], [r"\boxed{0.09}"], exact=True) == [True]


def test_verify_batch_repeated_golds():
    golds = [r"$\frac{1}{2}$"] * 4 + ["$3$"] * 4
    preds = [r"\boxed{0.5}", r"\boxed{1}", r"\boxed{1/2}", "x"] + [r"\boxed{3}", r"\boxed{4}", "3", r"\boxed{3.0}"]
    assert F.verify_batch(golds, preds) == [F.verify(F.parse(g), F.parse(p)) for g, p in zip(golds, preds)]
