/*
 * file: ve.js
 * scripts operaciones sobre vista del objeto
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// elimina el objeto
const onClickBorra = async (e) => {
    if (!confirm(confirmacion))
        return;
    const response = await fetch(
        location.href, 
        { method: 'DELETE' });
    if (!response.ok)
        if(response.status != 405)
           throw `Error: ${response.status}`;
    // lanza el enlace de retorno a pantalla anterior
    atras.click();
}

// redirige al formulario de modificacion
const onClickCambia = (e) => {
    var url = `${location.href}/cambia`;
    window.location.replace(encodeURI(url));
}

// lista de marcas del inventario
const onClickSubLista = (e) => {
    var url = `${location.href}/${hijos}`;
    window.location.replace(encodeURI(url));
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadVe = () => {
    window.hijos = 'inventariados';
    
    var nombre = document.getElementById('nombre').innerText;
    var confirmacion = `¿Esta seguro de eliminar inventario: ${nombre}?`;

    borra.addEventListener("click", onClickBorra);
    cambia.addEventListener("click", onClickCambia);
    sublista.addEventListener("click", onClickSubLista);
}

document.readyState === "complete" ? 
    onLoadVe() : addEventListener("load", onLoadVe);
