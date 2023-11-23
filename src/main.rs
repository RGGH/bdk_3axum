#![allow(unused)]
use axum::extract::Path;
use axum::extract::{Query,Extension};
use axum::middleware;
use axum::response::Response;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::routing::get_service;
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {


    let routes_all = Router::new()
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    // response mapper
    async fn main_response_mapper(res: Response) -> Response {
        println!("-->>{:<12}", "MAPPER RESPONSE");
        println!(" ");
        res
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING ON {}", &addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

// routes Static
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
