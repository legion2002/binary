.PHONY: dev dev-backend dev-frontend build clean docker docker-build docker-up docker-down docker-logs install test lint help env-dev env-test env-test-backend env-test-e2e

# Default target
.DEFAULT_GOAL := help

# Colors for output
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RESET := \033[0m

## Full Environment (Tempo + Backend + Frontend)

env-dev: ## Start full dev environment: Tempo node + contracts + backend + frontend
	@echo "$(CYAN)Starting full development environment...$(RESET)"
	bun run script/env.ts dev

env-test: ## Run integration tests against full stack (Tempo + backend)
	@echo "$(CYAN)Running integration tests...$(RESET)"
	bun run script/env.ts test

env-test-backend: ## Run backend integration tests against full stack
	bun run script/env.ts test -- cargo test --test integration_test -- --ignored

env-test-e2e: ## Run e2e tests with frontend
	bun run script/env.ts test --with-frontend -- pnpm --prefix frontend test:e2e

## Development (without Tempo - use env-dev for full stack)

dev: ## Run backend and frontend only (no Tempo node)
	@echo "$(CYAN)Starting backend and frontend...$(RESET)"
	@$(MAKE) -j2 dev-backend dev-frontend

dev-backend: ## Run backend in development mode
	@echo "$(GREEN)Starting backend...$(RESET)"
	cd backend && cargo run --bin backend

dev-frontend: ## Run frontend in development mode
	@echo "$(GREEN)Starting frontend...$(RESET)"
	cd frontend && pnpm dev

## Build

build: build-backend build-frontend ## Build both backend and frontend

build-backend: ## Build backend for release
	@echo "$(CYAN)Building backend...$(RESET)"
	cd backend && cargo build --release

build-frontend: ## Build frontend for production
	@echo "$(CYAN)Building frontend...$(RESET)"
	cd frontend && pnpm run build

## Docker

docker: docker-build docker-up ## Build and start Docker containers

docker-build: ## Build Docker images
	@echo "$(CYAN)Building Docker images...$(RESET)"
	docker compose build

docker-up: ## Start Docker containers
	@echo "$(GREEN)Starting Docker containers...$(RESET)"
	docker compose up -d

docker-down: ## Stop Docker containers
	@echo "$(YELLOW)Stopping Docker containers...$(RESET)"
	docker compose down

docker-logs: ## View Docker container logs
	docker compose logs -f

docker-clean: ## Remove Docker containers, images, and volumes
	@echo "$(YELLOW)Cleaning Docker resources...$(RESET)"
	docker compose down -v --rmi local

## Installation

install: install-backend install-frontend ## Install all dependencies

install-backend: ## Install backend dependencies
	@echo "$(CYAN)Installing backend dependencies...$(RESET)"
	cd backend && cargo fetch

install-frontend: ## Install frontend dependencies
	@echo "$(CYAN)Installing frontend dependencies...$(RESET)"
	cd frontend && pnpm install

## Testing

test: test-backend test-frontend ## Run all tests

test-backend: ## Run backend tests
	@echo "$(CYAN)Running backend tests...$(RESET)"
	cd backend && cargo test

test-frontend: ## Run frontend linting (no tests configured yet)
	@echo "$(CYAN)Running frontend lint...$(RESET)"
	cd frontend && pnpm run lint

## Linting

lint: lint-backend lint-frontend ## Run all linters

lint-backend: ## Run backend linter
	cd backend && cargo clippy

lint-frontend: ## Run frontend linter
	cd frontend && pnpm run lint

## Cleanup

clean: ## Clean build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(RESET)"
	cd backend && cargo clean
	cd frontend && rm -rf dist node_modules/.vite

## Help

help: ## Show this help message
	@echo "$(CYAN)Binary Markets - Available Commands$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-18s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Examples:$(RESET)"
	@echo "  make dev          # Start both services for development"
	@echo "  make docker       # Build and run with Docker"
	@echo "  make build        # Build for production"
