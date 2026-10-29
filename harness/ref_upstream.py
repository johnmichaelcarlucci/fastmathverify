"""Run upstream Math-Verify on a JSONL of {id, gold, pred}; write results + timings.

Each pair runs in a worker's main thread so upstream's signal-based timeouts work.
"""
import json, sys, time
from multiprocessing import Pool

def init():
    global parse, verify
    from math_verify import parse, verify

def sstr(x):
    try:
        return str(x)
    except ValueError:  # integers over Python's 4300-digit str() limit
        return "<unprintable>"

def run(line):
    d = json.loads(line)
    t0 = time.perf_counter()
    g = parse(d["gold"])
    p = parse(d["pred"])
    t1 = time.perf_counter()
    ok = verify(g, p)
    t2 = time.perf_counter()
    return json.dumps({"id": d["id"], "verify": bool(ok), "gold_parsed": [sstr(x) for x in g],
                       "pred_parsed": [sstr(x) for x in p], "t_parse": t1 - t0, "t_verify": t2 - t1})

if __name__ == "__main__":
    src, dst, procs = sys.argv[1], sys.argv[2], int(sys.argv[3])
    lines = open(src, encoding="utf-8").read().splitlines()
    t0 = time.time()
    with Pool(procs, initializer=init, maxtasksperchild=2000) as pool, open(dst, "w", encoding="utf-8") as out:
        for i, r in enumerate(pool.imap(run, lines, chunksize=16)):
            out.write(r + "\n")
            if i % 2000 == 0:
                print(i, round(time.time() - t0, 1), flush=True)
    print("done", len(lines), round(time.time() - t0, 1))
