# Makefile for migrate-from-db (run from this directory: bin/migrate-from-db)

.PHONY: help order-tables run run-patch-only

help:
	@echo "migrate-from-db targets (run from bin/migrate-from-db):"
	@echo "  make order-tables  - Generate MIGRATE_TABLES_ORDER from DB FK relations (writes Rust array)"
	@echo "  make run            - Run full migration (both passes; requires .env with MIGRATE_* vars)"
	@echo "  make run-patch-only - Run only the second pass (PATCH circular FK columns)"

# Generate table order from DB foreign keys. Writes migrate_tables_order.rs by default.
# Requires MIGRATE_FROM_DATABASE_URL (or MIGRATE_ORDER_DATABASE_URL).
# Optional: MIGRATE_ORDER_SCHEMA=public, MIGRATE_ORDER_OUTPUT=<path>
order-tables:
	@echo "📋 Generating migrate table order from database relations..."
	@export PATH="$$HOME/.cargo/bin:$$PATH"; \
	MIGRATE_ORDER_OUTPUT="$${MIGRATE_ORDER_OUTPUT:-migrate_tables_order.rs}" \
	cargo run --bin order-tables
	@echo "✅ Table order written to $${MIGRATE_ORDER_OUTPUT:-migrate_tables_order.rs}"

run:
	@export PATH="$$HOME/.cargo/bin:$$PATH"; cargo run --bin migrate-from-db

run-patch-only:
	@export PATH="$$HOME/.cargo/bin:$$PATH"; \
	MIGRATE_ONLY_PATCH_SECOND_PASS=1 cargo run --bin migrate-from-db
