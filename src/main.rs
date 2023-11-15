#![allow(unused)]
use std::net::SocketAddr;
use axum::extract::Query;

use axum::Router;
use axum::routing::get;
use axum::response::{Html, IntoResponse};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct HelloParams {
    name : Option<String>
}

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route(
        "/hello"
        ,get(handler_hello));

    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("->> LISTENING ON {}", &addr);
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

//
// eg /hello?name=bob
async fn handler_hello(Query(params):Query<HelloParams>)->impl IntoResponse{
    println!("->>{:<12}","Handler");
    let name = params.name.as_deref().unwrap_or("world"); 
    Html(format!("<strong>{name}!</strong>"))
}
