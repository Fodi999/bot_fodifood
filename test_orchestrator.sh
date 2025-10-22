#!/bin/bash
# 🎮 Backend Orchestrator Test Script
# Быстрое тестирование управления Go backend через Rust

set -e

RUST_API="http://127.0.0.1:8000"
BACKEND_URL="http://127.0.0.1:3000"

echo "🎮 Backend Orchestrator - Test Suite"
echo "======================================"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
success() {
    echo -e "${GREEN}✅ $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
}

info() {
    echo -e "${YELLOW}ℹ️  $1${NC}"
}

# Test 1: Check if Rust bot is running
echo "Test 1: Checking Rust Bot Status"
if curl -s -f "${RUST_API}/health" > /dev/null 2>&1; then
    success "Rust bot is running"
else
    error "Rust bot is not running"
    info "Start it with: cargo run --release --bin local"
    exit 1
fi
echo ""

# Test 2: Check orchestrator status
echo "Test 2: Checking Orchestrator Status"
STATUS=$(curl -s "${RUST_API}/api/v1/admin/backend/status" 2>/dev/null)
if [ $? -eq 0 ]; then
    success "Orchestrator API is available"
    echo "$STATUS" | jq '.' 2>/dev/null || echo "$STATUS"
else
    error "Orchestrator API not responding"
    info "Make sure ORCHESTRATOR_ENABLED=true in .env"
    exit 1
fi
echo ""

# Test 3: Start backend
echo "Test 3: Starting Go Backend"
RESULT=$(curl -s -X POST "${RUST_API}/api/v1/admin/backend/start" 2>/dev/null)
if echo "$RESULT" | grep -q '"success":true'; then
    success "Backend start command sent"
    echo "$RESULT" | jq '.' 2>/dev/null || echo "$RESULT"
else
    error "Failed to start backend"
    echo "$RESULT" | jq '.' 2>/dev/null || echo "$RESULT"
fi
echo ""

# Wait for backend to start
echo "⏳ Waiting for backend to start (5 seconds)..."
sleep 5
echo ""

# Test 4: Check backend health
echo "Test 4: Checking Backend Health"
if curl -s -f "${BACKEND_URL}/health" > /dev/null 2>&1; then
    success "Backend is responding on ${BACKEND_URL}"
else
    error "Backend is not responding"
    info "Check logs for errors"
fi
echo ""

# Test 5: Get backend status
echo "Test 5: Getting Backend Status"
STATUS=$(curl -s "${RUST_API}/api/v1/admin/backend/status" 2>/dev/null)
echo "$STATUS" | jq '.' 2>/dev/null || echo "$STATUS"

if echo "$STATUS" | grep -q '"status":"running"'; then
    success "Backend is running"
    PID=$(echo "$STATUS" | jq -r '.pid // "unknown"')
    UPTIME=$(echo "$STATUS" | jq -r '.uptime_secs // "unknown"')
    info "PID: $PID, Uptime: ${UPTIME}s"
else
    error "Backend is not running"
fi
echo ""

# Test 6: Restart backend
echo "Test 6: Restarting Backend"
read -p "Do you want to test restart? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    RESULT=$(curl -s -X POST "${RUST_API}/api/v1/admin/backend/restart" 2>/dev/null)
    if echo "$RESULT" | grep -q '"success":true'; then
        success "Backend restart command sent"
        echo "$RESULT" | jq '.' 2>/dev/null || echo "$RESULT"
        
        echo "⏳ Waiting for restart (5 seconds)..."
        sleep 5
        
        if curl -s -f "${BACKEND_URL}/health" > /dev/null 2>&1; then
            success "Backend restarted successfully"
        else
            error "Backend not responding after restart"
        fi
    else
        error "Failed to restart backend"
        echo "$RESULT" | jq '.' 2>/dev/null || echo "$RESULT"
    fi
else
    info "Skipping restart test"
fi
echo ""

# Test 7: Stop backend (optional)
echo "Test 7: Stopping Backend"
read -p "Do you want to stop the backend? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    RESULT=$(curl -s -X POST "${RUST_API}/api/v1/admin/backend/stop" 2>/dev/null)
    if echo "$RESULT" | grep -q '"success":true'; then
        success "Backend stop command sent"
        echo "$RESULT" | jq '.' 2>/dev/null || echo "$RESULT"
        
        echo "⏳ Waiting for shutdown (3 seconds)..."
        sleep 3
        
        if ! curl -s -f "${BACKEND_URL}/health" > /dev/null 2>&1; then
            success "Backend stopped successfully"
        else
            error "Backend still responding"
        fi
    else
        error "Failed to stop backend"
        echo "$RESULT" | jq '.' 2>/dev/null || echo "$RESULT"
    fi
else
    info "Skipping stop test"
fi
echo ""

# Summary
echo "======================================"
echo "🎯 Test Suite Complete!"
echo ""
echo "Available commands:"
echo "  Start:   curl -X POST ${RUST_API}/api/v1/admin/backend/start"
echo "  Status:  curl ${RUST_API}/api/v1/admin/backend/status"
echo "  Restart: curl -X POST ${RUST_API}/api/v1/admin/backend/restart"
echo "  Stop:    curl -X POST ${RUST_API}/api/v1/admin/backend/stop"
echo ""
echo "Monitoring:"
echo "  Metrics: curl ${RUST_API}/metrics | grep backend"
echo "  Logs:    tail -f logs/bot.log"
