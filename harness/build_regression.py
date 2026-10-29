"""Sample a small regression corpus (short inputs, upstream verdicts) for `cargo test`.

Only pairs where fastmathverify agrees with upstream are kept; known differences live in
DIFFERENCES.md. Output: tests/data/regression.jsonl
"""
import json, random
R = random.Random(7)
out = []
for pairs, ref in [("data/openr1_pairs.jsonl", "data/openr1_ref.jsonl"), ("data/holdout_pairs.jsonl", "data/holdout_ref.jsonl")]:
    ours = pairs.replace("_pairs", "_ours")
    for p, r, o in zip(open(pairs), open(ref), open(ours)):
        p, r, o = json.loads(p), json.loads(r), json.loads(o)
        if r["verify"] == o["verify"] and len(p["pred"]) <= 1200 and len(p["gold"]) <= 300:
            out.append({"id": p["id"], "gold": p["gold"], "pred": p["pred"], "verify": r["verify"]})
by_src = {}
for d in out:
    src = d["id"].split(":")[0]
    src = src if src in ("openr1", "dsr-pos", "dsr-cross", "near", "self") else "openr1-dev"
    by_src.setdefault(f"{src}={d['verify']}", []).append(d)
keep = []
for k, v in sorted(by_src.items()):
    R.shuffle(v)
    keep += v[:250]
keep.sort(key=lambda d: d["id"])
with open("tests/data/regression.jsonl", "w") as f:
    for d in keep:
        f.write(json.dumps(d, ensure_ascii=False) + "\n")
print(len(keep), {k: min(len(v), 250) for k, v in sorted(by_src.items())})
