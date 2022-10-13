//! tests/home.rs
//! author: fyaniquez
//! date: 12/10/2022
//! purpose: test de paginas que no requieren autenticación

use crate::helpers;

#[tokio::test]
async fn home_works() {
    // inicializa
    let address = helpers::spawn_app();
    let client = reqwest::Client::new();

    // ejecuta
    let response = client
        .get(format!("{}/", &address))
        .send()
        .await
        .expect("El request / fallo");

    // comprueba
    assert!(response.status().is_success());
}
