/*
 * file: /public/js/empresa/crea.js
 * scripts para administrar creacion de empresas
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// vuelve a la lista
const onClickCancela = (e) => {
    //sbm.setAttribute("novalidate", true);
    atras.click();
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCrea = () => {
    cancela.addEventListener("click", onClickCancela);
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
