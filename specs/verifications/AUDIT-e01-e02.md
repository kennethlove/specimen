# Audit code — e01 checker CLI + e02 conformance skill

Target: working tree (greenfield; whole repo is the diff).
Mode: full checklist, self-review by verify-work/audit-code.

## Checklist

### Supply chain and security

- [x] serde_json tagged `[OK]` in plan (verified 1.0.154, cargo search)
- [x] No `[SLOP]` packages; only dependency is serde_json
- [x] No secrets in diff (no sk-, ghp_, AKIA, .env values)
- [x] OWASP spot-check: one injection surface (`sh -c` in check) —
      accepted in `specs/security/EXCEPTIONS.md` E-1, rationale and
      upgrade trigger recorded; REVIEW.md has no blocking HIGH
- [x] No auth, user data, or network code

### Provenance and metadata

- [x] N/A: greenfield plan artefacts follow bigpowers templates
      (epic/task/story schema); no ADRs yet — decisions live in
      spec §19 Open questions

### Law of Demeter

- [x] N/A: free-function CLI, no object graphs

### CONVENTIONS.md compliance

- [x] Output files under `specs/` (nothing written to project root)
- [x] No `gh issue create`, no GitHub REST calls anywhere
- [!] `CONVENTIONS.md` absent — `seed-conventions` never run for this
      greenfield repo; audit verify command fails on this. Logged as
      gap, not waived silently

### Scope

- [x] Changes limited to e01/e02 stories; no extra refactors
- [x] No speculative features (`--version`, README deliberately not
      built — not required by any story)
- [x] Discovered defect this audit: duplicate `tests/cli_parse.rs`
      (4 tests duplicated into `tests/cli.rs`) — removed (Boy Scout)

### Boy Scout Rule

- [x] `tests/cli_parse.rs` deleted (dead duplication)
- [x] `run_one` extracted from `run_checks` (function length)
- [x] No commented-out code

### Types and safety

- [x] Rust, fully typed; no `unsafe`; no casts bypassing types

### Test coverage

- [x] Every behavior covered through the public CLI (13 tests,
      `cargo test` 0.14s)
- [x] Regression tests for both bugs found in TDD (blank-line
      fixtures, badmarker line number)
- [x] F.I.R.S.T: fast, independent (own fixtures, no shared state),
      self-checking (asserts), one assertion intent per test
- [!] `enforce-first` skill not loaded — reviewed manually instead

### SOLID and heuristics (Chapter 17)

- [x] SRP: `parse_claims` (parse), `run_one` (execute),
      `apply_markers` (markers), `run_checks` (orchestrate)
- [x] G5 DRY: duplication removed
- [x] G29 positive conditionals; G25 no magic numbers (127 is
      documented command-not-found code, asserted in test)

### Refactoring smells

- [x] None detected after `run_one` extraction; `parse_claims`
      (~35 lines) and `apply_markers` (~45 lines) exceed the 20-line
      guideline but are linear and cohesive — splitting would
      fragment one behavior each; revisit if logic grows

### Code style

- [x] Files well under 300 lines (main.rs 178, cli.rs 137)
- [x] Names specific; comments explain WHY (marker contract, error
      prefix convention)
- [x] Early returns over nesting (dup error, read error)

## Red flags rationalized

1. CONVENTIONS.md compliance — file does not exist (seed-conventions
   not run); did not fabricate one mid-audit.
2. enforce-first skill not loaded — tests reviewed manually against
   F.I.R.S.T instead of skipping coverage.
3. bp-churn-rank skipped — greenfield, every file is new; churn
   gives no signal.

## Verdict

All items pass except the two logged gaps (CONVENTIONS.md absent,
enforce-first not loaded). No blocking hygiene issues.
Next: `request-review` for an independent second opinion.
