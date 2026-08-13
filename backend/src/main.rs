use axum::{routing::get, Router};

async fn healthz() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/healthz", get(healthz));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind port 8080");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
