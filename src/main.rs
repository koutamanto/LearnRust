use std::collections::HashMap;

use axum::{Router, routing::get, extract::Query};
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(
    query:Query<HashMap<String, String>>
) -> String {
    if let Some(message_value) = query.get("message") {
        return message_value.to_string();
    }
    else {
        return "Add message.".to_string();
    }
}
