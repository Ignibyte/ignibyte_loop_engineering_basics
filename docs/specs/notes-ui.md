---
type: Spec
title: Notes UI
description: The page at / becomes a real notes app — list the notes, add one, refuse an empty one.
status: draft
tags: [spec, ui, browser, validation]
timestamp: 2026-07-13T21:23:36-05:00
---

# Goal

`GET /` returns a placeholder that describes the API in prose. Make it the actual
app: show the notes, add a note, and refuse an empty one. The service already
stores notes; nothing in the browser can reach that yet.

# Approach

- Serve one page from `static/index.html`, embedded with `include_str!` so the
  binary stays self-contained.
- Plain HTML, CSS and `fetch` — no framework, no build step. The page reads
  `GET /api/notes` and posts to `POST /api/notes`.
- Reject an empty note in **both** places: the browser, so the user gets an instant
  answer, and the API, because a browser check is a courtesy and not a control.

# Files to touch

* `static/index.html` - the page: list, form, empty state, error line.
* `src/routes.rs` - serve the page; map a rejected note to `400`.
* `src/model.rs` - validation lives here, not in the handler.
* `tests/api.rs` - the criteria a request can prove.
* `e2e/notes.spec.ts` - the criteria only a browser can prove.

# Scope fences

What this deliberately will **not** do:

* No delete, edit, search or pagination — the note-persistence fences still stand.
* No new endpoints. The page uses the two that exist.
* No framework, no bundler, no npm in the service itself. Playwright is a dev
  dependency of the browser suite and nothing else.
* No styling system. One `<style>` block, and it respects the OS light/dark setting.

# Acceptance criteria

Note which rows say `e2e`. Those are the ones an integration test cannot reach: a
page can pass every assertion in `tests/api.rs` and still render blank, reload the
whole document on submit, or post rubbish to the API. Somebody has to open it.

| # | Criterion (EARS) | Test | Runner |
|---|------------------|------|--------|
| 1 | WHEN a client requests `GET /`, the service shall return `200` with an HTML page containing the notes app. | `index_serves_the_notes_app` | rust |
| 2 | IF the submitted text is empty or whitespace-only, THEN the service shall return `400` and shall not store a note. | `create_rejects_empty_text` | rust |
| 3 | WHEN a note is stored, the service shall store its text with surrounding whitespace trimmed. | `create_trims_surrounding_whitespace` | rust |
| 4 | WHILE no notes exist, the page shall show an explicit empty state. | `shows_an_empty_state` | e2e |
| 5 | WHILE notes exist in the store, the page shall list them on first load. | `shows_existing_notes_on_load` | e2e |
| 6 | WHEN a user submits non-empty text, the page shall show the new note without reloading the document. | `adds_a_note_without_a_reload` | e2e |
| 7 | IF a user submits empty text, THEN the page shall show a validation message and send no request. | `rejects_an_empty_note_in_the_ui` | e2e |

# Definition of done

Every criterion above is verified by a passing test, the Rust suite is green, and
the browser suite is green. `.claude/hooks/gates.sh` runs all three at `Stop`, so
the session cannot end until they are.

# Citations

[1] [Beginner Loop Engineering, Part 5: Test](https://ignibyte.com/lab/beginner-loop-engineering-test)
[2] [Playwright](https://playwright.dev/)
