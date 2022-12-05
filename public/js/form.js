/*
 * file: form.js
 * scripts para administrar formularios
 * author: fyaniquez
 * fecha: 27/06/2022
 */
// vuelve a la lista
const onClickLista = (e) => {
    sbm.setAttribute("novalidate", true);
    var url = `${window.origin}/capitulos`;
    window.location.replace(encodeURI(url));
}
// cancela la adición y vuelve a la lista
const onClickCancela = (e) => {
    var sbm = document.getElementById("crea");
    sbm.setAttribute("novalidate", true);
    var url = `${window.origin}/capitulos`;
    window.location.replace(encodeURI(url));
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadPaginado = () => {
    cancela.addEventListener("click", onClickCancela);
    lista.addEventListener("click", onClickLista);
}

document.readyState === "complete" ? onLoadPaginado() : addEventListener("load", onLoadPaginado);
