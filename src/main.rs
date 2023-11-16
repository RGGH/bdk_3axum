#![allow(unused)]
use std::net::SocketAddr;
use axum::extract::Query;

use axum::extract::Path;
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
    let routes_all = Router::new().merge(routes_hello());

    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("->> LISTENING ON {}", &addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

// - Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello",get(handler_hello))
        .route("/hello2/:name",get(handler_hello2))
}
// eg /hello?name=bob
async fn handler_hello(Query(params):Query<HelloParams>)->impl IntoResponse{
    println!("->>{:<12}","Handler");
    let name = params.name.as_deref().unwrap_or("world"); 
    Html(format!("<strong>{name}!</strong>"))
}

// eg /hello2/xob
async fn handler_hello2(Path(name) : Path<String>) -> impl IntoResponse {
    println!("->>{:<12}","Handler");
    Html(format!("<strong>{name}!</strong>"))

}
