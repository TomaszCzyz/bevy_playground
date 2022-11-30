/// A cylinder with hemispheres at the top and bottom
#[derive(Debug, Copy, Clone)]
pub struct Cylinder {
    /// Radius on the `XZ` plane.
    pub radius: f32,
    /// Number of sections in cylinder between hemispheres.
    pub rings: usize,
    /// Height of the middle cylinder on the `Y` axis.
    pub depth: f32,
    /// Number of latitudes, distributed by inclination. Must be even.
    pub latitudes: usize,
    /// Number of longitudes, or meridians, distributed by azimuth.
    pub longitudes: usize,
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            radius: 0.5,
            rings: 0,
            depth: 1.0,
            latitudes: 8,
            longitudes: 16,
        }
    }
}
