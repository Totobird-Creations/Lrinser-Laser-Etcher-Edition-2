use std::collections::HashMap;

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
    pub fn evaluate(&self, variables : &HashMap<String, EvaluatedValues>) -> EvaluatedValues {
        return self.base.evaluate(variables);
    }
}


#[derive(Debug)]
pub enum NodeBase {
    AdditionOperation       (Box<Node>, Box<Node>), // Left, Right
    SubtractionOperation    (Box<Node>, Box<Node>), // Left, Right
    MultiplicationOperation (Box<Node>, Box<Node>), // Left, Right
    DivisionOperation       (Box<Node>, Box<Node>), // Top, Bottom
    PowerOperation          (Box<Node>, Box<Node>), // Base, Degree

    AbsFunction     (Box<Node>),
    SignFunction    (Box<Node>),
    NthRootFunction (Box<Node>, Box<Node>), // Degree (n), Powered
    SinFunction     (Box<Node>),
    CosFunction     (Box<Node>),
    TanFunction     (Box<Node>),
    CotFunction     (Box<Node>),
    SecFunction     (Box<Node>),
    CscFunction     (Box<Node>),

    MultiValue (Vec<Box<Node>>),
    Number     (f64),
    Variable   (String)
}
impl NodeBase {
    pub fn to_string(&self) -> String {
        return match (self) {

            NodeBase::AdditionOperation       (left, right) => format!("({} + {})", (*left).to_string(), (*right).to_string()),
            NodeBase::SubtractionOperation    (left, right) => format!("({} - {})", (*left).to_string(), (*right).to_string()),
            NodeBase::MultiplicationOperation (left, right) => format!("({} * {})", (*left).to_string(), (*right).to_string()),
            NodeBase::DivisionOperation       (left, right) => format!("({} / {})", (*left).to_string(), (*right).to_string()),
            NodeBase::PowerOperation          (left, right) => format!("({} ^ {})", (*left).to_string(), (*right).to_string()),

            NodeBase::AbsFunction     (arg)  => format!("|{}|", arg.to_string()),
            NodeBase::SignFunction    (arg)  => format!("sign({})", arg.to_string()),
            NodeBase::NthRootFunction (n, p) => format!("nthroot({}, {})", n.to_string(), p.to_string()),
            NodeBase::SinFunction     (arg)  => format!("sin({})", arg.to_string()),
            NodeBase::CosFunction     (arg)  => format!("cos({})", arg.to_string()),
            NodeBase::TanFunction     (arg)  => format!("tan({})", arg.to_string()),
            NodeBase::CotFunction     (arg)  => format!("cot({})", arg.to_string()),
            NodeBase::SecFunction     (arg)  => format!("sec({})", arg.to_string()),
            NodeBase::CscFunction     (arg)  => format!("scs({})", arg.to_string()),

            NodeBase::MultiValue        (values)      => {
                let mut string = vec![];
                for i in 0..values.len() {
                    string.push((*values[i]).to_string());
                }
                format!("[{}]", string.join(", "))
            },
            NodeBase::Number            (value)       => value.to_string(),
            NodeBase::Variable          (name)        => name.clone()

        };
    }
    pub fn evaluate(&self, variables : &HashMap<String, EvaluatedValues>) -> EvaluatedValues {
        let values = match (self) {
            NodeBase::AdditionOperation       (left, right) => left.evaluate(variables).addition(right.evaluate(variables)),
            NodeBase::SubtractionOperation    (left, right) => left.evaluate(variables).subtraction(right.evaluate(variables)),
            NodeBase::MultiplicationOperation (left, right) => left.evaluate(variables).multiplication(right.evaluate(variables)),
            NodeBase::DivisionOperation       (left, right) => left.evaluate(variables).division(right.evaluate(variables)),
            NodeBase::PowerOperation          (left, right) => left.evaluate(variables).power(right.evaluate(variables)),

            NodeBase::AbsFunction     (arg)  => arg.evaluate(variables).abs(),
            NodeBase::SignFunction    (arg)  => arg.evaluate(variables).sign(),
            NodeBase::NthRootFunction (n, p) => p.evaluate(variables).nthroot(n.evaluate(variables)),
            NodeBase::SinFunction     (arg)  => arg.evaluate(variables).sin(),
            NodeBase::CosFunction     (arg)  => arg.evaluate(variables).cos(),
            NodeBase::TanFunction     (arg)  => arg.evaluate(variables).tan(),
            NodeBase::CotFunction     (arg)  => arg.evaluate(variables).cot(),
            NodeBase::SecFunction     (arg)  => arg.evaluate(variables).sec(),
            NodeBase::CscFunction     (arg)  => arg.evaluate(variables).csc(),

            NodeBase::MultiValue        (values)      => {
                let mut evaluated_values = EvaluatedValues::new();
                for i in 0..values.len() {
                    evaluated_values = evaluated_values.add((*values[i]).evaluate(variables));
                }
                evaluated_values
            },
            NodeBase::Number            (value)       => EvaluatedValues::new().push(*value),
            NodeBase::Variable          (name)        => {
                if (variables.contains_key(name)) {
                    EvaluatedValues::new().add(variables.get(name).unwrap().clone())
                } else {
                    panic!("Variable `{}` not defined.", name);
                }
            }
        };
        return values;
    }
}


