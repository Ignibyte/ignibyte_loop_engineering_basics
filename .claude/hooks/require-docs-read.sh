#!/usr/bin/env bash
#
# PreToolUse(Edit|Write) — no code changes until the architecture notes have been read.
#
# AGENTS.md says "read docs/architecture.md before you plan a change." Saying it is
# a hope; this makes it a rule. Exit 2 denies the tool call outright and hands the
# message on stderr back to the agent, which then reads the file and tries again.
#
# Docs, specs and config stay editable — the gate guards the code, not the writing.

set -uo pipefail

input="$(cat)"
root="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}"

file="$(printf '%s' "$input" | jq -r '.tool_input.file_path // empty')"
session="$(printf '%s' "$input" | jq -r '.session_id // "unknown"')"

[ -n "$file" ] || exit 0

# Only guard the code.
case "$file" in
  "$root"/src/* | "$root"/tests/* | "$root"/e2e/* | "$root"/static/*) ;;
  *) exit 0 ;;
esac

# Already read it this session? Carry on.
[ -f "$root/.claude/.receipts/$session" ] && exit 0

cat >&2 <<EOF
Blocked: read docs/architecture.md before you change code.

AGENTS.md → "Before you plan a change". The architecture notes carry the decisions
and the reasons behind them — the context that stops a change from being built
against the grain of the codebase. They are short.

  Read docs/architecture.md (and the relevant docs/specs/*.md), then make this edit again.
EOF
exit 2
