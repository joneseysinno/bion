#!/usr/bin/env bash
# Fail if mod.rs (or mod files) define functions — only re-exports allowed.
set -euo pipefail
TARGET="${1:?usage: lint_single_function.sh <src-dir>}"
violations=0
while IFS= read -r -d '' file; do
  rel="${file#"$TARGET"/}"
  if grep -E '^\s*(pub\s+)?(async\s+)?fn\s' "$file" >/dev/null 2>&1; then
    echo "SINGLE-FN VIOLATION: $rel defines a function (move to its own file)"
    violations=1
  fi
done < <(find "$TARGET" -name 'mod.rs' -print0)
if [[ $violations -ne 0 ]]; then
  exit 1
fi
echo "lint_single_function ($TARGET): OK"
