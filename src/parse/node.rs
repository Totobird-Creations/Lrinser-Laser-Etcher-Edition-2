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
    
    AdditionOperation       (Box<Node>, Box<Node>), // Left, Right  : Left + Right
    SubtractionOperation    (Box<Node>, Box<Node>), // Left, Right  : Left - Right
    MultiplicationOperation (Box<Node>, Box<Node>), // Left, Right  : Left * Right
    DivisionOperation       (Box<Node>, Box<Node>), // Top, Bottom  : Top / Bottom
    PowerOperation          (Box<Node>, Box<Node>), // Base, Degree : Base.pow(Degree)

    AbsFunction     (Box<Node>),
    SignFunction    (Box<Node>),
    NthRootFunction (Box<Node>, Box<Node>), // Degree (n), Powered : ⁿ√(Powered)
    SinFunction     (Box<Node>),
    CosFunction     (Box<Node>),
    TanFunction     (Box<Node>),
    CotFunction     (Box<Node>),
    SecFunction     (Box<Node>),
    CscFunction     (Box<Node>),

    MultiValue (Vec<Box<Node>>),
    Number     (f64),
    Variable   (String),

    Equals (Box<Node>, Box<Right>), // Left, Right : Left = Right
    
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
            NodeBase::Variable          (name)        => name.clone(),

            NodeBase::Equals (left, right) => format!("({} = {})", (*left).to_string(), (*right).to_string())

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

            NodeBase::Equals (left, right) => {
                if (matches!(*left, NodeBase::Variable(name))) {
                    
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
    pub fn from(values : Vec<f64>) -> EvaluatedValues {
        return EvaluatedValues {values: values};
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
        return self.unary_operation(|a, new_values| {
            if (a >= settings.frame[1] && a < settings.frame[3]) {
                new_values.values.push(a);
            }
        });
    }

    pub fn addition(&self, other : EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(a + b));
    }
    pub fn subtraction(&self, other : EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(a - b));
    }
    pub fn multiplication(&self, other : EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(a * b));
    }
    pub fn division(&self, other : EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| {
            if (b != 0.0) {}
                new_values.values.push(a * b);
        });
    }
    pub fn power(&self, other : EvaluatedValues) -> EvaluatedValues {
        return self.binary_operation(other, |a, b, new_values| new_values.values.push(a.powf(b)));
    }

    pub fn abs(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.abs()));
    }
    pub fn sign(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| {
            new_values.values.push(if (a == 0.0) {0.0} else {a / a.abs()})
        });
    }
    pub fn nthroot(&self, _degree : EvaluatedValues) -> EvaluatedValues {
        panic!("Unimplemented.");
    }
    pub fn sin(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.sin()));
    }
    pub fn cos(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.cos()));
    }
    pub fn tan(&self) -> EvaluatedValues {
        return self.unary_operation(|a, new_values| new_values.values.push(a.tan()));
    }
    pub fn cot(&self) -> EvaluatedValues {
        return EvaluatedValues::from(vec![1.0]).division(self.tan());
    }
    pub fn sec(&self) -> EvaluatedValues {
        return EvaluatedValues::from(vec![1.0]).division(self.cos());
    }
    pub fn csc(&self) -> EvaluatedValues {
        return EvaluatedValues::from(vec![1.0]).division(self.sin());
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
    fn binary_operation<T>(&self, other : EvaluatedValues, target : T) -> EvaluatedValues
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
    
}
