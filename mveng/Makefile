# Mveng - African AI Assistant
# Makefile for Docker operations

.PHONY: help build up down logs shell clean restart status health

# Default target
help: ## Show this help message
	@echo "🌍 Mveng - African AI Assistant"
	@echo "================================"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Development commands
dev: ## Start development environment (minimal services)
	@echo "🚀 Starting Mveng development environment..."
	docker-compose up mveng redis

build: ## Build the Mveng Docker image
	@echo "🔨 Building Mveng Docker image..."
	docker-compose build mveng

up: ## Start all services
	@echo "🌟 Starting all Mveng services..."
	docker-compose up -d

down: ## Stop all services
	@echo "🛑 Stopping all Mveng services..."
	docker-compose down

restart: ## Restart all services
	@echo "🔄 Restarting Mveng services..."
	docker-compose restart

# Service-specific commands
up-core: ## Start core services (mveng + redis)
	@echo "🎯 Starting core Mveng services..."
	docker-compose up -d mveng redis

up-ai: ## Start with AI services (ollama)
	@echo "🤖 Starting Mveng with AI services..."
	docker-compose up -d mveng redis ollama

up-full: ## Start with database
	@echo "🗄️ Starting full Mveng stack..."
	docker-compose --profile database up -d

up-prod: ## Start production environment
	@echo "🏭 Starting Mveng production environment..."
	docker-compose --profile production --profile database up -d

# Monitoring and debugging
logs: ## Show logs for all services
	docker-compose logs -f

logs-mveng: ## Show logs for Mveng service only
	docker-compose logs -f mveng

status: ## Show status of all services
	@echo "📊 Mveng Services Status:"
	docker-compose ps

health: ## Check health of all services
	@echo "🏥 Mveng Health Check:"
	@echo "API Server:"
	@curl -s http://localhost:3000/hello | jq . || echo "❌ API not responding"
	@echo "Redis:"
	@docker-compose exec redis redis-cli ping || echo "❌ Redis not responding"

shell: ## Get shell access to Mveng container
	docker-compose exec mveng /bin/bash

shell-redis: ## Get Redis CLI access
	docker-compose exec redis redis-cli

# Maintenance commands
clean: ## Clean up containers and volumes
	@echo "🧹 Cleaning up Mveng environment..."
	docker-compose down -v
	docker system prune -f

clean-all: ## Nuclear clean - remove everything
	@echo "💥 Nuclear clean - removing all Mveng data..."
	docker-compose down -v --rmi all
	docker system prune -af

# Ollama management
ollama-pull: ## Pull Llama2 model for Ollama
	@echo "📥 Pulling Llama2 model..."
	docker-compose exec ollama ollama pull llama2

ollama-list: ## List available Ollama models
	docker-compose exec ollama ollama list

# Database operations (when using database profile)
db-migrate: ## Run database migrations
	@echo "📊 Running database migrations..."
	# Add your migration commands here

db-shell: ## Access PostgreSQL shell
	docker-compose exec postgres psql -U mveng -d mveng

# Testing
test-api: ## Test API endpoints
	@echo "🧪 Testing Mveng API..."
	@echo "Hello endpoint:"
	@curl -s http://localhost:3000/hello | jq .
	@echo "Mveng endpoint:"
	@curl -s http://localhost:3000/mveng | jq .

# Environment setup
install: ## Install and setup everything
	@echo "⚙️ Setting up Mveng environment..."
	make build
	make up-core
	@echo "✅ Mveng is ready!"
	@echo "🌐 API: http://localhost:3000"
	@echo "📚 Swagger: http://localhost:3000/swagger-ui"

# Quick commands
quick-start: build up-core ## Build and start core services quickly

# Info
info: ## Show environment information
	@echo "🌍 Mveng Environment Information"
	@echo "==============================="
	@echo "API Server: http://localhost:3000"
	@echo "Swagger UI: http://localhost:3000/swagger-ui"
	@echo "Redis: localhost:6379"
	@echo "Ollama: http://localhost:11434"
	@echo "Whisper: http://localhost:8001"
	@echo ""
	@echo "🐳 Docker Status:"
	@docker --version
	@docker-compose --version