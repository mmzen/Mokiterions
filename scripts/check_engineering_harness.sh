#!/usr/bin/env sh
set -eu
REPO_ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
python3 "$REPO_ROOT/scripts/validate_engineering_artifacts.py" --root "$REPO_ROOT"
python3 "$REPO_ROOT/scripts/generate_harness_dashboard.py" --root "$REPO_ROOT"
