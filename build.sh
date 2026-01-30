#!/bin/bash
set -e

# devkit Build Script
# Builds and tests the entire project

echo "🚀 Building devkit..."
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Parse arguments
BUILD_TYPE="${1:-debug}"
RUN_TESTS="${2:-yes}"

echo -e "${BLUE}Build Configuration:${NC}"
echo "  Type: $BUILD_TYPE"
echo "  Tests: $RUN_TESTS"
echo ""

# Clean build (optional)
if [[ "$3" == "--clean" ]]; then
    echo -e "${YELLOW}Cleaning build artifacts...${NC}"
    cargo clean
    echo ""
fi

# Build
echo -e "${BLUE}Building workspace...${NC}"
if [[ "$BUILD_TYPE" == "release" ]]; then
    cargo build --release --workspace
    BINARY_PATH="target/release/devkit"
else
    cargo build --workspace
    BINARY_PATH="target/debug/devkit"
fi

echo ""
echo -e "${GREEN}✓ Build complete${NC}"
echo ""

# Run tests
if [[ "$RUN_TESTS" == "yes" ]]; then
    echo -e "${BLUE}Running tests...${NC}"
    cargo test --workspace -- --nocapture
    echo ""
    echo -e "${GREEN}✓ Tests passed${NC}"
    echo ""
fi

# Show binary info
if [[ -f "$BINARY_PATH" ]]; then
    BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
    echo -e "${BLUE}Binary Information:${NC}"
    echo "  Location: $BINARY_PATH"
    echo "  Size: $BINARY_SIZE"
    echo ""

    # Show version
    echo -e "${BLUE}Version:${NC}"
    "$BINARY_PATH" --version || echo "  (version command not implemented)"
    echo ""
fi

# Summary
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ devkit build complete!${NC}"
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo ""
echo "Next steps:"
echo "  1. Run:    $BINARY_PATH"
echo "  2. Test:   $BINARY_PATH --help"
echo "  3. Install: sudo cp $BINARY_PATH /usr/local/bin/"
echo ""
