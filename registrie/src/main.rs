use axum::{
    extract::State,
    http::{header, Method, StatusCode},
    routing::post,
    Router,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use tower_http::cors;

use registrie::*;
use schemou::*;

const DB_PATH: &str = "db";

#[tokio::main]
async fn main() {
    let db = DB::get_or_create(DB_PATH);

    let cors = cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(cors::Any)
        .allow_headers([header::CONTENT_TYPE]);

    let router = Router::new()
        .route("/register", post(register))
        .with_state(db)
        .layer(cors);

    let address = "0.0.0.0:8081";
    let listner = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("->> Listening on: http://{}\n", address);
    axum::serve(listner, router).await.unwrap();
}

async fn register(
    State(db): State<DB>,
    BitCode(register_req): BitCode<RegisterReq>,
) -> Result<BitCode<RegisterRes>, StatusCode> {
    let pubkey = BASE64_STANDARD.encode(&register_req.pubkey);
    let commit_id = db
        .new_record(register_req.username.into(), pubkey)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_bytes()
        .into();

    Ok(BitCode(RegisterRes { commit_id }))
}
