/// Serialization layer: converts parsed pings to either JSON or a flat Float32Array.
///
/// JSON mode: Vec<PingJson> serialized with serde_json.
/// Binary mode: interleaved [X, Y, Z, U, V, W, Intensity] per valid bin, 7 f32s each.

use serde::Serialize;

#[derive(Serialize)]
pub struct BinJson {
    /// Negative depth in meters (Y axis, below surface is negative).
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub w: f32,
    pub intensity: f32,
}

#[derive(Serialize)]
pub struct PingJson {
    pub timestamp: String,
    pub x: Option<f32>, // null in JSON when no GPS fix (raw PD0 files)
    pub z: Option<f32>,
    pub bins: Vec<BinJson>,
}

pub fn to_json(pings: &[PingJson]) -> Result<String, serde_json::Error> {
    serde_json::to_string(pings)
}

/// Builds a flat Vec<f32> — 7 elements per valid bin: X, Y, Z, U, V, W, Intensity.
/// Transfers as a JS Float32Array backed by wasm linear memory (zero-copy path).
pub fn to_binary(pings: &[PingJson]) -> Vec<f32> {
    let total_bins: usize = pings.iter().map(|p| p.bins.len()).sum();
    let mut buf = Vec::with_capacity(total_bins * 7);
    for ping in pings {
        for bin in &ping.bins {
            buf.push(ping.x.unwrap_or(0.0));
            buf.push(bin.y);
            buf.push(ping.z.unwrap_or(0.0));
            buf.push(bin.u);
            buf.push(bin.v);
            buf.push(bin.w);
            buf.push(bin.intensity);
        }
    }
    buf
}
