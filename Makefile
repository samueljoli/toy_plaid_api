up:
	@docker-compose up -d

# Stop the services
down:
	@docker-compose down

# Rebuild the services
rebuild:
	@docker-compose up -d --build

# Show logs
logs:
	@docker-compose logs -f

# Clean up
clean:
	@docker-compose down --rmi all --volumes --remove-orphans

# Run server and watch for changes
dev:
	@cargo watch -x run

