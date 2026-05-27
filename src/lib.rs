/// wasm-bindgen entry point for enx-wasm-parser.
///
/// Exposes a single `parse_enx` function callable from JavaScript/TypeScript.
/// Returns a JS object with `data` (Float32Array or JSON string) and `metadata`.

mod coords;
mod echo_intensity;
mod error;
mod fixed_leader;
mod header;
mod output;
mod parser;
mod variable_leader;
mod velocity;
mod vmdas_nav;

use output::{to_binary, to_json};
use parser::parse_file;
use wasm_bindgen::prelude::*;
use js_sys::{Float32Array, Object, Reflect};

#[wasm_bindgen]
pub fn parse_enx(
    data: &[u8],
    anchor_lat: f64,
    anchor_lon: f64,
    output_format: &str,
) -> Result<JsValue, JsValue> {
    let t0 = js_sys::Date::now();

    let result = parse_file(data, anchor_lat, anchor_lon)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let duration_ms = js_sys::Date::now() - t0;

    let metadata = Object::new();
    Reflect::set(&metadata, &"totalPings".into(), &(result.total_pings as f64).into())?;
    Reflect::set(&metadata, &"validPings".into(), &(result.pings.len() as f64).into())?;
    Reflect::set(&metadata, &"droppedCorruptBins".into(), &(result.dropped_corrupt_bins as f64).into())?;
    Reflect::set(&metadata, &"pingsWithoutGps".into(), &(result.pings_without_gps as f64).into())?;
    Reflect::set(&metadata, &"durationMs".into(), &duration_ms.into())?;

    let out = Object::new();
    Reflect::set(&out, &"metadata".into(), &metadata.into())?;

    if output_format == "binary" {
        let flat = to_binary(&result.pings);
        let js_arr = Float32Array::new_with_length(flat.len() as u32);
        js_arr.copy_from(&flat);
        Reflect::set(&out, &"data".into(), &js_arr.into())?;
    } else {
        let json = to_json(&result.pings)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Reflect::set(&out, &"data".into(), &json.into())?;
    }

    Ok(out.into())
}
