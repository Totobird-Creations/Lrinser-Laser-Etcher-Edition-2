use std::collections::HashMap;
use std::process;

use image::{ImageBuffer, GrayImage};
use loggerithm::{logger, log};
use loggerithm::level::{TRACE, DEBUG, FATAL};
logger!(super);

mod node;
pub mod settings;
use node::RenderNode;
use settings::RenderSettings;

use crate::helper;
use crate::parse::node::Node;
use crate::parse::values::EvaluatedValues;
use crate::parse::var;


pub fn render(nodes : Vec<Node>, settings : RenderSettings) {
    let resolution = get_resolution(&settings);
    log!(DEBUG,
        "Setting resolution to {},{} for {} iteration{}.",
        helper::commaify_i64(resolution[0].into()), helper::commaify_i64(resolution[1].into()),
        settings.split_depth,
        if (settings.split_depth == 1) {""} else {"s"}
    );

    let column_values = generate_column_values(&settings, &resolution, &nodes);

    let render_node_tree = generate_render_node_tree(&settings, &column_values);

    // Write pixels.
    let mut buffer : GrayImage = ImageBuffer::new(resolution[0], resolution[1]);
    log!(DEBUG,
        "Writing {} pixel{} to image buffer.",
        helper::commaify_i64((resolution[0] * resolution[1]).into()),
        if (resolution[0] * resolution[1] == 1) {""} else {"s"}
    );
    for (pixel_x, pixel_y_reversed, pixel) in buffer.enumerate_pixels_mut() {
        let pixel_y = resolution[1] - (pixel_y_reversed + 1);
        let colour  = render_node_tree.get_pixel([
            (pixel_x as f32) / (resolution[0] as f32),
            (pixel_y as f32) / (resolution[1] as f32)
        ]);
        *pixel = image::Luma(colour);
    }

    // Write file.
    log!(DEBUG, "Writing image buffer to file `{}`.", settings.target.replace("\\", "\\\\").replace("`", "\\`"));
    match (buffer.save(settings.target)) {
        Ok(_)  => (),
        Err(_) => {
            log!(FATAL, "Image write failed.");
            process::exit(1);
        }
    };
}

// If settings define resolution as 0 or less, return 2 ** iterations.
fn get_resolution(settings : &RenderSettings) -> [u32; 2] {
    let mut resolution_x = settings.resolution[0];
    if (resolution_x <= 0) {
        resolution_x = u32::pow(2, settings.split_depth);
    }
    let mut resolution_y = settings.resolution[1];
    if (resolution_y <= 0) {
        resolution_y = u32::pow(2, settings.split_depth);
    }
    return [resolution_x, resolution_y];
}

// Generate values for each column.
fn generate_column_values(settings : &RenderSettings, resolution : &[u32; 2], nodes : &Vec<Node>) -> Vec<EvaluatedValues> {
    log!(DEBUG,
        "Generating values for {} column{}.",
        helper::commaify_i64((resolution[0] + 1).into()),
        if (resolution[0] + 1 == 1) {""} else {"s"}
    );
    let mut columns   = vec![];
    let mut variables = HashMap::new();
    for i in 0..resolution[0] + 1 {
        variables.clear();
        let x = EvaluatedValues::from(vec![
            settings.frame[0] + (settings.frame[2] - settings.frame[0]) * ((i as f64) / (resolution[0] as f64))
        ]);
        let mut values = EvaluatedValues::new();
        for node in nodes {
            variables.insert(String::from("x"), EvaluatedValues::copy(&x));
            insert_consts(&mut variables);
            node.evaluate(&String::from("y"), &mut variables);
            if (variables.contains_key(&String::from("y"))) {
                values = values.add(&variables.remove(&String::from("y")).unwrap());
            }
        }
        values = values.compress(&settings);
        log!(TRACE,
            "Value {} found for column {}.",
            values,
            helper::commaify_i64(i.into())
        );
        columns.push(values);
    }
    return columns;
}

// Add the constants to the variable set.
fn insert_consts(variables : &mut HashMap<String, EvaluatedValues>) {
    variables.insert(String::from("pi"  ), EvaluatedValues::from(vec![var::PI  ]));
    variables.insert(String::from("??"   ), EvaluatedValues::from(vec![var::PI  ]));
    variables.insert(String::from("tau" ), EvaluatedValues::from(vec![var::TAU ]));
    variables.insert(String::from("????"   ), EvaluatedValues::from(vec![var::TAU ]));
    variables.insert(String::from("phi" ), EvaluatedValues::from(vec![var::PHI ]));
    variables.insert(String::from("??"   ), EvaluatedValues::from(vec![var::PHI ]));
    variables.insert(String::from("e"   ), EvaluatedValues::from(vec![var::E   ]));
}

// Generate grid and split.
fn generate_render_node_tree(settings : &RenderSettings, column_values : &Vec<EvaluatedValues>) -> RenderNode {
    log!(DEBUG, "Generating render node tree.");
    let mut render_node_tree = RenderNode::new(settings.split_depth);
    for _i in 0..settings.split_depth + 1 {
        render_node_tree.check(settings, column_values);
        render_node_tree.split();
    }
    return render_node_tree;
}
