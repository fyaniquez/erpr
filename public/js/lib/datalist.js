/*
 * js/lib/datalist-datalist.jss
 * author: fyaniquez
 * date: 12/10/2022
 * purpose: muestra una lista progresiva resuando un datalist
 */

// muestra lista de elementos en un datalist
export const carga = (datalist, contenido, columnas, elementos) => {
    // borra los elementos anteriores
    datalist.innerHTML = "";

    // crea nuevos elementos
    for (var i = 0; i < elementos.length; i++) {
        const opcion = document.createElement("option");
        opcion.value = `${elementos[i]["nombre"]}|${elementos[i]["producto_id"]}`;
        datalist.append(opcion);
    }
};

