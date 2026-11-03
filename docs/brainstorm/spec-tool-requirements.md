# Spec tool — requirements + recommended tooling

Source: `docs/brainstorm/spec-tool.md`. Repo reality: **specimen = the tooling
repo** (Rust 2024, fresh hello-world). Source doc's "this one is content"
assumption refers to private-content. All builds live here: build 1 code plus
builds 2/3 as skills **in this repo's directory** — agent-agnostic markdown,
readable by Pi, Claude Code, or any agent. No Pi-only mechanism.

## Scope

In: builds 1–4. MVP = 1 + 2 (where review volume drops).

Out (source doc "Not building"): authoring UI/forms/schema server, step-def
glue or DSL runtime, format server/registry, separate linter tool, measurement
dashboard.

## Requirements

### Build 1 — checker CLI

- **R1** `specimen parse <spec.md>` emits JSON per claim: `id`, `sentence`,
  `check`, `scope[]`, `tags[]`. Missing `check:` = warning on stderr, claim
  still emitted, exit 0 (linter rides on parse for free).
- **R1a** `tags[]` added beyond source doc's field list: build 2's report must
  single out `[human]` claims — parser extracts tags when present, else `[]`.
- **R2** `specimen check <spec.md>` runs each claim's check via `sh -c`,
  prints claim → pass/fail (failed checks print captured output), exits
  non-zero iff any check fails.
- **R3** Checks run sequentially, output in document order, deterministic.
- **R4** No diff/hunk logic — `git diff` owns inventory; consumers do the
  hunk → claim mapping (AI judgment, build 2).
- **R5** Single file, no framework. Dependency budget: see below.

### Build 2 — conformance skill (project skill, agent-agnostic)

- **S1** Run `specimen check` (or note claims needing a run).
- **S2** Read claims JSON + diff hunk inventory (`git diff`).
- **S3** Map each hunk → claim ID (AI judgment — no deterministic tool does
  this).
- **S4** Emit report: claim coverage, pass/fail, **orphan hunks** flagged
  pre-human, `[human]` claims listed separately.
- Success: reviewer reads report + `[human]` claims, not line-by-line diff.

### Build 3 — grill/authoring skill (project skill, agent-agnostic)

- **A1** Rough idea → senior-reviewer questions (null fields, callers;
  ai-shift-left §1).
- **A2** Emit EARS + RFC 2119 claim lines with fit/check fields.
- **A3** Enforce tags; untagged prose allowed as context, never blocking.
- **A4** Verify referenced checks point at existing tests/commands.
- **A5** Record draft timestamp as byproduct (input to build 4).
- **A6** No test generation day one — red-test-first (§3) is a later opt-in.

### Build 4 — metrics (deferred)

- **M1** Tier 1 = ad hoc `git log` / `gh` queries until baseline collection
  starts.
- **M2** No dashboard, no tracker scripts before the experiment.

## Cross-cutting

- Plain markdown, line conventions only. Human edit = text edit; no tool
  required to use a spec. Anti-lock-in: tool gates nothing.
- Format frozen only after first real spec exercises it; migration = one
  grep script (all text).
- Trust boundary: `check:` is arbitrary shell. Specs live in repo = same
  trust as CI scripts. Noted, not solved.
- Skills live in this repo, agent-agnostic markdown — supersedes source doc's
  "skills: global" line.
- Claim-format knowledge (EARS patterns, tag taxonomy, fit criteria) is
  written in private-content `ideas/` (plain-text-spec,
  conformance-testable-spec, ai-shift-left, existing-spec-standards). Skills
  must be self-contained: copy the rules in, reference the source.

## Recommended tools / libraries

### Build 1 — Rust

| Need | Choice | Rejected / why |
|---|---|---|
| Args | `std::env::args`, hand-rolled | clap — dep + build cost for 2 subcommands |
| Claim-line parse | `str::lines` + `starts_with` / `split_once` | `regex` crate — conventions are prefix lines (`check:`, `scope:`, `[tag]`); add only if a real spec needs more (WHY comment at that point) |
| JSON output | `serde_json` — the one permitted dep | hand-rolled escaper: control-char/unicode escaping ships silent bugs; serde_json is the ecosystem's stdlib |
| Run checks | `std::process::Command`, `sh -c`, sequential | async/rayon — check counts small; concurrency last |
| Tests | `cargo test` + 2–3 fixture spec files | no test-framework dep |

### Build 2 / 3 — skills

Markdown skill files in this repo (e.g. `skills/<name>/SKILL.md`), plain
frontmatter + instructions any agent harness can load. No libraries. Hunk
inventory via `git diff` CLI. Report = markdown to stdout/file.

### Build 4 — later

`git` + `gh` (both installed), ad hoc shell. Nothing else until measurement
starts.

## Open

- Tag placement in claim lines (line-prefix vs inline) — unfrozen, exercised
  by first real spec.
- Binary/subcommand shape assumed `specimen parse|check`; rename cost is one
  grep.
