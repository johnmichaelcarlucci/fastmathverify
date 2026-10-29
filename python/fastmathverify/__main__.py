"""``python -m fastmathverify GOLD PRED``: grade one pair and print why."""
import argparse
import json
import sys

from . import explain


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="python -m fastmathverify", description="Grade one answer against a gold answer.")
    ap.add_argument("gold", help="gold answer text, e.g. '$\\frac{1}{2}$'")
    ap.add_argument("pred", help="model output text; '-' reads it from stdin")
    ap.add_argument("--exact", action="store_true", help="compare numbers by exact value")
    args = ap.parse_args(argv)
    pred = sys.stdin.read() if args.pred == "-" else args.pred
    r = explain(args.gold, pred, exact=args.exact)
    print(json.dumps(r, ensure_ascii=False))
    return 0 if r["correct"] else 1


if __name__ == "__main__":
    sys.exit(main())
