use std::collections::HashMap;


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


pub enum NodeBase {
    AdditionOperation       (Box<Node>, Box<Node>), // Left, Right
    SubtractionOperation    (Box<Node>, Box<Node>), // Left, Right
    MultiplicationOperation (Box<Node>, Box<Node>), // Left, Right
    DivisionOperation       (Box<Node>, Box<Node>), // Top, Bottom
    PowerOperation          (Box<Node>, Box<Node>), // Base, Degree

    AbsFunction     (Box<Node>),
    SignFunction    (Box<Node>),
    NthRootFunction (Box<Node>, Box<Node>), // Degree (n), Powered

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

            NodeBase::MultiValue        (values)      => {
                let mut string = vec![];
                for i in 0..values.len() {
                    string.push((*values[i]).to_string());
                }
                format!("{{{}}}", string.join(", "))
            },
            NodeBase::Number            (value)       => value.to_string(),
            NodeBase::Variable          (name)        => name.clone()

        };
    }
    pub fn evaluate(&self, variables : &HashMap<String, EvaluatedValues>) -> EvaluatedValues {
        return match (self) {
            NodeBase::AdditionOperation       (left, right) => left.evaluate(variables).addition(right.evaluate(variables)),
            NodeBase::SubtractionOperation    (left, right) => left.evaluate(variables).subtraction(right.evaluate(variables)),
            NodeBase::MultiplicationOperation (left, right) => left.evaluate(variables).multiplication(right.evaluate(variables)),
            NodeBase::DivisionOperation       (left, right) => left.evaluate(variables).division(right.evaluate(variables)),
            NodeBase::PowerOperation          (left, right) => left.evaluate(variables).power(right.evaluate(variables)),

            NodeBase::AbsFunction     (arg)  => arg.evaluate(variables).abs(),
            NodeBase::SignFunction    (arg)  => arg.evaluate(variables).sign(),
            NodeBase::NthRootFunction (n, p) => p.evaluate(variables).nthroot(n.evaluate(variables)),

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
    }
}


#[derive(Clone)]
pub struct EvaluatedValues {
    values : Vec<f64>
}
impl EvaluatedValues {
    pub fn new() -> EvaluatedValues {
        return EvaluatedValues {values: vec![]};
    }
    pub fn copy(values : EvaluatedValues) -> EvaluatedValues {
        return EvaluatedValues { values: values.values.clone() };
    }

    pub fn add(self, values : EvaluatedValues) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::copy(self);
        for i in 0..values.values.len() {
            let value = values.values[i];
            if (! new_values.values.contains(&value)) {
                new_values.values.push(value);
            }
        }
        return new_values;
    }
    pub fn push(self, value : f64) -> EvaluatedValues {
        let mut new_values = EvaluatedValues::copy(self);
        if (! new_values.values.contains(&value)) {
            new_values.values.push(value);
        }
        return new_values;
    }

    pub fn addition(self, _values : EvaluatedValues) -> EvaluatedValues {
        panic!("Unimplemented");
    }
    pub fn subtraction(self, _values : EvaluatedValues) -> EvaluatedValues {
        panic!("Unimplemented");
    }
    pub fn multiplication(self, _values : EvaluatedValues) -> EvaluatedValues {
        panic!("Unimplemented");
    }
    pub fn division(self, _values : EvaluatedValues) -> EvaluatedValues {
        panic!("Unimplemented");
    }
    pub fn power(self, _values : EvaluatedValues) -> EvaluatedValues {
        panic!("Unimplemented");
    }

    pub fn abs(self) -> EvaluatedValues {
        panic!("Unimplemented")
    }
    pub fn sign(self) -> EvaluatedValues {
        panic!("Unimplemented")
    }
    pub fn nthroot(self, _degree : EvaluatedValues) -> EvaluatedValues {
        panic!("Unimplemented")
    }
}
