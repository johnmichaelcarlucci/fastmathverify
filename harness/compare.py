"""Compare Rust results against upstream reference results (by id)."""
import json, sys, collections
ref = {json.loads(l)["id"]: json.loads(l) for l in open(sys.argv[1])}
ours = {json.loads(l)["id"]: json.loads(l) for l in open(sys.argv[2])}
n = agree = 0
kinds = collections.Counter()
dis = []
for k, r in ref.items():
    o = ours.get(k)
    if o is None:
        continue
    n += 1
    if r["verify"] == o["verify"]:
        agree += 1
    else:
        kinds["ref=%s ours=%s" % (r["verify"], o["verify"])] += 1
        dis.append((k, r, o))
print(f"pairs={n} agree={agree} ({100*agree/n:.3f}%)")
for k, v in kinds.most_common():
    print(" ", k, v)
same_parse = sum(1 for k, r in ref.items() if k in ours and r["pred_parsed"][-1:] == ours[k]["pred_parsed"][-1:])
print(f"pred fallback-string identical: {same_parse}/{n}")
if len(sys.argv) > 3:
    with open(sys.argv[3], "w") as f:
        for k, r, o in dis:
            f.write(json.dumps({"id": k, "ref": r["verify"], "gold_ref": r["gold_parsed"], "gold_ours": o["gold_parsed"],
                                "pred_ref": r["pred_parsed"], "pred_ours": o["pred_parsed"]}, ensure_ascii=False) + "\n")
