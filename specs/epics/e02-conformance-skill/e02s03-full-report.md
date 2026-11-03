<!-- story: e02s03 -->

STORY KEY: e02s03
TITLE:     Full conformance report
TYPE:      Story
PARENT:    e02
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-03
MATURITY:  3
SIZE:      S

### 1. Business narrative [draft]

The MVP's success criterion is one real review cycle read through
the conformance report instead of the raw diff. This story assembles
the report: check outcomes (e02s01), hunk mapping (e02s02), claim
coverage, and the claims that only a human can verify. The report is
the artifact a reviewer opens first.

### 2. Value statement [draft]

As a reviewer, I want one conformance report covering checks,
coverage, orphans, and human claims, so that I can review the change
without reading the diff line by line.

### 3. Actors and permissions [draft]

- Coding agent (system) — assembles the report from tool outputs.
- Reviewer (internal) — reads the report as the review entry point.

### 4. Trigger and preconditions [draft]

Trigger: an agent runs the skill's report procedure after mapping.
Precondition: check results and hunk mapping exist for the review
range; the claims list is available.

### 5. Main flow and business logic [draft]

1. The agent gathers check results (pass/fail/skipped per claim).
2. It gathers hunk mapping records from the mapping procedure.
3. It computes coverage: claims with at least one mapped hunk over
   total claims, as a percentage with both counts.
4. It collects claims tagged `[human]` into their own section,
   separate from mechanical results.
5. It renders the report: coverage, check outcomes, orphan hunks
   flagged, human claims section.

Interruption point: between steps 2 and 5 — inputs can be stored
and the render resumed.

### 6. Alternative flows and exceptions [draft]

6a. Every claim has a mapped hunk — coverage reads 100% with
counts shown.
6b. Claims have no mapped hunk — each is listed as uncovered with
its id and sentence.
6c. Orphan hunks exist — each appears in the flagged section with
file and line range.
6d. Checks fail — failed claims appear in the check section with
their exit codes; the report states the run failed.

### 7. Interface elements [draft]

Context: new
Static elements: report section headings (fixed wording)
Dynamic elements: coverage numbers, status lines, orphan list

### 8. Domain model [draft]

Entities touched: Claim, CheckResult, HunkMapping.
Entities created: ConformanceReport (coverage, check section,
orphan section, human section).
Relationships: report aggregates the three existing entities.

### 9. Integrations and boundaries [draft]

- specimen CLI (perennial, direction: in) — check results.
- git (perennial, direction: in) — hunk inventory via mapping.
- Spec markdown file (perennial, direction: in) — claim sentences
  for uncovered and human sections.

### 10. Background processes [draft]

Not applicable — the report renders in the agent session.

### 11. Notifications [draft]

Not applicable — delivered as conversation output; the reviewer
may ask for it as a file later.

### 12. Audit and logging [draft]

Audited entity: report run. Fields: review range, coverage counts,
orphan count, failed check count. Printed as the report footer.

### 13. Solution variabilities [draft]

Not applicable — one report format, no configuration.

### 14. Quality attributes *NFR* [draft]

- Report renders within 10 s for a 40-hunk, 30-claim review.
- Coverage arithmetic equals mapped-claim count divided by total
  claim count; counts always displayed alongside the percentage.

### 15. Security and compliance *NFR* [draft]

- AuthN: none.
- AuthZ: repository read access.
- Data class: none.
- Controls: the procedure never edits the spec or the working
  tree.

### 16. UX and accessibility *NFR* [draft]

- Not applicable — markdown report in an agent conversation.
- i18n: English only.
- Section headings fixed and greppable: Coverage, Checks, Orphans,
  Human claims.

### 17. Acceptance criteria [draft]

```gherkin
Scenario: Full coverage, all checks pass
  Given a review where every claim has a mapped hunk and all
    checks pass
  When  an agent runs the skill's report procedure
  Then  coverage shows 100% with both counts
  And   the check section shows every claim as pass
  And   the orphan section states none

Scenario: Uncovered claims exist (6b)
  Given a review where two of ten claims have no mapped hunk
  When  an agent runs the skill's report procedure
  Then  coverage shows 80% with counts 8 of 10
  And   both uncovered claims are listed with id and sentence

Scenario: Orphan hunks present (6c)
  Given a mapping with one orphan record
  When  an agent runs the skill's report procedure
  Then  the orphan section lists the file and line range
  And   the footer orphan count is 1

Scenario: Failing checks (6d)
  Given check results where one claim failed with exit code 1
  When  an agent runs the skill's report procedure
  Then  the check section lists that claim as fail with exit 1
  And   the report states the check run failed

Scenario: Human claims are separated
  Given a claims list where one claim carries the human tag
  When  an agent runs the skill's report procedure
  Then  that claim appears only in the human claims section
  And   it is excluded from mechanical pass/fail lines
```

### 18. Out of scope [draft]

- Writing the report to a file or publishing it.
- Trending coverage across multiple runs.
- Auto-fixing failing checks or orphan hunks.

### 19. Open questions [draft]

- Whether the report is always conversation output or also saved
  under specs/ — owner: kenneth, needed by: first review cycle.

### 20. References [draft]

- specs/product/SCOPE_LATEST.yaml (success criterion 5: one review
  cycle read via report).
- specs/epics/e02-conformance-skill/e02s01-check-report.md.
- specs/epics/e02-conformance-skill/e02s02-hunk-mapping.md.
- specs/UBIQUITOUS_LANGUAGE_LATEST.md (Conformance report,
  Coverage).
