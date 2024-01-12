import * as datalist from "../lib/datalist.js";
import * as valida from "../lib/valida-input.js";
/*
 * file: /public/js/compra/crea.js
 * purpose: scripts para administrar creacion de compras
 * author: fyaniquez
 * fecha: 27/06/2022
 */
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
        alert('No se introdujo una compra valida');
        return false;
    }
    const numero = Number(valor);
    if (numero < 0.10) {
        alert('No se introdujo una compra valida');
        return false;
    }
    return true;
}
////////////////////
// recopila los datos y los incluye en una estructura json
const creaJsonCompra = () => {
    var producto_ids = [];
    var costos = [];
    var cantidads = [];
    var descuentos = [];
    var vencimientos = [];
    var totals = [];

    const filas = window.form_tabla.getElementsByTagName('tr');
    for (var i = 0; i < filas.length; i++) {
        const id = filas[i].getAttribute("id");
        if (!(id && id.indexOf("fila") === 0)) continue;
        producto_ids.push(+filas[i].children[0].innerText);
        costos.push(+filas[i].children[2].innerText * 100);
        cantidads.push(+filas[i].children[3].innerText * 100);
        descuentos.push(+filas[i].children[4].innerText * 100);
        totals.push(+filas[i].children[5].innerText * 100);
        vencimientos.push(filas[i].children[6].innerText);
    }
    return {
        compra: {
            total: +window.mas_total.innerText * 100,
            descuento: +window.mas_descuento.value * 100,
            medio_id: +window.mas_medio.value,
            distribuidora_id: +window.mas_distribuidora.value,
            documento: window.mas_documento.value,
            observaciones: window.mas_observaciones.value,
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
    const response = await fetch(`/compra`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(compra)
    });
    if (response.ok) {
        const compra = await response.json();
        return compra;
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
            productos[i].costo /= 100.0;
        }
        return productos;
    } else {
        const errmsg = await response.json();
        alert(`codigo: ${errmsg.codigo}\nmensaje: ${errmsg.mensaje}`);
        return null;
    }
}

