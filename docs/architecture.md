---
type: Architecture
title: Architecture notes
description: The decisions behind the notes service, and the reasons for them that the code cannot tell you.
tags: [architecture, decisions, rust, axum]
timestamp: 2026-07-13T21:23:36-05:00
---

The *why* behind `ignibyte_loop_engineering_basics`. The code can tell a reader
what it does; only this file can tell them what was ruled out, and on what grounds.
Read it before planning a change — a hook will not let you skip it.

# Shape

- One axum `Router`, built in `lib.rs::app()`, served by a thin `main.rs`.
- Handlers (`routes.rs`) stay thin: parse the request, call into `AppState`, return
  a response. No business logic in handlers.
- `state.rs` is the only place that touches storage. Handlers never reach past it.
- `model.rs` owns validation. A note that is empty or whitespace-only is rejected
  there, before it can reach the store.

# Decisions

| Decision | Why |
|----------|-----|
| **In-memory, file-backed, no database.** | Notes live in a `Vec<Note>` behind a `Mutex` and are mirrored to a JSON file so they survive a restart. A database stays deliberately out of scope until a feature genuinely needs one — a flat file is enough for a single-process demo, and adding Postgres would make this repo a lesson about Postgres. |
| **lib + bin split.** | The router lives in the library so integration tests can drive it with `oneshot`, without opening a socket. `main.rs` only binds a port and serves. |
| **Errors, not panics, in request paths.** | Handlers return a `Result` or a proper status code. The store treats a poisoned lock as unrecoverable; nothing else panics. |
| **The page is embedded, not served from disk.** | `static/index.html` is baked into the binary with `include_str!`, so the service stays a single self-contained artifact — no asset directory to ship beside it, no static-file middleware, and no way for the two to drift apart. |
| **Vanilla HTML, no frontend framework.** | The page is small enough not to need one, and a build step would make this repo a lesson about bundlers instead of about the loop. |
| **Writes are whole-file.** | Every create rewrites the JSON file. Fine at this size; the first thing to revisit if the store ever grows. |

# Known limits

Stated plainly, so no future session mistakes a deliberate omission for a bug:

- **No concurrent-writer safety.** Single process, single file, last write wins.
  Documented rather than solved.
- **`id` is `len + 1`.** Nothing deletes notes, so ids cannot collide. The moment
  deletion exists this breaks — and that is a spec, not a patch.
- **No pagination.** The whole list is read and written every time.

# Citations

[1] [Beginner Loop Engineering](https://ignibyte.com/lab/beginner-loop-engineering-documentation-foundation) - the series this repo was built for.
