<!-- story: e02s02 -->

STORY KEY: e02s02
TITLE:     Hunk mapping
TYPE:      Story
PARENT:    e02
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-03
MATURITY:  3
SIZE:      M

### 1. Business narrative [draft]

A diff shows what changed, not which stated requirement the change
touches. Reviewers currently read the whole diff to find out. The
conformance skill instead inventories the diff hunks and maps each
hunk to the claim ID it serves, using the agent's reading of the
spec. Hunks that serve no claim are flagged as orphans before a
human reads anything — that flag is the whole point: mechanical
pre-filtering of review volume.

The git tool owns diff extraction; the skill owns the mapping
procedure; the agent supplies the judgment.

### 2. Value statement [draft]

As a reviewer, I want each diff hunk mapped to a claim ID before I
read the diff, so that I only review changes with stated purpose.

### 3. Actors and permissions [draft]

- Coding agent (system) — runs git commands, reads the spec,
  assigns claim IDs.
- Reviewer (internal) — receives the mapping and orphan flags.

### 4. Trigger and preconditions [draft]

Trigger: an agent runs the skill's mapping procedure for the
current change.
Precondition: git history exists; a claims JSON (e01) for the
target spec is available; the working tree or range to review is
identified.

### 5. Main flow and business logic [draft]

1. The agent runs `git diff` for the review range (working tree by
   default, range when given) and inventories hunks: file path,
   hunk header line range, added lines.
2. It loads the claim list (id and sentence) from the checker.
3. For each hunk it reads the surrounding code and the claim
   sentences, then assigns the claim ID the hunk serves.
4. It emits one mapping record per hunk: file, line range, claim
   id (or `ORPHAN`), and a one-line rationale.
5. It lists `ORPHAN` records under a flagged section for human
   review.

Interruption point: between steps 1 and 3 — the hunk inventory can
be stored and the mapping resumed hunk by hunk.

### 6. Alternative flows and exceptions [draft]

6a. A hunk serves a claim — record carries that claim id and the
mapping continues.
6b. No claim covers a hunk — record carries `ORPHAN`, listed in
the flagged section for human review; mapping continues.
6c. The diff is empty — the procedure reports "no changes" and
produces no records.
6d. A hunk plausibly serves several claims — record lists all
candidate ids and is marked for human decision instead of a
silent pick.

### 7. Interface elements [draft]

Context: new
Static elements: mapping record format, orphan section heading
Dynamic elements: hunk inventory, candidate id lists

### 8. Domain model [draft]

Entities touched: Claim (e01), Hunk (diff fragment).
Entities created: HunkMapping (file, line_range, claim_id,
rationale, candidates).
Relationships: HunkMapping points to zero or one claim; orphan
records point to none.

### 9. Integrations and boundaries [draft]

- git (perennial, direction: in) — `git diff` inventories hunks.
- specimen CLI (perennial, direction: in) — claims JSON supplies
  ids and sentences.
- Spec markdown file (perennial, direction: in) — read for claim
  sentences when judging fit.

### 10. Background processes [draft]

Not applicable — all commands run inside the agent session.

### 11. Notifications [draft]

Not applicable — the mapping is delivered in the conversation.

### 12. Audit and logging [draft]

Audited entity: mapping run. Fields: review range, hunk count,
orphan count, candidate-marked count. Reported at the end of the
procedure.

### 13. Solution variabilities [draft]

- Review range (config): working tree default; explicit range when
  the reviewer supplies one. Behaviour: same procedure over
  different hunk inventories.

### 14. Quality attributes *NFR* [draft]

- Every hunk in the inventory receives exactly one record; hunk
  count equals record count.
- Procedure completes within 60 s for a 40-hunk diff.

### 15. Security and compliance *NFR* [draft]

- AuthN: none.
- AuthZ: repository read access; git read commands only.
- Data class: none.
- Controls: the procedure never commits, stages, or rewrites git
  state.

### 16. UX and accessibility *NFR* [draft]

- Not applicable — text output in an agent conversation.
- i18n: English only.
- Orphan section heading is fixed wording so it is greppable.

### 17. Acceptance criteria [draft]

```gherkin
Scenario: Hunk maps to a claim
  Given a diff with one hunk and a claims list containing the
    claim the hunk implements
  When  an agent runs the skill's mapping procedure
  Then  one record exists for that hunk with the claim id
  And   a one-line rationale is present
  And   the orphan section is empty

Scenario: Orphan hunk (6b)
  Given a diff with one hunk that serves no claim
  When  an agent runs the skill's mapping procedure
  Then  the record carries ORPHAN
  And   the hunk appears under the flagged orphan section

Scenario: Empty diff (6c)
  Given a working tree with no changes
  When  an agent runs the skill's mapping procedure
  Then  the procedure reports no changes
  And   no records are produced

Scenario: Several candidate claims (6d)
  Given a diff hunk that plausibly serves two claims
  When  an agent runs the skill's mapping procedure
  Then  the record lists both candidate ids
  And   the record is marked for human decision

Scenario: Inventory completeness
  Given a diff containing five hunks across three files
  When  an agent runs the skill's mapping procedure
  Then  exactly five records exist, one per hunk
```

### 18. Out of scope [draft]

- Judging whether the code change is correct (review stays human).
- Mapping claims with no hunk (coverage belongs to e02s03).
- Auto-assigning claim ids by filename or keyword heuristics.

### 19. Open questions [draft]

- Default review range for uncommitted work vs branch range —
  owner: kenneth, needed by: first review cycle.

### 20. References [draft]

- docs/brainstorm/spec-tool-requirements.md (hunk mapping rule).
- specs/UBIQUITOUS_LANGUAGE_LATEST.md (Hunk, Orphan hunk).
- specs/epics/e01-checker-cli/e01s01-parse-claims.md (claim ids).
