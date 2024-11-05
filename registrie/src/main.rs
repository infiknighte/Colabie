use axum::{
    http::{header, Method},
    routing::post,
    Router,
};
use tower_http::cors;

use registrie::*;
use schemou::*;

#[tokio::main]
async fn main() {
    let cors = cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(cors::Any)
        .allow_headers([header::CONTENT_TYPE]);

    let router = Router::new().route("/register", post(register)).layer(cors);

    let address = "0.0.0.0:8081";
    let listner = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("->> Listening on: http://{}\n", address);
    axum::serve(listner, router).await.unwrap();
}

async fn register(BitCode(register_req): BitCode<RegisterReq>) -> BitCode<RegisterRes> {
    let commit_id = register_user(&register_req.username, &register_req.pubkey);
    BitCode(RegisterRes { commit_id })
}

fn register_user(username: &str, pubkey: &str) -> Box<str> {
    // TODO: Store in a real registrie
    format!("idx{username}:{pubkey}").into()
}
