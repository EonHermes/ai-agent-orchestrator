#!/bin/bash

set -e

echo "🚀 Setting up AI Agent Orchestrator..."

# Check prerequisites
echo "Checking prerequisites..."

# Check Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Check Node.js for frontend development
if ! command -v node &> /dev/null; then
    echo "⚠️  Node.js not found. Frontend development will not be available."
else
    NODE_VERSION=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
    if [ "$NODE_VERSION" -lt 18 ]; then
        echo "⚠️  Node.js version $(node -v) detected. Node.js 18+ is recommended."
    fi
fi

# Check Rust for backend development
if ! command -v cargo &> /dev/null; then
    echo "⚠️  Rust/Cargo not found. Backend development will not be available."
else
    echo "✅ Rust $(rustc --version) found"
fi

# Create data directory
mkdir -p data

# Copy environment file if it doesn't exist
if [ ! -f .env ]; then
    cp .env.example .env
    echo "✅ Created .env from .env.example"
    echo "⚠️  Please edit .env and add your OPENROUTER_API_KEY"
fi

# Install frontend dependencies
echo "📦 Installing frontend dependencies..."
cd frontend
if command -v npm &> /dev/null; then
    npm install --silent
    echo "✅ Frontend dependencies installed"
else
    echo "⚠️  npm not found. Skipping frontend dependencies."
fi
cd ..

# Build Docker images
echo "🐳 Building Docker images..."
if docker compose version &> /dev/null; then
    docker compose build
else
    docker-compose build
fi

echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Edit .env and add your OPENROUTER_API_KEY"
echo "2. Run: docker-compose up -d (or: docker compose up -d)"
echo "3. Access the dashboard at http://localhost:3000"
echo "4. API available at http://localhost:8081"
echo ""
echo "For development:"
echo "  Backend: cd backend && cargo run"
echo "  Frontend: cd frontend && npm run dev"
