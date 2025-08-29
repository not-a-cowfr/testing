use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use web_sys::Storage;

#[wasm_bindgen]
pub struct LocalStorage(Storage);

#[wasm_bindgen]
impl LocalStorage {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<LocalStorage, JsValue> {
        let window =
            web_sys::window().ok_or_else(|| JsValue::from_str("`window` does not exist."))?;
        let storage = window
            .local_storage()
            .map_err(|err| JsValue::from(err))?
            .ok_or_else(|| JsValue::from_str("`localStorage` does not exist."))?;

        Ok(LocalStorage(storage))
    }

    #[wasm_bindgen]
    pub fn get_value(&self, key: &str) -> Option<String> {
        self.0.get_item(key).unwrap_or(Some("Unknown".to_string()))
    }

    #[wasm_bindgen]
    pub fn set_value(&self, key: &str, value: &str) -> Result<(), JsValue> {
        self.0.set_item(key, value)
    }

    #[wasm_bindgen]
    pub fn remove_value(&self, key: &str) -> Result<(), JsValue> {
        self.0.remove_item(key)
    }
}
