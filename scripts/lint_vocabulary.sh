#!/usr/bin/env bash
# Fail if any DB_TERMS token appears in bion-soma/src.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VOCAB="$ROOT/VOCABULARY.md"
SOMA_SRC="$ROOT/crates/bion-soma/src"

in_db_section=0
while IFS= read -r line; do
  if [[ "$line" == "## DB_TERMS" ]]; then
    in_db_section=1
    continue
  fi
  if [[ "$line" == "## "* ]] && [[ "$line" != "## DB_TERMS" ]]; then
    in_db_section=0
    continue
  fi
  if [[ $in_db_section -eq 1 ]] && [[ -n "${line// }" ]]; then
    IFS=',' read -ra TERMS <<< "$line"
    for raw in "${TERMS[@]}"; do
      term="$(echo "$raw" | xargs)"
      [[ -z "$term" ]] && continue
      if grep -rq "$term" "$SOMA_SRC"; then
        echo "VOCABULARY VIOLATION: DB term '$term' found in bion-soma/src"
        grep -rn "$term" "$SOMA_SRC" || true
        exit 1
      fi
    done
  fi
done < "$VOCAB"
echo "lint_vocabulary: OK"
