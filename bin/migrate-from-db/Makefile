# Makefile for migrate-from-db (run from this directory: bin/migrate-from-db)

.PHONY: help order-tables run run-once run-patch-only

help:
	@echo "migrate-from-db targets (run from bin/migrate-from-db):"
	@echo "  make run            - Start the migration API server (POST /migrate, GET /status, GET /tables)"
	@echo "  make run-once       - Run full migration once from env (both passes; requires .env)"
	@echo "  make run-patch-only - Run only the second pass from env (PATCH circular FK columns)"
	@echo "  make order-tables   - Generate MIGRATE_TABLES_ORDER from DB FK relations (writes Rust array)"

# Start the migration API. Use POST /migrate with JSON body to run migration.
# Env: MIGRATE_API_PORT (default 3090)
run:
	@export PATH="$$HOME/.cargo/bin:$$PATH"; cargo run --bin migrate-from-db

# One-shot migration from .env (legacy / scripting)
run-once:
	@export PATH="$$HOME/.cargo/bin:$$PATH"; cargo run --bin migrate-once

# Generate table order from DB foreign keys. Writes migrate_tables_order.rs by default.
# Requires MIGRATE_FROM_DATABASE_URL (or MIGRATE_ORDER_DATABASE_URL).
# For unique indexes: set MIGRATE_TO_DATABASE_URL in .env or pass: make order-tables MIGRATE_TO_DATABASE_URL="postgres://..."
# Optional: MIGRATE_ORDER_SCHEMA=public, MIGRATE_ORDER_OUTPUT=<path>
order-tables:
	@echo "📋 Generating migrate table order from database relations..."
	@set -a && [ -f .env ] && . ./.env; set +a; \
	export PATH="$$HOME/.cargo/bin:$$PATH"; \
	MIGRATE_ORDER_OUTPUT="$${MIGRATE_ORDER_OUTPUT:-migrate_tables_order.rs}" \
	cargo run --bin order-tables
	@echo "✅ Table order written to $${MIGRATE_ORDER_OUTPUT:-migrate_tables_order.rs}"

run-patch-only:
	@export PATH="$$HOME/.cargo/bin:$$PATH"; \
	MIGRATE_ONLY_PATCH_SECOND_PASS=1 cargo run --bin migrate-once
