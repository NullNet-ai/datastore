#!/bin/sh

# Wait for TimescaleDB to be ready
echo "DB_HOST=$DB_HOST"
echo "DB_PORT=$DB_PORT"
echo "DB_USER=$DB_USER"
echo "Waiting for TimescaleDB to be ready..."

until pg_isready -h $DB_HOST -p $DB_PORT -U $DB_USER; do
  echo "Waiting for database connection..."
  sleep 2
done

echo "TimescaleDB is up - proceeding with migrations..."

# Run Drizzle commands
npm run drizzle:generate
npm run drizzle:migrate

# Start the application
echo "Starting the NestJS application..."
node dist/main.js
