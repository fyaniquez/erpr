/*
 * file: /public/js/venta/crea.js
 * scripts para administrar creacion de ventas
 * author: fyaniquez
 * fecha: 27/06/2022
 */
const fila2Producto = fila => {
    return {
        id: fila.id,
        nombre: fila.children[0].innerText,
        precio: fila.children[1].innerText
    }
}
const fila2Cliente = fila => {
    return {
        id: fila.id,
        nombre: fila.children[0].innerText,
        documento: fila.children[1].innerText
    }
}
// valida si el valor del input es entero, identifica el input por objeto
const validaEntero = (input, objeto) => {
    const valor = input.value.trim();
    if (valor === '' || isNaN(valor)) {
        alert(`Debe introducir el ${objeto}`);
        return false;
    }
    const numero = Number(valor);
    if (!Number.isInteger(numero)) {
        alert(`El ${objeto} no es válido`);
        return false;
    }
    if (numero < 1) {
        alert(`El ${objeto} no es válido`);
        return false;
    }
    return true;
}
// valida decimal
const validaDecimal = (input, objeto) => {
    const valor = input.value.trim();
    if (valor === '' || isNaN(valor)) {
        alert(`Debe introducir el ${objeto}`);
        input.value = "0";
        return false;
    }
    const numero = Number(valor);
    if (numero < 1) {
        alert(`El ${objeto} no es válido`);
        input.value = "0";
        return false;
    }
    return true;
}
// valida decimal
const validaDescuento = (input) => {
    const valor = input.value.trim();
    if (valor === '' || isNaN(valor)) {
        alert("Debe introducir un número válido");
        input.value = "0";
        return false;
    }
    const numero = Number(valor);
    if (numero < 0) {
        alert("El descuento no es válido");
        input.value = "0";
        return false;
    }
    return true;
}
const validaTotal = () => {
    const valor = window.det_total.innerText;
    if (valor === '' || isNaN(valor)) {
        alert('No se introdujo una venta valida');
        return false;
    }
    const numero = Number(valor);
    if (numero < 0.10) {
        alert('No se introdujo una venta valida');
        return false;
    }
    return true;
}
////////////////////
// recopila los datos y los incluye en una estructura json
const creaJsonVenta = () => {
    var producto_ids = [];
    var precios = [];
    var cantidads = [];
    var descuentos = [];
    var totals = [];

    const filas = window.form_tabla.getElementsByTagName('tr');
    for (var i = 0; i < filas.length; i++) {
        const id = filas[i].getAttribute("id");
        if (!(id && id.indexOf("fila") === 0)) continue;
        producto_ids.push(+filas[i].children[0].innerText);
        precios.push(+filas[i].children[2].innerText * 100);
        cantidads.push(+filas[i].children[3].innerText * 100);
        descuentos.push(+filas[i].children[4].innerText * 100);
        totals.push(+filas[i].children[5].innerText * 100);
    }
    return {
        venta: {
            total: +window.mas_total.innerText * 100,
            descuento: +window.mas_descuento.value * 100,
            medio_id: +window.mas_medio.value,
            cliente_id: +window.cli_id.value,
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
    if (response.ok) {
        const venta = await response.json();
        return venta;
    } else {
        const errmsg = await response.json();
        alert(`codigo: ${errmsg.codigo}\nmensaje: ${errmsg.mensaje}`)
        return null;
    }
}

// obtiene producto
const obtieneProducto = async id => {
    const response = await fetch(`/precio/${id}.json`);
    if (response.ok) {
        const producto = await response.json();
        producto.precio /= 100.0;
        return producto;
    } else {
        const errmsg = await response.json();
        alert(`codigo: ${errmsg.codigo}\nmensaje: ${errmsg.mensaje}`);
        return null;
    }
}

// obtiene lista productos
const obtieneProductos = async producto => {
    const filtro = encodeURIComponent(producto);
    const response = await fetch(`/precios.json?filtro=${filtro}`);
    if (response.ok) {
        const productos = await response.json();
        for (var i = 0; i < productos.length; i++) {
            productos[i].precio /= 100.0;
        }
        return productos;
    } else {
        const errmsg = await response.json();
        alert(`codigo: ${errmsg.codigo}\nmensaje: ${errmsg.mensaje}`);
        return null;
    }
}

// obtiene  cliente
const obtieneCliente = async id => {
    const response = await fetch(`/cliente/${id}.json`);
    if (response.ok) {
        const cliente = await response.json();
        return cliente;
    } else {
        const errmsg = await response.json();
        alert(`codigo: ${errmsg.codigo}\nmensaje: ${errmsg.mensaje}`);
        return null;
    }
}

// obtiene  cliente por documento
const obtieneClienteDocumento = async documento => {
    const filtro = encodeURIComponent(documento);
    const response = await fetch(`/cliente.json?documento=${filtro}`);
    if (response.ok) {
        const cliente = await response.json();
        return cliente;
    } else {
        const errmsh = await response.json();
        alert(`codigo: ${errmsg.codigo}\nmensaje: ${errmsg.mensaje}`);
        return null;
    }
}

// obtiene lista clientes de acuerdo al criterio
const obtieneClientes = async cliente => {
    const filtro = encodeURIComponent(cliente);
    const response = await fetch(`/clientes.json?nombre=${filtro}`);
    if (response.ok) {
        const clientes = await response.json();
        return clientes;
    } else {
        const errmsg = await response.json();
        alert(`codigo:${errmsg.codigo}\nmensaje:${errmsg.mensaje}`);
        return null;
    }
}
// muestra producto en formulario
const muestraProducto = producto => {
    window.det_id.value = producto.producto_id;
    window.det_nombre.value = producto.nombre;
    window.det_precio.innerText = producto.precio;
    window.det_cantidad.value = 1;
    window.det_descuento.value = 0;
    window.det_total.innerText = producto.precio;
    window.det_cantidad.focus();
}
// muestra productos en un popup
const muestraProductos = (productos) => {
    (window.busqueda_titulo).innerText = "Productos";
    const tabla_ant = document.getElementById("busqueda");
    if (tabla_ant) tabla_ant.remove();
    const tabla = document.createElement("table");
    tabla.classList.add("busqueda");
    tabla.id = "busqueda";
    busqueda_box.append(tabla);
    const formato = document.createElement("colgroup");
    const col1 = document.createElement("col");
    col1.style.width = "9rem";
    const col2 = document.createElement("col");
    col2.style.width = "4rem";
    formato.append(col1, col2);
    const cabecera = document.createElement("tr");
    cabecera.classList.add("form-tabla-fila");
    const tit1 = document.createElement("th");
    tit1.innerText = "Nombre";
    const tit2 = document.createElement("th");
    tit2.innerText = "Precio";
    cabecera.append(tit1, tit2);
    tabla.append(formato, cabecera);
    for (var i = 0; i < productos.length; i++) {
        const fila_nueva = document.createElement("tr");
        fila_nueva.id = productos[i].producto_id
        fila_nueva.classList.add("form-tabla-fila");
        const det1 = document.createElement("td");
        det1.innerText = productos[i].nombre;
        const det2 = document.createElement("td");
        det2.innerText = productos[i].precio;
        fila_nueva.append(det1, det2);
        tabla.append(fila_nueva);
    }
    window.busqueda_box.style.display = 'block';
}
// muestra cliente en formulario
const muestraCliente = cliente => {
    window.cli_id.value = cliente.id;
    window.cli_nombre.value = cliente.nombre;
    window.cli_documento.value = cliente.documento;
    window.btn_guarda.focus();
}
// muestra lista de clientes en un popup
const muestraClientes = (clientes) => {
    (window.busqueda_titulo).innerText = 'Clientes';
    const tabla_ant = document.getElementById("busqueda");
    if (tabla_ant) tabla_ant.remove();
    const tabla = document.createElement("table");
    tabla.classList.add("busqueda");
    tabla.id = "busqueda";
    busqueda_box.append(tabla);
    const formato = document.createElement("colgroup");
    const col1 = document.createElement("col");
    col1.style.width = "9rem";
    const col2 = document.createElement("col");
    col2.style.width = "4rem";
    formato.append(col1, col2);
    const cabecera = document.createElement("tr");
    cabecera.classList.add("form-tabla-fila");
    const tit1 = document.createElement("th");
    tit1.innerText = "Nombre";
    const tit2 = document.createElement("th");
    tit2.innerText = "Docto.";
    cabecera.append(tit1, tit2);
    tabla.append(formato, cabecera);
    for (var i = 0; i < clientes.length; i++) {
        const fila_nueva = document.createElement("tr");
        fila_nueva.id = clientes[i].id
        fila_nueva.classList.add("form-tabla-fila");
        const det1 = document.createElement("td");
        det1.innerText = clientes[i].nombre;
        const det2 = document.createElement("td");
        det2.innerText = clientes[i].documento;
        fila_nueva.append(det1, det2);
        tabla.append(fila_nueva);
    }
    window.busqueda_box.style.display = 'block';
}

// agrega item a tabla de vendidos
const muestraDetalle = () => {
    const fila = window.fila + 1;
    const fila_nueva = document.createElement('tr');
    fila_nueva.id = `fila_${fila}`;
    fila_nueva.classList.add("form-tabla-fila");
    fila_nueva.innerHTML = `<td>${window.det_id.value}</td>
        <td><div class="det-nombre">${window.det_nombre.value}</div></td>
        <td>${window.det_precio.innerText}</td>
        <td>${window.det_cantidad.value}</td>
        <td>${window.det_descuento.value}</td>
        <td>${window.det_total.innerText}</td>
        <td><div class="cmd">
            <div class="btn-min peligro" id="borra_${fila}">&#x2715</div>
        </div></td>`;
    const formtabla = window.form_tabla_fila;
    formtabla.insertAdjacentElement('afterend', fila_nueva);
    window.fila = fila;
    const btn_borra = document.getElementById(`borra_${fila}`);
    btn_borra.addEventListener("click", onClickBorraFila);
}
// calcula los totales para el registro maestro
const recalculaMaestro = () => {
    var total = 0.0;
    const filas = window.form_tabla.getElementsByTagName('tr');
    for (var i = 0; i < filas.length; i++) {
        const id = filas[i].getAttribute("id");
        if (!(id && id.indexOf("fila") === 0)) continue;
        const columnas = filas[i].children;
        total += Number(columnas[5].innerText);
    }
    window.mas_subtotal.innerText = total;
    const descuento = Number(window.mas_descuento.value);
    window.mas_total.innerText = total - descuento;
}
const recalculaDetalle = () => {
    const cantidad = Number(window.det_cantidad.value);
    const precio = Number(window.det_precio.innerText);
    const descuento = Number(window.det_descuento.value);
    const total = cantidad * precio - descuento;
    window.det_total.innerText = total.toFixed(2);
}
const calculaCambio = () => {
    const total = Number(window.mas_total.innerText);
    const pago = Number(window.mas_pago.value);
    const cambio = pago - total;
    window.mas_cambio.innerText = cambio.toFixed(2);
}

// limpia el formulario de detalle de venta
const limpiarDet = () => {
    window.det_id.value = '';
    window.det_nombre.value = '';
    window.det_precio.innerText = '0';
    window.det_cantidad.value = '1';
    window.det_descuento.value = '0';
    window.det_total.innerText = '0';
}

// borra fila de la tabla de ventas
const onClickBorraFila = (e) => {
    const id = e.target.getAttribute("id");
    const fila_id = id.split("_")[1];
    const fila = document.getElementById(`fila_${fila_id}`);
    if (confirm("¿Esta seguro de eliminar el producto?")) {
        e.target.removeEventListener("click", onClickBorraFila);
        fila.remove();
        limpiarDet();
        recalculaMaestro();
        window.det_id.focus();
    }
}

// obtiene producto a partir de su id
const onChangeDetId = async e => {
    const id = e.target.value.trim();
    // si se deja vacio salta a nombre
    if (id === "") {
        window.det_nombre.select();
        window.det_nombre.focus();
        return;
    }
    // si hay error pide el codigo de nuevo
    if (!validaEntero(e.target, "código de producto")) {
        window.det_id.select();
        window.det_id.focus();
        return;
    }
    const producto = await obtieneProducto(e.target.value.trim());
    //e.stopPropagation();
    if (!producto) {
        return;
    }
    muestraProducto(producto);
    window.det_cantidad.select();
    window.det_cantidad.focus();
}
// muestra un popup de productos a medida que el usuario teclea nombre
const onKeyupDetNombre = async e => {
    const producto = e.target.value.trim();
    if (producto === "") {
        if (window.det_id.value.trim() === "") {
            // si id y nombre estan vacios termina introducción del detalle
            window.mas_descuento.select();
            window.mas_descuento.focus();
        }
        return;
    }
    window.muestraBusqueda = muestraProducto;
    window.objetoBusqueda = fila2Producto;
    const productos = await obtieneProductos(producto);
    if (!productos) {
        e.target.select();
        return;
    }
    muestraProductos(productos);
    window.det_cantidad.select();
    window.det_cantidad.focus();
}
// actualiza el formulario luego de cambio de cantidad
const onChangeDetCantidad = e => {
    if (!validaDecimal(e.target, "cantidad")) {
        window.det_cantidad.value = 1;
        window.det_cantidad_old = 1;
        e.target.select();
        e.target.focus();
    }
    recalculaDetalle();
    recalculaMaestro();
    window.det_cantidad_old = window.dec_cantidad.value;
    window.det_descuento.select();
    window.det_descuento.focus();
}

// actualiza el formulario luego de cambio de descuento
const onChangeDetDescuento = e => {
    if (!validaDescuento(e.target)) {
        window.det_descuento.value = 0;
        e.target.select();
        e.target.focus();
    }
    recalculaDetalle();
    recalculaMaestro();
    window.det_agrega.focus();
}
// agrega item
const onClickDetAgrega = (e) => {
    if (!validaEntero(window.det_id, 'producto')) {
        window.det_id.select();
        window.det_id.focus();
        return;
    }
    if (!validaTotal()) {
        window.det_cantidad.select();
        window.det_cantidad.focus();
        return;
    }
    muestraDetalle();
    recalculaMaestro();
    limpiarDet();
    window.det_id.select();
    window.det_id.focus();
}
// valida descuento en el maestro
const onChangeMasDescuento = e => {
    if (!validaDescuento(e.target)) {
        window.mas_descuento.value = 0;
        e.target.select();
        e.target.focus();
    }
    recalculaMaestro();
    window.mas_pago.select();
    window.mas_pago.focus();
}
// valida descuento en el maestro
const onChangeMasPago = e => {
    if (!validaDecimal(e.target, "pago")) {
        window.mas_pago.value = 0;
        e.target.select();
        e.target.focus();
    }
    calculaCambio();
    window.cli_id.select();
    window.cli_id.focus();
}
// obtiene cliente a partir de su id
const onChangeCliId = async e => {
    if (!validaEntero(e.target, 'código de cliente')) {
        e.target.select();
        e.target.focus();
        return;
    }
    const id = e.target.value.trim();
    const cliente = await obtieneCliente(id);
    if (!cliente) {
        e.target.select();
        e.target.focus();
        return;
    }
    muestraCliente(cliente);
}
// muestra un popup de clientes a medida que el usuario teclea nombre
const onKeyupCliNombre = async e => {
    const cliente = e.target.value.trim();
    if (cliente === '') {
        e.target.focus();
        return;
    }
    window.muestraBusqueda = muestraCliente;
    window.objetoBusqueda = fila2Cliente;
    const clientes = await obtieneClientes(cliente);
    if (!clientes) {
        e.target.focus();
        return;
    }
    muestraClientes(clientes);
}
// obtiene producto a partir de su nit
const onChangeCliDocumento = async e => {
    const documento = e.target.value.trim();
    if (documento === '') {
        e.target.focus();
        return;
    }
    const cliente = await obtieneClienteDocumento(documento);
    if (!cliente) {
        e.target.focus();
        return;
    }
    muestraCliente(cliente);
}
// muestra en el formulario la linea seleccionada en el popup de busqueda
const onClickBusqueda = (e) => {
    const fila = e.target.parentNode;
    window.muestraBusqueda(window.objetoBusqueda(fila));
    window.busqueda_box.style.display = 'none';
}
const onClickBusquedaClose = (e) => {
    window.busqueda_box.style.display = 'none';
    window.det_nombre.select();
    window.det_nombre.focus();
}
// graba la venta en la base de datos
const onClickBtnGuarda = async e => {
    const venta = creaJsonVenta();
    const venta_id = await grabaVenta(venta);
    if (!venta_id) return;
    window.location.href = `${location.href}/${venta_id}`;
}
// graba la venta en la base de datos
const onClickBtnCancela = async e => {
    if (!confirm('¿Esta seguro de cancelar la venta?')) {
        return;
    }
    limpiarDet();

    const filas = window.form_tabla.getElementsByTagName('tr');
    for (var i = filas.length - 1; i > 0; i--) {
        const id = filas[i].getAttribute("id");
        if (!(id && id.indexOf("fila") === 0)) continue;
        const fila_id = id.split('_')[1];
        const btn = document.getElementById(`borra_${fila_id}`);
        btn.removeEventListener("click", onClickBorraFila);
        filas[i].remove();
    }

    window.mas_subtotal.innerText = '0';
    window.mas_descuento.value = '0';
    window.mas_total.innerText = '0';
    window.mas_pago.value = '0';
    window.mas_cambio.innerText = '0';

    window.cli_id.value = '';
    window.cli_nombre.value = '';
    window.cli_documento.value = '';

    window.det_id.focus();

}
// hacer que el 'Enter' funcione como 'Tab'
// hacer que el 'End' acabe la introducción de productos
const onKeyDownDet = async (e) => {
    if (e.key === "Enter") {
        e.preventDefault();
        const tabindex = +e.target.tabIndex + 10;
        const sigele = document.querySelector(`[tabIndex="${tabindex}"]`);
        if (typeof sigele.select === "function") {
            sigele.select();
        }
        sigele.focus();
    } else if (e.key === "End") {
        window.mas_descuento.select();
        window.mas_descuento.focus();
    }
}
// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCrea = () => {

    window.det_id.addEventListener("change", onChangeDetId);


    window.det_nombre.addEventListener("keyup", onKeyupDetNombre);
    window.det_cantidad.addEventListener("change", onChangeDetCantidad);
    window.det_descuento.addEventListener("change", onChangeDetDescuento);
    window.det_agrega.addEventListener("click", onClickDetAgrega);

    window.mas_descuento.addEventListener("change", onChangeMasDescuento);
    window.mas_pago.addEventListener("change", onChangeMasPago);

    window.cli_id.addEventListener("change", onChangeCliId);
    window.cli_nombre.addEventListener("keyup", onKeyupCliNombre);
    window.cli_documento.addEventListener("change", onChangeCliDocumento);

    window.busqueda_box.addEventListener("click", onClickBusqueda);
    window.busqueda_close.addEventListener("click", onClickBusquedaClose);

    window.btn_guarda.addEventListener("click", onClickBtnGuarda);
    window.btn_cancela.addEventListener("click", onClickBtnCancela);

    window.det_id.addEventListener("keydown", onKeyDownDet);
    window.det_cantidad.addEventListener("keydown", onKeyDownDet);
    window.det_descuento.addEventListener("keydown", onKeyDownDet);

    window.mas_descuento.addEventListener("keydown", onKeyDownDet);
    window.mas_pago.addEventListener("keydown", onKeyDownDet);

    window.cli_documento.addEventListener("keydown", onKeyDownDet);
    window.cli_id.addEventListener("keydown", onKeyDownDet);
    window.cli_nombre.addEventListener("keydown", onKeyDownDet);

    window.det_id.tabIndex = 50;
    window.det_nombre.tabIndex = 60;
    window.det_cantidad.tabIndex = 70;
    window.det_descuento.tabIndex = 80;
    window.det_agrega.tabIndex = 90;
    window.mas_descuento.tabIndex = 100;
    window.mas_pago.tabIndex = 110;
    window.cli_documento.tabIndex = 120;
    window.cli_id.tabIndex = 130;
    window.cli_nombre.tabIndex = 140;
    window.btn_guarda.tabIndex = 150;
    window.btn_cancela.tabIndex = 160;

    window.det_id.select();
    window.det_id.focus();

    window.fila = 0;
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
