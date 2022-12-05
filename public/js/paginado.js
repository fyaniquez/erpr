/*
 * file: paginado.js
 * libreria para manejar listas paginadas simples
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// genera el query string y llamar a la página correspondiente
export const paginar = (pagina) => {
    var url = window.location.href;
    url = url.substring(0, url.indexOf('?')) + '?';
    url += `pagina=${pagina}`;
    window.location.replace(encodeURI(url.substring(0, url.length)));
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

// llama al formulario de alta de capitulo
const onClickAgrega = (e) => {
    var url = `${window.origin}/capitulo`;
    window.location.replace(encodeURI(url));
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadPaginado = () => {
    paginas.addEventListener("click", onClickPaginas);
    agrega.addEventListener("click", onClickAgrega);
}
document.readyState === "complete" ? onLoadPaginado() : addEventListener("load", onLoadPaginado);
