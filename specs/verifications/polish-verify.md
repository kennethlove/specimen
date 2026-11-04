# Ad-hoc additions — post-release polish (PR #2)

Scope: README.md, CONVENTIONS.md, `--version` flag. No story ledger
(P3, not story work). Evidence captured from runs made 2026-11-03.

## Checks

- `cargo test` → 14 passed (3 suites), exit 0
- `cargo clippy --all-targets` → No issues found
- `cargo run -q -- --version` → `specimen 0.1.0`
- TDD gate `verify-tdd-red-commit.sh` → gate-exit=0,
  `PASS: test-only commit 9015cdf7 fails in isolation`
- Rebuilt chain tree byte-identical to previously gated GREEN:
  `git diff --stat 4b09f784 067b5f98` → empty, exit 0
- `validate-specs-yaml.sh specs` → OK

## Commits (PR #2 squash `04857faa`, merged)

- `chore: README, CONVENTIONS, cargo fmt` (5103176d)
- `test(cli): version flag (red)` (9015cdf7) — test-only, gate-verified
- `feat(cli): --version and -V flags` (067b5f98)

Note: first attempt swept docs+fmt into the RED commit (describe
takes whole workdir); chain was rebuilt from committed blobs so the
RED commit is genuinely test-only.
