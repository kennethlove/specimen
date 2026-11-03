# Security exceptions

## E-1: check strings execute arbitrary shell commands

- Finding: `REVIEW.md` #1
- Rationale: requirements state the injection risk is
  documented-not-solved. Spec files are trusted input — the same
  trust class as CI scripts. Executing claim checks is the tool's
  core purpose.
- Date: 2026-11-03
- Upgrade trigger: before accepting spec files from untrusted
  authors, bind checks to an allowlist or run them in a sandbox.
