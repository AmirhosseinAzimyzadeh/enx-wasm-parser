/// Unified error type for the parser.
///
/// All fatal parse errors bubble up as `ParseError::Fatal` with a human-readable
/// message. wasm-bindgen converts this to a JS Error on the TypeScript side.

use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    Fatal(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Fatal(msg) => write!(f, "ENX parse error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

impl From<ParseError> for wasm_bindgen::JsValue {
    fn from(e: ParseError) -> Self {
        wasm_bindgen::JsValue::from_str(&e.to_string())
    }
}
