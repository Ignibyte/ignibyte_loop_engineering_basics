---
type: Spec
title: <Short name>
description: <One sentence. What will be true when this ships?>
status: draft
tags: [spec]
timestamp: <ISO 8601, e.g. 2026-07-13T21:23:36-05:00>
---

<!--
  The template a spec is written from. It lives here, outside docs/, because docs/
  is an Open Knowledge Format bundle and every file in it is real knowledge — a
  half-filled template with placeholder text is not.

  status: draft -> a human reads it -> status: approved -> only then, code.
-->

# Goal

What is true today, what should be true after, and nothing else. One paragraph.

# Approach

The shape of the change in three or four bullets. Enough that a reader can picture the
diff. Not the diff.

# Files to touch

* `path/to/file` - what changes in it.

Naming these up front turns code review into a comparison. When the real diff touches a
file that isn't on this list, that is either a justified ripple or an agent wandering off
— and you only know which because you wrote the list.

# Scope fences

What this deliberately will **not** do:

* No <the obvious adjacent feature>.
* No <the tempting refactor>.

Most bad agent changes are not wrong. They are *extra*. Fences are how you say no in
advance, to something nobody has suggested yet.

# Acceptance criteria

Write them in [EARS](https://alistairmavin.com/ears/), before any code exists — that is
what stops the tests from being written *from* the implementation, asserting whatever the
code happens to do, bug and all.

Five templates, and between them they cover most of what you'll need:

* **Ubiquitous** — The service shall …
* **Event-driven** — WHEN <trigger>, the service shall …
* **State-driven** — WHILE <state>, the service shall …
* **Optional** — WHERE <feature is present>, the service shall …
* **Unwanted** — IF <bad thing>, THEN the service shall …

Name the test that will prove each one, and its runner. Use `e2e` for anything a request
cannot prove: that the page renders, that it doesn't reload, that nothing is posted.
`.claude/hooks/spec-done.sh` reads this table and checks that every test in it is real, so
the names must match exactly. Keep `|` out of the criterion text — it is a table.

| # | Criterion (EARS) | Test | Runner |
|---|------------------|------|--------|
| 1 | WHEN …, the service shall … | `test_name_here` | rust |
| 2 | IF …, THEN the page shall … | `test_name_here` | e2e |

# Definition of done

Every criterion above is verified by a passing test, and every gate is green. The `Stop`
hook checks all of it before the session is allowed to end.

# Citations

[1] [Title](https://example.com) - why it's here.
