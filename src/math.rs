use std::collections::HashMap;
use std::hash::Hash;
use crate::Derivative;
use crate::symbol::Symbol;
use crate::evaluator::*;
use std::f32::consts::E;

#[derive(Debug, Clone, PartialEq)]
pub enum Math {
    Constant(f32),
    Variable(Symbol),
    Power(Box<Math>, Box<Math>),
    Multiply(Box<Math>, Box<Math>),
    Add(Box<Math>, Box<Math>),
    Subtract(Box<Math>, Box<Math>),
    Divide(Box<Math>, Box<Math>),
    Log(u32, Box<Math>),
    Ln(Box<Math>),
}

type Replacer<'a> = HashMap<&'a Symbol, Math>;

pub fn constant(f: f32) -> Box<Math> {
    Math::Constant(f).into()
}

pub fn var(s: Symbol) -> Box<Math> {
    Math::Variable(s).into()
}

pub fn pow(left: Box<Math>, right: Box<Math>) -> Box<Math> {
    Math::Power(left, right).into()
}

pub fn mult(left: Box<Math>, right: Box<Math>) -> Box<Math> {
    Math::Multiply(left, right).into()
}

pub fn add(left: Box<Math>, right: Box<Math>) -> Box<Math> {
    Math::Add(left, right).into()
}

pub fn sub(left: Box<Math>, right: Box<Math>) -> Box<Math> {
    Math::Subtract(left, right).into()
}

pub fn div(left: Box<Math>, right: Box<Math>) -> Box<Math> {
    Math::Divide(left, right).into()
}

pub fn ln(expr: Box<Math>) -> Box<Math> {
    Math::Ln(expr).into()
}

pub fn log(base: u32, expr: Box<Math>) -> Box<Math> {
    Math::Log(base, expr).into()
}


impl Math {
    pub fn simplify(&self) -> Math {
        evaluate(&self.map(|math| math.simplify()))
    }

    /// Solves an expression by replacing variables with other expressions
    pub fn solve(&self, mappings: &Replacer) -> Math {
        self.map(|expr| {
            match expr {
                Math::Variable(variable) => {
                    mappings.get(variable).unwrap_or(expr).clone()
                }
                _ => expr.solve(&mappings)
            }
        })
    }

    /// Map an operation across all "functional parts" of an operation.
    /// Terminates itself for things like constants or variables.
    pub fn map<F>(&self, mut f: F) -> Self where F: FnMut(&Math) -> Self {
        match self {
            Math::Add(left, right) => Self::Add(f(&*left).into(), f(&*right).into()),
            Math::Subtract(left, right) => Self::Subtract(f(&*left).into(), f(&*right).into()),
            Math::Multiply(left, right) => Self::Multiply(f(&*left).into(), f(&*right).into()),
            Math::Divide(left, right) => Self::Divide(f(&*left).into(), f(&*right).into()),
            Math::Power(expr, power) => Self::Power(f(&*expr).into(), f(&*power).into()),
            Math::Ln(expr) => Self::Ln(f(&*expr).into()),
            _ => self.clone(),
        }
    }
}

impl ToString for Math {
    fn to_string(&self) -> String {
        match &self {
            Self::Constant(c) => c.to_string(),
            Self::Variable(symbol) => symbol.clone().to_string(),
            Self::Power(expr, power) => {
                if **power == Math::Constant(1.0) {
                    expr.to_string()
                } else if let Math::Constant(_) = **power {
                    format!("{}^{}", expr.to_string(), power.to_string())
                } else {
                    format!("{}^({})", expr.to_string(), power.to_string())
                }
            }
            Self::Subtract(left, right) => {
                format!("({} - {})", left.to_string(), right.to_string())
            }
            Self::Add(left, right) => format!("({} + {})", left.to_string(), right.to_string()),
            Self::Multiply(left, right) => match (&**left, &**right) {
                (Math::Constant(..), Math::Power(..) | Math::Variable(..)) => {
                    format!("{}{}", left.to_string(), right.to_string())
                }
                (Math::Power(..) | Math::Variable(..), Math::Constant(..)) => {
                    format!("{}{}", right.to_string(), left.to_string())
                }
                (left, right) => format!("({} * {})", left.to_string(), right.to_string()),
            },
            Self::Divide(left, right) => format!("({}/{})", left.to_string(), right.to_string()),
            Self::Log(base, expr) => {
                format!("log{}({})", base, expr.to_string())
            },
            Self::Ln( expr) => {
                format!("ln({})", expr.to_string())
            },
        }
    }
}

