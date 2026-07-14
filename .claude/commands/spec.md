---
description: Plan a change — read the docs, write a spec, stop for approval.
---

Plan this change, and **write no code**: `$ARGUMENTS`

1. **Read first.** `docs/architecture.md`, then any spec in `docs/specs/` that touches
   the same ground. Build *with* the codebase, not against it. If your idea contradicts
   a decision recorded there, say so and stop — that is a conversation, not a commit.

2. **Write the spec** to `docs/specs/<slug>.md`, following `.claude/templates/spec.md`
   exactly: Open Knowledge Format frontmatter, then Goal, Approach, Files to touch,
   Scope fences, Acceptance criteria, Definition of done.

3. **Scope fences are as important as the goal.** Name what this will *not* do. Most bad
   agent changes are not wrong, they are *extra*.

4. **Write the acceptance criteria in EARS**, and give each one the name of the test that
   will prove it, and the runner (`rust` or `e2e`). Reach for `e2e` for anything a request
   cannot prove — that the page renders, that it doesn't reload, that nothing is posted.
   These criteria become the tests, so write them as if the code does not exist. It doesn't.

5. **Set `status: draft` and stop.** Show me the spec. I approve it by changing `status`
   to `approved` — and only then does anyone write code. An approved spec is a contract:
   from that point on, deviation is a signal to come back and re-spec, not licence to
   improvise.
