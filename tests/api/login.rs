//! tests/api/login.rs
//! author: fyaniquez
//! date: 12/10/2022
//! purpose: test de api de autenticacion

use crate::helpers;
use erpr::configuration::get_configuration;
use sqlx::{PgConnection, Connection};

const EMAILFORM: &str = "login_email_form";
const EMAIL: &str = "login_email";

#[tokio::test]
async fn login_email_form_works() {
    // inicializa
    let address = helpers::spawn_app();
    let client = reqwest::Client::new();

    // ejecuta
    let response = client
        .get(format!("{}/{}", &address, EMAILFORM))
        .send()
        .await
        .expect(&format!("El request /{} fallo", EMAILFORM));

    // comprueba
    assert!(response.status().is_success());
}

#[tokio::test]
async fn login_email_retorna_200_si_data_ok_en_email_form() {
    // Arrange
    let address = helpers::spawn_app();
    let conf = get_configuration().expect("Falla al leer configuración");
    let connection_string = conf.database.connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect(&format!("Error al conectar a {}", connection_string));
    let client = reqwest::Client::new();

    // Act
    let body = "email=fyaniquez%40gmail.com";
    let response = client
        // Use the returned application address
        .post(format!("{}/{}", &address, EMAIL))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("El request falló.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn login_email_retorna_400_si_no_email_ok_en_form() {
    // Arrange
    let address = helpers::spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("", "faltan la variable y su valor"), 
        ("email=", "falta el valor"), 
        ("email=fyaniquez", "error en formato de email")
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            // Use the returned application address
            .post(&format!("{}/{}", &address, EMAIL))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("El request falló.");

        // Assert
        assert_eq!(400, response.status().as_u16(), "causa: {}", error_message);
    }
}
