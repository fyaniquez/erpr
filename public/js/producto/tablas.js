
import {cargaSelect} from "../lib/select-dinamico.js";

// obtiene las marcas de la categoria proporcionada como parametro
// llena el select con ellas
export function marcasXcategoria(categoria_id) {
    const url = `/categoria/${categoria_id}/categorias_marcas.json`;
    const select_marcas = document.getElementById("marca_id");
    cargaSelect(url, select_marcas);
}
// obtiene las categorias del capitulo proporcionado como parametro
// llena el select con ellas
export function categoriasXcapitulo(capitulo_id) {
    const url = `/capitulo/${capitulo_id}/categorias.json`;
    let select_categorias = document.getElementById("categoria_id");
    cargaSelect(url, select_categorias);
    select_categorias = document.getElementById("categoria_id");
    const categoria_id = select_categorias.value;
    marcasXcategoria(categoria_id);
}

// obtiene las fabricas del pais proporcionado como parametro
// llena el select con ellas
export function fabricasXpais(pais_id) {
    const url = `/pais/${pais_id}/fabricas.json`;
    const select_paises = document.getElementById("fabrica_id");
    cargaSelect(url, select_paises);
}
