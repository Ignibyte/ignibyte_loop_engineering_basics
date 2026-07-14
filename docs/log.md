## 2026-07-13

**Update** — The page at `/` became the actual notes app: list, add, empty state,
and a validation message. `POST /api/notes` now rejects an empty or whitespace-only
note with `400` and trims the text it stores. Planned in
[Notes UI](specs/notes-ui.md); the four criteria a request cannot prove are covered
by a Playwright suite in `e2e/`.

**Update** — This bundle moved to the [Open Knowledge Format][okf]: every concept
now carries `type`, `title`, `description` and `timestamp`, and `index.md` files
make it walkable a directory at a time instead of all at once.

**Update** — The enforcement layer arrived in `.claude/`. Reading
[Architecture](architecture.md) is now a precondition of editing `src/`; the quality
gates, the browser suite and the definition-of-done audit all run before a session is
allowed to end. Every rule the documentation used to *ask* for, something now *checks*.

**Update** — [Note persistence](specs/note-persistence.md) shipped. Notes are
mirrored to a JSON file, so they survive a restart. `README.md` and
[Architecture](architecture.md) were rewritten to match, which is the point: the
loop is not closed until the documentation describes the system that now exists.

**Creation** — The repository began with `AGENTS.md`, `CLAUDE.md` and this
documentation, written before a single line of code. That ordering is the whole
argument of the series.

[okf]: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
