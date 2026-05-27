/// GPS-to-Cartesian conversion using equirectangular approximation.
///
/// Accurate to sub-meter for survey distances < 500 km from the anchor.
/// All math is f64 to avoid the precision loss that plagues WebGL f32.

const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Returns (x_east_m, z_north_m) offsets in meters relative to `anchor`.
pub fn to_cartesian(anchor_lat: f64, anchor_lon: f64, lat: f64, lon: f64) -> (f64, f64) {
    let dlat = (lat - anchor_lat).to_radians();
    let dlon = (lon - anchor_lon).to_radians();
    let mid_lat = ((lat + anchor_lat) / 2.0).to_radians();
    let x = dlon * mid_lat.cos() * EARTH_RADIUS_M;
    let z = dlat * EARTH_RADIUS_M;
    (x, z)
}

/// Returns the center depth (positive = below surface, meters) of bin `bin_index` (0-based).
pub fn bin_depth_m(blank_distance_cm: u16, cell_size_cm: u16, bin_index: usize, transducer_depth_dm: u16) -> f64 {
    let blank = blank_distance_cm as f64 / 100.0;
    let cell = cell_size_cm as f64 / 100.0;
    let transducer = transducer_depth_dm as f64 / 10.0;
    blank + cell * (bin_index as f64 + 0.5) + transducer
}
