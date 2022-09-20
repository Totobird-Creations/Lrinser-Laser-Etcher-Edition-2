#![allow(unused_parens)]

use std::io::Write;

use env_logger;

pub mod parse;
pub mod render;
pub use parse::node::{Node, NodeBase};
use render::{render, settings::RenderSettings};


fn main() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(buf, "{}: {}", record.level(), record.args())
        })
        .init();


    // sin(|x|)
    let a = Node::new(NodeBase::SinFunction(
        Node::new(NodeBase::AbsFunction(
            Node::new(NodeBase::Variable(String::from("x")))
        ))
    ));
    // x + [-1, 1]
    let b = Node::new(NodeBase::AdditionOperation(
        Node::new(NodeBase::Variable(String::from("x"))),
        Node::new(NodeBase::MultiValue(vec![
            Node::new(NodeBase::Number(-1.0)),
            Node::new(NodeBase::Number(1.0))
        ]))
    ));
    let tree = Node::new(NodeBase::MultiValue(vec![a, b]));
    let eq   = *Node::new(NodeBase::Equals(
        Node::new(NodeBase::Variable(String::from("y"))),
        tree
    ));

    let settings = RenderSettings {
        frame: [-5.0, -5.0, 5.0, 5.0],
        iterations: 6,
        resolution: [0, 0],
        target: String::from("target.png"),
    };
    render(vec![eq], settings);
}
