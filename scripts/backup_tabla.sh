if [ $# -lt 1 ]; then
    echo "uso: $0 tabla timestamp (en formato date +'%Y%m%d%H%M%S')"
fi 
SALIDA=`pwd`/migrations/${2}_crea_tabla_${1}.sql
echo "resultado guardado en ${SALIDA}"
#/usr/bin/pg_dump --file "/tmp/$1" --host "localhost" --port "5432" --username "erp" --no-password --verbose --format=p --inserts --column-inserts --disable-dollar-quoting --encoding "UTF8" --table "public.$1" "erprustdb"
/usr/bin/pg_dump --file "${SALIDA}" --host "localhost" --port "5432" --username "erp" --verbose --format=p --inserts --column-inserts --disable-dollar-quoting --encoding "UTF8" --table "public.$1" "erprustdb"
