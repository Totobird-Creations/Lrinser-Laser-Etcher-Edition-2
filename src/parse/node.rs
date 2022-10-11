use std::collections::HashMap;
use std::process;

use loggerithm::{logger, log};
use loggerithm::level::ERROR;
logger!(super);

use crate::parse::values::EvaluatedValues;


#[derive(Debug)]
pub struct Node {
    pub base : NodeBase,
}
impl Node {
    pub fn new(base : NodeBase) -> Box<Node> {
        return Box::new(Node {
            base
        });
    }
    pub fn to_string(&self) -> String {
        return self.base.to_string();
    }
    pub fn evaluate(&self, variables : &mut HashMap<String, EvaluatedValues>) -> EvaluatedValues {
        return self.base.evaluate(variables);
    }
}


#[derive(Debug)]
pub enum NodeBase {
    
    Addition       (Box<Node>, Box<Node>), // Left (l), Right (r)  : l + r
    Subtraction    (Box<Node>, Box<Node>), // Left (l), Right (r)  : l - r
    Multiplication (Box<Node>, Box<Node>), // Left (l), Right (r)  : l * r
    Division       (Box<Node>, Box<Node>), // Top (t), Bottom (b)  : t / b
    Power          (Box<Node>, Box<Node>), // Base (b), Degree (d) : bᵈ

    AbsoluteValue       (Box<Node>),
    SquareRoot          (Box<Node>),
    NthRoot             (Box<Node>, Box<Node>), // Degree (n), Powered : ⁿ√(Powered)
    Sine                (Box<Node>),
    Cosine              (Box<Node>),
    Tangent             (Box<Node>),
    Cosecant            (Box<Node>),
    Secant              (Box<Node>),
    Cotangent           (Box<Node>),
    InverseSine         (Box<Node>),
    InverseCosine       (Box<Node>),
    InverseTangent      (Box<Node>),
    InverseCosecant     (Box<Node>),
    InverseSecant       (Box<Node>),
    InverseCotangent    (Box<Node>),
    HyperbolicSine      (Box<Node>),
    HyperbolicCosine    (Box<Node>),
    HyperbolicTangent   (Box<Node>),
    HyperbolicCosecant  (Box<Node>),
    HyperbolicSecant    (Box<Node>),
    HyperbolicCotangent (Box<Node>),
    Exponential         (Box<Node>),            // Degree (d)           : eᵈ
    NaturalLogarithm    (Box<Node>),            // Result (r)           : logₑ(r)
    Logartithm          (Box<Node>, Box<Node>), // Base (b), Result (r) : logᵦ(r)
    Modulo              (Box<Node>, Box<Node>),
    Ceiling             (Box<Node>),
    Floor               (Box<Node>),
    Round               (Box<Node>),
    Sign                (Box<Node>),

    MultiValue (Vec<Box<Node>>),
    Number     (f64),
    Variable   (String),

