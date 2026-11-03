---
name: conformance
description: >
  Run a spec's checks and render a pass/fail listing. Use when a reviewer
  asks whether a spec's claims still pass, or when told to run the
  conformance check.
---

# Conformance check report

Run a spec's checks and render a pass/fail listing for the reviewer.

## When to use

Use this skill when a reviewer asks whether the claims in a spec still
pass, or when asked to produce a conformance report.

## Procedure

1. Get the spec path from the reviewer. If no path is available, ask
   the reviewer for a path. Do not run against a guessed path.
2. Build the checker once:

   ```sh
   cargo build
   ```

   If the build fails, report the build error verbatim and stop.
3. Run the checker:

   ```sh
   specimen check <path>
   ```

   From the repo root the equivalent is
   `cargo run -q -- check <path>`.
4. Read the JSON results on stdout and render one listing line per
   claim:

   ```text
   PASS <id> <sentence>
   FAIL <id> <sentence> — check: <check string>
   SKIPPED <id> <sentence> — no check
   ```
5. Note the exit code under the listing: `0` = every check passed,
   `1` = at least one check failed, `2` = input error (report the
   stderr message verbatim).

## Hunk mapping

1. Inventory the diff:

   ```sh
   git diff <range>
   ```

   The default range is the working tree against `HEAD`. Only read:
   never commit, stage, or rewrite anything.
2. For every hunk, judge which claim it serves and record the
   claim ID beside it. Map each hunk to a claim ID by reading the
   claim sentences; use only IDs from the claims JSON, never
   invented ones.
3. A hunk that serves no claim is an orphan. Flag every orphan
   hunk before any human reads the report: list it under an
   `ORPHAN` section with its file and line range.
4. Empty diff: report `no changes` and stop.
5. Multiple candidate claims for one hunk: list all candidates and
   mark the hunk for human decision. Never silently pick one.

## Report template

```text
Spec: <path>

<listing lines>

exit code: <code> (<meaning>)
```

## Full conformance report

Assemble this report from the check listing and the hunk mapping.
The four sections have fixed, greppable names, in this order:

1. `Coverage` — mapped hunks over total claims as a percentage
   with both counts, e.g. `coverage: 80% (8 of 10)`. Under it,
   list every uncovered claim with its id and sentence.
2. `Checks` — the pass/fail listing, with the exit code noted when
   the run failed.
3. `Orphans` — each orphan hunk with file and line range, plus a
   footer orphan count; state `none` when there are none.
4. `Human claims` — every claim tagged `[human]`, listed only in
   this section and excluded from the mechanical pass/fail lines.
   State the human claim count.

Example:

```text
Coverage
coverage: 80% (8 of 10)
uncovered: C7 The system SHALL log failures.

Checks
exit code: 1 (at least one check failed)
PASS C1 The system SHALL record outcomes.
FAIL C4 The system SHALL retry once. — check: false

Orphans
none
orphan count: 0

Human claims
[C3] The team reviews the release notes.
human claim count: 1
```

## Branches

- A check fails (6a): keep the FAIL line with its check string and
  note exit code 1. Other claims keep their own status lines.
- A claim has no check (6b): render SKIPPED for that claim; other
  claims still show their own status.
- CLI will not build (6c): run `cargo build`, report the error
  verbatim, and stop.
- Spec path missing (6d): ask the reviewer for a path; run nothing
  against a guessed path.
