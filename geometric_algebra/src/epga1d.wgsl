struct Scalar {
    // 1
     g0: f32,
}

struct ComplexNumber {
    // 1, e01
     g0: vec2<f32>,
}

fn scalar_zero() -> Scalar  {
    return Scalar(0.0);
}

fn scalar_one() -> Scalar  {
    return Scalar(1.0);
}

fn scalar_neg(self_: Scalar) -> Scalar  {
    return Scalar(self_.g0 * -1.0);
}

fn scalar_automorphism(self_: Scalar) -> Scalar  {
    return Scalar(self_.g0);
}

fn scalar_reversal(self_: Scalar) -> Scalar  {
    return Scalar(self_.g0);
}

fn scalar_conjugation(self_: Scalar) -> Scalar  {
    return Scalar(self_.g0);
}

fn scalar_scalar_add(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 + other.g0);
}

fn scalar_scalar_sub(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 - other.g0);
}

fn scalar_scalar_mul(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_div(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * 1.0 / other.g0 * 1.0);
}

fn scalar_scalar_geometric_product(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_outer_product(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_inner_product(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_left_contraction(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_right_contraction(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_scalar_product(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_complex_number_add(self_: Scalar, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) + other.g0);
}

fn scalar_complex_number_sub(self_: Scalar, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) - other.g0);
}

fn scalar_complex_number_geometric_product(self_: Scalar, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_complex_number_regressive_product(self_: Scalar, other: ComplexNumber) -> Scalar  {
    return Scalar(self_.g0 * other.g0.y);
}

fn scalar_complex_number_outer_product(self_: Scalar, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_complex_number_inner_product(self_: Scalar, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_complex_number_left_contraction(self_: Scalar, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_complex_number_right_contraction(self_: Scalar, other: ComplexNumber) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_complex_number_scalar_product(self_: Scalar, other: ComplexNumber) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_squared_magnitude(self_: Scalar) -> Scalar  {
    return scalar_scalar_scalar_product(self_, scalar_reversal(self_));
}

fn scalar_magnitude(self_: Scalar) -> Scalar  {
    return Scalar(sqrt(scalar_squared_magnitude(self_).g0));
}

fn scalar_scale(self_: Scalar, other: f32) -> Scalar  {
    return scalar_scalar_geometric_product(self_, Scalar(other));
}

fn scalar_signum(self_: Scalar) -> Scalar  {
    return scalar_scalar_geometric_product(self_, Scalar(1.0 / scalar_magnitude(self_).g0));
}

fn scalar_inverse(self_: Scalar) -> Scalar  {
    return scalar_scalar_geometric_product(scalar_reversal(self_), Scalar(1.0 / scalar_squared_magnitude(self_).g0));
}

fn complex_number_zero() -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(0.0));
}

fn complex_number_one() -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(1.0, 0.0));
}

fn complex_number_neg(self_: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(self_.g0 * vec2<f32>(-1.0));
}

fn complex_number_automorphism(self_: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(self_.g0);
}

fn complex_number_reversal(self_: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(self_.g0 * vec2<f32>(1.0, -1.0));
}

fn complex_number_conjugation(self_: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(self_.g0 * vec2<f32>(1.0, -1.0));
}

fn complex_number_dual(self_: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(self_.g0.yx);
}

fn complex_number_scalar_into(self_: ComplexNumber) -> Scalar  {
    return Scalar(self_.g0.x);
}

fn complex_number_scalar_add(self_: ComplexNumber, other: Scalar) -> ComplexNumber  {
    return ComplexNumber(self_.g0 + vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn complex_number_scalar_sub(self_: ComplexNumber, other: Scalar) -> ComplexNumber  {
    return ComplexNumber(self_.g0 - vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn complex_number_scalar_geometric_product(self_: ComplexNumber, other: Scalar) -> ComplexNumber  {
    return ComplexNumber(self_.g0 * vec2<f32>(other.g0));
}

fn complex_number_scalar_regressive_product(self_: ComplexNumber, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.y * other.g0);
}

fn complex_number_scalar_outer_product(self_: ComplexNumber, other: Scalar) -> ComplexNumber  {
    return ComplexNumber(self_.g0 * vec2<f32>(other.g0));
}

fn complex_number_scalar_inner_product(self_: ComplexNumber, other: Scalar) -> ComplexNumber  {
    return ComplexNumber(self_.g0 * vec2<f32>(other.g0));
}

fn complex_number_scalar_left_contraction(self_: ComplexNumber, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn complex_number_scalar_right_contraction(self_: ComplexNumber, other: Scalar) -> ComplexNumber  {
    return ComplexNumber(self_.g0 * vec2<f32>(other.g0));
}

fn complex_number_scalar_scalar_product(self_: ComplexNumber, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn complex_number_complex_number_add(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(self_.g0 + other.g0);
}

fn complex_number_complex_number_sub(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(self_.g0 - other.g0);
}

fn complex_number_complex_number_mul(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(self_.g0 * other.g0);
}

fn complex_number_complex_number_div(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0.x, self_.g0.y) * vec2<f32>(1.0, 1.0) / vec2<f32>(other.g0.x, other.g0.y) * vec2<f32>(1.0, 1.0));
}

fn complex_number_complex_number_geometric_product(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0.x) * other.g0 + vec2<f32>(self_.g0.y) * other.g0.yx * vec2<f32>(-1.0, 1.0));
}

fn complex_number_complex_number_regressive_product(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0.y) * other.g0 + vec2<f32>(self_.g0.x) * other.g0.yx * vec2<f32>(1.0, 0.0));
}

fn complex_number_complex_number_outer_product(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0.x) * other.g0 + self_.g0 * vec2<f32>(other.g0.x) * vec2<f32>(0.0, 1.0));
}

