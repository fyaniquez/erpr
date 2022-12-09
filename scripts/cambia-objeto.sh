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

if [ ! -d "${RUTAS}/tmp" ]; then
    mkdir ${RUTAS}/tmp
fi

if [ ! -d "$RUTAS/$1" ]; then
    echo "debe existir $RUTAS/$1 para copiarlo\n"
    exit 1
fi

mkdir $RUTAS/$3
touch $RUTAS/$3/mod.rs

MODELO=src/modelo
touch $MODELO/$3.rs
echo "pub mod $3;" >> $MODELO/mod.rs

## copia programas
cp -r $RUTAS/$1 $RUTAS/tmp/$3
echo "pub mod $3;" >> $RUTAS/mod.rs

MANT=`echo $1 | sed 's/^./\U&/'`
MNUE=`echo $3 | sed 's/^./\U&/'`
 
for i in borra cambia crea json lista ve; do
    for j in $RUTAS/tmp/$3/$i/*.rs; do
        sed -i "s/${2}/${4}/g" $j 
        sed -i "s/${1}/${3}/g" $j 
        sed -i "s/${MANT}/${MNUE}/g" $j 
    done
done

## copia scripts
JS=public/js
if [ ! -d ${JS} ]; then
    mkdir ${JS}
fi

cp -r $JS/$1 $JS/$3

## ajusta startup
echo ".service(${3}::lista::get::formulario)" >> ${ACTUAL}/src/startup.rs
echo ".service(${3}::ve::get::pantalla)" >> ${ACTUAL}/src/startup.rs
echo ".service(${3}::crea::get::formulario)" >> ${ACTUAL}/src/startup.rs
echo ".service(${3}::crea::post::proceso)" >> ${ACTUAL}/src/startup.rs
echo ".service(${3}::cambia::get::formulario)" >> ${ACTUAL}/src/startup.rs
echo ".service(${3}::cambia::post::proceso)" >> ${ACTUAL}/src/startup.rs
echo ".service(${3}::borra::delete::proceso)" >> ${ACTUAL}/src/startup.rs
