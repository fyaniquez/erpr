import {paginar} from "../paginado.js";
/*
 * file: public/js/producto/lista.js
 * scripts para administrar listas de objetos
 * requiere que la variable global window.objeto contenga el objeto
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// responde a un click en la fila de objetos y crea un precio
const onClickCrea = (e) => {
    var path = window.location.pathname;
    var partes = path.split('/');
    var catalogo = partes[partes.length - 2];
    var producto = e.target.parentElement.id;
    var query = `catalogo=${catalogo}&producto=${producto}`; 
    var url = `${window.origin}/${objeto}?${query}`;
    window.location.href = encodeURI(url);
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadLista = () => {
    window.objeto = 'precio';
    const filas = document.getElementsByClassName("lista-item");
    for (var fila of filas) {
        fila.addEventListener("click", onClickCrea);
    }
}

document.readyState === "complete" ? onLoadLista() : addEventListener("load", onLoadLista);
