#!/bin/bash

# Cargo audit script with maintainable ignore list
# This script reads ignored advisories from .cargo/audit-ignore.txt

set -euo pipefail

# Install cargo-audit if not present
if ! command -v cargo-audit &> /dev/null; then
    echo "Installing cargo-audit..."
    cargo install cargo-audit --quiet
fi

# Build ignore arguments
IGNORE_ARGS=""
IGNORE_FILE=".cargo/audit-ignore.txt"

if [ -f "$IGNORE_FILE" ]; then
    while IFS= read -r line; do
        # Skip empty lines and comments, extract advisory ID
        if [[ -n "$line" && ! "$line" =~ ^[[:space:]]*# ]]; then
            # Extract the advisory ID (first word)
            advisory_id=$(echo "$line" | awk '{print $1}')
            if [[ -n "$advisory_id" ]]; then
                IGNORE_ARGS="$IGNORE_ARGS --ignore $advisory_id"
            fi
        fi
    done < "$IGNORE_FILE"
fi

# Run cargo audit with ignore arguments
echo "Running cargo audit with ignore list..."
if [ -n "$IGNORE_ARGS" ]; then
    echo "Ignoring advisories: $IGNORE_ARGS"
    cargo audit --deny warnings $IGNORE_ARGS
else
    cargo audit --deny warnings
fi
