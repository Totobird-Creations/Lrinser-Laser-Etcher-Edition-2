mod node;
pub mod settings;
use node::RenderNode;
use settings::RenderSettings;

use super::parse::node::Node;
use image::ImageBuffer;


pub fn render(_node : Node, settings : RenderSettings) {
    let     resolution = get_resolution(&settings);
    let mut buffer     = ImageBuffer::new(resolution[0], resolution[1]);

    let render_node_tree = generate_render_node_tree(&settings);
    println!("{:?}", render_node_tree);

    // Write pixels.
    for (pixel_x, pixel_y_reversed, pixel) in buffer.enumerate_pixels_mut() {
        let pixel_y = resolution[1] - pixel_y_reversed;
        *pixel = image::Rgba([
            render_node_tree.get_pixel([
                (pixel_x as f32) / (resolution[0] as f32),
                (pixel_y as f32) / (resolution[1] as f32)
            ]),
            0u8,
            0u8,
            255u8
        ]);
    }

    // Write file.
    match (buffer.save(settings.target)) {
        Ok(_)  => (),
        Err(_) => panic!("Image write failed.")
    };
}

// If settings define resolution as 0 or less, return 2 ** iterations.
fn get_resolution(settings : &RenderSettings) -> [u32; 2] {
    let mut resolution_x = settings.resolution[0];
    if (resolution_x <= 0) {
        resolution_x = u32::pow(2, settings.iterations);
    }
    let mut resolution_y = settings.resolution[1];
    if (resolution_y <= 0) {
        resolution_y = u32::pow(2, settings.iterations);
    }
    return [resolution_x, resolution_y];
}

// Generate grid and split.
fn generate_render_node_tree(settings : &RenderSettings) -> RenderNode {
    let mut render_node_tree = RenderNode::new();
    for _i in 0..settings.iterations {
        render_node_tree.check();
        render_node_tree.split();
    }
    render_node_tree.check();
    return render_node_tree;
}