#[derive(Clone, Debug)]
pub struct EvaluatedValues {
    values : Vec<f64>
}
impl EvaluatedValues {
    pub fn new() -> EvaluatedValues {
        return EvaluatedValues {values: vec![]};
    }
    pub fn copy(values : &EvaluatedValues) -> EvaluatedValues {
        return EvaluatedValues { values: values.values.clone() };
    }

    pub fn add(&self, values : EvaluatedValues) -> EvaluatedValues {
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
        let mut new_values = EvaluatedValues::new();
        for i in 0..self.values.len() {
            let value = self.values[i];
            if (value >= settings.frame[1] && value < settings.frame[3]) {
                new_values.values.push(value);
            }
        }
        return new_values;
    }

    pub fn addition(&self, other : EvaluatedValues) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            for b in 0..other.values.len() {
                new_values.values.push(self.values[a] + other.values[b]);
            }
        }
        return new_values;
    }
    pub fn subtraction(&self, other : EvaluatedValues) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            for b in 0..other.values.len() {
                new_values.values.push(self.values[a] - other.values[b]);
            }
        }
        return new_values;
    }
    pub fn multiplication(&self, other : EvaluatedValues) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            for b in 0..other.values.len() {
                new_values.values.push(self.values[a] * other.values[b]);
            }
        }
        return new_values;
    }
    pub fn division(&self, other : EvaluatedValues) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            for b in 0..other.values.len() {
                let c = other.values[b];
                if (c != 0.0) {
                    new_values.values.push(self.values[a] / c);
                }
            }
        }
        return new_values;
    }
    pub fn power(&self, other : EvaluatedValues) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            for b in 0..other.values.len() {
                new_values.values.push(self.values[a].powf(other.values[b]));
            }
        }
        return new_values;
    }

    pub fn abs(&self) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            new_values.values.push(self.values[a].abs());
        }
        return new_values;
    }
    pub fn sign(&self) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            let b = self.values[a];
            new_values.values.push(if (b == 0.0) {0.0} else {b / b.abs()});
        }
        return new_values;
    }
    pub fn nthroot(&self, _degree : EvaluatedValues) -> EvaluatedValues {
        panic!("Unimplemented.");
    }
    pub fn sin(&self) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            new_values.values.push(self.values[a].sin());
        }
        return new_values;
    }
    pub fn cos(&self) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            new_values.values.push(self.values[a].cos());
        }
        return new_values;
    }
    pub fn tan(&self) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::new();
        for a in 0..self.values.len() {
            new_values.values.push(self.values[a].tan());
        }
        return new_values;
    }
    pub fn cot(&self) -> EvaluatedValues {
        panic!("Unimplemented.");
    }
    pub fn sec(&self) -> EvaluatedValues {
        panic!("Unimplemented.");
    }
    pub fn csc(&self) -> EvaluatedValues {
        panic!("Unimplemented.");
    }

}
