# Conformance dashboard (spec)

Rough idea: a dashboard for the conformance tests in a project.

Context: specimen already produces per-claim pass/fail via
`specimen check`. Reviewers read reports as prose today; this
spec covers a static, human-facing view over those results.
Source of truth for format: `specs/planning-context.yaml` rules
(EARS + RFC 2119 claim lines, implemented parser format).

Answers recorded from the author batch: humans primary, agent
JSON interface post-MVP; fail loudly always; empty state says it
is empty plus a getting-started guide; metadata = pass/fail with
human claims highlighted; fields selectable via config; stale
data is a user error because refresh is one action; static HTML,
single repo, summary counts in MVP, each claim readable in full.
Config decisions: field selection lives at
`~/.config/specimen.toml`; getting-started guide text lives in
the HTML templates that generate the static site.

## Scope

scope: [dashboard]

- [C1] The dashboard MUST show the project name. check: cargo test dash_name
scope: [dashboard]
tags: [test]
- [C2] The dashboard MUST list every claim. check: cargo test dash_list
scope: [dashboard]
tags: [test]
- [C3] Each claim MUST show pass or fail. check: cargo test dash_status
scope: [dashboard]
tags: [test]
- [C4] Human claims MUST be highlighted. check: cargo test dash_human
scope: [dashboard]
tags: [test]
- [C5] When nothing has run, the dashboard MUST show an empty state with a getting-started guide. check: cargo test dash_empty
scope: [dashboard]
tags: [test]
- [C6] If the check run fails, the dashboard MUST show the project name, the error, and how to fix it. check: cargo test dash_err
scope: [dashboard]
tags: [test]
- [C7] The dashboard MUST be static HTML, no server. check: cargo test dash_static
scope: [dashboard]
tags: [test]
- [C8] The config file MUST select which fields are shown. check: cargo test dash_config
scope: [dashboard]
tags: [test]
- [C9] The dashboard MUST show summary counts. check: cargo test dash_counts
scope: [dashboard]
tags: [test]
- [C10] Each claim MUST be readable in full. check: cargo test dash_full
scope: [dashboard]
tags: [test]
- [C11] The refresh action MUST recompute all results. check: cargo test dash_refresh
scope: [dashboard]
tags: [test]

## Out of scope

- Agent JSON interface (post-MVP, agreed).
- Multi-project collection (later).
- Server or live process — C7 forbids it.
- File-level scope marker (one marker line for the whole file) —
  post-MVP, deferred by the author.

## Open questions

None — config path and guide-text location answered by the
author; see decisions above.
