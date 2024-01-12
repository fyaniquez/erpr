/*
 * js/popup.jss
 * author: fyaniquez
 * date: 12/10/2022
 * purpose: muestra una lista en el popup popup
 */

// cierra el popup
const onClickPopupClose = (e) => {
    window.popup_tabla.removeEventListener("click", () => { return null; });
    window.popup.style.display = 'none';
    window.det_nombre.select();
    window.det_nombre.focus();
}

// muestra lista de elementos en un popup
export const popupMuestra = (popup, elementos) => {
    // borra la tabla anterior
    const tabla_ant = document.getElementById("popup_tabla");
    if (tabla_ant) tabla_ant.remove();

    // crea nueva tabla
    (window.popup_titulo).innerText = popup.titulo;
    const tabla = document.createElement("table");
    tabla.classList.add("popup-tabla");
    tabla.id = "popup_tabla";

    // dimensiona las columnas y coloca nombres de columnas
    const formato = document.createElement("colgroup");
    const cabecera = document.createElement("tr");
    cabecera.classList.add("popup-tabla-fila");
    for (var i = 0; i <  popup.columnas.length; i++) {
        const columna = document.createElement("col");
        columna.style.width = popup.columnas[i].ancho;
        formato.append( columna );
        const titulo = document.createElement("th");
        titulo.innerText = popup.columnas[ i ].nombre;
        cabecera.append( titulo );
    }
    tabla.append( formato, cabecera);

    // agrega las filas obtenidas de la bd
    for (var i = 0; i < elementos.length; i++) {
        const fila = document.createElement("tr");
        fila.id = elementos[i][popup.columna_id];
        fila.classList.add("popup-tabla-fila");
        for (var j = 0; j <  popup.columnas.length; j++) {
            const data = document.createElement("td");
            data.innerText = elementos[i][popup.columnas[j].columna];
            data.style.textAlign = elementos[i][popup.columnas[j].alineacion];
            fila.append(data);
        }
        tabla.append(fila);
    }

    // agrega el evento para capturar el elemento seleccionado
    tabla.addEventListener("click", popup.onClickFila);

    // muestra el popup
    window.popup_contenido.append(tabla);
    window.popup.style.display = 'block';
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadPopup = () => {
    window.popup.addEventListener("click", onClickPopupClose);
}
document.readyState === "complete" ? onLoadPopup() : addEventListener("load", onLoadPopup);
