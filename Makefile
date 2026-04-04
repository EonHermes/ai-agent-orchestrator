.PHONY: help build-backend build-frontend build test-backend test-frontend test run-backend run-frontend run clean docker-build docker-run logs

# Development
help:
	@echo "AI Agent Orchestrator - Development Commands"
	@echo ""
	@echo "Building:"
	@echo "  make build-backend     - Build Rust backend"
	@echo "  make build-frontend    - Build React frontend"
	@echo "  make build             - Build both"
	@echo ""
	@echo "Testing:"
	@echo "  make test-backend      - Run Rust tests"
	@echo "  make test-frontend     - Run frontend tests"
	@echo "  make test              - Run all tests"
	@echo ""
	@echo "Running:"
	@echo "  make run-backend       - Run backend locally"
	@echo "  make run-frontend      - Run frontend dev server"
	@echo "  make run               - Run both"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build      - Build and run with docker-compose"
	@echo "  make docker-run        - Alias for docker-build"
	@echo "  make logs             - Show container logs"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean             - Remove build artifacts"

build-backend:
	cd backend && cargo build --release

build-frontend:
	cd frontend && npm run build

build: build-backend build-frontend

test-backend:
	cd backend && cargo test

test-frontend:
	cd frontend && npm test

test: test-backend test-frontend

run-backend:
	cd backend && cargo run

run-frontend:
	cd frontend && npm run dev

run: run-backend run-frontend

docker-build:
	docker-compose build
	docker-compose up -d

docker-run: docker-build

logs:
	docker-compose logs -f

clean:
	rm -rf backend/target
	rm -rf frontend/dist
	rm -rf frontend/node_modules
	docker-compose down -v