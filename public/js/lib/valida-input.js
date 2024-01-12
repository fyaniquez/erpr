/*
 * file: /public/js/lib/valida-input.js
 * purpose: scripts para validar inputs
 * author: fyaniquez
 * fecha: 27/06/2022
 */

export const alfanumerico = (e) => {
    var codigo = e.keyCode || e.which;
    if ((codigo >= 65 && codigo <= 90) 
        || (codigo >= 97 && codigo <= 122)
        || (codigo >= 48 && codigo <= 57)
        || codigo === 32
        || (codigo >= 44 && codigo <= 47)
        || (codigo >= 160 && codigo <= 165)
        || codigo == 130) {
        return true;
    } else {
        return false;
    }
}
// valida si el valor del input es entero, identifica el input por objeto
export const entero = (input, objeto) => {
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
export const decimal = (input, objeto) => {
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
// valida descuento
export const descuento = (input) => {
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
// valida total
export const total = () => {
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
