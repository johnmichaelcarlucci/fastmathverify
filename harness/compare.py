"""Compare Rust results against upstream reference results line by line
(both files preserve the input order of data/openr1_pairs.jsonl)."""
import json, sys, collections
ref = [json.loads(l) for l in open(sys.argv[1])]
ours = [json.loads(l) for l in open(sys.argv[2])]
assert len(ref) == len(ours), (len(ref), len(ours))
n = len(ref)
agree = sum(r["verify"] == o["verify"] for r, o in zip(ref, ours))
kinds = collections.Counter(f"ref={r['verify']} ours={o['verify']}" for r, o in zip(ref, ours) if r["verify"] != o["verify"])
same_fb = sum(r["pred_parsed"][-1:] == o["pred_parsed"][-1:] for r, o in zip(ref, ours))
print(f"pairs={n} agree={agree} ({100*agree/n:.3f}%)")
for k, v in kinds.most_common():
    print(" ", k, v)
print(f"pred fallback-string identical: {same_fb}/{n}")
if len(sys.argv) > 3:
    with open(sys.argv[3], "w") as f:
        for r, o in zip(ref, ours):
            if r["verify"] != o["verify"]:
                f.write(json.dumps({"id": r["id"], "ref": r["verify"], "gold_ref": r["gold_parsed"], "gold_ours": o["gold_parsed"],
                                    "pred_ref": r["pred_parsed"], "pred_ours": o["pred_parsed"]}, ensure_ascii=False) + "\n")
