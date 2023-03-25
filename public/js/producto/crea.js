/*
 * file: /public/js/producto/crea.js
 * scripts para administrar creacion de productos
 * author: fyaniquez
 * fecha: 27/06/2022
 */
// reemplaza el contenido de select con la lista jsonList
function generaSelect(jsonList, select) {
  select.innerHTML = "";
  const list = JSON.parse(jsonList);
  list.forEach(function (item) {
    const option = document.createElement("option");
    option.text = item.nombre;
    option.value = item.id;
    select.appendChild(option);
  });
  return select;
}

// invoca al url que recibe como parametro
// llama a la función callback para consumir el json que recibe
function loadJSON(url, callback, select) {
  const xhr = new XMLHttpRequest();
  xhr.overrideMimeType("application/json");
  xhr.open("GET", url, true);
  xhr.onreadystatechange = function () {
    if (xhr.readyState === 4 && xhr.status === 200) {
      callback(xhr.responseText, select);
    }
  };
  xhr.send(null);
}

// obtiene las marcas de la categoria proporcionada como parametro
// llena el select con ellas
function marcasXcategoria(categoria_id) {
    const url = `/categoria/${categoria_id}/categorias_marcas.json`;
    const select_marcas = document.getElementById("marca_id");
    loadJSON(url, generaSelect, select_marcas);
}
// obtiene las categorias del capitulo proporcionado como parametro
// llena el select con ellas
function categoriasXcapitulo(capitulo_id) {
    const url = `/capitulo/${capitulo_id}/categorias.json`;
    let select_categorias = document.getElementById("categoria_id");
    loadJSON(url, generaSelect, select_categorias);
    select_categorias = document.getElementById("categoria_id");
    const categoria_id = select_categorias.value;
    marcasXcategoria(categoria_id);
}

// obtiene las fabricas del pais proporcionado como parametro
// llena el select con ellas
function fabricasXpais(pais_id) {
    const url = `/pais/${pais_id}/fabricas.json`;
    const select_paises = document.getElementById("fabrica_id");
    loadJSON(url, generaSelect, select_paises);
}
// vuelve a la lista
const onClickCancela = (e) => {
    atras.click();
}

// busca las marcas de la categoria seleccionada
const onChangeCategoriaId = e => {
    marcasXcategoria(e.target.value);
}

// busca las categorias del capitulo seleccionado
const onChangeCapituloId = e => {
    categoriasXcapitulo(e.target.value);
}

// busca las fabricas del pais seleccionado
const onChangePaisId = e => {
    fabricasXpais(e.target.value);
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCrea = () => {
    cancela.addEventListener("click", onClickCancela);
    capitulo_id.addEventListener("change", onChangeCapituloId);
    categoria_id.addEventListener("change", onChangeCategoriaId);
    pais_id.addEventListener("change", onChangePaisId);
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
