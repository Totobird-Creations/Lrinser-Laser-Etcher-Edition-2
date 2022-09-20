mod node;
pub mod settings;
use node::RenderNode;
use settings::RenderSettings;

use crate::parse::node::{Node, EvaluatedValues};

use image::ImageBuffer;
use std::collections::HashMap;


pub fn render(nodes : Vec<Node>, settings : RenderSettings) {
    let     resolution = get_resolution(&settings);
    let mut buffer     = ImageBuffer::new(resolution[0], resolution[1]);

    let column_values = generate_column_values(&settings, &resolution, &node);

    let render_node_tree = generate_render_node_tree(&settings, &column_values);

    // Write pixels.
    for (pixel_x, pixel_y_reversed, pixel) in buffer.enumerate_pixels_mut() {
        let pixel_y = resolution[1] - (pixel_y_reversed + 1);
        let rgb     = render_node_tree.get_pixel([
            (pixel_x as f32) / (resolution[0] as f32),
            (pixel_y as f32) / (resolution[1] as f32)
        ]);
        *pixel = image::Rgba([
            rgb[0], rgb[1], rgb[2],
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

// Generate values for each column.
fn generate_column_values(settings : &RenderSettings, resolution : &[u32; 2], node : &Vec<Node>) -> Vec<EvaluatedValues> {
    let mut columns = vec![];
    for i in 0..resolution[0] + 1 {
        let mut variables  = HashMap::new();
        variables.insert(String::from("x"), EvaluatedValues::new().push(
            settings.frame[0] + (settings.frame[2] - settings.frame[0]) * ((i as f64) / (resolution[0] as f64))
        ));
        columns.push(node.evaluate(&variables).compress(&settings));
    }
    return columns;
}

// Generate grid and split.
fn generate_render_node_tree(settings : &RenderSettings, column_values : &Vec<EvaluatedValues>) -> RenderNode {
    let mut render_node_tree = RenderNode::new(settings.iterations);
    for _i in 0..settings.iterations + 1 {
        render_node_tree.check(settings, column_values);
        render_node_tree.split();
    }
    return render_node_tree;
}
