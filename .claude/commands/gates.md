---
description: Run every gate and fix what's red.
allowed-tools: Bash(./check.sh:*), Bash(npx playwright test:*), Bash(.claude/hooks/spec-done.sh:*), Bash(cargo:*), Bash(npm:*)
---

Run every gate, in this order, and fix everything that comes back red:

1. `./check.sh` — formatting, clippy with `-D warnings`, the Rust suite, the docs build.
2. `npx playwright test` — the browser suite.
3. `.claude/hooks/spec-done.sh` — every acceptance criterion in every approved spec is
   backed by a test that really exists.

**Mostly green is red.** There is no partial credit, no "that failure is unrelated", and
no shipping with a warning to fix later. Later does not come.

If a gate fails, fix the *code*, not the gate. If you genuinely believe a gate itself is
wrong, stop and say so — changing a gate is a decision for a human, and it needs a reason
that will still make sense in six months.
