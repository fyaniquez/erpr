/*
 * file: form_cambia.js
 * scripts para cambios
 * author: fyaniquez
 * fecha: 27/06/2022
 */
// cancela la modificacion y vuelve a la lista
const onClickCancela = (e) => {
    var sbm = document.getElementById("cancela");
    sbm.setAttribute("novalidate", true);
    var url = `${window.origin}/capitulos`;
    window.location.replace(encodeURI(url));
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadPaginado = () => {
    cancela.addEventListener("click", onClickCancela);
}

document.readyState === "complete" ? onLoadPaginado() : addEventListener("load", onLoadPaginado);
