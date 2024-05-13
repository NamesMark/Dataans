use serde::Serialize;
use common::Theme;
use serde_wasm_bindgen::{to_value, from_value};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct EmptyArgs {}

pub async fn load_theme() -> Theme {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let theme_value = invoke("theme", args).await;
    trace!("{:?}", theme_value);

    from_value(theme_value).expect("Theme object deserialization from JsValue should not fail.")
}