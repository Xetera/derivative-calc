use crate::math::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Derivative(pub Box<Math>);

impl ToString for Derivative {
    fn to_string(&self) -> String {
        format!("d({})", self.0.to_string())
    }
}

impl Derivative {
    pub fn of(math: &Math) -> Math {
        Self(Box::new(math.clone())).derive()
    }
    pub fn derive(&self) -> Math {
        match &*self.0 {
            Math::Constant(_) => Math::Constant(0.0),
            Math::Variable(_) => Math::Constant(1.0),
            Math::Power(expr, power) => {
                let new_power = Math::Subtract(power.clone(), Math::Constant(1.0).into());
                let new_expr = Derivative::of(&expr);
                let inner = Math::Power(expr.clone(), new_power.into());
                Math::Multiply(
                    power.clone(),
                    Math::Multiply(inner.clone().into(), new_expr.into()).into(),
                )
            }
            Math::Multiply(left, right) => Math::Add(
                Math::Multiply(Box::clone(left), Derivative::of(&right).into()).into(),
                Math::Multiply(Box::clone(right), Derivative::of(&left).into()).into(),
            ),
            Math::Divide(top, bottom) => {
                let left = mult(bottom.clone(), Derivative::of(&top).into());
                let right = mult(top.clone(), Derivative::of(&bottom).into());
                Math::Divide(
                    sub(left, right),
                    pow(bottom.clone(), Math::Constant(2.0).into()),
                )
            }
            ops @ (Math::Add(..) | Math::Subtract(..)) => ops.map(|part| Derivative::of(&part)),
            Math::Log(base, expr) => {
                Math::Divide(
                    constant(1.0),
                    mult(
                        expr.clone(),
                        ln(constant(*base as f32)),
                    ),
                )
            }
            Math::Ln(expr) => Math::Divide(constant(1.0), expr.clone())
        }
    }
}
