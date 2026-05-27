# src/coords.rs

## Purpose
GPS-to-Cartesian conversion and depth-bin center calculation.
All arithmetic is f64 to preserve sub-meter precision before downcast to f32 for output.

## Coordinate system
- **X** = East offset from anchor (meters)
- **Y** = negative depth below water surface (meters; output as −depth)
- **Z** = North offset from anchor (meters)

## GPS conversion
Uses equirectangular approximation with a midpoint latitude correction for the
East component. Accurate to sub-meter for survey distances under ~500 km.

```
x = dlon_rad × cos(mid_lat) × R_earth
z = dlat_rad × R_earth
```

## Depth-bin center formula
```
depth_m = blank_distance_cm / 100
        + cell_size_cm / 100 × (bin_index + 0.5)
        + transducer_depth_dm / 10
```

`bin_index` is 0-based. `transducer_depth_dm` offsets for ADCP mounted below the hull.
