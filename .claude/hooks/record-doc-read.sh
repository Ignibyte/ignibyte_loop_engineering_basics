#!/usr/bin/env bash
#
# PostToolUse(Read) — remember that the agent read the architecture notes.
#
# Half of the "force the read" gate. This leaves a receipt, keyed by session, the
# moment docs/architecture.md is actually read. The other half (require-docs-read.sh)
# refuses to let code be edited until the receipt exists.
#
# The receipt is per-session on purpose: a fresh session has read nothing, which is
# exactly the amnesia the docs exist to cure.

set -uo pipefail

input="$(cat)"
root="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}"

file="$(printf '%s' "$input" | jq -r '.tool_input.file_path // empty')"
session="$(printf '%s' "$input" | jq -r '.session_id // "unknown"')"

case "$file" in
  */docs/architecture.md)
    mkdir -p "$root/.claude/.receipts"
    : > "$root/.claude/.receipts/$session"
    ;;
esac

exit 0
