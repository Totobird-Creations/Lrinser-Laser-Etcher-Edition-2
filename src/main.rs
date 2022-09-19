#![allow(unused_parens)]

pub mod parse;
pub use parse::node::{Node, NodeBase};
pub mod render;
use render::{render, settings::RenderSettings};

fn main() {
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
    let tree = *Node::new(NodeBase::MultiValue(vec![a, b]));

    let settings = RenderSettings {
        frame: [-5.0, -5.0, 5.0, 5.0],
        iterations: 7,
        resolution: [0, 0],
        target: String::from("target.png"),
    };
    render(tree, settings);
}
