import * as lee from "./tablas.js"
/*
 * file: /public/js/producto/cambia.js
 * scripts para cambios en producto
 * author: fyaniquez
 * fecha: 7/12/2022
 */

// cancela la modificacion y vuelve a la lista
const onClickCancela = (e) => {
    //var sbm = document.getElementById("cancela");
    //sbm.setAttribute("novalidate", true);
    atras.click();
}

// busca las marcas de la categoria seleccionada
const onChangeCategoriaId = e => {
    lee.marcasXcategoria(e.target.value);
}

// busca las categorias del capitulo seleccionado
const onChangeCapituloId = e => {
    lee.categoriasXcapitulo(e.target.value);
}

// busca las fabricas del pais seleccionado
const onChangePaisId = e => {
    lee.fabricasXpais(e.target.value);
}
// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCambia = () => {
    cancela.addEventListener("click", onClickCancela);
    capitulo_id.addEventListener("change", onChangeCapituloId);
    categoria_id.addEventListener("change", onChangeCategoriaId);
    pais_id.addEventListener("change", onChangePaisId);
}

document.readyState === "complete" ? onLoadCambia() : addEventListener("load", onLoadCambia);
