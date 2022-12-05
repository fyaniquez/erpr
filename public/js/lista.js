import {paginar} from './paginado.js';
/*
 * file: lista.js
 * scripts para administrar listas de objetos
 * author: fyaniquez
 * fecha: 27/06/2022
 */
// responde a un click en la fila de capitulos
const onClickFila = (e) => {
    var url = `${window.origin}/capitulo/${e.target.parentElement.id}`;
    window.location.replace(encodeURI(url));
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadPaginado = () => {
    const filas = document.getElementsByClassName("lista-item");
    for (var fila of filas) {
        fila.addEventListener("click", onClickFila);
    }
}

document.readyState === "complete" ? onLoadPaginado() : addEventListener("load", onLoadPaginado);
