import * as lee from "./tablas.js";
/*
 * file: /public/js/producto/crea.js
 * scripts para administrar creacion de productos
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// vuelve a la lista
const onClickCancela = (e) => {
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
const onLoadCrea = () => {
    cancela.addEventListener("click", onClickCancela);
    capitulo_id.addEventListener("change", onChangeCapituloId);
    categoria_id.addEventListener("change", onChangeCategoriaId);
    pais_id.addEventListener("change", onChangePaisId);
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
