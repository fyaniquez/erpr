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
 var pos_int = location.href.indexOf('?');
    var path;
    if ( pos_int >= 0 )
        path = location.href.substr(0, pos_int);
    else
        path = location.href;
    var urlpath = path.replace('productos', 'precio');
    var producto = e.target.parentElement.id;
    var url = `${urlpath}?producto=${producto}`;
    window.location.replace(encodeURI(url));
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
