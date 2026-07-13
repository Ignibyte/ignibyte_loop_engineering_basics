#!/usr/bin/env bash
# One command, every gate — fast to slow. Any failure stops the run (non-zero exit),
# so "done" means "all gates green," not "looks done."
set -euo pipefail

echo "==> fmt";    cargo fmt --check
echo "==> clippy"; cargo clippy --all-targets -- -D warnings
echo "==> test";   cargo test --quiet
echo "==> docs";   RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --quiet
echo "✓ all gates green"
