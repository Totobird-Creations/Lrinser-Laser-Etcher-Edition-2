use std::fmt;

use crate::parse::var;
use crate::render::settings::RenderSettings;


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
