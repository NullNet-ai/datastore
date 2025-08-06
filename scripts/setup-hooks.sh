#!/bin/bash

# Setup script for Git hooks

echo "Setting up Git hooks..."

# Get the repository root and scripts directory
REPO_ROOT="$(git rev-parse --show-toplevel)"
SCRIPTS_DIR="$REPO_ROOT/scripts"
HOOKS_DIR="$REPO_ROOT/.git/hooks"

# Ensure hooks directory exists
if [ ! -d "$HOOKS_DIR" ]; then
    echo "❌ Error: .git/hooks directory not found. Are you in a Git repository?"
    exit 1
fi

# Copy and make executable the pre-push hook
if [ -f "$SCRIPTS_DIR/pre-push" ]; then
    cp "$SCRIPTS_DIR/pre-push" "$HOOKS_DIR/pre-push"
    chmod +x "$HOOKS_DIR/pre-push"
    echo "✅ Pre-push hook installed successfully!"
    echo "   The hook will now run cargo fmt --check before every push."
else
    echo "❌ Error: pre-push hook script not found at $SCRIPTS_DIR/pre-push"
    exit 1
fi

# Copy and make executable the post-checkout hook
if [ -f "$SCRIPTS_DIR/post-checkout" ]; then
    cp "$SCRIPTS_DIR/post-checkout" "$HOOKS_DIR/post-checkout"
    chmod +x "$HOOKS_DIR/post-checkout"
    echo "✅ Post-checkout hook installed successfully!"
    echo "   This hook will automatically set up hooks for new developers."
else
    echo "⚠️  Warning: post-checkout hook script not found at $SCRIPTS_DIR/post-checkout"
fi

echo "🎉 Git hooks setup complete!"