// agrega item a tabla de vendidos
const muestraDetalle = () => {
    const fila = window.fila + 1;
    const fila_nueva = document.createElement('tr');
    fila_nueva.id = `fila_${fila}`;
    fila_nueva.classList.add("form-tabla-fila");
    fila_nueva.innerHTML = `<td>${window.det_id.value}</td>
        <td><div class="det-nombre">${window.det_nombre.value}</div></td>
        <td>${window.det_costo.value}</td>
        <td>${window.det_cantidad.value}</td>
        <td>${window.det_descuento.value}</td>
        <td>${window.det_total.innerText}</td>
        <td>${window.det_vencimiento.value}</td>
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
    const costo = Number(window.det_costo.value);
    const descuento = Number(window.det_descuento.value);
    const total = cantidad * costo - descuento;
    window.det_total.innerText = total.toFixed(2);
}
const calculaCambio = () => {
    const total = Number(window.mas_total.innerText);
    const pago = Number(window.mas_pago.value);
    const cambio = pago - total;
    window.mas_cambio.innerText = cambio.toFixed(2);
}

// limpia el formulario de detalle de compra
const limpiarDet = () => {
    window.det_id.value = '';
    window.det_nombre.value = '';
    window.det_costo.value = '0';
    window.det_cantidad.value = '1';
    window.det_descuento.value = '0';
    window.det_total.innerText = '0';
}

// borra fila de la tabla de compras
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
const onKeyDownDetId = async e => {
    if(e.key === "Enter" || e.key === "Tab") {
        e.preventDefault();
        const id = e.target.value.trim();

        // si se deja vacio salta a nombre
        if (id === "") {
            window.det_nombre.select();
            window.det_nombre.focus();
            return;
        }

        // si hay error pide el codigo de nuevo
        if (!valida.entero(e.target, "código de producto")) {
            window.det_id.select();
            window.det_id.focus();
            return;
        }

        const producto = await obtieneProducto(e.target.value.trim());
        // si no existe el codigo lo pide de nuevo
        if (!producto) {
            window.det_id.select();
            window.det_id.focus();
            return;
        }

        // si todo ok, continua
        //muestraProducto(producto);
        window.det_id.value = producto.producto_id;
        window.det_nombre.value = producto.nombre;
        window.det_costo.value = producto.precio;
        window.det_cantidad.value = 1;
        window.det_descuento.value = 0;
        //window.det_vencimiento.value = producto.vencimiento;
        window.det_total.innerText = producto.precio;

        window.det_costo.select();
        window.det_costo.focus();
    }
}
// muestra un popup de productos a medida que el usuario teclea nombre
const onKeyupDetNombre = async e => {
    if(e.key === "Enter") {
        // si id y nombre vacios termina introducción del detalle
        if (e.target.value.trim() === "" 
        && window.det_id.value.trim() === "") {
            e.preventDefault();
            window.mas_descuento.select();
            window.mas_descuento.focus();
        }  else {
            window.det_cantidad.select();
            window.det_cantidad.focus();
        }
        return;
    }
    if(e.key === "Tab") {
        return;
    }

    var producto = e.target.value.trim();
    if (!valida.alfanumerico(e) ) {
        //producto += e.key;
    //} else {
        return;
    }

    // si la casilla es vacia 
    if (producto === "") {
        if (window.det_id.value.trim() === "") {
            // si id y nombre vacios termina introducción del detalle
            window.mas_descuento.select();
            window.mas_descuento.focus();
        } else {
            // si no hay nombre cambiar de producto
            window.det_id.select();
            window.det_id.focus();
        }
        return;
    }

    // la consulta se activa si se buscan mas de 3 caracteres
    if (producto.length < 3) {
        return;
    }

    // buscar productos coincidentes
    const productos = await obtieneProductos(producto);
    
    // si no existe el codigo lo pide de nuevo
    if (!productos || productos.length < 0) {
        e.target.select();
        e.target.focus();
        return;
    }

    // mostrar lista de productos
    datalist.carga(det_nombre_datalist, "nombre", ["producto_id", "precio"], productos)
}
// actualiza el formulario luego de cambio de cantidad
const onChangeDetCantidad = e => {
    if (!valida.decimal(e.target, "cantidad")) {
        window.det_cantidad.value = 1;
        window.det_cantidad_old = 1;
        e.target.select();
        e.target.focus();
    }
    recalculaDetalle();
    recalculaMaestro();
    window.det_cantidad_old = window.det_cantidad.value;
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
    if (!valida.entero(window.det_id, 'producto')) {
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
    if (!valida.decimal(e.target, "pago")) {
        window.mas_pago.value = 0;
        e.target.select();
        e.target.focus();
    }
    calculaCambio();
    window.mas_documento.select();
    window.mas_documento.focus();
}
// graba la compra en la base de datos
const onClickBtnGuarda = async e => {
    const compra = creaJsonCompra();
    const compra_id = await grabaCompra(compra);
    if (!compra_id) return;
    window.location.href = `${location.href}/${compra_id}`;
}
// graba la compra en la base de datos
const onClickBtnCancela = async e => {
    if (!confirm('¿Esta seguro de cancelar la compra?')) {
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

    window.mas_documento.value = '';
    window.mas_observaciones.value = '';

    window.det_id.focus();

}
// hacer que el 'Enter' funcione como 'Tab'
// hacer que el 'End' acabe la introducción de productos
const onKeyDownDet = async (e) => {
    if (e.key === "Enter" || e.key == "Tab") {
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
const onBlurDetNombre = e => {
    const producto = e.target.value.trim();

        // si la casilla esta vacia 
    if (producto === "") {
        if (window.det_id.value.trim() === "") {
                // si id y nombre vacios termina introducción del detalle
            window.mas_descuento.select();
            window.mas_descuento.focus();
        } else {
            // si no hay nombre cambiar de producto
            window.det_id.select();
            window.det_id.focus();
        }
        return;
    } else {
        window.det_costo.value = 123;
        window.det_id.value = 123;
    }
}
const onChangeDetNombre = e => {
    const data = e.target.value.trim();

    // si la casilla esta vacia 
    if (data === "") {
        if (window.det_id.value.trim() === "") {
                // si id y nombre vacios termina introducción del detalle
            window.mas_descuento.select();
            window.mas_descuento.focus();
        } else {
            // si no hay nombre cambiar de producto
            window.det_id.select();
            window.det_id.focus();
        }
        return;
    } else {
        const datos = data.split("|");
        window.det_id.value = datos[1];
        window.det_nombre.value = datos[0];
    }
}
const onInputDetNombre = e => {
    window.det_costo.value = 1234;
    window.det_id.value = 1234;
}
// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadCrea = () => {

    window.det_id.addEventListener("keydown", onKeyDownDetId);

    window.det_nombre.addEventListener("keyup", onKeyupDetNombre);
    window.det_nombre.addEventListener("change",  onChangeDetNombre);

    window.det_cantidad.addEventListener("change", onChangeDetCantidad);
    window.det_descuento.addEventListener("change", onChangeDetDescuento);
    window.det_agrega.addEventListener("click", onClickDetAgrega);

    window.mas_descuento.addEventListener("change", onChangeMasDescuento);
    window.mas_pago.addEventListener("change", onChangeMasPago);

    window.btn_guarda.addEventListener("click", onClickBtnGuarda);
    window.btn_cancela.addEventListener("click", onClickBtnCancela);

    window.det_nombre.addEventListener("keydown", onKeyDownDet);
    window.det_costo.addEventListener("keydown", onKeyDownDet);
    window.det_cantidad.addEventListener("keydown", onKeyDownDet);
    window.det_descuento.addEventListener("keydown", onKeyDownDet);
    window.det_vencimiento.addEventListener("keydown", onKeyDownDet);

    window.mas_descuento.addEventListener("keydown", onKeyDownDet);
    window.mas_pago.addEventListener("keydown", onKeyDownDet);

    window.det_id.tabIndex = 50;
    window.det_nombre.tabIndex = 60;
    window.det_costo.tabIndex = 70;
    window.det_cantidad.tabIndex = 80;
    window.det_descuento.tabIndex = 90;
    window.det_vencimiento.tabIndex = 100;
    window.det_agrega.tabIndex = 110;
    window.mas_descuento.tabIndex = 120;
    window.mas_pago.tabIndex = 130;
    window.mas_distribuidora.tabIndex = 140;
    window.mas_medio.tabIndex = 150;
    window.mas_documento.tabIndex = 160;
    window.mas_observaciones.tabIndex = 170;
    window.btn_guarda.tabIndex = 180;
    window.btn_cancela.tabIndex = 190;

    window.det_id.select();
    window.det_id.focus();

    window.fila = 0;
}

document.readyState === "complete" ? onLoadCrea() : addEventListener("load", onLoadCrea);
