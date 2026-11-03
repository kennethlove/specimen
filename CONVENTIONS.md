# Conventions

Rules for humans and agents working in this repo.

## Stack

- Rust 2024, edition pinned in `Cargo.toml`.
- One dependency: `serde_json`. New dependencies need justification
  in a commit comment (`WHY:` prefix).
- No build scripts, no workspaces, single binary `specimen`.

## Claim format

- Claim line: `- [ID] sentence check: <shell command>`.
- ID format is `C<number>`; ids unique per file (parser enforces).
- `scope:` / `tags:` markers follow the claim they belong to, values
  in brackets, comma-separated.
- Check strings run via `sh -c`; spec files are trusted input
  (see `specs/security/EXCEPTIONS.md` E-1).

## Tests

- Integration tests only, in `tests/`, exercising the public CLI
  (`env!("CARGO_BIN_EXE_specimen")`). No unit tests inside
  `src/main.rs`.
- Fixtures live in `tests/fixtures/` and are intentionally
  malformed (missing checks, duplicates, bad markers) — never
  "fix" them.
- `tests/conformance.sh` greps skill markdown; it fails when the
  `skills/conformance/SKILL.md` content drifts.

## Verify commands

```sh
cargo test                      # 13 tests, must pass
cargo clippy --all-targets      # no warnings
cargo tarpaulin                 # coverage gate: >= 80%
bash tests/conformance.sh       # skill content greps
```

## Commits

- Conventional Commits: `feat:` / `fix:` / `test:` / `chore:` /
  `refactor:` with a scope (`parse`, `check`, `conformance`, …).
- No `Co-authored-by` or AI-attribution footers.
- jj is the primary VCS. After `jj describe`, sync git with
  `git update-ref HEAD <sha> && git reset -q`. Never `git commit`
  alongside jj (creates parallel objects).

## Branching

- No direct pushes to `main` — the pre-tool-use hook blocks them.
  Work on a bookmark, open a PR, and stop. **The human merges** —
  the agent never runs `gh pr merge`.
- Branch name: `specimen-<topic>`.

## Specs layout

- `specs/epics/` — epic capsules (archived ones in `epics/archive/`)
- `specs/verifications/` — per-story verify bundles + audit reports
- `specs/security/` — REVIEW.md + EXCEPTIONS.md
- `specs/state.yaml` — active flow, handoff, next skill
- Keep YAML/markdown lines <= 80 columns.

## Skills

- Agent-facing procedures live in `skills/<name>/SKILL.md` in this
  repo — portable with the code, not installed agent-wide.
