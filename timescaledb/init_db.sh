#!/bin/bash
set -e

# Wait for PostgreSQL to be ready using default settings (Unix socket)
until pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB"; do
  echo "Waiting for PostgreSQL to be ready..."
  sleep 2
done

# Run the SQL file to create tables
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" -f /docker-entrypoint-initdb.d/tables.sql

echo "Tables have been created!"
