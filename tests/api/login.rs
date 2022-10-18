//! tests/api/login.rs
//! author: fyaniquez
//! date: 12/10/2022
//! purpose: test de api de autenticacion

use crate::helpers;

const EMAILFORM: &str = "login_email_form";
const EMAIL: &str = "login_email";

#[tokio::test]
async fn login_email_form_works() {
    // inicializa
    let test_app = helpers::spawn_app().await;
    let client = reqwest::Client::new();
    let app = format!("{}/{}", &test_app.app_address, EMAILFORM);

    // ejecuta
    let response = client
        .get(&app)
        .send()
        .await
        .expect(&format!("El request {} fallo", app));

    // comprueba
    assert!(response.status().is_success());
}

#[tokio::test]
async fn login_email_retorna_200_si_data_ok_en_email_form() {
    // Arrange
    let test_app = helpers::spawn_app().await;
    let client = reqwest::Client::new();
    let app = format!("{}/{}", &test_app.app_address, EMAIL);

    // Act
    let body = "email=fyaniquez%40gmail.com";
    let response = client
        // Use the returned application address
        .post(&app)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect(&format!("El request {} falló.", app));

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!(
        "SELECT email FROM usuarios WHERE email=$1", 
        "fyaniquez@gmail.com")
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Error al recuperar usaurio");

    assert_eq!(saved.email, Some(String::from("fyaniquez@gmail.com")));
}

#[tokio::test]
async fn login_email_retorna_400_si_no_email_ok_en_form() {
    // Arrange
    let test_app = helpers::spawn_app().await;
    let client = reqwest::Client::new();
    let app = format!("{}/{}", &test_app.app_address, EMAIL);
    let test_cases = vec![
        ("", "faltan la variable y su valor"), 
        ("email=", "falta el valor"), 
        ("email=fyaniquez", "error en formato de email")
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            // Use the returned application address
            .post(&app)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("El request falló.");

        // Assert
        assert_eq!(400, response.status().as_u16(), "causa: {}", error_message);
    }
}
