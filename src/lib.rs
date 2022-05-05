mod error;

use error::Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_wasm::*;

pub type Result<T> = std::result::Result<T, Error>;

#[wasm_bindgen(start)]
pub async fn main() -> std::result::Result<(), JsValue> {
    console_error_panic_hook::set_once();
    if let Err(e) = init().await {
        console_error!("{}", e);
    }
    Ok(())
}

async fn init() -> Result<()> {
    console_log!("Hello, World!");
    Ok(())
}