use crate::helpers::spawn_app;

#[tokio::test]
async fn home_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/", &app.address))
        .send()
        .await
        .expect("El request Falló.");

    // Assert
    assert!(response.status().is_success());
}
