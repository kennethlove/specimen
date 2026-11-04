# AUDIT-e04 — conformance dashboard

Self-review (audit-code) of `main..@` for epic e04.
Date: 2026-11-04. Verdict: PASS with two noted deferrals (below).

## Checklist

### Supply chain & security
- [x] slopcheck: new dep `toml 1.1.8` tagged `[OK]` in plan-work
  (e04s03 spec §15 — stdlib has no TOML reader); WHY comment at use
  site in `src/dashboard.rs`
- [x] No `[SLOP]` packages
- [x] No secrets in diff (fixtures/tests are plain markdown)
- [x] OWASP spot-check: HTML injection handled — `esc()` on all
  claim-derived text, regression test `dashboard_escapes_claim_text`;
  config field names matched against allowlist, never evaluated;
  `sh -c` check execution unchanged and covered by
  `specs/security/EXCEPTIONS.md` E-1
- [x] No new HIGH findings; `specs/security/REVIEW.md` stands

### Provenance & metadata
- [x] Story specs carry `TYPE:` / `MATURITY:` headers; task files
  carry `spec:`, `status:`, `risk:` fields
- [x] Impl decisions referenced by commit SHAs (gate output in
  TDD commits: 1d6f5f83/e4b0e02c, c74933f5/065bde23,
  9a23ea6f/fce9995a)

### Law of Demeter
- [x] Free functions over small inputs; no chained traversal

### CONVENTIONS compliance
- [x] Outputs under `specs/` (this file: `specs/verifications/`)
- [x] No `gh issue create`; `gh` used only for PR/clone flows
- [x] No direct `curl`/`fetch` against api.github.com in this diff
  (release flow earlier used `gh api` via the gh CLI — noted, not
  part of e04)

### Scope
- [x] Diff limited to e04 stories, capsule, dashboard code, tests
- [x] No speculative features; file-scope config explicitly
  post-MVP per author answer
- [x] No failing gates discovered (build/clippy/tests green)
- [x] Boy Scout: parser inline-marker fix landed only because
  e04s02 C4 requires it (tagged claim line must parse)

### Boy Scout Rule
- [x] No dead code (old monolithic `run()` removed in split)
- [x] No commented-out code

### Types and safety
- [x] Rust: no `unsafe`, no casts bypassing types

### Test coverage
- [x] Every new function has coverage via public CLI tests:
  `esc` (dedicated), `project_name`, `load_fields` (3 config
  tests), `header_name`, `failure_page`, `evaluate`, `page_html`
- [x] Bug fix (inline `tags:` on claim lines) covered by
  `dashboard_human_claim_labeled`
- [x] Tests exercise the binary through its public interface
- [x] F.I.R.S.T: suite runs in 0.11s, isolated temp dirs

### SOLID
- [x] SRP after split: `run` (flow), `evaluate` (checks),
  `page_html` (render), `load_fields` (config)
- [x] O/C: field set extended via `VALID_FIELDS` allowlist
- [x] No global mutable state

### Code style
- [x] Files < 300 lines (dashboard 248, main 208)
- [x] Names specific; early returns; positives in conditionals
- [x] Comments explain WHY (toml dep, C6 fail-loudly, structural
  fields)
- [ ] Functions 4–20 lines: `page_html`, `load_fields`, `run` sit
  at ~45 lines each — deferred (declarative `format!` templates and
  error-path returns; prior epics accepted `run_checks` at similar
  length)
- [ ] DRY across test files: `run()` helper duplicated in 4
  integration files — deferred (pre-existing pattern from e01–e02;
  extraction to `tests/common/` is test-infra churn)

## Red flags named

- Churn ranking (`bp-churn-rank.sh`) skipped: script resolves its
  repo root from its own package location (known quirk) — would rank
  bigpowers, not this repo.
- Rationalization caught: accepting `run()` length because "templates
  are declarative" — partially true; the two deferrals above are
  recorded rather than silently passed.

## Grep sanity

- `grep -c 'unsafe' src/` → 0
- `grep -rn 'gh issue\|api.github.com' src/ tests/` → no matches
