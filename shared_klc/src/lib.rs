use std::time::Duration;
use async_std::future::{timeout, pending};

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_family = "wasm"))]
uniffi::setup_scaffolding!("shared");

#[cfg_attr(not(target_family = "wasm"), uniffi::export)]
pub async fn say_after(ms: u64, who: String) -> String {
    let never = pending::<()>();
    timeout(Duration::from_millis(ms), never).await.unwrap_err();
    format!("Hello, {who}!")
}

#[cfg(target_family = "wasm")]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub async fn sayAfter(ms: u64, who: String) -> Result<String, JsValue> {
    Ok(say_after(ms, who).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task;

    #[test]
    fn it_works() {
        let result = task::block_on(say_after(2000, "Kalalau".to_string()));
        assert_eq!(result, "Hello, Kalalau!");
    }
}
