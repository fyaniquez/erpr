#!/usr/bin/env bash
# antes de correr este script debe ejecutarse
# $ source init_vars_prod.sh o init_vars_pru.sh
# queda pendiente programar que el script aguarde hasta que 
# la nueva base de datos este lista
set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=erprdb}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=172.18.0.2}"

# si una DB postgres esta corriendo saltarla
if [[ -z "${SKIP_DOCKER}" ]]
then
docker run \
    --name erpr-db \
    --network erpr-net \
    --mount source=erpr-vol,target=/var/lib/postgresql \
	-e POSTGRES_USER=${DB_USER} \
	-e POSTGRES_PASSWORD=${DB_PASSWORD} \
	-e POSTGRES_DB=${DB_NAME} \
	-p "${DB_PORT}":5432 \
	-d postgres:11-alpine \
	postgres -N 1000
fi

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
