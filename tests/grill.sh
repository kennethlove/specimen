#!/bin/sh
set -e
test -f skills/grill/SKILL.md
grep -q 'callers' skills/grill/SKILL.md
grep -q 'check:' skills/grill/SKILL.md
grep -q 'scope:' skills/grill/SKILL.md
grep -q 'specs/' skills/grill/SKILL.md
echo 'grill e03s01 verifies: ok'

grep -q 'warn' skills/grill/SKILL.md
grep -q 'test -f' skills/grill/SKILL.md
grep -q 'authoring-log' skills/grill/SKILL.md
grep -q 'context' skills/grill/SKILL.md
echo 'grill e03s02 verifies: ok'