fn complex_number_complex_number_inner_product(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0.x) * other.g0 + vec2<f32>(self_.g0.y) * other.g0.yx * vec2<f32>(-1.0, 1.0));
}

fn complex_number_complex_number_left_contraction(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0.x) * other.g0 + self_.g0.yx * other.g0.yx * vec2<f32>(-1.0, 0.0));
}

fn complex_number_complex_number_right_contraction(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return ComplexNumber(vec2<f32>(self_.g0.y) * other.g0.yx * vec2<f32>(-1.0, 1.0) + vec2<f32>(self_.g0.x) * vec2<f32>(other.g0.x) * vec2<f32>(1.0, 0.0));
}

fn complex_number_complex_number_scalar_product(self_: ComplexNumber, other: ComplexNumber) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y);
}

fn complex_number_squared_magnitude(self_: ComplexNumber) -> Scalar  {
    return complex_number_complex_number_scalar_product(self_, complex_number_reversal(self_));
}

fn complex_number_magnitude(self_: ComplexNumber) -> Scalar  {
    return Scalar(sqrt(complex_number_squared_magnitude(self_).g0));
}

fn complex_number_scale(self_: ComplexNumber, other: f32) -> ComplexNumber  {
    return complex_number_scalar_geometric_product(self_, Scalar(other));
}

fn complex_number_signum(self_: ComplexNumber) -> ComplexNumber  {
    return complex_number_scalar_geometric_product(self_, Scalar(1.0 / complex_number_magnitude(self_).g0));
}

fn complex_number_inverse(self_: ComplexNumber) -> ComplexNumber  {
    return complex_number_scalar_geometric_product(complex_number_reversal(self_), Scalar(1.0 / complex_number_squared_magnitude(self_).g0));
}

fn complex_number_powi(self_: ComplexNumber, exponent: i32) -> ComplexNumber  {
    if (exponent == 0) {
        return complex_number_one();
    }
    let x: ComplexNumber = select(self_, complex_number_inverse(self_), exponent < 0);
    let y: ComplexNumber = complex_number_one();
    let n: i32 = abs(exponent);
    while (1 < n) {
        if ((n & 1) == 1) {
            let y = complex_number_complex_number_geometric_product(x, y);
        }
        let x = complex_number_complex_number_geometric_product(x, x);
        let n = n >> 1;
    }
    return complex_number_complex_number_geometric_product(x, y);
}

fn complex_number_complex_number_geometric_quotient(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return complex_number_complex_number_geometric_product(self_, complex_number_inverse(other));
}

fn complex_number_complex_number_transformation(self_: ComplexNumber, other: ComplexNumber) -> ComplexNumber  {
    return complex_number_complex_number_geometric_product(complex_number_complex_number_geometric_product(self_, other), complex_number_reversal(self_));
}

fn complex_number_scalar_geometric_quotient(self_: ComplexNumber, other: Scalar) -> ComplexNumber  {
    return complex_number_scalar_geometric_product(self_, scalar_inverse(other));
}

fn complex_number_scalar_transformation(self_: ComplexNumber, other: Scalar) -> Scalar  {
    return complex_number_scalar_into(complex_number_complex_number_geometric_product(complex_number_scalar_geometric_product(self_, other), complex_number_reversal(self_)));
}

fn scalar_complex_number_geometric_quotient(self_: Scalar, other: ComplexNumber) -> ComplexNumber  {
    return scalar_complex_number_geometric_product(self_, complex_number_inverse(other));
}

fn scalar_complex_number_transformation(self_: Scalar, other: ComplexNumber) -> ComplexNumber  {
    return complex_number_scalar_geometric_product(scalar_complex_number_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_powi(self_: Scalar, exponent: i32) -> Scalar  {
    if (exponent == 0) {
        return scalar_one();
    }
    let x: Scalar = select(self_, scalar_inverse(self_), exponent < 0);
    let y: Scalar = scalar_one();
    let n: i32 = abs(exponent);
    while (1 < n) {
        if ((n & 1) == 1) {
            let y = scalar_scalar_geometric_product(x, y);
        }
        let x = scalar_scalar_geometric_product(x, x);
        let n = n >> 1;
    }
    return scalar_scalar_geometric_product(x, y);
}

fn scalar_scalar_geometric_quotient(self_: Scalar, other: Scalar) -> Scalar  {
    return scalar_scalar_geometric_product(self_, scalar_inverse(other));
}

fn scalar_scalar_transformation(self_: Scalar, other: Scalar) -> Scalar  {
    return scalar_scalar_geometric_product(scalar_scalar_geometric_product(self_, other), scalar_reversal(self_));
}

