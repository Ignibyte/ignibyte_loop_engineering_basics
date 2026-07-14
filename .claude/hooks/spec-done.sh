#!/usr/bin/env bash
#
# The definition of done, audited mechanically.
#
# Every approved spec ends in a table of EARS acceptance criteria, and every row
# names the test that proves it. This walks those tables and checks that each named
# test really exists — in the Rust suite or the browser suite.
#
# That is the half a machine can check. The other half — that those tests *pass* —
# is what ./check.sh and the browser suite are for; gates.sh runs all three together,
# and only then is a spec done. Write a criterion and never test it, or quietly
# rename the test out from under it, and this fails.
#
# Run it yourself any time:  .claude/hooks/spec-done.sh

set -uo pipefail

root="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}"
cd "$root" || exit 1

status=0
audited=0

# Every test name the Rust suite actually defines ("name: test" per line).
rust_tests="$(cargo test --quiet --test api -- --list 2>/dev/null | sed 's/: test$//')"

for spec in docs/specs/*.md; do
  [ -e "$spec" ] || continue
  case "$(basename "$spec")" in index.md | _*) continue ;; esac
  grep -qE '^status:[[:space:]]*approved[[:space:]]*$' "$spec" || continue

  echo "Definition of done — $spec"
  audited=$((audited + 1))
  criteria=0

  # Rows look like:  | 1 | WHEN ... shall ... | `test_name` | rust |
  while IFS='|' read -r _ _ _ test runner _; do
    test="$(printf '%s' "$test" | tr -d '`' | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')"
    runner="$(printf '%s' "$runner" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')"

    # Skip the header row and the |---|---| separator.
    case "$test" in "" | "Test" | -*) continue ;; esac

    criteria=$((criteria + 1))

    case "$runner" in
      rust)
        if printf '%s\n' "$rust_tests" | grep -qxF "$test"; then
          printf '  ✓ %-38s %s\n' "$test" "$runner"
        else
          printf '  ✗ %-38s %s  — no such test in tests/api.rs\n' "$test" "$runner"
          status=1
        fi
        ;;
      e2e)
        if grep -qF "test(\"$test\"" e2e/*.spec.ts 2>/dev/null; then
          printf '  ✓ %-38s %s\n' "$test" "$runner"
        else
          printf '  ✗ %-38s %s  — no such test in e2e/\n' "$test" "$runner"
          status=1
        fi
        ;;
      *)
        printf '  ✗ %-38s %s  — unknown runner (use "rust" or "e2e")\n' "$test" "$runner"
        status=1
        ;;
    esac
  done < <(awk '/^# Acceptance criteria/ {inside = 1; next} /^# / {inside = 0} inside' "$spec" | grep '^|')

  if [ "$criteria" -eq 0 ]; then
    echo "  ✗ approved, but it states no acceptance criteria — nothing can prove it is done"
    status=1
  fi
  echo
done

if [ "$audited" -eq 0 ]; then
  echo "No approved specs to audit."
fi

if [ "$status" -ne 0 ]; then
  echo "A criterion with no test is a promise nobody checked. Write the test, or drop the criterion."
fi

exit "$status"
