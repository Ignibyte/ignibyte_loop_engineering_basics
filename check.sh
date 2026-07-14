#!/usr/bin/env bash
# One command, every gate — fast to slow. Any failure stops the run (non-zero exit),
# so "done" means "all gates green," not "looks done."
#
# The browser suite is deliberately NOT here: it needs npm and a downloaded browser,
# and this script has to stay runnable on a bare `git clone` with nothing but Rust.
# Run it with `npm run e2e`. CI runs both, and so does the Stop hook.
set -euo pipefail

# Runnable from anywhere — the hooks call it with a cwd of their own.
cd "$(dirname "$0")"

echo "==> fmt";    cargo fmt --check
echo "==> clippy"; cargo clippy --all-targets -- -D warnings
echo "==> test";   cargo test --quiet
echo "==> docs";   RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --quiet
echo "✓ all gates green"
