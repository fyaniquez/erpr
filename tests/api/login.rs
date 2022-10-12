//! tests/api/login.rs

use crate::helpers;

/*
 * author: fyaniquez
 * date: 12/10/2022
 * purpose: test de api de autenticacion
 */

#[tokio::test]
async fn login_email_works() {
    // inicializa
    let address = helpers::spawn_app();
    let client = reqwest::Client::new();

    // ejecuta
    let response = client
        .get(format!("{}/login_email", &address))
        .send()
        .await
        .expect("El request /login_email fallo");

    // comprueba
    assert!(response.status().is_success());
}
