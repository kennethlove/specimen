# MVP test plan — non-functional requirements

Source: user, 2026-11-03 — "NFRs: it has to work. it has to reduce
the amount of work to review."

These are the project's quality attributes. A P0 story's NFR gate
checks against this file.

## NFR-1: It has to work

Category: reliability. The CLI must parse claims, run checks, and
exit exactly per contract — on fixtures and on real spec files.

Verification (all runnable from repo root):

```sh
cargo test                          # exit-code contract,14 tests
cargo clippy --all-targets          # no warnings
cargo run -q -- parse tests/fixtures/smoke.md    # JSON, exit0
cargo run -q -- check tests/fixtures/passing.md  # exit0
cargo run -q -- check tests/fixtures/failing.md  # exit1
cargo run -q -- check tests/fixtures/nope.md     # exit2
cargo run -q -- --version            # prints version, exit0
```

Failure of any command above is a release blocker.

## NFR-2: Reduce the amount of work to review

Category: outcome/usability. A reviewer must be able to review a
change from the conformance report instead of reading the whole
diff: mechanical claims pass/fail without human reading, orphan
hunks and uncovered claims are flagged up front, human-only claims
are separated.

Verification (runnable):

```sh
bash tests/conformance.sh
# greps assert the skill covers: run check, PASS/FAIL/SKIPPED
# listing, git diff hunk inventory, claim-id mapping, orphan
# flagging, coverage % with both counts, [human] separation
```

Manual acceptance (not yet demonstrated): one real review cycle
where the reviewer reads the report, not the diff. This is the
scope success criterion; it stays open until the tool is used on a
real spec.
