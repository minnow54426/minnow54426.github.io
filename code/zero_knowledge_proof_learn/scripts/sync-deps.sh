#!/bin/bash
# sync-deps.sh - Synchronize common dependencies across all weeks
#
# Usage: ./scripts/sync-deps.sh [--dry-run]
#
# This script updates common dependencies (anyhow, thiserror, serde, etc.)
# to consistent versions across all week projects while preserving
# week-specific dependencies.

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Common dependencies and their versions
declare -A COMMON_DEPS=(
    ["anyhow"]="1.0"
    ["thiserror"]="1.0"
    ["serde"]='{ version = "1.0", features = ["derive"] }'
    ["serde_json"]="1.0"
    ["bincode"]="1.3"
    ["hex"]="0.4"
)

# Dev dependencies
declare -A DEV_DEPS=(
    ["criterion"]="0.5"
)

# Dry run check
DRY_RUN=false
if [ "$1" == "--dry-run" ]; then
    DRY_RUN=true
    echo -e "${YELLOW}DRY RUN MODE - No changes will be made${NC}\n"
fi

echo -e "${BLUE}Syncing dependencies across all weeks...${NC}\n"

# Function to update dependencies in a Cargo.toml file
update_cargo_toml() {
    local cargo_file=$1
    local week_name=$2

    echo -e "${BLUE}Processing ${week_name}/Cargo.toml...${NC}"

    if [ "$DRY_RUN" = true ]; then
        echo "  [DRY RUN] Would update dependencies in ${cargo_file}"
        return
    fi

    # Backup original
    cp "$cargo_file" "${cargo_file}.bak"

    # Update common dependencies
    for dep in "${!COMMON_DEPS[@]}"; do
        version="${COMMON_DEPS[$dep]}"

        # Check if dependency exists
        if grep -q "^${dep} = " "$cargo_file"; then
            echo "  Updating ${dep} = ${version}"
            # Replace existing dependency
            sed -i.bak "s/^${dep} = .*/${dep} = ${version}/" "$cargo_file"
        else
            echo "  Adding ${dep} = ${version}"
            # Add to [dependencies] section
            sed -i.bak "/^\[dependencies\]/a ${dep} = ${version}" "$cargo_file"
        fi
    done

    # Update dev dependencies
    for dep in "${!DEV_DEPS[@]}"; do
        version="${DEV_DEPS[$dep]}"

        if grep -q "^${dep} = " "$cargo_file"; then
            if grep -A 100 "^\[dev-dependencies\]" "$cargo_file" | grep -q "^${dep} = "; then
                echo "  Updating dev ${dep} = ${version}"
                sed -i.bak "/^\[dev-dependencies\]/,/^\[/ s/^${dep} = .*/${dep} = ${version}/" "$cargo_file"
            fi
        else
            # Add to [dev-dependencies] section if it exists
            if grep -q "^\[dev-dependencies\]" "$cargo_file"; then
                echo "  Adding dev ${dep} = ${version}"
                sed -i.bak "/^\[dev-dependencies\]/a ${dep} = ${version}" "$cargo_file"
            fi
        fi
    done

    # Clean up backup files
    rm -f "${cargo_file}.bak"

    echo -e "${GREEN}✓ Updated${NC}\n"
}

# Find all week directories and update their Cargo.toml
for week_dir in week*/code; do
    if [ -d "$week_dir" ]; then
        cargo_file="${week_dir}/Cargo.toml"

        if [ -f "$cargo_file" ]; then
            week_name=$(basename $(dirname "$week_dir"))
            update_cargo_toml "$cargo_file" "$week_name"
        else
            echo -e "${YELLOW}Skipping ${week_dir} - no Cargo.toml found${NC}\n"
        fi
    fi
done

echo -e "${GREEN}==================================${NC}"
echo -e "${GREEN}Dependency sync complete!${NC}"
echo -e "${GREEN}==================================${NC}\n"

if [ "$DRY_RUN" = false ]; then
    echo "Next steps:"
    echo "1. Review changes: git diff"
    echo "2. Test that everything still works: for week in week*/code; do (cd \$week && cargo test); done"
    echo "3. Commit if all tests pass: git add . && git commit -m 'chore: sync dependencies'"
fi
