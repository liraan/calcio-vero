#!/bin/sh
# Gates A and B - mechanical checks from NEW-SESSION-PROMPT.md.
# Exit 0 = green (all mechanical checks pass), 1 = red.
# Run from the repo root.
#
# Usage: scripts/gates.sh [startup|closing]
#   closing (default) - full set: A1, A2, A3, B1, B3, B6, B7 + manual B2
#   startup           - fast set: A1, A3, B1, B3, B6, B7 + manual B2
#                        (A2 runs cargo test: skipped, the startup already
#                         runs the tests in step 4; B2 is only meaningful
#                         after work has been done)
#
# Manual checks are NOT reported as "ok": they are listed as "manual" and must
# be done in the report. Reporting them as "ok" without checking would be a
# leak (D5).

set -u
MODE="${1:-closing}"
fail=0

say_ok()     { printf '  ok      %s\n' "$1"; }
say_fail()   { printf '  FAIL    %s\n' "$1"; fail=1; }
say_manual() { printf '  manual  %s\n' "$1"; }

case "$MODE" in
  startup|closing) : ;;
  *) echo "gates: unknown mode '$MODE' (expected startup or closing)" >&2; exit 2 ;;
esac

echo "Gates mode: $MODE"

# ---------- Gate A: documentation coherence ----------
echo "Gate A - documentation coherence"

a_structure=$(grep -cE '^\| 0[0-9][0-9] ' STRUCTURE.md 2>/dev/null || echo 0)
a_index=$(grep -cE '^\| \[0[0-9][0-9]\]' specs/_index.md 2>/dev/null || echo 0)
a_files=$(ls docs/adr/*.md 2>/dev/null | grep -v '000-template' | wc -l | tr -d ' ')
if [ "$a_structure" = "$a_index" ] && [ "$a_index" = "$a_files" ]; then
  say_ok "A1 ADR counts: STRUCTURE=$a_structure, _index=$a_index, files=$a_files"
else
  say_fail "A1 ADR counts diverge: STRUCTURE=$a_structure, _index=$a_index, files=$a_files"
fi

if [ "$MODE" = "closing" ]; then
  r_badge=$(grep -oE 'tests-[0-9]+_passing' README.md 2>/dev/null | grep -oE '[0-9]+' | head -1)
  r_real=$(cargo test --workspace 2>/dev/null | grep 'test result' | awk -F'[ ;]' '{p+=$4} END{print p+0}')
  if [ -n "$r_badge" ] && [ "$r_badge" = "$r_real" ]; then
    say_ok "A2 README badge ($r_badge) = real ($r_real)"
  else
    say_fail "A2 README badge (${r_badge:-missing}) != real ($r_real)"
  fi
else
  say_manual "A2 README badge vs real tests - skipped in startup (step 4 runs the tests)"
fi

a3=$(grep -nE '[0-9]+ (tests?|crates?|ADRs?)\b|(tests?|crates?|ADRs?) [0-9]+' docs/context/NEW-SESSION-PROMPT.md 2>/dev/null | head -5)
if [ -z "$a3" ]; then
  say_ok "A3 no hardcoded counts in NEW-SESSION-PROMPT.md"
else
  say_fail "A3 hardcoded counts found in NEW-SESSION-PROMPT.md:"
  printf '%s\n' "$a3"
fi

# ---------- Gate B: fresh-session simulation (mechanical subset) ----------
echo "Gate B - fresh-session simulation (mechanical subset)"

SS=docs/context/SESSION-STATE.md

first_line=$(awk -F'|' 'NF>=7 && $3 ~ /^[[:space:]]*[0-9]+[ab]?[[:space:]]*$/ {print; exit}' "$SS" 2>/dev/null)
if [ -z "$first_line" ]; then
  say_fail "B1 no open step found in the Plan"
else
  step=$(echo "$first_line" | awk -F'|' '{gsub(/^[[:space:]]+|[[:space:]]+$/,"",$3); print $3}')
  action=$(echo "$first_line" | awk -F'|' '{gsub(/^[[:space:]]+|[[:space:]]+$/,"",$6); print $6}')
  if [ -n "$action" ]; then
    say_ok "B1 first open step ($step) has a Prima azione"
  else
    say_fail "B1 first open step ($step) has an empty Prima azione"
  fi
fi

say_manual "B2 Plan vs Detail order - check in the report"

oi_count=$(grep -cE '^\| OI-[0-9]+ ' "$SS" 2>/dev/null || echo 0)
if [ "$oi_count" -gt 0 ]; then
  say_ok "B3 OI table present ($oi_count items)"
else
  say_fail "B3 no OI table in SESSION-STATE.md"
fi

if [ "$MODE" = "closing" ]; then
  say_manual "B4 Prime azioni match reality - check in the closing report"
fi

proposed=0
surfaced=0
for f in docs/adr/*.md; do
  s=$(sed -n '3p' "$f" 2>/dev/null)
  if echo "$s" | grep -q 'Status:\*\* proposed$'; then
    proposed=$((proposed + 1))
    id=$(basename "$f" | cut -d- -f1)
    if grep -q "ADR-$id" "$SS"; then surfaced=$((surfaced + 1)); fi
  fi
done
if [ "$proposed" -eq 0 ]; then
  say_ok "B6 no proposed ADR"
elif [ "$surfaced" -eq "$proposed" ]; then
  say_ok "B6 proposed ADRs surfaced ($surfaced/$proposed)"
else
  say_fail "B6 proposed ADRs surfaced ($surfaced/$proposed)"
fi

uncited=0
for r in docs/context/analysis/*.md; do
  [ -e "$r" ] || continue
  base=$(basename "$r")
  if ! grep -q "$base" "$SS"; then uncited=$((uncited + 1)); fi
done
if [ "$uncited" -eq 0 ]; then
  say_ok "B7 all reports in analysis/ are cited in SESSION-STATE.md"
else
  say_fail "B7 $uncited report(s) in analysis/ not cited in SESSION-STATE.md"
fi

echo ""
if [ "$fail" -eq 0 ]; then
  if [ "$MODE" = "closing" ]; then
    echo "gates: GREEN (closing; manual: B2 - do it in the closing report)"
  else
    echo "gates: GREEN (startup; manual: B2, A2 - A2 in step 4)"
  fi
  exit 0
else
  echo "gates: RED - fix before pushing"
  exit 1
fi
