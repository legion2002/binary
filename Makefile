.PHONY: dev test test-unit test-e2e build clean docker docker-build docker-up docker-down docker-logs install help

# Default target
.DEFAULT_GOAL := help

# Colors for output
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RESET := \033[0m

## Development - Full Stack

dev: ## Start full dev environment: Tempo node + contracts + backend + frontend
	@echo "$(CYAN)Starting full development environment...$(RESET)"
	bun run script/env.ts dev

## Testing

test: ## Run integration tests against full stack
	@echo "$(CYAN)Running integration tests...$(RESET)"
	bun run script/env.ts test

test-unit: ## Run unit tests only (no orchestration needed)
	@echo "$(CYAN)Running unit tests...$(RESET)"
	cd backend && cargo test --lib
	cd frontend && bun run lint

test-e2e: ## Run e2e tests with frontend
	bun run script/env.ts test --with-frontend -- bun --cwd frontend test:e2e

## Build

build: build-backend build-frontend ## Build both backend and frontend

build-backend: ## Build backend for release
	@echo "$(CYAN)Building backend...$(RESET)"
	cd backend && cargo build --release

build-frontend: ## Build frontend for production
	@echo "$(CYAN)Building frontend...$(RESET)"
	cd frontend && bun run build

## Docker (Tempo node only)

docker-up: ## Start Tempo node via Docker
	@echo "$(GREEN)Starting Tempo node...$(RESET)"
	docker compose up -d

docker-down: ## Stop Tempo node
	@echo "$(YELLOW)Stopping Tempo node...$(RESET)"
	docker compose down

docker-logs: ## View Tempo node logs
	docker compose logs -f

## Installation

install: ## Install all dependencies
	@echo "$(CYAN)Installing dependencies...$(RESET)"
	cd backend && cargo fetch
	cd frontend && bun install
	cd script && bun install

## Linting

lint: ## Run all linters
	cd backend && cargo clippy
	cd frontend && bun run lint

## Cleanup

clean: ## Clean build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(RESET)"
	cd backend && cargo clean
	cd frontend && rm -rf dist node_modules/.vite
	rm -f backend/dev.db backend/markets.db

## Help

help: ## Show this help message
	@echo "$(CYAN)Binary Markets - Available Commands$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Quick Start:$(RESET)"
	@echo "  make dev       # Start everything locally"
	@echo "  make test      # Run integration tests"
	@echo "  make build     # Build for production"
