#!/bin/bash

# rust-hoex quality check script
# Runs formatting, linting, and testing across the workspace

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🦀 Running quality checks for rust-hoex...${NC}"

# Function to run a command and report results
run_check() {
    local name="$1"
    local cmd="$2"
    
    echo -e "${YELLOW}📋 $name...${NC}"
    if eval "$cmd"; then
        echo -e "${GREEN}✅ $name passed${NC}"
        return 0
    else
        echo -e "${RED}❌ $name failed${NC}"
        return 1
    fi
}

# Initialize error counter
errors=0

# Check formatting
if ! run_check "Format check" "cargo fmt --all -- --check"; then
    echo -e "${YELLOW}💡 Run 'cargo fmt --all' to fix formatting${NC}"
    ((errors++))
fi

# Run clippy (linter)
if ! run_check "Clippy (linter)" "cargo clippy --workspace --all-targets --all-features -- -D warnings"; then
    echo -e "${YELLOW}💡 Fix clippy warnings above${NC}"
    ((errors++))
fi

# Run tests
if ! run_check "Tests" "cargo test --workspace"; then
    echo -e "${YELLOW}💡 Fix test failures above${NC}"
    ((errors++))
fi

# Check documentation
if ! run_check "Documentation check" "cargo doc --workspace --no-deps"; then
    echo -e "${YELLOW}💡 Fix documentation issues above${NC}"
    ((errors++))
fi

# Security audit (if cargo-audit is installed)
if command -v cargo-audit &> /dev/null; then
    if ! run_check "Security audit" "cargo audit"; then
        echo -e "${YELLOW}💡 Review security advisories above${NC}"
        ((errors++))
    fi
else
    echo -e "${YELLOW}⚠️  cargo-audit not installed. Consider installing: cargo install cargo-audit${NC}"
fi

# Summary
echo
if [ $errors -eq 0 ]; then
    echo -e "${GREEN}🎉 All checks passed! Your code is ready.${NC}"
    exit 0
else
    echo -e "${RED}❌ $errors check(s) failed. Please fix the issues above.${NC}"
    exit 1
fi