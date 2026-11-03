<!-- story: e03s02 -->

STORY KEY: e03s02
TITLE:     Authoring hygiene
TYPE:      Story
PARENT:    e03
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-03
MATURITY:  3
SIZE:      S

### 1. Business narrative [draft]

A spec whose checks point at tests that do not exist is a claim
wearing a costume — it looks verifiable and is not. A spec with no
first-draft record cannot later be measured against review effort.
The hygiene step keeps the grill/authoring output honest: verify
each `check:` target exists (warn, never block), stamp the draft
into a greppable log, and mark untagged prose as context so
reviewers know what is a claim and what is conversation. All
advisory — the tool warns, the author decides, nothing gates.

### 2. Value statement [draft]

As a spec author, I want check targets verified and drafts
timestamped, so that every claim in the spec is plausibly
verifiable and authoring effort is measurable later.

### 3. Actors and permissions [draft]

- Spec author (internal) — reads warnings, fixes or accepts.
- Coding agent (system) — probes targets, appends the log.
- Reviewer (internal) — trusts the tag context split.

### 4. Trigger and preconditions [draft]

Trigger: the grill/authoring skill has just written a spec file
(e03s01 step 5).
Precondition: the spec file exists under `specs/`; the shell can
run `test -f` and `command -v`.

### 5. Main flow and business logic [draft]

1. For each claim, take the `check:` command target: a path-like
   token gets `test -f <path>`; a bare command name gets
   `command -v <name>`.
2. Report unresolvable targets as warnings — one line per claim
   with claim id and target. The claim stays in the file; the
   skill never edits or removes it.
3. Assert tag state: claims carry tags from the taxonomy, or their
   prose is marked `context` in the report. Untagged prose is
   never blocking.
4. Append the first-draft timestamp to `specs/authoring-log.yaml`:
   `- first_draft: <iso8601> spec: <path>`. Create the file with a
   one-line header if it does not exist.
5. Report the summary: claims checked, warnings raised, log
   appended.

Interruption point: after step 2 — the author may fix a target and
re-run; the log appends once per draft, not per attempt.

### 6. Alternative flows and exceptions [draft]

6a. A `check:` target is a path that does not exist — warn with
    claim id and path; emit the claim anyway.
6b. A `check:` target is a command not on PATH — warn the same
    way; emit the claim anyway.
6c. `specs/authoring-log.yaml` is missing — create it with a
    header comment, then append.
6d. The check target cannot be classified (neither path-like nor
    bare command) — warn that the target shape is unrecognized;
    never block the draft.

### 7. Interface elements [draft]

Context: new
Static elements: warning lines, log file, tag/context report
Dynamic elements: probe results, timestamp line, summary counts

### 8. Domain model [draft]

Entities touched: Claim, Check, tags (glossary).
Entities created: Authoring log entry (timestamp + spec path).
Relationships: log entry belongs to one spec file; warnings attach
to claim ids.

### 9. Integrations and boundaries [draft]

- Filesystem (perennial, direction: out) — `test -f`, `command -v`
  probes; authoring-log.yaml append.
- Spec file from e03s01 (perennial, direction: in) — read only,
  never modified by this story.
- Author (perennial, direction: in) — receives warnings.

### 10. Background processes [draft]

Not applicable — synchronous with authoring.

### 11. Notifications [draft]

Not applicable — warnings appear in the agent conversation.

### 12. Audit and logging [draft]

Audited entity: draft record. Fields: timestamp (ISO 8601), spec
path. Destination: specs/authoring-log.yaml. Build 4 reads this
file to correlate drafts with review cycles.

### 13. Solution variabilities [draft]

- Probe strategy may grow (e.g. URL liveness) behind the same
  warn-only contract.
- Log schema may add fields; `first_draft` and `spec` stay.

### 14. Quality attributes *NFR* [draft]

- Hygiene never blocks: zero paths where a warning stops the
  draft (NFR: reduce review work without adding gate friction).
- Log append is one line per draft — no duplicates per attempt.

### 15. Security and compliance *NFR* [draft]

- AuthN: none.
- AuthZ: repository write access; writes only
  `specs/authoring-log.yaml`.
- Data class: none — no secrets.
- Controls: probes run `test -f`/`command -v` only — no command
  execution from check text in this story.

### 16. UX and accessibility *NFR* [draft]

- Not applicable — text flow in an agent conversation.
- i18n: English only.
- Wording: warning lines state claim id, target, and remedy.

### 17. Acceptance criteria [draft]

```gherkin
Scenario: All targets resolve
  Given a spec whose check targets exist
  When  the hygiene step runs
  Then  zero warnings are reported
  And   one first_draft line is appended to
    specs/authoring-log.yaml

Scenario: Missing file target (6a)
  Given a claim whose check names a nonexistent path
  When  the hygiene step probes it
  Then  a warning names the claim id and path
  And   the claim remains in the spec file

Scenario: Unknown command target (6b)
  Given a claim whose check names a command not on PATH
  When  the hygiene step probes it
  Then  a warning names the claim id and command
  And   the claim remains in the spec file

Scenario: Log file absent (6c)
  Given specs/authoring-log.yaml does not exist
  When  the hygiene step appends
  Then  the file is created with a header
  And   the first_draft line follows

Scenario: Unclassifiable target (6d)
  Given a check target that is neither path-like nor a bare name
  When  the hygiene step probes it
  Then  an unrecognized-shape warning is reported
  And   the draft proceeds
```

### 18. Out of scope [draft]

- Running checks (e01s02 / `specimen check`).
- Question flow and spec emission (e03s01).
- Test generation (A6, later opt-in).
- Measuring review-effort deltas (build 4).
- Any Rust code change.

### 19. Open questions [draft]

- Whether the log records warning counts alongside the timestamp —
  owner: kenneth, needed by: first real authoring session.

### 20. References [draft]

- docs/brainstorm/spec-tool-requirements.md (A4, A5).
- private-content/ideas/conformance-testable-spec.md (tags,
  coverage).
- specs/UBIQUITOUS_LANGUAGE_LATEST.md (Claim, Check, Tag).
