/*
 * file: /public/js/venta/crea.js
 * scripts para administrar creacion de ventas
 * author: fyaniquez
 * fecha: 27/06/2022
 */
// valida detalle ingresado
const validaDetalle = () => {
    const producto_id_val = validaEntero(window.producto_id_frm);
    if (producto_id_val < 1) {
        alert("Debe introducir un id del producto válido");
        producto_id.value = producto_id_val;
        producto_id.focus();
        return null;
    }
    const cantidad = document.getElementById("cantidad_frm");
    const cantidad_val = validaDecimal("cantidad_frm");
    if ( cantidad_val <= 0.0 ) {
        alert("Debe introducir la cantidad del producto vendido");
        cantidad.value = cantidad_val;
        cantidad.focus();
        return null;
    }
    const descuento = document.getElementById("descuento_frm");
    const descuento_val = validaDecimal("descuento_frm");
    if ( descuento_val < 0.0 ) {
        alert("Debe introducir un descuento mayor o igual a cero");
        descuento.value = descuento_val;
        descuento.focus();
        return null;
    }
    const precio = document.getElementById("precio_frm").innerText;
    const producto = document.getElementById("producto_frm").innerText;
    const total = document.getElementById("total_frm").value;
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
// valida decimal si error usa defa(ult)
const validaDec = (numero, defa) => {
    const decimal = parseFloat(numero);
    if (isNaN(decimal) || decimal <= 0) return defa;
    return Math.trunc(decimal *  100.0) / 100.0;
}
// agrega item a la tabla de vendidos
const agregaItem = (detalle) => {
    var fila = window.fila + 1;
    var precio = parseFloat(detalle.precio);
    var cantidad = parseFloat(detalle.cantidad);
    var descuento = parseFloat(detalle.descuento);
    var total = parseFloat(detalle.total);
    const nuevo_item = `<div><div>${detalle.producto_id}</div>
        <div>${detalle.producto}</div><div>${precio}</div>
        <div>${cantidad}</div><div>${descuento}</div>
        <div>${total}</div><div id="fila-${fila}">
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

    t_total += detalle.total;
    t_descuento += detalle.descuento;

    window.t_total.innerText = parseFloat(t_total);
    window.t_descuento.innerText = parseFloat(t_descuento);
}

// obtiene producto
const obtieneProducto = async id => {
    const response = await fetch(`/precio/${id}.json`);
    return response.json();
}

// obtiene lista productos
const obtieneListaProductos = async producto => {
    var catalogo = window.catalogo_id.value;
    var filtro = encodeURIComponent(producto);
    const response = await 
        fetch(`/catalogo/${catalogo}/precios.json?filtro=${filtro}`);
    return response.json();
} 
// muestra productos en un popup
const muestraListaProductos = (lista) => {
    (window.busqueda_titulo).innerText = 'Productos';
    var tbl = '';
    for (var i = 0; i < lista.length; i++) {
        const fil = `<div id="${lista[i].producto_id}">
            <div>${lista[i].nombre}</div>
            <div>${lista[i].precio}</div></div>`;
        tbl += fil;
    }
    window.busqueda.innerHTML = tbl;
    window.busqueda.style.display = 'block';
}

const calculaCampos = (producto) => {
    var precio = +producto.precio / 100;
    var cantidad = 1;
    var descuento = 0;
    var total = precio * cantidad - descuento;

    return { cantidad, descuento, precio, total };
}

// muestra producto en formulario
const muestraProductoFrm = producto => {
    var campos = calculaCampos(producto);
    window.producto_id_frm.value = producto.id;
    window.producto_frm.value = producto.nombre;
    window.precio_frm.innerText = campos.precio;
    window.cantidad_frm.value = campos.cantidad;
    window.descuento_frm.value = campos.descuento;
    window.total_frm.value = campos.total;
}

// muestra cliente en formulario
const muestraCliente = cliente => {
    window.cliente_id.value = cliente.id;
    window.nombre.value = cliente.nombre;
    window.documento.value = cliente.documento;
}

// obtiene  cliente
const obtieneCliente = async id => {
    const response = await fetch(`/cliente/${id}.json`);
    return response.json();
}

// obtiene  cliente por documento
const obtieneClienteDocumento = async documento => {
    const response = await fetch(`/cliente/doc_${documento}.json`);
    return response.json();
}

// obtiene lista clientes
const obtieneListaClientes = async cliente => {
    const response = await fetch(`/clientes.json?filtro=${cliente}`);
    return response.json();
} 
// muestra productos en un popup
const muestraListaClientes = (lista) => {
    (window.busqueda_titulo).innerText = 'Clientes';
    var tbl = '';
    for (var i = 0; i < lista.length; i++) {
        const fil = `<div id="${lista[i].id}">
            <div>${lista[i].nombre}</div>
            <div>${lista[i].documento}</div></div>`;
        tbl += fil;
    }
    window.busqueda.innerHTML = tbl;
    window.busqueda.style.display = 'block';
}

// recopila los datos y los incluye en una estructura json
const creaJsonVenta = () => {
    var producto_ids = [];
    var precios = [];
    var cantidads = [];
    var descuentos = [];
    var totals = [];
    for (var i=1; i < window.det_tabla.children.length - 1; i++) {
        var f = window.det_tabla.children[i];
        producto_ids.push( +f.children[0].innerText );
        precios.push( +f.children[2].innerText );
        cantidads.push( +f.children[3].innerText );
        descuentos.push( +f.children[4].innerText );
        totals.push( +f.children[5].innerText );
    }
        
    return {
        venta: {
            total: +window.total.value,
            descuento: +window.descuento_tot.value,
            cliente_id: +window.cliente_id.value,
            puesto_id: +window.puesto_id.value,
            usuario_id: +window.usuario_id.value,
            medio_id: +window.medio.value,
        },
        vendidos: {
            producto_ids: producto_ids,
            precios: precios,
            cantidads: cantidads,
            descuentos: descuentos,
            totals: totals,
        }
    }
}
// envia venta al servidor para grabarla
const grabaVenta = async venta => {
    const response = await fetch(`/venta`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(venta)
    });
    return response.json();
}

// agrega item
const onClickAgregaItem = (e) => {
    const detalle = validaDetalle();
    if (detalle === null) return false;
    var dt = document.getElementById("det_fila");
    if (dt !== null) dt.remove();
    agregaItem(detalle);
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
}

// muestra un popup de productos a medida que el usuario teclea nombre
const onKeyupProducto = async e => {
    var producto = e.target.value.trim();
    if (producto == '') return;
    var lista = await obtieneListaProductos(producto);
    muestraListaProductos(lista)
}

const onChangeCantidad = e => {
    var cantidad = validaDec(e.target.value, 1);   
    var precio = validaDec(window.precio_frm.innerText, 0);
    var descuento = validaDec(window.descuento_frm.innerText, 0);
    window.total.value = cantidad * precio - descuento;
}

const onChangeDescuento = e => {
    var descuento = validaDec(e.target.value, 1);   
    var precio = validaDec(window.precio_frm.innerText, 0);
    var cantidad = validaDec(window.cantidad_frm.innerText, 0);
    window.total.value = cantidad * precio - descuento;
}

// obtiene cliente a partir de su id
const onChangeClienteId = async e => {
    const id = validaEntero(e.target);
    if (id < 1) {
        window.cliente_id.focus();
        return;
    }
    var cliente = await obtieneCliente(id);
    muestraCliente(cliente);
}
    
// obtiene producto a partir de su nit
const onChangeDocumento = async e => {
    const documento = e.target.value.trim();
    if (documento === '') {
        window.documento.focus();
        return;
    }
    var cliente = await obtieneClienteDocumento(documento);
    muestraCliente(cliente);
}

// pasa el elemento seleccionado del popup al formulario
const onClickBusqueda = e => {
    var fil = e.target.parentNode;
    if (window.busqueda_titulo.innerText === 'Productos') {
        var prod = {
            id: fil.id,
            nombre: fil.children[0].innerText,
            precio: fil.children[1].innerText,
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
// muestra un popup de clientes a medida que el usuario teclea nombre
const onKeyupNombre = async e => {
    var nombre = e.target.value.trim();
    if (nombre == '') return;
    var lista = await obtieneListaClientes(nombre);
    muestraListaClientes(lista);
}

// graba la venta en la base de datos
const onClickCrea = async e => {
    var venta = creaJsonVenta();
    var venta_id = await grabaVenta(venta);
    var url =  `${location.href}/${venta_id}`;
// TODO: manejar los errores
    window.location.href = url;
}
// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCrea = () => {
    window.agrega_item.addEventListener("click", onClickAgregaItem);
    window.producto_id_frm.addEventListener("change", onChangeProductoId);
    window.producto_frm.addEventListener("keyup", onKeyupProducto);
    window.cantidad_frm.addEventListener("change", onChangeCantidad);
    window.descuento_frm.addEventListener("change", onChangeDescuento);
    window.cliente_id.addEventListener("change", onChangeClienteId);
    window.documento.addEventListener("change", onChangeDocumento);
    window.busqueda.addEventListener("click", onClickBusqueda);
    window.nombre.addEventListener("keyup", onKeyupNombre);
    window.crea.addEventListener("click", onClickCrea);
    window.fila = 0;
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
