use std::fmt;

use http::HttpError;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    JsWasm(js_wasm::error::Error),
    JsValue(JsValue),
    Http(http::HttpError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::JsWasm(e) => write!(f, "{}", e),
            Error::JsValue(e) => write!(f, "{:?}", e),
            Error::Http(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<js_wasm::error::Error> for Error {
    fn from(e: js_wasm::error::Error) -> Self {
        Error::JsWasm(e)
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Error::JsValue(e)
    }
}

impl From<HttpError> for Error {
    fn from(e: HttpError) -> Self {
        Error::Http(e)
    }
}