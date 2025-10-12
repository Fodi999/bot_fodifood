# Makefile for FodiFood Bot

.PHONY: help setup dev build test lint fmt clean deploy logs

help: ## Show this help message
	@echo "FodiFood Bot - Available commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Initial project setup
	@echo "🦐 Setting up FodiFood Bot..."
	@command -v cargo >/dev/null 2>&1 || { echo "❌ Rust not found. Install from https://rustup.rs/"; exit 1; }
	@command -v cargo-shuttle >/dev/null 2>&1 || cargo install cargo-shuttle
	@[ -f .env ] || cp .env.example .env
	@echo "✅ Setup complete! Edit .env and run 'make dev'"

dev: ## Run locally for development
	@echo "🚀 Starting development server..."
	@cargo shuttle run

watch: ## Run with auto-reload on file changes
	@echo "👀 Starting with auto-reload..."
	@cargo watch -x 'shuttle run'

build: ## Build the project
	@echo "🔨 Building..."
	@cargo build

build-release: ## Build optimized release version
	@echo "🔨 Building release..."
	@cargo build --release

test: ## Run all tests
	@echo "🧪 Running tests..."
	@cargo test

test-verbose: ## Run tests with output
	@echo "🧪 Running tests (verbose)..."
	@cargo test -- --nocapture

lint: ## Run clippy linter
	@echo "🔍 Running linter..."
	@cargo clippy -- -D warnings

fmt: ## Format code
	@echo "✨ Formatting code..."
	@cargo fmt

fmt-check: ## Check if code is formatted
	@echo "🔍 Checking formatting..."
	@cargo fmt -- --check

check: ## Quick compile check
	@echo "⚡ Quick check..."
	@cargo check

clean: ## Clean build artifacts
	@echo "🧹 Cleaning..."
	@cargo clean
	@rm -rf target/

deploy: ## Deploy to Shuttle
	@echo "🚀 Deploying to Shuttle..."
	@cargo shuttle deploy

deploy-prod: lint test deploy ## Full deployment (lint + test + deploy)
	@echo "✅ Production deployment complete!"

logs: ## Show Shuttle logs
	@cargo shuttle logs --follow

logs-tail: ## Show last 100 log lines
	@cargo shuttle logs --tail 100

status: ## Show Shuttle project status
	@cargo shuttle status

secrets: ## List Shuttle secrets
	@cargo shuttle secrets list

secrets-set: ## Set a secret (usage: make secrets-set KEY=value)
	@cargo shuttle secrets set $(KEY)

install-tools: ## Install development tools
	@echo "📦 Installing development tools..."
	@cargo install cargo-watch
	@cargo install cargo-shuttle
	@cargo install flamegraph
	@echo "✅ Tools installed!"

benchmark: ## Run benchmarks (if available)
	@echo "⚡ Running benchmarks..."
	@cargo bench

doc: ## Generate and open documentation
	@echo "📚 Generating documentation..."
	@cargo doc --open

security-audit: ## Run security audit
	@echo "🔒 Running security audit..."
	@cargo audit

update-deps: ## Update dependencies
	@echo "📦 Updating dependencies..."
	@cargo update

outdated: ## Check for outdated dependencies
	@echo "🔍 Checking outdated dependencies..."
	@cargo outdated

all: fmt lint test build ## Format, lint, test and build

ci: fmt-check lint test ## CI pipeline (format check, lint, test)

# Docker commands (if you want to containerize)
docker-build: ## Build Docker image
	@echo "🐳 Building Docker image..."
	@docker build -t fodifood-bot .

docker-run: ## Run Docker container
	@echo "🐳 Running Docker container..."
	@docker run --env-file .env -p 8000:8000 fodifood-bot

# Development utilities
repl: ## Start Rust REPL (evcxr)
	@evcxr

tree: ## Show project structure
	@tree -I 'target|.git' -L 3

size: ## Show binary size
	@ls -lh target/release/fodifood-bot 2>/dev/null || echo "Run 'make build-release' first"

# Database migrations (if you add database)
# migrate-up: ## Run database migrations
# 	@sqlx migrate run

# migrate-down: ## Rollback last migration
# 	@sqlx migrate revert
