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

    fn save_raw(key: &str, value: &[u8]);
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn register(username: &str) -> Result<(), JsValue> {
    // TODO: Check if the username is already registere

    let (pb_key, sk_key) = generate_keypair();

    // TODO: Save the private key securely in a file instead
    save_raw("sk_key", &sk_key);

    let register = RegisterReq {
        username: username.into(),
        pubkey: pb_key,
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

// TODO: Use more reliable hybrid cryptographic methods instead
fn generate_keypair() -> (Box<[u8]>, Box<[u8]>) {
    use fips204::ml_dsa_87;
    use fips204::traits::SerDes;
    use rand_chacha::rand_core::SeedableRng;

    let mut rng = rand_chacha::ChaChaRng::from_entropy();
    let (pb_key, sk_key) = ml_dsa_87::try_keygen_with_rng(&mut rng).unwrap();
    (pb_key.into_bytes().into(), sk_key.into_bytes().into())
}
