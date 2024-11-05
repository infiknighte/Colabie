use bitcode::decode;
use schemou::{RegisterReq, RegisterRes};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Uint8Array;

#[wasm_bindgen(module = "/glue.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn get_raw(url: &str) -> Result<Uint8Array, JsValue>;

    #[wasm_bindgen(catch)]
    async fn post_raw(url: &str, body: &[u8]) -> Result<Uint8Array, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn register(username: &str) -> Result<(), JsValue> {
    let register = RegisterReq {
        username: username.into(),
        // TODO: Generate a keypair
        pubkey: "keyxyz".into(),
    };

    let resp: RegisterRes = decode(
        &post_raw(
            "http://localhost:8081/register",
            &bitcode::encode(&register),
        )
        .await?
        .to_vec(),
    )
    .map_err(|_| JsValue::from_str("Invalid Response"))?;

    alert(&format!("Registered: {}", resp.commit_id));

    Ok(())
}
