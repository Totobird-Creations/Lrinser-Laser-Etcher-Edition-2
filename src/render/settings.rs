#[derive(Clone)]
pub struct RenderSettings {
    // Corners of graph : Left, Bottom, Right, Top
    pub frame: [f64; 4],
    // Split depth: Times
    pub iterations: u32,
    // Image size : Width, Height
    pub resolution: [u32; 2],
    // Filename
    pub target: String,
}
