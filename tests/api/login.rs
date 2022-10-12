//! tests/api/login.rs

//! tests/api/login.rs
//! author: fyaniquez
//! date: 12/10/2022
//! purpose: test de api de autenticacion

use crate::helpers;

#[tokio::test]
async fn login_email_form_works() {
    // inicializa
    let address = helpers::spawn_app();
    let client = reqwest::Client::new();

    // ejecuta
    let response = client
        .get(format!("{}/login_email_form", &address))
        .send()
        .await
        .expect("El request /login_email_form fallo");

    // comprueba
    assert!(response.status().is_success());
}
