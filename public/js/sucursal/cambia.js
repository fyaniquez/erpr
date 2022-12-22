/*
 * file: /public/js/sucursal/cambia.js
 * scripts para cambios en sucursal
 * author: fyaniquez
 * fecha: 7/12/2022
 */

// cancela la modificacion y vuelve a la lista
const onClickCancela = (e) => {
    var sbm = document.getElementById("cancela");
    sbm.setAttribute("novalidate", true);
    atras.click();
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCambia = () => {
    cancela.addEventListener("click", onClickCancela);
}

document.readyState === "complete" ? onLoadCambia() : addEventListener("load", onLoadCambia);
