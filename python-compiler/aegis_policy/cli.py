import argparse
import hashlib
import sys

from .binfmt import build_dummy, verify_bin


def main(argv=None):
    ap = argparse.ArgumentParser(prog="aegis-compiler")
    sub = ap.add_subparsers(dest="cmd", required=True)
    d = sub.add_parser("dummy-bin", help="emit a valid deterministic dummy .bin")
    d.add_argument("out")
    d.add_argument("--k", type=int, default=4)
    d.add_argument("--dim", type=int, default=64)
    v = sub.add_parser("verify", help="validate a .bin against the v1 rules")
    v.add_argument("path")
    a = ap.parse_args(argv)
    if a.cmd == "dummy-bin":
        data = build_dummy(a.out, k=a.k, d=a.dim)
        print(f"wrote {a.out} ({len(data)} bytes) sha256={hashlib.sha256(data).hexdigest()}")
    else:
        print(verify_bin(open(a.path, "rb").read()))
    return 0


if __name__ == "__main__":
    sys.exit(main())
