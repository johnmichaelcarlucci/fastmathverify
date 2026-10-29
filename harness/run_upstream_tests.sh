#!/bin/sh
# Copies upstream's tests next to the shim conftest and runs them against fastmathverify.
set -e
cd "$(dirname "$0")/.."
D=$(mktemp -d)
cp -r ref/upstream/tests "$D/tests"
cp harness/upstream_shim/conftest.py "$D/conftest.py"
cd "$D" && python -m pytest -q -p no:cacheprovider "$@" tests
