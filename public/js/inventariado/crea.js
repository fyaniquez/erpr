/*
 * file: /public/js/inventariado/crea.js
 * scripts para administrar creacion de inventariados
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// vuelve a la lista
const onClickCancela = (e) => {
    atras.click();
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCrea = () => {
    cancela.addEventListener("click", onClickCancela);
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
