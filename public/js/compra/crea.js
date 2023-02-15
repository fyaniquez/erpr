/*
 * file: /public/js/compra/crea.js
 * scripts para administrar creacion de compras
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
    const costo = document.getElementById("costo");
    const costo_val = validaDecimal("costo");
    if ( costo_val <= 0.0 ) {
        alert("Debe introducir el costo del producto comprado");
        costo.value = costo_val;
        costo.focus();
        return null;
    }
    const cantidad = document.getElementById("cantidad");
    const cantidad_val = validaDecimal("cantidad");
    if ( cantidad_val <= 0.0 ) {
        alert("Debe introducir la cantidad del producto comprado");
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
    return {
        id: producto_id_val,
        producto: window.producto.value,
        costo: costo_val,
        cantidad: cantidad_val,
        descuento: descuento_val,
        total: window.total_detalle.value,
        vencimiento: window.vencimiento.value
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
// valida decimal si error usa defa(ult)
const validaDec = (numero, defa) => {
    const decimal = parseFloat(numero);
    if (isNaN(decimal) || decimal <= 0) return defa;
    return Math.trunc(decimal *  100.0) / 100.0;
}
// agrega item a la tabla de comprados
const agregaDetalle = (detalle) => {
    var fila = window.fila + 1;
    const nuevo_item = `<div><div>${detalle.id}</div>
        <div>${detalle.producto}</div><div>${detalle.costo}</div>
        <div>${detalle.cantidad}</div><div>${detalle.descuento}</div>
        <div>${detalle.total}</div><div>${detalle.vencimiento}</div>
        <div id="fila-${fila}">
        <img src="/img/waste-24.png"/></div></div>`;
    var item = window.totales;
    item.insertAdjacentHTML('beforebegin', nuevo_item);
    window.fila = fila;
    var nf = document.getElementById(`fila-${fila}`);
    nf.addEventListener("click", onClickBorraItem);
}

// modifica y muestra los totales de la tabla
const muestraTotalTabla = (detalle) => {
    var t_total = validaDec(window.t_total.innerText, 0);
    var t_descuento = validaDec(window.t_descuento.innerText, 0);

    t_total += +detalle.total;
    t_descuento += +detalle.descuento;

    window.t_total.innerText = parseFloat(t_total).toFixed(2);
    window.t_descuento.innerText = parseFloat(t_descuento).toFixed(2);

// muestra total general
    var g_total = validaDec(window.total.value, 0);
    var g_descuento = validaDec(window.descuento.value, 0);

    window.total.value = t_total - g_descuento;
}

// obtiene producto
const obtieneProducto = async id => {
    const response = await fetch(`/producto/${id}.json`);
    return response.json();
}

// obtiene lista productos
const obtieneListaProductos = async producto => {
    var filtro = encodeURIComponent(producto);
    const response = await fetch(`/productos.json?filtro=${filtro}`);
    return response.json();
} 

// muestra productos en un popup
const muestraListaProductos = (lista) => {
    (window.busqueda_titulo).innerText = 'Productos';
    var tbl = '';
    for (var i = 0; i < lista.length; i++) {
        const fil = `<div id="${lista[i].producto_id}">
            <div>${lista[i].nombre}</div>
            <div>${lista[i].costo}</div></div>`;
        tbl += fil;
    }
    window.busqueda.innerHTML = tbl;
    window.busqueda.style.display = 'block';
}

const calculaCampos = (producto) => {
    var costo = 0;
    var cantidad = 1;
    var descuento = 0;
    var total = 0;

    return { cantidad, descuento, costo, total };
}

// muestra producto en formulario
const muestraProductoFrm = producto => {
    window.producto_id.value = producto.id;
    window.producto.value = producto.nombre;
    window.costo.value = 0;
    window.cantidad.value = 1;
    window.descuento.value = 0;
    window.total.value = 0;
}

// recopila los datos y los incluye en una estructura json
const creaJson = () => {
    var producto_ids = [];
    var costos = [];
    var cantidads = [];
    var descuentos = [];
    var vencimientos = [];
    var totals = [];
    var fec;
    for (var i=1; i < window.det_tabla.children.length - 1; i++) {
        var f = window.det_tabla.children[i].children;
        producto_ids.push( +f[0].innerText );
        costos.push( +f[2].innerText );
        cantidads.push( +f[3].innerText );
        descuentos.push( +f[4].innerText );
        fec = new Date( f[6].innerText );
        vencimientos.push( fec.toISOString() );
        totals.push( +f[5].innerText );
    }
        
    return {
        compra: {
            total: +window.total.value,
            descuento: +window.descuento.value,
            medio_id: +window.medio.value,
            documento: window.documento.value,
            observaciones: window.observaciones.value,
        },
        comprados: {
            producto_ids: producto_ids,
            costos: costos,
            cantidads: cantidads,
            descuentos: descuentos,
            vencimientos: vencimientos,
            totals: totals,
        }
    }
}
// envia compra al servidor para grabarla
const grabaCompra = async compra => {
    var url = location.href;
    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(compra)
    });
    return response.json();
}

const recalculaDetalle = () => {
    var cantidad = validaDec(window.cantidad.value, 1);   
    var costo = validaDec(window.costo.value, 0);
    var descuento = validaDec(window.descuento_detalle.value, 0);
    window.total_detalle.value = cantidad * costo - descuento;
}

// cambia el total general
const onChangeDescuento = (e) => {
    var descuento = validaDec(window.descuento.value, 0);
    window.total.value = window.total.value - descuento;
}

// agrega item
const onClickAgregaItem = (e) => {
    const detalle = validaDetalle();
    if (detalle === null) return false;
    var dt = document.getElementById("det_fila");
    if (dt !== null) dt.remove();
    agregaDetalle(detalle);
    muestraTotalTabla(detalle);
}

// borra item
const onClickBorraItem = (e) => {
    const padre = e.target.parentElement;
    const nombre = padre.children[1].value;
    if (confirm(`¿Esta seguro de eliminar el producto: ${nombre}?`)) {
        e.target.removeEventListener("click", onClickBorraItem);
        padre.remove();
    }
}

// obtiene producto a partir de su id
const onChangeProductoId = async e => {
    const id = validaEntero(e.target);
    if (id < 1) {
        window.producto_id.focus();
        return;
    }
    var producto = await obtieneProducto(id);
    muestraProductoFrm(producto);
    window.costo.focus();
}

// muestra un popup de productos a medida que el usuario teclea nombre
const onKeyupProducto = async e => {
    var producto = e.target.value.trim();
    if (producto == '') return;
    var lista = await obtieneListaProductos(producto);
    muestraListaProductos(lista)
}

const onChangeCantidad = e => {
    recalculaDetalle();
    window.descuento.focus();
}

const onChangeCosto = e => {
    recalculaDetalle();
    window.cantidad.focus();
}

const onChangeDescuentoDetalle = e => {
    recalculaDetalle();
    window.vencimiento.focus();
}

// pasa el elemento seleccionado del popup al formulario
const onClickBusqueda = e => {
    var fil = e.target.parentNode;
    if (window.busqueda_titulo.innerText === 'Productos') {
        var prod = {
            id: fil.id,
            nombre: fil.children[0].innerText,
            costo: fil.children[1].innerText,
        }
        muestraProductoFrm(prod);
    } else {
        var cli = {
            id: fil.id,
            nombre: fil.children[0].innerText,
            documento: fil.children[1].innerText,
        }
        muestraCliente(cli);
    }
    window.busqueda.style.display = 'none';
}
// graba la compra en la base de datos
const onClickCrea = async e => {
    var compra = creaJson();
    var compra_id = await grabaCompra(compra);
    var url =  `/compra/${compra_id}`;
// TODO: manejar los errores
    window.location.href = url;
}
// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCrea = () => {
    window.descuento.addEventListener("change", onChangeDescuento);
    window.agrega_item.addEventListener("click", onClickAgregaItem);
    window.producto_id.addEventListener("change", onChangeProductoId);
    window.producto.addEventListener("keyup", onKeyupProducto);
    window.costo.addEventListener("change", onChangeCosto);
    window.cantidad.addEventListener("change", onChangeCantidad);
    window.descuento_detalle.addEventListener(
        "change", onChangeDescuentoDetalle);
    window.busqueda.addEventListener("click", onClickBusqueda);
    window.crea.addEventListener("click", onClickCrea);
    window.fila = 0;
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
