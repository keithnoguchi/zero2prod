#!/usr/bin/env bash

set -x
set -eo pipefail

# Check if a custom user has been set, otherwise default to 'postgres'.
DB_USER=${POSTGRES_USER:=postgres}

# Check if a custom password has been set, otherwise default to 'password'.
DB_PASSWORD=${POSTGRES_PASSWORD:=password}

# Chceck if a custom database name has been set, otherwise default
# to 'newsletter'.
DB_NAME=${POSTGRES_DB:=newsletter}

# Check if a custom address has been set, otherwise default to
# 127.0.0.1, localhost.
DB_ADDR=${POSTGRES_ADDRESS:=127.0.0.1}

# Check if a custom port has been set, otherwise default to '5432'.
DB_PORT=${POSTGRES_PORT:=5432}

# Launch postgres using Docker.
if [[ -z "${SKIP_DOCKER}" ]]
then
	docker run \
		-e POSTGRES_USER=${DB_USER} \
		-e POSTGRES_PASSWORD=${DB_PASSWORD} \
		-e POSTGRES_DB=${DB_NAME} \
		-p "${DB_ADDR}:${DB_PORT}":5432 \
		-d postgres \
		postgres -N 1000
		# ^ Incresed maximum number of connections for testing purpose.
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h ${DB_ADDR} -U ${DB_USER} -p ${DB_PORT} -d postgres -c '\q'; do
	>&2 echo "Postgres is still unavailable - sleeping"
	sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

# install sqlx cli.
cargo install sqlx-cli --no-default-features --features rustls,postgres

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_ADDR}:${DB_PORT}/${DB_NAME}
export DATABASE_URL

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
