#!/usr/bin/env bash
#
# SessionStart — put the ground truth in front of the agent before it does anything.
#
# On SessionStart, whatever this prints to stdout is added to the agent's context.
# Every session therefore begins knowing where the contract, the decisions and the
# approved specs live. That is Part 1 of the series, made un-forgettable.

set -uo pipefail

root="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}"

echo "Ground truth for this repo (loop engineering, enforced by hooks):"
echo
echo "  • AGENTS.md            — the contract. CLAUDE.md only points here."
echo "  • docs/architecture.md — the decisions and their *why*."
echo "                           REQUIRED READING before you change src/ or tests/;"
echo "                           a PreToolUse hook blocks the edit until you have read it."
echo "  • docs/index.md        — the docs bundle (Open Knowledge Format)."

approved=""
for spec in "$root"/docs/specs/*.md; do
  [ -e "$spec" ] || continue
  name="$(basename "$spec")"
  case "$name" in index.md | _*) continue ;; esac
  if grep -qE '^status:[[:space:]]*approved[[:space:]]*$' "$spec"; then
    approved="$approved  • docs/specs/$name"$'\n'
  fi
done

if [ -n "$approved" ]; then
  echo
  echo "Approved specs — the acceptance criteria in these are the definition of done:"
  printf '%s' "$approved"
fi

echo
echo "Before this session can end, a Stop hook runs ./check.sh, the browser suite,"
echo "and the definition-of-done audit. Red gates mean the work is not finished."
