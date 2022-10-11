//! tests/home.rs

use std::net::TcpListener;

fn spawn_app() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("falla al crear listener");
    let server = erpr::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
#[tokio::test]
async fn home_works() {
    // inicializa
    spawn_app();
    let client = reqwest::Client::new();

    // ejecuta
    let response = client
        .get("http://127.0.0.1:8000/home")
        .send()
        .await
        .expect("El request /home fallo");

    // comprueba
    assert!(response.status().is_success());
}
