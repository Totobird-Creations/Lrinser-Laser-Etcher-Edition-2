#![allow(unused_parens)]
#![feature(decl_macro)]


use static_init::dynamic;
use colored::Colorize;
use chrono::{DateTime, Utc};
#[dynamic]
static START : DateTime<Utc> = Utc::now();
use loggerithm::{logger, log};
use loggerithm::logger::Logger;
use loggerithm::level::{DEBUG, INFO, SUCCESS};
logger!(Logger::new()
    .set_min_severity(DEBUG::SEVERITY)
    .add_target(|context| {
        let duration = (Utc::now() - *START).to_std().unwrap();
        let hours    = duration.as_secs() / 3600;
        let minutes  = duration.as_secs() % 3600 / 60;
        let seconds  = duration.as_secs() % 60;
        let decimal  = duration.subsec_nanos();
        eprintln!(
            " [ {:0<29} ] [ {} ] [ {} ] [ {} ] {}",
            context.time_local()
                .format("%Y-%m-%d %H:%M:%S.%f").to_string()
                .bright_green().dimmed(),
            format!(
                "elapsed {:0>2}:{:0>2}:{:0>2}.{:0>9}",
                hours,
                minutes,
                seconds,
                decimal,
            ).bright_green(),
            context.module_p()
                .green().dimmed(),
            context.level_name_fp(),
            context.formatted(context.message())
        )
    })
);

pub mod helper;
pub mod parse;
pub mod render;
use parse::node::{Node, NodeBase};
use render::{render, settings::RenderSettings};


fn main() {
    log!(INFO, "Initialised.");
    log!(INFO, "Reading equations from {}.", "<null>");

    /*let eq0 = *Node::new(NodeBase::Equals(
        Node::new(NodeBase::Addition(
            Node::new(NodeBase::Power(
                Node::new(NodeBase::Variable(String::from("x"))),
                Node::new(NodeBase::Number(2.0))
            )),
            Node::new(NodeBase::Power(
                Node::new(NodeBase::Variable(String::from("y"))),
                Node::new(NodeBase::Number(2.0))
            ))
        )),
        Node::new(NodeBase::Power(
            Node::new(NodeBase::Number(2.5)),
            Node::new(NodeBase::Number(2.0))
        ))
    ));*/
    /*let eq1 = *Node::new(NodeBase::Equals(
        Node::new(NodeBase::Variable(String::from("y"))),
        Node::new(NodeBase::SquareRoot(
            Node::new(NodeBase::Subtraction(
                Node::new(NodeBase::Power(
                    Node::new(NodeBase::Number(2.5)),
                    Node::new(NodeBase::Number(2.0))
                )),
                Node::new(NodeBase::Power(
                    Node::new(NodeBase::Variable(String::from("x"))),
                    Node::new(NodeBase::Number(2.0))
                ))
            ))
        ))
    ));*/
    let eq0 = *Node::new(NodeBase::Equals(
        Node::new(NodeBase::Variable(String::from("y"))),
        Node::new(NodeBase::Addition(
            Node::new(NodeBase::Sine(
                Node::new(NodeBase::Variable(String::from("x")))
            )),
            Node::new(NodeBase::Multiplication(
                Node::new(NodeBase::Variable(String::from("x"))),
                Node::new(NodeBase::Number(0.0))
            ))
        ))
    ));
    let equations = vec![eq0];
    log!(DEBUG, "Loaded {} equation{}.", equations.len(), if (equations.len() == 1) {""} else {"s"});

    let settings = RenderSettings {
        frame: [-5.0, -5.0, 5.0, 5.0],
        split_depth: 8,
        resolution: [0, 0],
        target: String::from("target.png"),
    };
    render(equations, settings);

    log!(SUCCESS, "Finished.");
}
