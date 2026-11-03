# Security review — specimen MVP

Reviewed: working tree (greenfield — whole tree is the diff).
Scope: `src/main.rs`, `skills/conformance/SKILL.md`, `tests/`.

## Findings

| # | Severity | Confidence | Finding | Status |
|---|----------|------------|---------|--------|
| 1 | HIGH | 10 | `specimen check` runs each claim's `check:` string verbatim via `sh -c`, so a hostile spec file executes arbitrary commands | Accepted — see `EXCEPTIONS.md` E-1 |
| 2 | LOW | 8 | `parse`/`check` read any filesystem path the invoker names | Accepted — CLI runs with the invoker's privileges, same trust as `cat` |

No blocking HIGH findings beyond documented exceptions. No secrets,
no network code, no unsafe Rust, no deserialization of untrusted input
(serde_json parses only self-emitted output in tests).
