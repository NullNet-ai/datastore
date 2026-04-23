#!/bin/bash

# Setup Git hooks using core.hooksPath (hooks are versioned in scripts/, no copying)
# Run automatically via post-checkout, or manually: make setup-hooks

set -e

REPO_ROOT="$(git rev-parse --show-toplevel)"
SCRIPTS_DIR="$REPO_ROOT/scripts"

if [ ! -d "$SCRIPTS_DIR" ]; then
    echo "❌ Error: scripts directory not found at $SCRIPTS_DIR"
    exit 1
fi

# Use absolute path for reliability across checkouts
git config core.hooksPath "$SCRIPTS_DIR"

# Ensure hook scripts are executable
for hook in pre-push post-checkout post-merge; do
    if [ -f "$SCRIPTS_DIR/$hook" ]; then
        chmod +x "$SCRIPTS_DIR/$hook"
    fi
done

echo "✅ Git hooks configured (core.hooksPath = scripts/)"
echo "   Hooks run on: push (tests + format), checkout (keeps hooks active)"
