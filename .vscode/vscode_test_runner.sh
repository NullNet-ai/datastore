#!/bin/bash
# VS Code Rust Test Runner Wrapper
# This script ensures cargo is available in PATH for VS Code test runner

# Setup environment - preserve system PATH and add Rust toolchain
export PATH="$HOME/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"

# Add common Homebrew paths if they exist
if [ -d "/opt/homebrew/bin" ]; then
    export PATH="/opt/homebrew/bin:$PATH"
fi

if [ -d "/usr/local/bin" ]; then
    export PATH="/usr/local/bin:$PATH"
fi

# Set PROTOC if available
if [ -f "/opt/homebrew/bin/protoc" ]; then
    export PROTOC="/opt/homebrew/bin/protoc"
fi

# Execute the command
exec "$@"