# Derivatives


An expression-based calculator capable of dealing with derivatives and simplifying the results with a varying degree of success.

## Example

Input: $\frac{d}{dx} \dfrac{x^3 * x^3 - (2x - x)}{x^2}$

Simplification: $\frac{d}{dx} \dfrac{x^9 - 3x}{x^2}$

Derivative: $\dfrac{x^2(6x^5 - 3) - 2x(x^6 - 3x)}{x^4}$

Solution at x = 2 $32.75$

## Features

Some of these features are left out because they're tedious to encode since they involve deep pattern matching on commutative operations.

Some other identities are tricky since they don't necessarily always make things simpler. For example: $ln(xy) = ln(x) + ln(y)$

### Derivatives
- [x] Sum Rule
- [x] Difference Rule
- [x] Product Rule
- [x] Chain Rule
- [x] Power Rule
- [x] Quotient Rule
- [x] Logarithms
- [ ] `e`
- [ ] Regular trig rules (sin, cos, tan)
- [ ] Confusing trig rules (cot, sin^-1, cos^-1, tan^-1)
- [ ] The ones that made me fail calc1 class (sinh, cosh, tanh)
- [ ] Square roots

### Simplification & Algebra
- [x] Constant operations for addition, subtraction, division, multiplication
- [x] Identity operations, eg: `n * 1 = n` or `n + 0 = n`
- [x] Exponent Rules, eg: `a^m + a^n = a^(m + n)`
- [x] Variable substitution
- [ ] Combining same terms at arbitrary levels `2x^2 + 5x^2`
- [ ] Distributing terms
- [ ] A bunch of other things

## Encoding an Equation
```rs
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

let derivative = Derivative::of(&operation.simplify())
// (((x^2 * (6x^5 - 3)) - ((x^6 - 3x) * 2x))/x^4)

let terms = HashMap::from(
  [(&x, Math::Constant(2.0))]
);

let solution = derivative.solve(&terms);
// Math::Constant(32.75)
```

```
input = "(((x^3 * x^3) - (2x + x))/x^2)"
input (simplified) = "((x^6 - 3x)/x^2)"
derivative (simplified) = "(((x^2 * (6x^5 - 3)) - ((x^6 - 3x) * 2x))/x^4)"
derivative solution (simplified) {x = 2} = "32.75"
```
