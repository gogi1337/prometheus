use axum::{extract::{Host, Request, State}, response::{IntoResponse, Redirect}};

use crate::config::ProxyConfig;

pub async fn handle_request(State(config): State<Vec<ProxyConfig>>, Host(host): Host, req: Request) -> impl IntoResponse {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("http://localhost:3000{path_query}");

    Redirect::permanent(&uri)
}