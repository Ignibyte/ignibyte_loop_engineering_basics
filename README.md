# ignibyte_loop_engineering_basics

The demo app for the **Beginner Loop Engineering** blog series from the
[Ignibyte](https://github.com/Ignibyte) lab. It is deliberately small: a tiny
Rust web service that keeps a list of notes, so the series can plan, build,
test, and document a real change without the app itself getting in the way.

It serves:

- `GET /` — a plain HTML index page
- `GET /healthz` — a liveness check
- `GET /api/notes` — the notes list, as JSON
- `POST /api/notes` — add a note (`{ "text": "..." }`)

Notes are held in memory and persisted to a JSON file (`notes.json`; override
with the `NOTES_FILE` environment variable), so they survive a restart. There's
no database — the file is the whole store.

## Run it

```
cargo run
# then open http://127.0.0.1:3000
```

## Quality gates

Every change must pass, with zero warnings:

```
./check.sh   # fmt check -> clippy -D warnings -> tests -> docs
```

## How this repo is organized

The agent-facing contract lives in [AGENTS.md](AGENTS.md) (with a pointer for
Claude Code in [CLAUDE.md](CLAUDE.md)). The very first commit in this repo is
those documents and this README — **written before any code existed.** That is
the whole subject of Part 1.
