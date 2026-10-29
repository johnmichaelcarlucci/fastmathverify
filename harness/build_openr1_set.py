"""Build the OpenR1-Math-220k evaluation pairs (deterministic sample)."""
import json, random
import pyarrow.parquet as pq

rows = []
for shard in ["00000", "00005"]:
    t = pq.read_table(f"data/openr1-{shard}.parquet", columns=["uuid", "answer", "generations", "solution"])
    rows.extend(t.to_pylist())
random.Random(1234).shuffle(rows)
rows = rows[:8000]
with open("data/openr1_pairs.jsonl", "w", encoding="utf-8") as f:
    n = 0
    for r in rows:
        gold = f"${r['answer']}$"
        for gi, g in enumerate(r["generations"]):
            f.write(json.dumps({"id": f"{r['uuid']}:g{gi}", "gold": gold, "pred": g}) + "\n"); n += 1
        f.write(json.dumps({"id": f"{r['uuid']}:sol", "gold": gold, "pred": r["solution"]}) + "\n"); n += 1
print(n, "pairs")
