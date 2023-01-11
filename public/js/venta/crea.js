/*
 * file: /public/js/venta/crea.js
 * scripts para administrar creacion de ventas
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// vuelve a la lista
const onClickAgregaItem = (e) => {
    // inserta nueva fila
    window.fila += 1;
    var item = document.getElementById("fila-0");
    var nuevo_item = item.outerHTML.replaceAll("-0", '-'+window.fila);
    item.insertAdjacentHTML('beforebegin', nuevo_item);
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCrea = () => {
    agrega_item.addEventListener("click", onClickAgregaItem);
    //fila
    window.fila = 0;
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
