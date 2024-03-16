use axum::{extract::Path, response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| greet(Path("World".to_string()))))
        .route("/:name", get(greet));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn greet(Path(name): Path<String>) -> Html<String> {
    Html(format!("<h1>Hello, {name}!</h1>"))
}