    Equals (Box<Node>, Box<Node>), // Left, Right : Left = Right
    
}
impl NodeBase {
    pub fn to_string(&self) -> String {
        return match (self) {

            NodeBase::Addition       (left, right) => format!("({} + {})", (*left).to_string(), (*right).to_string()),
            NodeBase::Subtraction    (left, right) => format!("({} - {})", (*left).to_string(), (*right).to_string()),
            NodeBase::Multiplication (left, right) => format!("({} * {})", (*left).to_string(), (*right).to_string()),
            NodeBase::Division       (left, right) => format!("({} / {})", (*left).to_string(), (*right).to_string()),
            NodeBase::Power          (left, right) => format!("({} ^ {})", (*left).to_string(), (*right).to_string()),

            NodeBase::AbsoluteValue       (arg)  => format!("|{}|", arg.to_string()),
            NodeBase::SquareRoot          (arg)  => format!("sqrt({})", arg.to_string()),
            NodeBase::NthRoot             (n, p) => format!("nthroot({}, {})", n.to_string(), p.to_string()),
            NodeBase::Sine                (arg)  => format!("sin({})", arg.to_string()),
            NodeBase::Cosine              (arg)  => format!("cos({})", arg.to_string()),
            NodeBase::Tangent             (arg)  => format!("tan({})", arg.to_string()),
            NodeBase::Cosecant            (arg)  => format!("scs({})", arg.to_string()),
            NodeBase::Secant              (arg)  => format!("sec({})", arg.to_string()),
            NodeBase::Cotangent           (arg)  => format!("cot({})", arg.to_string()),
            NodeBase::InverseSine         (arg)  => format!("asin({})", arg.to_string()),
            NodeBase::InverseCosine       (arg)  => format!("acos({})", arg.to_string()),
            NodeBase::InverseTangent      (arg)  => format!("atan({})", arg.to_string()),
            NodeBase::InverseCosecant     (arg)  => format!("ascs({})", arg.to_string()),
            NodeBase::InverseSecant       (arg)  => format!("asec({})", arg.to_string()),
            NodeBase::InverseCotangent    (arg)  => format!("acot({})", arg.to_string()),
            NodeBase::HyperbolicSine      (arg)  => format!("sinh({})", arg.to_string()),
            NodeBase::HyperbolicCosine    (arg)  => format!("cosh({})", arg.to_string()),
            NodeBase::HyperbolicTangent   (arg)  => format!("tanh({})", arg.to_string()),
            NodeBase::HyperbolicCosecant  (arg)  => format!("scsh({})", arg.to_string()),
            NodeBase::HyperbolicSecant    (arg)  => format!("sech({})", arg.to_string()),
            NodeBase::HyperbolicCotangent (arg)  => format!("coth({})", arg.to_string()),
            NodeBase::Exponential         (arg)  => format!("exp({})", arg.to_string()),
            NodeBase::NaturalLogarithm    (arg)  => format!("ln({})", arg.to_string()),
            NodeBase::Logartithm          (b, r) => format!("log({}, {})", b.to_string(), r.to_string()),
            NodeBase::Modulo              (a, b) => format!("mod({}, {})", a.to_string(), b.to_string()),
            NodeBase::Ceiling             (arg)  => format!("ceil({})", arg.to_string()),
            NodeBase::Floor               (arg)  => format!("floor({})", arg.to_string()),
            NodeBase::Round               (arg)  => format!("round({})", arg.to_string()),
            NodeBase::Sign                (arg)  => format!("sign({})", arg.to_string()),

            NodeBase::MultiValue        (values)      => {
                let mut string = vec![];
                for i in 0..values.len() {
                    string.push((*values[i]).to_string());
                }
                format!("[{}]", string.join(", "))
            },
            NodeBase::Number            (value)       => value.to_string(),
            NodeBase::Variable          (name)        => String::from(name),

            NodeBase::Equals (left, right) => format!("({} = {})", (*left).to_string(), (*right).to_string())

        };
    }
    pub fn evaluate(&self, target_variable : String, variables : &mut HashMap<String, EvaluatedValues>) -> EvaluatedValues {
        let values = match (self) {
            
            NodeBase::Addition       (left, right) => left.evaluate(variables).addition(&right.evaluate(variables)),
            NodeBase::Subtraction    (left, right) => left.evaluate(variables).subtraction(&right.evaluate(variables)),
            NodeBase::Multiplication (left, right) => left.evaluate(variables).multiplication(&right.evaluate(variables)),
            NodeBase::Division       (left, right) => left.evaluate(variables).division(&right.evaluate(variables)),
            NodeBase::Power          (left, right) => left.evaluate(variables).power(&right.evaluate(variables)),

            NodeBase::AbsoluteValue       (arg)  => arg.evaluate(variables).absolute_value(),
            NodeBase::SquareRoot          (arg)  => arg.evaluate(variables).square_root(),
            NodeBase::NthRoot             (n, p) => p.evaluate(variables).nth_root(&n.evaluate(variables)),
            NodeBase::Sine                (arg)  => arg.evaluate(variables).sine(),
            NodeBase::Cosine              (arg)  => arg.evaluate(variables).cosine(),
            NodeBase::Tangent             (arg)  => arg.evaluate(variables).tangent(),
            NodeBase::Cosecant            (arg)  => arg.evaluate(variables).cosecant(),
            NodeBase::Secant              (arg)  => arg.evaluate(variables).secant(),
            NodeBase::Cotangent           (arg)  => arg.evaluate(variables).cotangent(),
            NodeBase::InverseSine         (arg)  => arg.evaluate(variables).inverse_sine(),
            NodeBase::InverseCosine       (arg)  => arg.evaluate(variables).inverse_cosine(),
            NodeBase::InverseTangent      (arg)  => arg.evaluate(variables).inverse_tangent(),
            NodeBase::InverseCosecant     (arg)  => arg.evaluate(variables).inverse_cosecant(),
            NodeBase::InverseSecant       (arg)  => arg.evaluate(variables).inverse_secant(),
            NodeBase::InverseCotangent    (arg)  => arg.evaluate(variables).inverse_cotangent(),
            NodeBase::HyperbolicSine      (arg)  => arg.evaluate(variables).hyperbolic_sine(),
            NodeBase::HyperbolicCosine    (arg)  => arg.evaluate(variables).hyperbolic_cosine(),
            NodeBase::HyperbolicTangent   (arg)  => arg.evaluate(variables).hyperbolic_tangent(),
            NodeBase::HyperbolicCosecant  (arg)  => arg.evaluate(variables).hyperbolic_cosecant(),
            NodeBase::HyperbolicSecant    (arg)  => arg.evaluate(variables).hyperbolic_secant(),
            NodeBase::HyperbolicCotangent (arg)  => arg.evaluate(variables).hyperbolic_cotangent(),
            NodeBase::Exponential         (arg)  => arg.evaluate(variables).exponential(),
            NodeBase::NaturalLogarithm    (arg)  => arg.evaluate(variables).natural_logarithm(),
            NodeBase::Logartithm          (b, r) => r.evaluate(variables).logarithm(&b.evaluate(variables)),
            NodeBase::Modulo              (a, b) => a.evaluate(variables).modulo(&b.evaluate(variables)),
            NodeBase::Ceiling             (arg)  => arg.evaluate(variables).ceiling(),
            NodeBase::Floor               (arg)  => arg.evaluate(variables).floor(),
            NodeBase::Round               (arg)  => arg.evaluate(variables).round(),
            NodeBase::Sign                (arg)  => arg.evaluate(variables).sign(),

            NodeBase::MultiValue        (values)      => {
                let mut evaluated_values = EvaluatedValues::new();
                for i in 0..values.len() {
                    evaluated_values = evaluated_values.add(&values[i].evaluate(variables));
                }
                evaluated_values
            },
            NodeBase::Number            (value)       => EvaluatedValues::new().push(*value),
            NodeBase::Variable          (name)        => {
                if (variables.contains_key(name)) {
                    EvaluatedValues::new().add(&variables.get(name).unwrap())
                } else {
                    log!(ERROR, "Variable `{}` not defined.", name);
                    process::exit(1);
                }
            }

            NodeBase::Equals (left, right) => {
                match (&left.base) {
                    NodeBase::Variable(name) => {
                        let values = right.evaluate(variables);
                        variables.insert(String::from(name), values);
                    },
                    _ => ()
                }
                 EvaluatedValues::new()
            }
            
        };
        return values;
    }
}
