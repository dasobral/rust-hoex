#!/bin/bash

# rust-hoex setup script
# Sets up the development environment for the Rust learning repository

set -e

echo "🦀 Setting up rust-hoex development environment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}❌ Rust is not installed!${NC}"
    echo "Please install Rust first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "Then restart your terminal and run this script again."
    exit 1
fi

echo -e "${GREEN}✅ Rust is installed: $(rustc --version)${NC}"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo is not available!${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Cargo is available: $(cargo --version)${NC}"

# Install/update Rust components
echo -e "${YELLOW}📦 Installing/updating Rust components...${NC}"
rustup component add clippy rustfmt rust-src

# Check for rust-analyzer
if ! command -v rust-analyzer &> /dev/null; then
    echo -e "${YELLOW}⚠️  rust-analyzer not found. Consider installing it for better IDE support:${NC}"
    echo "  - VS Code: Install the 'rust-analyzer' extension"
    echo "  - Vim/Neovim: Follow rust-analyzer setup instructions"
    echo "  - Other editors: Check rust-analyzer.github.io"
fi

# Create directory structure if it doesn't exist
echo -e "${YELLOW}📁 Creating directory structure...${NC}"
mkdir -p examples projects exercises benchmarks tests docs utils

# Make scripts executable
chmod +x utils/*.sh 2>/dev/null || true

# Check workspace
echo -e "${YELLOW}🔍 Checking workspace configuration...${NC}"
if cargo check --workspace &> /dev/null; then
    echo -e "${GREEN}✅ Workspace configuration is valid${NC}"
else
    echo -e "${YELLOW}⚠️  Some workspace members may not exist yet (this is normal for a new setup)${NC}"
fi

# Final recommendations
echo -e "${GREEN}🎉 Setup complete!${NC}"
echo
echo "Next steps:"
echo "1. Start with the first example: cd examples/01-helloWorld"
echo "2. Read the documentation in docs/"
echo "3. Run './utils/check.sh' to verify code quality"
echo "4. Consider setting up your editor with rust-analyzer"
echo
echo "Happy learning! 🦀"