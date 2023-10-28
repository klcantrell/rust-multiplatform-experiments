use std::collections::HashMap;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_family = "wasm"))]
uniffi::setup_scaffolding!("shared");

#[cfg_attr(not(target_family = "wasm"), uniffi::export(async_runtime = "tokio"))]
pub async fn get_ip() -> String {
    let url = "https://httpbin.org/ip";
    let resp = reqwest::get(url).await.unwrap();
    let data = resp.json::<HashMap<String, String>>().await.unwrap();
    format!("data: {:?}", data)
}

#[cfg(target_family = "wasm")]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub async fn getIp() -> Result<String, JsValue> {
    Ok(get_ip().await)
}

