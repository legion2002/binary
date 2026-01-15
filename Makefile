.PHONY: dev test test-unit test-e2e build clean docker docker-build docker-up docker-down docker-logs install help testnet testnet-redeploy

# Default target
.DEFAULT_GOAL := help

# Colors for output
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RESET := \033[0m

## Development - Full Stack

dev: ## Start full dev environment: Tempo node + UniV2 + contracts + backend + frontend
	@echo "$(CYAN)Starting full development environment...$(RESET)"
	bun run script/env.ts dev

## Testing

test: ## Run integration tests against full stack
	@echo "$(CYAN)Running integration tests...$(RESET)"
	bun run script/env.ts test

test-unit: ## Run unit tests only (no orchestration needed)
	@echo "$(CYAN)Running backend unit tests...$(RESET)"
	cd backend && cargo test --test unit_tests
	@echo "$(CYAN)Running backend lib tests...$(RESET)"
	cd backend && cargo test --lib
	@echo "$(CYAN)Running frontend lint...$(RESET)"
	cd frontend && bun run lint

test-e2e: ## Run e2e tests with frontend
	bun run script/env.ts test --with-frontend -- bun --cwd frontend test:e2e

## Testnet Deployment

testnet: ## Deploy and run full stack on Tempo testnet
	@echo "$(CYAN)Deploying to Tempo testnet...$(RESET)"
	bun run script/env-testnet.ts

testnet-backend: ## Run backend only on testnet (no frontend)
	@echo "$(CYAN)Starting testnet backend...$(RESET)"
	bun run script/env-testnet.ts --skip-frontend

testnet-redeploy: ## Force redeploy contracts on testnet
	@echo "$(CYAN)Redeploying contracts to testnet...$(RESET)"
	bun run script/env-testnet.ts --redeploy

testnet-liquidity: ## Add liquidity to testnet markets
	@echo "$(CYAN)Adding liquidity to testnet markets...$(RESET)"
	bun run script/seed-testnet-liquidity.ts

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

clean: ## Clean build artifacts and UniV2 config
	@echo "$(YELLOW)Cleaning build artifacts...$(RESET)"
	cd backend && cargo clean
	cd frontend && rm -rf dist node_modules/.vite
	rm -f backend/dev.db backend/markets.db .univ2-config.json .testnet-config.json

## Help

help: ## Show this help message
	@echo "$(CYAN)Binary Markets (UniV2 AMM) - Available Commands$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-18s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Quick Start:$(RESET)"
	@echo "  make dev              # Start everything locally (deploys UniV2 + contracts)"
	@echo "  make test             # Run integration tests"
	@echo "  make testnet          # Deploy to Tempo testnet"
	@echo "  make build            # Build for production"
