use std::net::Ipv4Addr;

use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", routing::get(greet_world))
        .route("/{name}", routing::get(greet_individual))
        .route("/health_check", routing::get(health_check));

    let host = Ipv4Addr::UNSPECIFIED;
    let port = 8080u16;
    let socket_address = (host, port);

    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn greet_world() -> impl IntoResponse {
    "Hello, world!"
}

async fn greet_individual(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {name}!")
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
