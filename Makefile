# Start the services
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
	RUST_BACKTRACE=1 cargo watch -x 'run -- --bin toy_plaid_api'

celery:
	cargo run --bin celery

lint:
	@cargo clippy --fix; cargo fmt

migrate:
	@sqlx migrate run --database-url postgres://postgres:password@localhost:5432/plaid

seed_db:
	@node ./scripts/seed_db.js
