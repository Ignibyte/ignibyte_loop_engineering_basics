# The enforcement layer

Documentation *asks*. Hooks *check*.

Everything in `AGENTS.md` is a rule an agent is free to forget — and it will, because
every session starts with total amnesia and infinite goodwill. These five hooks turn
the rules that matter into things a machine verifies. They are the difference between
a workflow you hope is being followed and one that is.

Nothing here is clever. It is five short shell scripts. That is rather the point.

## What runs, and when

| Hook | Fires on | What it does |
|------|----------|--------------|
| [`session-start.sh`](hooks/session-start.sh) | `SessionStart` | Prints the ground truth — contract, architecture, approved specs — straight into the agent's context. Whatever a `SessionStart` hook writes to stdout, the agent reads. Session one and session fifty now begin the same way. |
| [`require-docs-read.sh`](hooks/require-docs-read.sh) | `PreToolUse` on `Edit`/`Write` | **Denies** any edit to `src/`, `tests/`, `e2e/` or `static/` until `docs/architecture.md` has been read this session. Exit 2 blocks the tool call and hands the reason back to the agent, which reads the file and tries again. |
| [`record-doc-read.sh`](hooks/record-doc-read.sh) | `PostToolUse` on `Read` | The other half of that gate: leaves a receipt, keyed by session, the moment the architecture notes are actually read. |
| [`after-write.sh`](hooks/after-write.sh) | `PostToolUse` on `Edit`/`Write` | Runs `rustfmt` on the file that was just written, and records that this session touched code. Formatting stops being something anyone reviews, argues about, or fails a build on. |
| [`gates.sh`](hooks/gates.sh) | `Stop` | The one that matters. See below. |

Wired up in [`settings.json`](settings.json). They need `jq`, which is one `brew install jq`
or `apt install jq` away.

## The Stop hook is the whole idea

An agent asked *"are you finished?"* is strongly inclined to say yes. So nobody asks it.

When the agent tries to end its turn, `gates.sh` runs. If anything is red it exits 2,
and the failure becomes the agent's next instruction. It does not get to stop while the
work is broken — not because it promised, but because it can't.

Three things have teeth:

1. **`./check.sh`** — formatting, clippy with `-D warnings`, the Rust suite, the docs build.
   The regression floor: *nothing broke*.
2. **The browser suite** — `npx playwright test`. A page can pass every assertion in
   `tests/api.rs` and still render blank. Somebody has to open it.
3. **[`spec-done.sh`](hooks/spec-done.sh)** — every acceptance criterion in every approved
   spec names the test that proves it, and this checks that the test is real. Write a
   criterion you never tested, or quietly rename the test out from under it, and the
   session cannot end.

And one thing only nudges, **once**:

4. **Doc drift** — code moved, documentation didn't. This one is a judgement call: a pure
   refactor genuinely changes nothing worth documenting. So it asks, accepts the answer,
   and doesn't ask again (that's what `stop_hook_active` is for). Gates that cry wolf get
   switched off, and a switched-off gate protects nothing.

That split — facts get walls, judgement gets a nudge — is the design. Decide which of your
own rules is which before you start writing hooks.

## It cannot trap you

A gate with no way out is a broken gate. After five consecutive blocks `gates.sh` gives up
and says so, on the grounds that an agent which can't get to green in five tries is stuck,
and a human should look. Claude Code has its own backstop too.

A session that only read and explained code never triggers any of this. The gates cost
nothing until you write something.

## Turning it off

Delete the hook from `settings.json`, or run `/hooks` and switch it off. Because
`settings.json` is checked in, cloning this repo will prompt you to trust it before
anything runs — read the five scripts first. They are short, and you should never let a
stranger's repo run shell on your machine without looking.

## Slash commands

* [`/spec`](commands/spec.md) - draft a spec from the template, with EARS criteria, and stop for approval before writing any code.
* [`/gates`](commands/gates.md) - run every gate and fix what's red.
