#!/usr/bin/env bash
#
# PostToolUse(Edit|Write) — format Rust the instant it is written, and remember
# that this session touched code.
#
# Formatting: `cargo fmt --check` fails a build over a stray space. Rather than let
# the agent burn a turn discovering that, format the file it just touched. Style
# stops being something anyone reviews, argues about, or fails on.
#
# The marker: the Stop hook uses it to decide whether the gates are worth running.
# A session that only read and explained code has nothing to gate; a session that
# wrote code does — and the marker survives even if the agent commits its work,
# which a dirty-working-tree check would not.

set -uo pipefail

input="$(cat)"
root="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}"

file="$(printf '%s' "$input" | jq -r '.tool_input.file_path // empty')"
session="$(printf '%s' "$input" | jq -r '.session_id // "unknown"')"

[ -n "$file" ] || exit 0

# One file, not the whole crate — this runs after every single edit.
case "$file" in
  *.rs)
    if [ -f "$file" ] && command -v rustfmt >/dev/null 2>&1; then
      rustfmt --edition 2021 "$file" >/dev/null 2>&1 || true
    fi
    ;;
esac

case "$file" in
  "$root"/src/* | "$root"/tests/* | "$root"/e2e/* | "$root"/static/* | "$root"/Cargo.toml)
    mkdir -p "$root/.claude/.receipts"
    : > "$root/.claude/.receipts/$session.code"
    ;;
esac

exit 0
