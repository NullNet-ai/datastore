# Variables
DOCKER_IMAGE_NAME = datastore-postgres
DOCKERFILE_PATH   = docker/Dockerfile

# Compose Files
DOCKER_COMPOSE_CLIENT1 = docker/client-one/docker-compose-postgres-client.yaml
DOCKER_COMPOSE_CLIENT2 = docker/client-two/docker-compose-postgres-client.yaml
DOCKER_COMPOSE_SERVER  = docker/server/docker-compose-postgres-server.yaml

# Build the Docker image
build-image:
	@echo "Building Docker image: $(DOCKER_IMAGE_NAME)"
	docker build -t $(DOCKER_IMAGE_NAME) -f $(DOCKERFILE_PATH) .

# Delete the Docker image
delete-image:
	@echo "Deleting Docker image: $(DOCKER_IMAGE_NAME)"
	docker rmi $(DOCKER_IMAGE_NAME) --force

# Run Client1
run-client1:
	@echo "Running Client1 services..."
	docker compose -f $(DOCKER_COMPOSE_CLIENT1) up --build

# Run Client2
run-client2:
	@echo "Running Client2 services..."
	docker compose -f $(DOCKER_COMPOSE_CLIENT2) up --build

# Run Server
run-server:
	@echo "Running Server services..."
	docker compose -f $(DOCKER_COMPOSE_SERVER) up --build

# Run All (Client1, Client2, and Server) in Detached Mode
run-all:
	@echo "Running all services (client1, client2, server) in detached mode..."
	docker compose -f $(DOCKER_COMPOSE_CLIENT1) up -d --build
	docker compose -f $(DOCKER_COMPOSE_CLIENT2) up -d --build
	docker compose -f $(DOCKER_COMPOSE_SERVER) up -d --build

# Stop All Services
down-all:
	@echo "Stopping all services..."
	docker compose -f $(DOCKER_COMPOSE_CLIENT1) down
	docker compose -f $(DOCKER_COMPOSE_CLIENT2) down
	docker compose -f $(DOCKER_COMPOSE_SERVER) down

# Default Goal
.DEFAULT_GOAL := build-image
