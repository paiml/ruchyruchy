#!/bin/bash
# PMAT TDG Real-Time Monitoring Script for RuchyRuchy Bootstrap Compiler
# Following patterns from ../ruchy with bootstrap-specific features

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 RuchyRuchy PMAT TDG Real-Time Monitoring${NC}"
echo -e "${BLUE}   Bootstrap Compiler Quality Dashboard${NC}"

# Check if PMAT TDG dashboard is available
if ! command -v pmat >/dev/null 2>&1; then
    echo -e "${RED}❌ PMAT not found. Install pmat to use TDG monitoring.${NC}"
    echo "   Visit: https://github.com/paiml/pmat"
    exit 1
fi

# Function to start TDG dashboard
start_dashboard() {
    echo -e "${GREEN}📊 Starting TDG Dashboard...${NC}"
    echo "  - Real-time monitoring with 5-second updates"
    echo "  - Bootstrap stage analysis (Stage 0-3)"
    echo "  - Validation infrastructure monitoring"
    echo "  - Performance profiling with flame graphs"
    echo "  - Interactive analysis with Server-Sent Events"

    # Start dashboard in background if not already running
    if ! pgrep -f "pmat tdg dashboard" > /dev/null; then
        pmat tdg dashboard --port 8080 --update-interval 5 --open &
        DASHBOARD_PID=$!
        echo -e "${GREEN}✅ TDG Dashboard started on http://localhost:8080 (PID: $DASHBOARD_PID)${NC}"
    else
        echo -e "${YELLOW}ℹ️ TDG Dashboard already running${NC}"
    fi
}

# Function to start MCP server (optional enterprise integration)
start_mcp() {
    echo -e "${GREEN}🔧 Starting PMAT MCP Server...${NC}"
    echo "  - Enterprise-grade analysis with persistence"
    echo "  - System health and performance monitoring"
    echo "  - Advanced profiling with flame graphs"
    echo "  - Configurable alert system"

    if ! pgrep -f "pmat mcp serve" > /dev/null; then
        pmat mcp serve --port 3000 &
        MCP_PID=$!
        echo -e "${GREEN}✅ MCP Server started on http://localhost:3000 (PID: $MCP_PID)${NC}"
    else
        echo -e "${YELLOW}ℹ️ MCP Server already running${NC}"
    fi
}

# Function to run baseline TDG check
baseline_check() {
    echo -e "${YELLOW}📋 Running TDG baseline check...${NC}"

    # Get current TDG score
    TDG_SCORE=$(pmat tdg . --quiet 2>/dev/null || echo "0")
    if (( $(echo "$TDG_SCORE >= 85" | bc -l 2>/dev/null || echo "0") )); then
        echo -e "${GREEN}✅ Current TDG Score: $TDG_SCORE (≥85 A- required)${NC}"
    else
        echo -e "${RED}⚠️ Current TDG Score: $TDG_SCORE (below 85 A- threshold)${NC}"
        echo "   Run: pmat tdg . --include-components --format=table"
    fi

    # Create baseline if it doesn't exist
    if [ ! -f ".tdg_baseline.json" ]; then
        echo "📝 Creating TDG baseline..."
        pmat tdg . --format=json > .tdg_baseline.json 2>/dev/null || true
    fi
}

# Function to check bootstrap stage quality
check_stages() {
    echo -e "${BLUE}🏗️ Checking Bootstrap Stage Quality...${NC}"

    for stage in stage0 stage1 stage2 stage3; do
        if [ -d "bootstrap/$stage" ]; then
            echo -e "  Analyzing ${YELLOW}$stage${NC}..."
            SCORE=$(pmat tdg "bootstrap/$stage" --quiet 2>/dev/null || echo "N/A")
            if [ "$SCORE" != "N/A" ]; then
                echo -e "    Score: ${GREEN}$SCORE${NC}"
            else
                echo -e "    Score: ${YELLOW}Pending implementation${NC}"
            fi
        fi
    done
}

# Function to check validation infrastructure quality
check_validation() {
    echo -e "${BLUE}🔬 Checking Validation Infrastructure Quality...${NC}"

    if [ -d "validation" ]; then
        SCORE=$(pmat tdg validation/ --quiet 2>/dev/null || echo "N/A")
        if [ "$SCORE" != "N/A" ]; then
            echo -e "  Validation Score: ${GREEN}$SCORE${NC}"
        else
            echo -e "  Validation Score: ${YELLOW}Analysis pending${NC}"
        fi
    fi
}

# Main execution
case "${1:-start}" in
    "start")
        baseline_check
        check_stages
        check_validation
        start_dashboard
        # start_mcp  # Uncomment for enterprise MCP integration
        echo ""
        echo -e "${GREEN}🎯 PMAT monitoring started successfully!${NC}"
        echo "  - Dashboard: http://localhost:8080"
        echo "  - Bootstrap stages: stage0, stage1, stage2, stage3"
        echo "  - Validation: validation/"
        echo "  - Use 'Ctrl+C' to stop monitoring"
        ;;
    "stop")
        echo "🛑 Stopping PMAT monitoring..."
        pkill -f "pmat tdg dashboard" && echo -e "${GREEN}✅ Dashboard stopped${NC}" || echo -e "${YELLOW}ℹ️ Dashboard not running${NC}"
        pkill -f "pmat mcp serve" && echo -e "${GREEN}✅ MCP Server stopped${NC}" || echo -e "${YELLOW}ℹ️ MCP Server not running${NC}"
        ;;
    "status")
        echo "📊 PMAT Monitoring Status:"
        if pgrep -f "pmat tdg dashboard" > /dev/null; then
            echo -e "  ${GREEN}✅ TDG Dashboard: Running (http://localhost:8080)${NC}"
        else
            echo -e "  ${RED}❌ TDG Dashboard: Not running${NC}"
        fi

        if pgrep -f "pmat mcp serve" > /dev/null; then
            echo -e "  ${GREEN}✅ MCP Server: Running (http://localhost:3000)${NC}"
        else
            echo -e "  ${RED}❌ MCP Server: Not running${NC}"
        fi

        echo ""
        baseline_check
        check_stages
        check_validation
        ;;
    "baseline")
        echo "📊 Creating/Updating TDG Baseline..."
        pmat tdg . --format=json > .tdg_baseline.json
        echo -e "${GREEN}✅ Baseline saved to .tdg_baseline.json${NC}"
        ;;
    *)
        echo "Usage: $0 {start|stop|status|baseline}"
        echo ""
        echo "Commands:"
        echo "  start    - Start TDG dashboard and monitoring"
        echo "  stop     - Stop all monitoring services"
        echo "  status   - Check monitoring service status"
        echo "  baseline - Create/update TDG baseline"
        echo ""
        echo "Example workflow:"
        echo "  ./pmat_monitor.sh baseline  # Create initial baseline"
        echo "  ./pmat_monitor.sh start     # Start monitoring"
        echo "  ./pmat_monitor.sh status    # Check status"
        echo "  ./pmat_monitor.sh stop      # Stop when done"
        exit 1
        ;;
esac
