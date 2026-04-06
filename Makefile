.PHONY: help build up down logs test clean

help:
	@echo "Task Dashboard - Available commands:"
	@echo "  make build      - Build Docker images"
	@echo "  make up         - Start all services"
	@echo "  make down       - Stop all services"
	@echo "  make logs       - View logs"
	@echo "  make test       - Run tests"
	@echo "  make test-backend - Run backend tests only"
	@echo "  make test-frontend - Run frontend tests only"
	@echo "  make clean      - Remove containers and volumes"

build:
	docker-compose build

up:
	docker-compose up -d
	@echo "Services starting..."
	@echo "Frontend: http://localhost:3000"
	@echo "Backend API: http://localhost:8080"
	@echo "API Health: http://localhost:8080/api/health"

down:
	docker-compose down

logs:
	docker-compose logs -f

test:
	docker-compose exec backend cargo test --workspace
	docker-compose exec frontend npm test -- --run

test-backend:
	docker-compose exec backend cargo test --workspace

test-frontend:
	docker-compose exec frontend npm test -- --run

clean:
	docker-compose down -v
	docker volume rm task-dashboard_postgres_data 2>/dev/null || true
	docker rm -f task-dashboard-backend task-dashboard-frontend task-dashboard-db 2>/dev/null || true
	@echo "Clean complete"
