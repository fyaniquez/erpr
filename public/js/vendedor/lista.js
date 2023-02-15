import {paginar} from "../paginado.js";
/*
 * file: public/js/vendedor/lista.js
 * scripts para administrar listas de objetos
 * requiere que la variable global window.objeto contenga el objeto
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// responde a un click en la fila de objetos
const onClickVe = (e) => {
    var url = `${window.origin}/${objeto}/${e.target.parentElement.id}`;
    window.location.replace(encodeURI(url));
}

// llama al formulario de alta de objeto
const onClickCrea = (e) => {
    var partes = location.href.split('/');
    var distribuidora = partes[ partes.length - 2 ];
    var url = `${window.origin}/${objeto}?distribuidora=${distribuidora}`;
    window.location.replace(encodeURI(url));
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadLista = () => {
    window.objeto = 'vendedor';
    const filas = document.getElementsByClassName("lista-item");
    for (var fila of filas) {
        fila.addEventListener("click", onClickVe);
    }
    crea.addEventListener("click", onClickCrea);
}

document.readyState === "complete" ? onLoadLista() : addEventListener("load", onLoadLista);
