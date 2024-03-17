use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let app = Router::new().route("/health_check", get(health_check));
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
