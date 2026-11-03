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

## Report template

```text
Spec: <path>

<listing lines>

exit code: <code> (<meaning>)
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
