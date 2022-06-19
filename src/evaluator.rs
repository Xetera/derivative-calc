use crate::math::*;
use crate::Symbol;

pub fn evaluate(math: &Math) -> Math {
    match math {
        Math::Subtract(left, right) => match (&**left, &**right) {
            (Math::Constant(0.0), any) | (any, Math::Constant(0.0)) => any.clone(),
            (Math::Constant(a), Math::Constant(b)) => Math::Constant(a - b),
            (Math::Multiply(x1, y1), Math::Multiply(x2, y2)) => {
                match (&**x1, &**y1, &**x2, &**y2) {
                    (Math::Constant(c1), var1, var2, Math::Constant(c2))
                    | (Math::Constant(c1), var1, Math::Constant(c2), var2)
                    | (var1, Math::Constant(c1), var2, Math::Constant(c2))
                    | (var1, Math::Constant(c1), Math::Constant(c2), var2) => {
                        if var1 == var2 {
                            Math::Multiply(constant(c1 - c2), var1.clone().into())
                                .simplify()
                        } else {
                            math.clone()
                        }
                    }
                    _ => math.clone(),
                }
            }
            _ => math.clone(),
        },
        Math::Add(left, right) => match (&**left, &**right) {
            (Math::Constant(0.0), any) | (any, Math::Constant(0.0)) => any.clone(),
            (Math::Constant(a), Math::Constant(b)) => Math::Constant(a + b),
            (var @ Math::Variable(a), Math::Variable(b)) => {
                if a == b {
                    Math::Multiply(Math::Constant(2.0).into(), var.clone().into()).simplify()
                } else {
                    math.clone()
                }
            }
            (Math::Multiply(left, right), Math::Variable(var))
                | (Math::Variable(var), Math::Multiply(left, right)) => {
                match (&**left, &**right) {
                    (Math::Constant(c), v @ Math::Variable(var2))
                        | (v @ Math::Variable(var2), Math::Constant(c)) => {
                        if var == var2 {
                            Math::Multiply(constant(c + 1.0), v.clone().into())
                        } else {
                            math.clone()
                        }
                    }
                    _ => math.clone()
                }
            }
            (Math::Multiply(x1, y1), Math::Multiply(x2, y2)) => {
                match (&**x1, &**y1, &**x2, &**y2) {
                    (Math::Constant(c1), var1, var2, Math::Constant(c2))
                    | (Math::Constant(c1), var1, Math::Constant(c2), var2)
                    | (var1, Math::Constant(c1), var2, Math::Constant(c2))
                    | (var1, Math::Constant(c1), Math::Constant(c2), var2) => {
                        if var1 == var2 {
                            mult(Math::Constant(c1 + c2).into(), var1.clone().into()).simplify()
                        } else {
                            math.clone()
                        }
                    }
                    _ => math.clone(),
                }
            }
            _ => math.clone(),
        },
        own @ Math::Multiply(left, right) => match (&**left, &**right) {
            (Math::Constant(a), Math::Constant(b)) => Math::Constant(a * b),
            // a^m * a^n = a^(m + n)
            (Math::Power(expr1, pow1), Math::Power(expr2, pow2)) => {
                if expr1 == expr2 {
                    Math::Power(expr1.clone(), Math::Add(pow1.clone(), pow2.clone()).simplify().into())
                } else {
                    own.clone()
                }
            }
            (Math::Multiply(x1, y1), Math::Multiply(x2, y2)) => {
                match (&**x1, &**y1, &**x2, &**y2) {
                    (Math::Constant(c1), var1, var2, Math::Constant(c2))
                    | (Math::Constant(c1), var1, Math::Constant(c2), var2)
                    | (var1, Math::Constant(c1), var2, Math::Constant(c2))
                    | (var1, Math::Constant(c1), Math::Constant(c2), var2) => {
                        if var1 == var2 {
                            mult(constant(c1 * c2), var1.clone().into()).simplify()
                        } else {
                            math.clone()
                        }
                    }
                    _ => math.clone(),
                }
            }
            (Math::Power(input, pow1), Math::Variable(var2))
            | (Math::Variable(var2), Math::Power(input, pow1)) => {
                multiply_powers(var2, input, &*pow1)
            }
            (var @ Math::Variable(a), Math::Variable(b)) => {
                if a == b {
                    Math::Power(Box::new(var.clone()), Math::Constant(2.0).into())
                } else {
                    math.clone()
                }
            }
            (Math::Constant(c1), Math::Multiply(x1, x2))
            | (Math::Multiply(x1, x2), Math::Constant(c1)) => match (&**x1, &**x2) {
                (Math::Constant(c2), var) | (var, Math::Constant(c2)) => {
                    Math::Multiply(Math::Constant(c1 * c2).into(), var.clone().into()).simplify()
                }
                _ => math.clone(),
            },
            (_, Math::Constant(0.0)) | (Math::Constant(0.0), _) => Math::Constant(0.0),
            (any, Math::Constant(1.0)) | (Math::Constant(1.0), any) => any.clone(),
            _ => math.clone(),
        },
        Math::Divide(top, bottom) => {
            match (&**top, &**bottom) {
                (top, Math::Constant(1.0)) => top.clone(),
                (Math::Constant(a), Math::Constant(b)) => Math::Constant(a / *b),
                // a^m / a^n = a^(m - n)
                (Math::Power(base1, pow1), Math::Power(base2, pow2)) => {
                    if base1 == base2 {
                        Math::Power(base1.clone(), Math::Subtract(pow1.clone(), pow2.clone()).simplify().into())
                    } else {
                        math.clone()
                    }
                }
                (top, bottom) => {
                    if top == bottom {
                        Math::Constant(1.0)
                    } else {
                        math.clone()
                    }
                }
            }
        }
        own @ Math::Power(expr, power) => {
            match (&**expr, &**power) {
                (any, Math::Constant(1.0)) => any.clone(),
                (_, Math::Constant(0.0)) => Math::Constant(1.0),
                (Math::Constant(a), Math::Constant(b)) => Math::Constant(a.powf(*b)),
                (Math::Power(base, first_power), second_power) => {
                    Math::Power(base.clone().into(),
                                mult(first_power.clone(), second_power.clone().into()).simplify().into(),
                    )
                }
                _ => own.clone()
            }
        }
        id => id.clone(),
    }
}

fn multiply_powers(variable: &Symbol, power_expr: &Math, pow1: &Math) -> Math {
    match power_expr {
        Math::Variable(var1) => {
            if *var1 == *variable {
                Math::Power(
                    power_expr.clone().into(),
                    add(pow1.clone().into(), constant(1.0)),
                )
            } else {
                power_expr.clone()
            }
        }
        id => id.clone(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplifies_powers() {
        let x_symbol = Symbol::of_character('x');
        let x = var(x_symbol.clone());

        let operation = pow(
            pow(
                x.clone(),
                constant(2.0),
            ),
            constant(3.0),
        ).simplify();

        let expected = pow(
            x.clone(),
            constant(6.0),
        );
        assert_eq!(operation, *expected);
    }
}
