if [ $# -lt 1 ]; then
    echo "uso: $0 tabla"
fi 
echo "resultado guardado en /tmp/$1"
#/usr/bin/pg_dump --file "/tmp/$1" --host "localhost" --port "5432" --username "erp" --no-password --verbose --format=p --inserts --column-inserts --disable-dollar-quoting --encoding "UTF8" --table "public.$1" "erprustdb"
/usr/bin/pg_dump --file "/tmp/$1" --host "localhost" --port "5432" --username "erp" --verbose --format=p --inserts --column-inserts --disable-dollar-quoting --encoding "UTF8" --table "public.$1" "erprustdb"
