/*
 * file: paginado.js
 * libreria para manejar listas paginadas simples
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// genera el query string y llamar a la página correspondiente
export const paginar = (pagina) => {
    var origin = window.location.origin;
    var pathname = window.location.pathname;
    var url = `${origin}${pathname}?pagina=${pagina}`;
    var href = window.location.href;
    var fil = filtro.value.trim();
    if (fil !== '' && href.indexOf('filtro') > 0)
        url += `&filtro=${fil}`;
    window.location.href = encodeURI(url);
}

// genera el query string para busqueda y llamar a la página 1
export const filtrar = (filtro) => {
    var origin = window.location.origin;
    var pathname = window.location.pathname;
    var fil = filtro.trim();
    var url = `${origin}${pathname}`;
    if (fil !== '')
        url += `?filtro=${filtro}`;
    window.location.href = encodeURI(url);
}

// eventos que generan un cambio de página
const onClickPaginas = (e) => {
    let btn = e.target.id;
    switch(btn) {
        case "actual": 
            break;
        case "primero":
            paginar(1);
            break;
        case "ultimo":
            paginar(parseInt(ultimo.dataset.index));
            break;
        case "previo":
            var p = parseInt(actual.innerText) - 1;
            p < 1 ?  paginar(1) : paginar(p);
            break;
        case "siguiente":
            var p = parseInt(actual.innerText) + 1;
            var u = parseInt(ultimo.dataset.index);
            p > u ? paginar(u) : paginar(p);
            break;
        default:
            paginar(parseInt(e.target.innerText));
            break;
    }
};

// click en la lupa de busqueda
const onClickBuscar = (e) => {
    filtrar(filtro.value);
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadPaginado = () => {
    paginas.addEventListener("click", onClickPaginas);
    buscar.addEventListener("click", onClickBuscar);
}

document.readyState === "complete" ? onLoadPaginado() : addEventListener("load", onLoadPaginado);
