use std::collections::HashMap;
use derivative::Derivative;
use self::symbol::Symbol;
use math::*;

mod derivative;
mod math;
mod symbol;
mod evaluator;


fn main() {
    let x_symbol = Symbol::of_character('x');
    let x = var(x_symbol.clone());

    let operation = div(
        sub(
            mult(
                pow(
                    x.clone(),
                    constant(3.0),
                ),
                pow(x.clone(), constant(3.0)),
            ),
            add(mult(constant(2.0), x.clone()), x.clone())
        ),
        pow(
            x.clone(),
            constant(2.0),
        ),
    );

    println!("input = {:?}", operation.to_string());

    let simple = operation.simplify();
    println!("input (simplified) = {:?}", &simple.to_string());

    let mappings = HashMap::from(
        [(&x_symbol, Math::Constant(2.0))]
    );

    let out = Derivative::of(&simple);
    println!("derivative (simplified) = {:?}", out.simplify().to_string());

    let der2solution = out.simplify().solve(&mappings).simplify();
    println!("derivative solution (simplified) {{{} = {}}} = {:?}", x_symbol.to_string(), 2.0, der2solution.to_string());
}
