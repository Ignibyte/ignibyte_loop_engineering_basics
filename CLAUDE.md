# CLAUDE.md

This project keeps its agent instructions in the tool-neutral **[AGENTS.md](AGENTS.md)**,
so the same contract works for any coding agent. Claude Code reads this file — so this
file exists only to point you there.

**→ Read [AGENTS.md](AGENTS.md).**

The Claude-specific parts — the hooks that enforce the contract, and the `/spec` and
`/gates` commands — live in [`.claude/`](.claude/README.md). They will stop you editing
`src/` before you have read the architecture notes, and stop you ending a session while a
gate is red. That is deliberate.
