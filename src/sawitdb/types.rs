use alloc::string::String;
use alloc::fmt;
use core::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
         match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            
            // Cross-type comparisons (matching Go implementation behavior)
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            
            // String vs Number -> Unordered (or define rule? Go code implies strict type check or return 0)
            // For now, let's treat different types as incomparable (None), or define an order.
            // Go implementation defaults to 0 if types don't match (except int/float mix).
            _ => None 
        }
    }
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
        }
    }
}
