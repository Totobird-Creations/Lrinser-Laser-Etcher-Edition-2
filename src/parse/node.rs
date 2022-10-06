use std::collections::HashMap;
use std::fmt;
use std::process;

use loggerithm::{logger, log};
use loggerithm::level::ERROR;
logger!(super);

use crate::parse::var;
use crate::render::settings::RenderSettings;


#[derive(Debug)]
pub struct Node {
    pub base : NodeBase,
}
impl Node {
    pub fn new(base : NodeBase) -> Box<Node> {
        return Box::new(Node {base : base});
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
    pub fn evaluate(&self, variables : &mut HashMap<String, EvaluatedValues>) -> EvaluatedValues {
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


#[derive(Debug)]
pub struct EvaluatedValues {
    values : Vec<f64>
}
impl EvaluatedValues {
    pub fn new() -> EvaluatedValues {
        return EvaluatedValues {values: vec![]};
    }
    pub fn from(values : Vec<f64>) -> EvaluatedValues {
        return EvaluatedValues {values: values};
    }
    pub fn copy(values : &EvaluatedValues) -> EvaluatedValues {
        return EvaluatedValues::from(values.values.clone());
    }

    pub fn add(&self, values : &EvaluatedValues) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::copy(self);
        for i in 0..values.values.len() {
            let value = values.values[i];
            if (! new_values.values.contains(&value)) {
                new_values.values.push(value);
            }
        }
        return new_values;
    }
    pub fn push(&self, value : f64) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::copy(self);
        if (! new_values.values.contains(&value)) {
            new_values.values.push(value);
        }
        return new_values;
    }
    pub fn get_values(&self) -> &Vec<f64> {
        return &self.values;
    }
    pub fn compress(&self, settings : &RenderSettings) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| {
            if (a >= settings.frame[1] && a < settings.frame[3]) {
                new_values.values.push(a);
            }
        });
    }

    pub fn addition(&self, other : &EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(a + b));
    }
    pub fn subtraction(&self, other : &EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(a - b));
    }
    pub fn multiplication(&self, other : &EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(a * b));
    }
    pub fn division(&self, other : &EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| {
            if (b != 0.0) {
                new_values.values.push(a * b);
            }
        });
    }
    pub fn power(&self, other : &EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(a.powf(b)));
    }


    pub fn absolute_value(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.abs()));
    }
    pub fn square_root(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| {
            if (a >= 0.0) {
                new_values.values.push(a.sqrt());
            }
        });
    }
    pub fn nth_root(&self, _degree : &EvaluatedValues) -> EvaluatedValues {
        panic!("Unimplemented.");
    }
    pub fn sine(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.sin()));
    }
    pub fn cosine(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.cos()));
    }
    pub fn tangent(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.tan()));
    }
    pub fn cosecant(&self) -> EvaluatedValues {
        return EvaluatedValues::from(vec![1.0]).division(&self.sine());
    }
    pub fn secant(&self) -> EvaluatedValues {
        return EvaluatedValues::from(vec![1.0]).division(&self.cosine());
    }
    pub fn cotangent(&self) -> EvaluatedValues {
        return EvaluatedValues::from(vec![1.0]).division(&self.tangent());
    }
    pub fn inverse_sine(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.asin()));
    }
    pub fn inverse_cosine(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.acos()));
    }
    pub fn inverse_tangent(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.atan()));
    }
    pub fn inverse_cosecant(&self) -> EvaluatedValues {
        panic!("Unimplemented.");
        //return EvaluatedValues::from(vec![1.0]).division(self).inverse_sine();
    }
    pub fn inverse_secant(&self) -> EvaluatedValues {
        panic!("Unimplemented.");
        //return EvaluatedValues::from(vec![1.0]).division(self).inverse_cosine();
    }
    pub fn inverse_cotangent(&self) -> EvaluatedValues {
        panic!("Unimplemented.");
        //return EvaluatedValues::from(vec![1.0]).division(self).inverse_tangent();
    }
    pub fn hyperbolic_sine(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.sinh()));
    }
    pub fn hyperbolic_cosine(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.cosh()));
    }
    pub fn hyperbolic_tangent(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.tanh()));
    }
    pub fn hyperbolic_cosecant(&self) -> EvaluatedValues {
        return self.hyperbolic_operation(None, Some(-1.0));
    }
    pub fn hyperbolic_secant(&self) -> EvaluatedValues {
        return self.hyperbolic_operation(None, Some(1.0));
    }
    pub fn hyperbolic_cotangent(&self) -> EvaluatedValues {
        return self.hyperbolic_operation(Some(1.0), Some(-1.0));
    }
    pub fn exponential(&self) -> EvaluatedValues {
        return EvaluatedValues::from(vec![var::E]).power(self);
    }
    pub fn natural_logarithm(&self) -> EvaluatedValues {
        return self.logarithm(&EvaluatedValues::from(vec![var::E]));
    }
    pub fn logarithm(&self, other : &EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(a.log(b)));
    }
    pub fn modulo(&self, other : &EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(-b * (a / b).floor() + a));
    }
    pub fn ceiling(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.ceil()));
    }
    pub fn floor(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.floor()));
    }
    pub fn round(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.round()));
    }
    pub fn sign(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| {
            new_values.values.push(if (a == 0.0) {0.0} else {a / a.abs()})
        });
    }

    
    fn unary_operation<T>(&self, target : T) -> EvaluatedValues
        where T : Fn(f64, &mut EvaluatedValues)
    {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            target(self.values[a], &mut new_values);
        }
        return new_values;
    }
    fn binary_operation<T>(&self, other : &EvaluatedValues, target : T) -> EvaluatedValues
        where T : Fn(f64, f64, &mut EvaluatedValues)
    {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            for b in 0..other.values.len() {
                target(self.values[a], other.values[b], &mut new_values);
            }
        }
        return new_values;
    }
    fn hyperbolic_operation(&self, top : Option<f64>, bottom : Option<f64>) -> EvaluatedValues {
        return self.hyperbolic_operation_get_side(top)
            .division(&self.hyperbolic_operation_get_side(bottom));
        }
    fn hyperbolic_operation_get_side(&self, value : Option<f64>) -> EvaluatedValues {
        return match (value) {
            Some(sign) => EvaluatedValues::from(vec![var::E])
                .power(&self.multiplication(&EvaluatedValues::from(vec![-1.0])))
                .multiplication(&EvaluatedValues::from(vec![sign]))
                .addition(&EvaluatedValues::from(vec![var::E])
                    .power(&self.multiplication(self))
                ),
            None => EvaluatedValues::from(vec![2.0])
        };
    }
    
}
impl fmt::Display for EvaluatedValues {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.values
            .iter().map(|v|v.to_string())
            .collect::<Vec<String>>()
            .join(",")
        );
    }
}
