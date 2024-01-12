 
/*
 * file: /public/js/lib/select-dinamico.js
 * scripts para cargar dinámicamente el contenido de un select
 * author: fyaniquez
 * fecha: 27/06/2022
 */
// reemplaza el contenido de select con la lista jsonList
function genera(jsonList, select) {
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
export function cargaSelect(url, select) {
  const xhr = new XMLHttpRequest();
  xhr.overrideMimeType("application/json");
  xhr.open("GET", url, true);
  xhr.onreadystatechange = function () {
    if (xhr.readyState === 4 && xhr.status === 200) {
      genera(xhr.responseText, select);
    }
  };
  xhr.send(null);
}
