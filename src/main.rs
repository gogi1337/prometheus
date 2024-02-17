use axum::routing::get;
use axum::Router;
use proxy::handle_request;
use crate::config::read_config;

pub mod config;
mod proxy;

#[tokio::main]
async fn main() {
    let config = read_config("proxies.yml").unwrap();

    let app = Router::new()
        .route("/*O", get(handle_request))
        .with_state(config);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}