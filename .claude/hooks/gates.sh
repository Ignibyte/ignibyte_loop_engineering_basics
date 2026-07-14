#!/usr/bin/env bash
#
# Stop — the agent does not get to decide when the work is done.
#
# This is "pass the whole suite, or redo it" (Part 4), made un-skippable. An agent
# asked "are you finished?" is strongly inclined to say yes. So nobody asks it.
# When it tries to stop, the gates run; if they are red, exit 2 sends it back to
# work with the failure as its next instruction.
#
# Three things have teeth, in the order they get cheaper to be wrong about:
#   1. ./check.sh          — fmt, clippy, the Rust suite, the docs build
#   2. the browser suite   — a page can pass every assertion and still render blank
#   3. spec-done.sh        — every acceptance criterion is backed by a real test
#
# And one thing only nudges, once:
#   4. doc drift           — code moved, documentation did not (Part 6)
#
# Why the difference: 1–3 are facts, and a machine should hold the line on facts.
# 4 is a judgement call — a pure refactor genuinely changes nothing documented —
# so it asks once and then trusts the answer. Gates that cry wolf get switched off.

set -uo pipefail

input="$(cat)"
root="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}"
cd "$root" || exit 0

session="$(printf '%s' "$input" | jq -r '.session_id // "unknown"')"
stop_active="$(printf '%s' "$input" | jq -r '.stop_hook_active // false')"

state="$root/.claude/.receipts"
mkdir -p "$state"
blocks_file="$state/$session.blocks"

# Did this session touch code at all? If not, there is nothing to gate, and a
# question about the codebase shouldn't cost a cargo build.
touched_code=0
[ -f "$state/$session.code" ] && touched_code=1
[ -n "$(git status --porcelain 2>/dev/null)" ] && touched_code=1
if [ "$touched_code" -eq 0 ]; then
  rm -f "$blocks_file"
  exit 0
fi

blocks=0
[ -f "$blocks_file" ] && blocks="$(cat "$blocks_file" 2>/dev/null)"
case "$blocks" in '' | *[!0-9]*) blocks=0 ;; esac

# A gate with no way out is a broken gate. If the agent cannot get green after this
# many tries, it is stuck — hand it back to a human instead of looping forever.
max_blocks=5
if [ "$blocks" -ge "$max_blocks" ]; then
  rm -f "$blocks_file"
  echo "gates.sh: still red after $max_blocks attempts. Stopping so a human can look." >&2
  exit 0
fi

block() {
  headline="$1"
  shift
  echo $((blocks + 1)) >"$blocks_file"
  {
    echo "$headline"
    echo
    printf '%s\n' "$@"
  } >&2
  exit 2
}

# 1. The gates.
if ! out="$(./check.sh 2>&1)"; then
  block "The quality gates are red, so the work is not done. Fix every failure, then stop again." \
    "$(printf '%s' "$out" | tail -40)"
fi

# 2. The browser.
if [ -d node_modules/@playwright ]; then
  if ! out="$(npx playwright test --reporter=line 2>&1)"; then
    block "The browser suite is red. The API may be fine; the page a human actually sees is not." \
      "$(printf '%s' "$out" | tail -30)"
  fi
else
  echo "gates.sh: browser suite skipped — Playwright is not installed here (npm ci && npm run e2e:install). CI still runs it." >&2
fi

# 3. The definition of done.
if ! out="$(.claude/hooks/spec-done.sh 2>&1)"; then
  block "An approved spec has a criterion that nothing tests, so it cannot be called done." "$out"
fi

# 4. Done is documented.
if [ "$stop_active" != "true" ] &&
  [ -n "$(git status --porcelain -- src static 2>/dev/null)" ] &&
  [ -z "$(git status --porcelain -- docs README.md AGENTS.md 2>/dev/null)" ]; then
  block "Behaviour changed, but no documentation did." \
    "src/ or static/ has uncommitted changes; docs/, README.md and AGENTS.md do not." \
    "" \
    "If this change alters anything a future session would need to read about — an" \
    "endpoint, a decision, a command, a constraint — update the docs now. The next" \
    "session starts by trusting them completely." \
    "" \
    "If it genuinely changes nothing documented, say so and stop again. This will not ask twice."
fi

rm -f "$blocks_file"
exit 0
