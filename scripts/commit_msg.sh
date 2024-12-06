#!/bin/sh

commit_msg_file="$1"
commit_msg=$(cat "$commit_msg_file")

conventional_commit_regex='^(feat|fix|docs|chore|style|refactor|perf|test|ci|revert|remove|build): .+'

if ! echo "$commit_msg" | grep -Pq "$conventional_commit_regex"; then
  echo "❌ Commit message does not follow the conventional format."
  echo "🔧 It should start with a valid type (e.g., 'feat', 'fix') followed by a colon and a message."
  echo "Example: 'feat: add login functionality'"
  exit 1
fi

echo "✅ Commit message follows the conventional commit format."