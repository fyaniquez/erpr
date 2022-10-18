//! tests/home.rs
//! author: fyaniquez
//! date: 12/10/2022
//! purpose: test de paginas que no requieren autenticación

use crate::helpers;

#[tokio::test]
async fn home_works() {
    // inicializa
    let test_app = helpers::spawn_app().await;
    let client = reqwest::Client::new();
    let app = format!("{}/", &test_app.app_address);

    // ejecuta
    let response = client
        .get(&app)
        .send()
        .await
        .expect(&format!("El request {} fallo", app));

    // comprueba
    assert!(response.status().is_success());
}
