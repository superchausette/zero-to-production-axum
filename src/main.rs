use zero2prod_axum::run;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    run(listener).await
}