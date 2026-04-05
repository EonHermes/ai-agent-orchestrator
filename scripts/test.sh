#!/bin/bash

set -e

echo "🧪 Running tests for AI Agent Orchestrator..."

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

FAILED=0

# Backend tests
if command -v cargo &> /dev/null; then
    echo ""
    echo "📦 Backend (Rust) Tests"
    echo "=========================="
    cd backend
    
    echo "Running cargo test..."
    if cargo test --workspace; then
        echo -e "${GREEN}✅ Backend tests passed${NC}"
    else
        echo -e "${RED}❌ Backend tests failed${NC}"
        FAILED=1
    fi
    
    # Clippy lint check
    echo ""
    echo "Running clippy..."
    if cargo clippy -- -D warnings; then
        echo -e "${GREEN}✅ Clippy check passed${NC}"
    else
        echo -e "${RED}❌ Clippy check failed${NC}"
        FAILED=1
    fi
    
    cd ..
else
    echo "⚠️  Rust/Cargo not found. Skipping backend tests."
fi

# Frontend tests
if command -v npm &> /dev/null && [ -d "frontend/node_modules" ]; then
    echo ""
    echo "📦 Frontend (TypeScript/React) Tests"
    echo "======================================"
    cd frontend
    
    echo "Running npm test..."
    if npm test; then
        echo -e "${GREEN}✅ Frontend tests passed${NC}"
    else
        echo -e "${RED}❌ Frontend tests failed${NC}"
        FAILED=1
    fi
    
    # Type check
    echo ""
    echo "Running TypeScript check..."
    if npx tsc --noEmit; then
        echo -e "${GREEN}✅ TypeScript check passed${NC}"
    else
        echo -e "${RED}❌ TypeScript check failed${NC}"
        FAILED=1
    fi
    
    cd ..
else
    echo "⚠️  Frontend dependencies not installed. Run npm install in frontend/ first."
fi

# Docker integration tests (if containers are running)
if docker ps 2>/dev/null | grep -q orchestrator; then
    echo ""
    echo "🐳 Integration Tests"
    echo "====================="
    
    if command -v curl &> /dev/null; then
        echo "Testing backend health..."
        if curl -s http://localhost:8081/health > /dev/null; then
            echo -e "${GREEN}✅ Backend is healthy${NC}"
        else
            echo -e "${RED}❌ Backend health check failed${NC}"
            FAILED=1
        fi
    fi
fi

echo ""
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✨ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Please fix the issues above.${NC}"
    exit 1
fi
