//! src/main.rs

use erpr::startup::run;

/*
 * author: fyaniquez
 * date: 30/09/2022
 * purpose: sistema de ventas minoristas y mayoristas
 */



#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
