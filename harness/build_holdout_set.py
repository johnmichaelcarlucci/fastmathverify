"""Build a held-out validation set that was never used while developing fastmathverify.

- OpenR1-Math-220k shards 00002 and 00007: real reasoning traces (+ reference solutions).
- DeepScaleR-Preview: (answer, solution) pairs, plus cross pairs (answer_i, solution_j).
- DeepScaleR + MATH-500 answers: each gold against a near-miss neighbouring answer
  (sorted order, so "\\frac{1}{2}" meets "\\frac{1}{3}"), in several answer formats.
  These are mostly *negative* pairs and catch a grader that is too lenient.
"""
import json, random
import pyarrow.parquet as pq

R = random.Random(4321)
out = []

def add(src, i, gold, pred):
    out.append({"id": f"{src}:{i}", "gold": gold, "pred": pred})

rows = []
for shard in ["00002", "00007"]:  # stream in batches; full shards don't fit in 3 GB of RAM
    f = pq.ParquetFile(f"data/openr1-{shard}.parquet")
    for b in f.iter_batches(batch_size=500, columns=["uuid", "answer", "generations", "solution"]):
        rows.extend(r for r in b.to_pylist() if R.random() < 0.25)
for r in rows:
    gold = f"${r['answer']}$"
    for gi, g in enumerate(r["generations"]):
        add("openr1", f"{r['uuid']}:g{gi}", gold, g)
    add("openr1", f"{r['uuid']}:sol", gold, r["solution"])

ds = json.load(open("data/deepscaler.json", encoding="utf-8"))
R.shuffle(ds)
ds = ds[:12000]
for i, r in enumerate(ds[:8000]):
    add("dsr-pos", i, f"${r['answer']}$", r["solution"])
for i in range(4000):
    a, b = ds[8000 + i], ds[R.randrange(len(ds))]
    add("dsr-cross", i, f"${a['answer']}$", b["solution"])

answers = sorted({r["answer"].strip() for r in json.load(open("data/deepscaler.json", encoding="utf-8"))} |
                 {json.loads(l)["answer"].strip() for l in open("data/math500.jsonl", encoding="utf-8")})
fmts = ["$\\boxed{%s}$", "The answer is $%s$.", "Final Answer: $%s$", "\\[ %s \\]", "%s"]
idx = list(range(len(answers) - 1))
R.shuffle(idx)
for k, i in enumerate(idx[:12000]):
    fmt = fmts[k % len(fmts)]
    add("near", k, f"${answers[i]}$", fmt.replace("%s", answers[i + 1]))
    if k % 4 == 0:  # some positives in each format too
        add("self", k, f"${answers[i]}$", fmt.replace("%s", answers[i]))

with open("data/holdout_pairs.jsonl", "w", encoding="utf-8") as f:
    for d in out:
        f.write(json.dumps(d) + "\n")
print(len(out), "pairs")
