/*
 * file: ve.js
 * scripts operaciones sobre vista del capitulo
 * author: fyaniquez
 * fecha: 27/06/2022
 */
// obtiene el id a partir del url
const get_id = () => {
    var url = location.href;
    var p = url.lastIndexOf('/') + 1;
    return url.substring(p);
}
// obtiene la lista de categorias para el capitulo
const onClickPaginas = (e) => {
    var capitulo = get_id();
    var url = `${window.origin}/capitulo/${capitulo}/categorias`;
    window.location.replace(encodeURI(url));
}

// redirige al formulario de modificacion
const onClickCambia = (e) => {
    var capitulo = get_id();
    var url = `${window.origin}/capitulo/${capitulo}/cambia`;
    window.location.replace(encodeURI(url));
}

// elimina el capitulo
const onClickBorra = async (e) => {
    var nombre = document.getElementById('nombre').innerText;
    if (!confirm(`¿Esta seguro de eliminar el capítulo: ${nombre}?`))
        return;
    var capitulo = get_id();
    var url = `${window.origin}/capitulo/${capitulo}`;
    const response = await fetch(url, { method: 'DELETE' });
    if (!response.ok)
        if(response.status != 405)
           throw `Error: ${response.status}`;
    var lista = `${window.origin}/capitulos`;
    window.location.replace(encodeURI(lista));
}

// lista de categorias del capitulo
const onClickCategorias = (e) => {
    var capitulo = get_id();
    var url = `${window.origin}/capitulo/${capitulo}/categorias`;
    window.location.replace(encodeURI(url));
}
// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadPaginado = () => {
    borra.addEventListener("click", onClickBorra);
    cambia.addEventListener("click", onClickCambia);
    categorias.addEventListener("click", onClickCategorias);
}

document.readyState === "complete" ? onLoadPaginado() : addEventListener("load", onLoadPaginado);
