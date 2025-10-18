#!/bin/bash
# Install Git Hooks for RuchyRuchy
# Automatic installation of pre-commit and commit-msg hooks

set -e

echo "🔧 Installing RuchyRuchy Git Hooks..."
echo "====================================="
echo ""

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ ERROR: Not in a git repository"
    echo "   Run this script from the project root"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Install pre-commit hook
if [ -f "scripts/pre-commit" ]; then
    echo "📋 Installing pre-commit hook..."
    cp scripts/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo "   ✅ pre-commit hook installed"
else
    echo "❌ ERROR: scripts/pre-commit not found"
    exit 1
fi

# Install commit-msg hook
if [ -f "scripts/commit-msg" ]; then
    echo "📋 Installing commit-msg hook..."
    cp scripts/commit-msg .git/hooks/commit-msg
    chmod +x .git/hooks/commit-msg
    echo "   ✅ commit-msg hook installed"
else
    echo "❌ ERROR: scripts/commit-msg not found"
    exit 1
fi

# Make validation script executable
if [ -f "scripts/validate-roadmap.sh" ]; then
    chmod +x scripts/validate-roadmap.sh
    echo "   ✅ roadmap validator ready"
fi

echo ""
echo "====================================="
echo "✅ Git hooks installed successfully!"
echo ""
echo "Hooks installed:"
echo "  • pre-commit:  Quality gates + ticket enforcement"
echo "  • commit-msg:  Ticket ID validation"
echo ""
echo "Quality Gates Enforced:"
echo "  ✓ Zero SATD tolerance"
echo "  ✓ Documentation synchronization"
echo "  ✓ Ruchy syntax validation"
echo "  ✓ Ruchy lint (A+ grade)"
echo "  ✓ PMAT TDG score (≥85)"
echo "  ✓ Roadmap structure validation"
echo "  ✓ Ticket ID in commit messages"
echo ""
echo "💡 Commit message format:"
echo "   BOOTSTRAP-001: Implement token types"
echo "   VALID-003: Add property testing framework"
echo ""
echo "📖 See roadmap.yaml for available tickets"
echo ""

exit 0
