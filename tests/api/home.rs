//! tests/home.rs

use std::net::TcpListener;
use crate::helpers;

/*
 * author: fyaniquez
 * date: 12/10/2022
 * purpose: test de paginas que no requieren autenticación
 */

#[tokio::test]
async fn home_works() {
    // inicializa
    let address = helpers::spawn_app();
    let client = reqwest::Client::new();

    // ejecuta
    let response = client
        .get(format!("{}/home", &address))
        .send()
        .await
        .expect("El request /home fallo");

    // comprueba
    assert!(response.status().is_success());
}
