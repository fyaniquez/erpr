/*
 * file: /public/js/venta/crea.js
 * scripts para administrar creacion de ventas
 * author: fyaniquez
 * fecha: 27/06/2022
 */
// valida detalle ingresado
const validaDetalle = () => {
    const producto_id_val = validaEntero(window.producto_id);
    if (producto_id_val < 1) {
        alert("Debe introducir un id del producto válido");
        producto_id.value = producto_id_val;
        producto_id.focus();
        return null;
    }
    const cantidad = document.getElementById("cantidad");
    const cantidad_val = validaDecimal("cantidad");
    if ( cantidad_val <= 0.0 ) {
        alert("Debe introducir la cantidad del producto vendido");
        cantidad.value = cantidad_val;
        cantidad.focus();
        return null;
    }
    const descuento = document.getElementById("descuento");
    const descuento_val = validaDecimal("descuento");
    if ( descuento_val < 0.0 ) {
        alert("Debe introducir un descuento mayor o igual a cero");
        descuento.value = descuento_val;
        descuento.focus();
        return null;
    }
    const precio = document.getElementById("precio").value;
    const producto = document.getElementById("producto").value;
    const total = document.getElementById("total").value;
    return {
        producto_id: producto_id_val,
        producto: producto,
        precio: precio,
        cantidad: cantidad_val,
        descuento: descuento_val,
        total: total
    };
}
// valida producto_id
const validaEntero = (numero) => {
    const entero = parseInt(numero.value);
    if (isNaN(entero) || entero < 1) return 0;
    return entero;
}
// valida decimal
const validaDecimal = (numero) => {
    const contenido = document.getElementById(numero);
    const decimal = parseFloat(contenido.value);
    if (isNaN(decimal) || decimal <= 0) return 0.0;
    return Math.trunc(decimal *  100.0) / 100.0;
}
// agrega item
const agregaItem = (detalle) => {
    var fila = window.fila + 1;
    const nuevo_item = `<div><div>${detalle.producto_id}</div><div>${detalle.producto}</div><div>${detalle.precio}</div><div>${detalle.cantidad}</div><div>${detalle.descuento}</div><div>${detalle.total}</div><div id="fila-${fila}"><img src="/img/waste-24.png"/></div></div>`;
    var item = document.getElementById("det_fila");
    item.insertAdjacentHTML('beforebegin', nuevo_item);
    window.fila = fila;
    var nf = document.getElementById(`fila-${fila}`);
    nf.addEventListener("click", onClickBorraItem);
}

// obtiene producto
const obtieneProducto = async id => {
    const response = await fetch(`/precio/${id}.json`);
    return response.json();
}

// obtiene lista productos
const obtieneListaProductos = async producto => {
    const response = await fetch(`/precio/${id}.json`);
    return response.json();
}

// muestra producto en formulario
const muestraProducto = producto => {
    window.producto.innerText = producto.nombre;
    window.precio.innerText = producto.precio;
}

// agrega item
const onClickAgregaItem = (e) => {
    const detalle = validaDetalle();
    if (detalle === null) return false;
    agregaItem(detalle);
}

// borra item
const onClickBorraItem = (e) => {
    const padre = e.target.parentElement;
    const nombre = padre.children[1].innerText;
    if (confirm(`¿Esta seguro de eliminar el producto: ${nombre}?`)) {
        e.target.removeEventListener("click", onClickBorraItem);
        padre.remove();
    }
}

// obtiene producto a partir de su id
const onChangeProductoId = (async e) => {
    const id = validaEntero(e.target);
    if (id < 1) {
        window.producto.focus();
        return;
    }
    var producto = await obtieneProducto(id);
    muestraProducto(producto);
}

// muestra un popup de productos a medida que el usuario teclea nombre
const onKeyupProducto = async e => {
    var producto = e.target.value.trim();
    if (producto == '') return;
    var lista = await obtieneLista(producto);
    muestraListaProductos(lista)
}

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCrea = () => {
    window.agrega_item.addEventListener("click", onClickAgregaItem);
    window.producto_id.addEventListener("change" onChangeProductoId);
    window.producto.addEventListener("keyup" onKeyupProducto);
    window.fila = 0;
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
