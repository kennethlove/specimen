<!-- story: e03s01 -->

STORY KEY: e03s01
TITLE:     Grill and emit
TYPE:      Story
PARENT:    e03
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-03
MATURITY:  3
SIZE:      M

### 1. Business narrative [draft]

Code written before its spec gets reviewed three weeks later, at
full price: questions a senior reviewer would ask at spec time
(null fields, broken callers, missing failure modes) surface as
rework code and extra review rounds. The grill/authoring skill
shifts left. It takes a rough idea, asks the reviewer's questions
in one batch, and writes a conformance-testable spec — EARS +
RFC 2119 claim lines the shipped specimen parser already accepts —
before any code exists. A ten-line answer kills a hundred-line
diff.

The skill lives in this repository as agent-agnostic markdown, so
any coding agent can run it, and it writes the finished spec into
the repo where humans edit text directly.

### 2. Value statement [draft]

As a spec author, I want my rough idea grilled into claim lines
with checks, so that reviewer questions get answered before code
exists instead of as rework.

### 3. Actors and permissions [draft]

- Spec author (internal) — supplies the rough idea, answers the
  batched questions.
- Coding agent (system) — follows the skill, drafts claims, writes
  the spec file.
- Reviewer (internal) — later reads claims, not prose archaeology.

### 4. Trigger and preconditions [draft]

Trigger: an agent loads skills/grill/SKILL.md with a rough idea
for a feature or change.
Precondition: the specimen CLI builds (`cargo run -q -- parse`
runs); the repository is writable; the rough idea exists in some
form (a sentence is enough).

### 5. Main flow and business logic [draft]

1. The agent reads skills/grill/SKILL.md and takes the rough
   idea from the author.
2. It asks one batch of senior-reviewer questions: null/missing
   fields, callers and dependents that break, failure modes and
   exit behavior, minimal scope (smallest change satisfying the
   idea).
3. The author answers the batch in one pass.
4. The agent drafts claims: one atomic, falsifiable sentence per
   claim, EARS pattern (WHEN/IF/WHILE/UNLESS) with an RFC 2119
   verb (MUST/SHOULD/MAY), each bound to a `check:` command, plus
   `scope:` and `tags:` markers where they apply.
5. It writes the finished spec file into the repository under
   `specs/` (path confirmed with the author when it already
   exists).
6. It runs `specimen parse` on the written file and reports the
   result: claims emitted, warnings raised.

Interruption point: between steps 3 and 4 — answers may arrive
across turns; drafting waits for the batch to be complete.

### 6. Alternative flows and exceptions [draft]

6a. No rough idea is available — the skill asks for one sentence
    before any question is drafted; it never invents the idea.
6b. Answers are incomplete — unanswered questions are recorded
    under an open-questions section in the written spec, marked
    with owner; the draft still ships. Never block on perfect
    answers.
6c. The spec path already exists — the skill asks the author
    whether to overwrite or pick a new name; it never silently
    replaces a file.
6d. `specimen parse` reports warnings on the written file — the
    skill reports them verbatim and stops; the author fixes, not
    the skill guessing.

### 7. Interface elements [draft]

Context: new
Static elements: skill procedure, question batch template, claim
line format
Dynamic elements: questions, claim lines, spec file, parse result

### 8. Domain model [draft]

Entities touched: Claim, Check (e01), Spec (glossary).
Entities created: Rough idea (transient input), open question
(persisted in spec file when answers are incomplete).
Relationships: claims belong to one spec file; each claim binds
exactly one check.

### 9. Integrations and boundaries [draft]

- specimen CLI (perennial, direction: out) — `parse` validates the
  emitted file.
- Spec file in specs/ (perennial, direction: out) — written by the
  skill, edited by humans thereafter.
- Author answers (perennial, direction: in) — conversation only.
- private-content ideas/ (ethereal, direction: in) — source rules;
  copied into the skill, never read at runtime.

### 10. Background processes [draft]

Not applicable — one foreground conversation flow.

### 11. Notifications [draft]

Not applicable — output stays in the agent conversation.

### 12. Audit and logging [draft]

Audited entity: authoring session. Fields: rough idea accepted,
question batch sent, spec path written, parse result. Recorded in
the conversation transcript; first-draft timestamp appended to
specs/authoring-log.yaml (e03s02).

### 13. Solution variabilities [draft]

- Question batch content may grow; its categories are fixed
  (fields, callers, failure modes, scope).
- Claim format is the implemented parser format; if the format
  unfreezes, migration is one grep script.

### 14. Quality attributes *NFR* [draft]

- From rough idea to written spec file within one authoring
  session (NFR-1: it has to work — parse exits 0 on the result).
- Every emitted claim survives `specimen parse` without errors;
  warnings are reported, never suppressed.

### 15. Security and compliance *NFR* [draft]

- AuthN: none.
- AuthZ: repository write access; the skill writes only under
  specs/.
- Data class: none — no secrets, no user data.
- Controls: no shell execution in this story; checks are written,
  not run.

### 16. UX and accessibility *NFR* [draft]

- Not applicable — text flow in an agent conversation.
- i18n: English only.
- Wording uses canonical terms: claim, check, scope, tags, spec.

### 17. Acceptance criteria [draft]

```gherkin
Scenario: Rough idea becomes a spec file
  Given a one-sentence rough idea
  When  an agent follows the grill procedure
  Then  one batch of questions covering fields, callers,
    failure modes and scope is asked
  And   the author's answers become claim lines with
    `check:` commands in implemented format
  And   the spec file exists under specs/
  And   `specimen parse` on it exits 0

Scenario: No rough idea (6a)
  Given no rough idea has been supplied
  When  the agent starts the procedure
  Then  it asks for one sentence and stops

Scenario: Incomplete answers (6b)
  Given answers that leave questions open
  When  the agent writes the spec
  Then  open questions are recorded in the file with owner
  And   the spec file is still written

Scenario: Existing spec path (6c)
  Given a target path that already exists
  When  the agent would write the file
  Then  the author is asked overwrite-or-rename
  And   no file is replaced silently

Scenario: Parse warns on output (6d)
  Given a written spec that specimen parse warns about
  When  the agent runs the parse step
  Then  warnings are reported verbatim
  And   the skill does not edit the file to silence them
```

### 18. Out of scope [draft]

- Running checks (that is `specimen check`, e01s02).
- Check-existence verification and draft timestamps (e03s02).
- Test generation / red-test-first (later opt-in, A6).
- Any Rust code change — the parser already accepts the format.
- Authoring UI, forms, Gherkin files.

### 19. Open questions [draft]

- Overwrite confirmation wording (prompt style) — owner: kenneth,
  needed by: first real authoring session.
- Whether the question batch should include example answers —
  owner: kenneth, needed by: first real authoring session.

### 20. References [draft]

- docs/brainstorm/spec-tool-requirements.md (A1–A3, A6).
- private-content/ideas/ai-shift-left.md §1 (spec-time grilling).
- private-content/ideas/plain-text-spec.md (claim shape).
- specs/UBIQUITOUS_LANGUAGE_LATEST.md (Claim, Check, Scope line).
