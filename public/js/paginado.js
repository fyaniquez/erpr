/*
 * file: paginado.js
 * libreria para manejar listas paginadas
 * author: fyaniquez
 * fecha: 27/06/2022
 */

// genera el query string y llamar a la página correspondiente
export const paginar = (ctrl, valor) => {
    var url = window.location.href;
    url = url.substring(0, url.indexOf('?')) + '?';
    ctrls.forEach(function (value, key) {
        if (ctrl === key) {
            url += `${ctrl}=${valor}&`;
        } else {
            if ("pagina" === key) {
                url += 'pagina=1&';
            } else {
                var valor_ctrl = value.call();
                url += `${key}=${valor_ctrl}&`;
            }
        }
    });
    window.location.replace(encodeURI(url.substring(0, url.length - 1)));
}

// recarga la pagina actual
export const recarga_pagina = () => {
    var act = document.getElementById("actual");
    paginar("pagina", act.innerText);
}

// carga el hashmap de controles usados para generar el querystring
const fns = () => {
    ctrls.set("longitud", () => {
        var l = document.getElementById("longitud");
        return l.options[l.selectedIndex].value;
    });
    ctrls.set("pagina", () => {
        var act = document.getElementById("actual");
        return +act.innerText;
    });
}

// eventos que generan un cambio de página
const onChangeLongitud = (e) => {
    var idx = e.target.selectedIndex;
    paginar(e.target.id, e.target.options[idx].value);
};
const onClickPaginas = (e) => {
    var act = document.getElementById("actual");
    if (e.target.className !== "pagina"
        || isNaN(e.target.innerText)
        || e.target.innerText === act.innerText) return;
    paginar("pagina", e.target.innerText);
};
const onClickPrimero = () => {
    var act = document.getElementById("actual");
    if (parseInt(act.innerText) < 2) return;
    paginar("pagina", 1);
};
const onClickUltimo = (e) => {
    var act = document.getElementById("actual");
    if (parseInt(act.innerText) == parseInt(e.target.dataset.index)) return;
    paginar("pagina", e.target.dataset.index);
};
const onClickPrevio = () => {
    var act = document.getElementById("actual");
    if (parseInt(act.innerText) < 2) return;
    paginar("pagina", +act.innerText - 1);
};
const onClickSiguiente = () => {
    var act = document.getElementById("actual");
    var ult = document.getElementById("ultimo");
    if (act.innerText === ult.dataset.index) return;
    paginar("pagina", +act.innerText + 1);
};

// inicializa los eventos y listeners al terminar el cargado de la página
const onLoadPaginado = () => {
    window.ctrls = new Map();
    fns();
    longitud.addEventListener("change", onChangeLongitud);
    paginas.addEventListener("click", onClickPaginas);
    primero.addEventListener("click", onClickPrimero);
    ultimo.addEventListener("click", onClickUltimo);
    previo.addEventListener("click", onClickPrevio);
    siguiente.addEventListener("click", onClickSiguiente);
}
document.readyState === "complete" ? onLoadPaginado() : addEventListener("load", onLoadPaginado);
