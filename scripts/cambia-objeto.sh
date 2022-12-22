# crea un nuevo objeto a partir de uno anterior
# y reemplaza las ocurrencias del objeto anterior en sus archivos
# $1 objeto anterior
# $2 objeto nuevo

## validaciones
if [ $# -lt 4 ]; then
    echo "uso: $0 objeto_anterior plural objeto_nuevo plural\n"
    exit 1
fi

ACTUAL=`basename \`pwd\``
if [ ! -d "src" -o ! -d "public" ]; then
    echo "deben existir las carpetas $ACTUAL/src y $ACTUAL/public"
    exit 1
fi
RUTAS=src/rutas
if [ ! -d "$RUTAS" ]; then
    echo "debe existir la carpeta $RUTAS"
    exit 1
fi

if [ ! -d "$RUTAS/$1" ]; then
    echo "debe existir $RUTAS/$1 para copiarlo\n"
    exit 1
fi

mkdir $RUTAS/$3
cp $RUTAS/$1/mod.rs $RUTAS/$3
echo "pub mod $3;" >> $RUTAS/mod.rs

## copia programas
cp -r $RUTAS/$1/* $RUTAS/$3

MANT=`echo $1 | sed 's/^./\U&/'`
MNUE=`echo $3 | sed 's/^./\U&/'`
 
for i in borra cambia crea lista ve; do
    for j in $RUTAS/$3/$i/*.rs; do
        sed -i "s/${2}/${4}/g" $j 
        sed -i "s/${1}/${3}/g" $j 
        sed -i "s/${MANT}/${MNUE}/g" $j 
    done
done

## copia domain
DOMAIN=src/domain
mkdir $DOMAIN/$3
echo "pub mod $3;" >> $DOMAIN/mod.rs
cp -r $DOMAIN/$1/* $DOMAIN/$3
mv $DOMAIN/$3/$1.rs $DOMAIN/$3/$3.rs
for i in $DOMAIN/$3/*; do
    sed -i "s/${1}/${3}/g" $i 
    sed -i "s/${MANT}/${MNUE}/g" $i 
done

## copia javascripts
JS=public/js
if [ ! -d ${JS} ]; then
    mkdir ${JS}
fi
if [ ! -d ${JS}/$3 ]; then
    mkdir ${JS}/$3
fi

cp -r $JS/$1/* $JS/$3
for i in cambia crea lista ve; do
    sed -i "s/${1}/${3}/g" $JS/$3/$i.js
done

## ajusta startup
sed "s/OBJETO/$3/" scripts/startup.mod.vi > scripts/startup.vi
vi -u NONE -s scripts/startup.vi src/startup.rs

## ajusta migracion
TIMES=`date +'%Y%m%d%H%M%S'`
scripts/backup_tabla.sh $4 $TIMES
sed "s/OBJETO/$4/" scripts/triggers.mod.vi >> migrations/${TIMES}_crea_tabla_${4}.sql
