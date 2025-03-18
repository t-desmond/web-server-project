mod routes;
use routes::{index, upload};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index).post(upload));

    let listener = TcpListener::bind("localhost:5050").await.unwrap();

    let address = listener.local_addr().unwrap();

    println!("Server running on http://{}", address);
    axum::serve(listener, app).await.unwrap();
}