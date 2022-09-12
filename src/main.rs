#![allow(unused_parens)]

pub mod parse;
use parse::node::{Node, NodeBase};
pub mod render;
use render::{render, settings::RenderSettings};

fn main() {
    let tree = *Node::new(NodeBase::AdditionOperation(
        Node::new(NodeBase::MultiValue(vec![
            Node::new(NodeBase::Number(1.0)),
            Node::new(NodeBase::Number(-1.0)),
        ])),
        Node::new(NodeBase::Variable(String::from("x"))),
    ));
    println!("{}", tree.to_string());
    let settings = RenderSettings {
        frame: [-10.0, -10.0, 10.0, 10.0],
        iterations: 5,
        resolution: [0, 0],
        target: String::from("target.png"),
    };
    render(tree, settings);
}
