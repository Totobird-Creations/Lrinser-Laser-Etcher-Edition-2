#![allow(unused_parens)]

pub mod parse;
use parse::import;
pub mod render;
use render::{render, settings::RenderSettings};

fn main() {
    let tree = import::desmos::load("rjoka3r1q8");

    let settings = RenderSettings {
        frame: [-5.0, -5.0, 5.0, 5.0],
        iterations: 7,
        resolution: [0, 0],
        target: String::from("target.png"),
    };
    render(tree, settings);
}
