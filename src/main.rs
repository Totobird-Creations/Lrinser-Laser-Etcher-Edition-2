#![allow(unused_parens)]

use log;

pub mod logger;
use logger::Logger;
pub mod parse;
pub mod render;
use parse::node::{Node, NodeBase};
use render::{render, settings::RenderSettings};


fn main() {
    Logger::init().expect("Logger failed to initialise.");
    log::info!("Initialised.");
    log::info!("Reading equations from {}.", "<null>");

    let eq = *Node::new(NodeBase::Equals(
        Node::new(NodeBase::Variable(String::from("y"))),
        Node::new(NodeBase::InverseTangent(
            Node::new(NodeBase::Variable(String::from("x")))
        ))
    ));
    let equations = vec![eq];
    log::debug!("Loaded {} equation{}.", equations.len(), if (equations.len() == 1) {""} else {"s"});

    let settings = RenderSettings {
        frame: [-5.0, -5.0, 5.0, 5.0],
        iterations: 6,
        resolution: [0, 0],
        target: String::from("target.png"),
    };
    render(equations, settings);

    log::info!("Finished.");
}
