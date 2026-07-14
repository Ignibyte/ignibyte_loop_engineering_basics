# ignibyte_loop_engineering_basics

The demo app for the **[Beginner Loop Engineering](https://ignibyte.com/lab/beginner-loop-engineering-documentation-foundation)**
series from the [Ignibyte](https://github.com/Ignibyte) lab.

The app is deliberately small — a notes list, in Rust — because the app is not the
point. **The scaffolding around it is the point:** documentation an agent actually
reads, gates it cannot skip, specs whose acceptance criteria become the tests, and
hooks that enforce every bit of it without asking anyone's permission.

Read the commit history top to bottom and you will watch the loop turn.

## The app

- `GET /` — the notes app: list the notes, add one
- `GET /healthz` — a liveness check
- `GET /api/notes` — the notes list, as JSON
- `POST /api/notes` — add a note (`{ "text": "..." }`)

Notes are held in memory and mirrored to a JSON file (`notes.json`; override with
`NOTES_FILE`), so they survive a restart. There is no database — the file is the whole
store, on purpose, and [`docs/architecture.md`](docs/architecture.md) says why.

```
cargo run
# then open http://127.0.0.1:3000
```

## The scaffolding

| | |
|---|---|
| **[AGENTS.md](AGENTS.md)** | The contract every session reads. [CLAUDE.md](CLAUDE.md) only points here, so the same file works for any agent. The very first commit in this repo is this file — **written before any code existed.** |
| **[docs/](docs/index.md)** | The knowledge bundle, in the [Open Knowledge Format][okf]: the architecture decisions and their *why*, the approved specs, and a log. Plain markdown with YAML frontmatter — readable without tooling, diffable in git. |
| **[check.sh](check.sh)** | Every gate in one command: `fmt` → `clippy -D warnings` → tests → docs build. CI runs the same script, so there is one definition of green. |
| **[e2e/](e2e/notes.spec.ts)** | Playwright. A page can pass every assertion in `tests/api.rs` and still render blank; somebody has to open it. |
| **[.claude/](.claude/README.md)** | The hooks that make all of the above un-skippable. **This is the interesting part.** |

## The hooks are the interesting part

Documentation *asks*. Hooks *check*. Five short shell scripts:

- You **cannot edit `src/`** until you have read `docs/architecture.md` this session. The
  edit is denied, with the reason handed back to the agent.
- Rust is **formatted the instant it is written**, so style never reaches a review.
- The session **cannot end** while `./check.sh` is red, while the browser suite is red, or
  while any approved spec has an acceptance criterion that no test proves. An agent asked
  *"are you done?"* says yes — so nobody asks it.

That last one is the definition of done, made mechanical: every spec's criteria table names
the test that proves each criterion, and [`spec-done.sh`](.claude/hooks/spec-done.sh) checks
that the test is real. Write a criterion you never tested and the loop won't close.

The hooks are in [`.claude/`](.claude/README.md), they need `jq`, and — because they are
checked in — cloning this repo will ask you to trust them before anything runs. Read them
first. They're short, and you should never let a stranger's repo run shell on your machine
without looking.

## Run everything

```
./check.sh                        # fmt, clippy, tests, docs
npm ci && npm run e2e:install     # once
npm run e2e                       # the browser suite
.claude/hooks/spec-done.sh        # every criterion, and the test that proves it
```

## Licence

MIT.

[okf]: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
