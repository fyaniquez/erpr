//! src/main.rs

use erpr::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8000")
        .expect("Error al crear listener");
    run(listener)?.await
}
