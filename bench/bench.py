"""Benchmark fastmathverify against upstream math_verify on OpenR1-Math-220k pairs.

Usage: python bench/bench.py data/openr1_pairs.jsonl N
Runs every configuration on the same N pairs (deterministic first-N sample).
"""
import json, os, statistics, subprocess, sys, time
from multiprocessing import Pool

N = int(sys.argv[2]) if len(sys.argv) > 2 else 2000
pairs = [json.loads(l) for _, l in zip(range(N), open(sys.argv[1]))]
golds = [p["gold"] for p in pairs]
preds = [p["pred"] for p in pairs]
CORES = os.cpu_count()


def up_one(gp):
    from math_verify import parse, verify
    g, p = gp
    return verify(parse(g), parse(p))


def timed(label, fn):
    t = time.perf_counter()
    out = fn()
    dt = time.perf_counter() - t
    print(f"{label:<44} {dt:8.2f} s  {N / dt:9.0f} pairs/s", flush=True)
    return out, dt


results = {}

import math_verify  # noqa: E402
up_one((golds[0], preds[0]))  # warm imports
lat_up = []
def run_up():
    out = []
    for g, p in zip(golds, preds):
        t = time.perf_counter()
        out.append(up_one((g, p)))
        lat_up.append(time.perf_counter() - t)
    return out
r_up, t_up = timed("math_verify (1 process)", run_up)

def run_up_pool():
    with Pool(CORES) as pool:
        return pool.map(up_one, list(zip(golds, preds)), chunksize=8)
r_pool, t_pool = timed(f"math_verify (multiprocessing, {CORES} procs)", run_up_pool)

import fastmathverify as F  # noqa: E402
lat_f = []
def run_f():
    out = []
    for g, p in zip(golds, preds):
        t = time.perf_counter()
        out.append(F.verify(F.parse(g), F.parse(p)))
        lat_f.append(time.perf_counter() - t)
    return out
r_f, t_f = timed("fastmathverify (1 thread, Python loop)", run_f)
r_b, t_b = timed(f"fastmathverify.verify_batch ({CORES} threads)", lambda: F.verify_batch(golds, preds))

agree = sum(a == b for a, b in zip(r_up, r_f)) / N
print(f"\nagreement with math_verify on this sample: {100 * agree:.2f}%  (batch == loop: {r_b == r_f})")
print(f"median latency: math_verify {1e3 * statistics.median(lat_up):.2f} ms, "
      f"fastmathverify {1e3 * statistics.median(lat_f):.3f} ms")
print(f"p99 latency:    math_verify {1e3 * sorted(lat_up)[int(.99 * N)]:.1f} ms, "
      f"fastmathverify {1e3 * sorted(lat_f)[int(.99 * N)]:.2f} ms")
print(f"speedup (1 core): {t_up / t_f:.1f}x   speedup (all cores vs upstream pool): {t_pool / t_b:.1f}x   "
      f"(all cores vs upstream 1 process): {t_up / t_b:.1f}x")

# Upstream issue #65: reported to hang forever despite the timeout.
HANG = r"\boxed{\dfrac{5^{\left(5^{\left(5^{\left(5^5\right)}\right)} - 4\right)} - 5}{16}}"
for mod in ["math_verify", "fastmathverify"]:
    code = f"import time,{mod} as M; t=time.time(); M.parse({HANG!r}); print(round(time.time()-t,3))"
    try:
        out = subprocess.run([sys.executable, "-c", code], capture_output=True, text=True, timeout=60)
        print(f"issue #65 input, {mod}: {out.stdout.strip()} s")
    except subprocess.TimeoutExpired:
        print(f"issue #65 input, {mod}: did not finish in 60 s (killed)")
