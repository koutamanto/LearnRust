use axum::{Router, routing::get, extract::Query};
use serde::Deserialize;
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(
    message:Query<Message>
) -> &'static str {
    let message: = message.0;
    &message.message
}

#[derive(Deserialize)]
struct Message {
    message: String,
}