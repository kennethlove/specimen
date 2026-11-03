# What to build (spec tooling)

Assumption: new repo — this one is content (blog/resume), tooling lives
elsewhere. Skills that accompany it: global (portable across repos).

Build order = value order. MVP is 1 + 2: that's where review volume drops.

## Build 1 — checker CLI (smallest code, single file, stdlib only)

One script, no deps, no framework:
- `parse <spec.md>` — regex over claim lines → JSON (id, sentence, check
  cmd, scope lines). Missing `check:` = warning (linter falls out of the
  parser for free).
- `check <spec.md>` — run each check command, print claim → pass/fail,
  exit non-zero on any failure.
- Diff hunk inventory: NOT ours — `git diff` already does it. CLI emits
  claims JSON; AI does the mapping.

## Build 2 — conformance skill (agent-side; the review-volume reducer)

Pi skill, not code:
1. Run `spec check` (or note claims needing run).
2. Read diff hunk inventory + claims JSON.
3. Map each hunk → claim ID (AI judgment — this is the mapping no
   deterministic tool can do).
4. Emit report: claim coverage, pass/fail, **orphan hunks** (flagged
   pre-human = lines that never reach review unaccounted).

Human reads report + [human] claims, not line-by-line diff.

## Build 3 — grill/authoring skill (spec-time shift-left)

User need: "authoring tools would help." Skill that takes rough idea →
- asks the senior-reviewer questions (ai-shift-left §1: null fields, callers)
- emits EARS + RFC 2119 claim lines with fit/check fields
- enforces tags; untagged prose allowed as context
- checks reference existing tests/commands; test generation is a later
  opt-in (red-test-first, §3), not baked in day one

## Build 4 — metrics (only when measurement starts)

Tier 1 = git/PR queries, ad hoc until baseline collection begins. No
dashboard, no tracker scripts before the experiment. Spec authoring time
gets logged by build 3 as a byproduct (timestamp first draft → approved).

## Not building

- Authoring UI / forms / schema server — humans edit text directly
- Step-definition glue or any DSL runtime — format is markdown
- Format server / registry — files in repos, that's the storage
- Separate linter tool — warning inside `parse`
- Measurement dashboard — `git log` until proven insufficient

## Risks / open

- Format stability: freeze nothing until first real spec exercises it;
  migration is one grep-script since it's all text.
- Check commands arbitrary shell = injection surface if specs come from
  untrusted sources. Trust boundary: specs live in the repo, same trust
  as CI scripts. Note, don't solve yet.
