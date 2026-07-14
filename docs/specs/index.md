# Concepts

* [Note persistence](note-persistence.md) - Notes are mirrored to a JSON file so they survive a restart. Approved; shipped.
* [Notes UI](notes-ui.md) - The page at `/` becomes a real notes app: list, add, refuse an empty note. Approved; shipped.

A spec is done when every acceptance criterion in it is backed by a passing test.
`.claude/hooks/spec-done.sh` audits that, and the `Stop` hook runs it.
