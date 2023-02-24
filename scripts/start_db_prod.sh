# arranca el contenedor de producción previamente inicializado
# con i
# $ scripts/init_db_prod.sh
# $ sqlx migrate run
docker start -a erpr-db &
