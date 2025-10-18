#!/bin/bash
# Validate roadmap.yaml structure

set -e

ROADMAP="roadmap.yaml"

if [ ! -f "$ROADMAP" ]; then
    echo "❌ ERROR: roadmap.yaml not found"
    exit 1
fi

echo "🔍 Validating roadmap.yaml structure..."

# Check for required sections
REQUIRED_SECTIONS=("meta" "sprints" "validation" "execution_protocol" "metrics_tracking")

for section in "${REQUIRED_SECTIONS[@]}"; do
    if ! grep -q "^$section:" "$ROADMAP"; then
        echo "❌ Missing required section: $section"
        exit 1
    fi
done

# Check for valid ticket ID format
if ! grep -E "id: (INFRA|VALID|BOOTSTRAP|PROP|FUZZ|BOUND)-[0-9]{3}" "$ROADMAP" >/dev/null; then
    echo "⚠️  Warning: No tickets found with standard ID format"
fi

# Check for required meta fields
META_FIELDS=("project" "approach" "quality_gates")

for field in "${META_FIELDS[@]}"; do
    if ! grep -q "  $field:" "$ROADMAP"; then
        echo "❌ Missing required meta field: $field"
        exit 1
    fi
done

# Validate YAML syntax if yq is available
if command -v yq &> /dev/null; then
    if ! yq eval . "$ROADMAP" >/dev/null 2>&1; then
        echo "❌ Invalid YAML syntax"
        exit 1
    fi
    echo "  ✅ YAML syntax valid"
else
    echo "  ⏭️  yq not installed (skipping syntax validation)"
fi

# Check for quality gate thresholds
QUALITY_FIELDS=("max_complexity" "max_cognitive" "min_coverage" "satd_tolerance")

for field in "${QUALITY_FIELDS[@]}"; do
    if ! grep -q "    $field:" "$ROADMAP"; then
        echo "❌ Missing quality gate: $field"
        exit 1
    fi
done

echo "  ✅ All required sections present"
echo "  ✅ Quality gates defined"
echo ""
echo "✅ roadmap.yaml structure valid"

exit 0
