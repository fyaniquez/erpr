#valores para pruebas
export POSTGRES_USER=postgres
export POSTGRES_PASSWORD=password
export POSTGRES_DB=erprdb
export POSTGRES_PORT=5432
export POSTGRES_HOST=localhost
export APP_ENVIRONMENT=local
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"
