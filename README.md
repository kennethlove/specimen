# specimen

Mechanical spec-conformance tooling: claims in, pass/fail out.

A spec is a markdown file. Each claim is one line with a shell check
bound to it. `specimen` parses those lines and runs the checks, so a
reviewer reads a conformance report instead of eyeballing every diff.

## Install

```sh
cargo install --path .
```

Or run from the repo: `cargo run -q -- <subcommand>`.

## Usage

```sh
specimen parse path/to/spec.md   # claims -> JSON
specimen check path/to/spec.md   # run each claim's check
specimen --version               # print version
```

`parse` prints a JSON array of claims:

```json
[
  {
    "id": "C1",
    "sentence": "The system SHALL record outcomes.",
    "check": "grep -q 'record outcomes' src/",
    "scope": ["api"],
    "tags": ["test"]
  }
]
```

- A claim without a `check:` field warns on stderr and still parses
  (exit 0).
- Duplicate claim ids are an error (exit 1).
- Missing file, unreadable input: exit 1. Bad usage: exit 2.

`check` runs each check with `sh -c`, in document order, and prints
`PASS`/`FAIL`/`SKIPPED` lines as JSON. Exit 0 when nothing failed,
1 when any check failed, 2 on usage or unparsable input.

## Claim format

```markdown
- [C1] The system SHALL record outcomes. check: grep -q record src/
scope: [api]
tags: [test]
```

- Claim line: `- [ID] sentence check: <shell command>`
- `scope:` / `tags:` markers attach to the previous claim; values are
  bracketed, comma-separated lists. Absent markers mean empty lists.
- Lines that are not claim lines are context prose and are skipped.

## Conformance skill

`skills/conformance/SKILL.md` is the agent-facing procedure: run the
checker, map `git diff` hunks to claim ids, flag orphan hunks before a
human reads them, and emit the coverage report.

## Development

See `CONVENTIONS.md`. Planning artifacts live in `specs/`; story
verification evidence is in `specs/verifications/`.
