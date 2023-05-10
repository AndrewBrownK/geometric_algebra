struct Scalar {
    // 1
     g0: f32,
}

struct SplitComplexNumber {
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

fn scalar_split_complex_number_add(self_: Scalar, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) + other.g0);
}

fn scalar_split_complex_number_sub(self_: Scalar, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) - other.g0);
}

fn scalar_split_complex_number_geometric_product(self_: Scalar, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_split_complex_number_regressive_product(self_: Scalar, other: SplitComplexNumber) -> Scalar  {
    return Scalar(self_.g0 * other.g0.y);
}

fn scalar_split_complex_number_outer_product(self_: Scalar, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_split_complex_number_inner_product(self_: Scalar, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_split_complex_number_left_contraction(self_: Scalar, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_split_complex_number_right_contraction(self_: Scalar, other: SplitComplexNumber) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_split_complex_number_scalar_product(self_: Scalar, other: SplitComplexNumber) -> Scalar  {
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

fn split_complex_number_zero() -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(0.0));
}

fn split_complex_number_one() -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(1.0, 0.0));
}

fn split_complex_number_neg(self_: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 * vec2<f32>(-1.0));
}

fn split_complex_number_automorphism(self_: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0);
}

fn split_complex_number_reversal(self_: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 * vec2<f32>(1.0, -1.0));
}

fn split_complex_number_conjugation(self_: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 * vec2<f32>(1.0, -1.0));
}

fn split_complex_number_dual(self_: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0.yx);
}

fn split_complex_number_scalar_into(self_: SplitComplexNumber) -> Scalar  {
    return Scalar(self_.g0.x);
}

fn split_complex_number_scalar_add(self_: SplitComplexNumber, other: Scalar) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 + vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn split_complex_number_scalar_sub(self_: SplitComplexNumber, other: Scalar) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 - vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn split_complex_number_scalar_geometric_product(self_: SplitComplexNumber, other: Scalar) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 * vec2<f32>(other.g0));
}

fn split_complex_number_scalar_regressive_product(self_: SplitComplexNumber, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.y * other.g0);
}

fn split_complex_number_scalar_outer_product(self_: SplitComplexNumber, other: Scalar) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 * vec2<f32>(other.g0));
}

fn split_complex_number_scalar_inner_product(self_: SplitComplexNumber, other: Scalar) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 * vec2<f32>(other.g0));
}

fn split_complex_number_scalar_left_contraction(self_: SplitComplexNumber, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn split_complex_number_scalar_right_contraction(self_: SplitComplexNumber, other: Scalar) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 * vec2<f32>(other.g0));
}

fn split_complex_number_scalar_scalar_product(self_: SplitComplexNumber, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn split_complex_number_split_complex_number_add(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 + other.g0);
}

fn split_complex_number_split_complex_number_sub(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 - other.g0);
}

fn split_complex_number_split_complex_number_mul(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(self_.g0 * other.g0);
}

fn split_complex_number_split_complex_number_div(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0.x, self_.g0.y) * vec2<f32>(1.0, 1.0) / vec2<f32>(other.g0.x, other.g0.y) * vec2<f32>(1.0, 1.0));
}

fn split_complex_number_split_complex_number_geometric_product(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0.x) * other.g0 + vec2<f32>(self_.g0.y) * other.g0.yx);
}

fn split_complex_number_split_complex_number_regressive_product(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0.y) * other.g0 + vec2<f32>(self_.g0.x) * other.g0.yx * vec2<f32>(1.0, 0.0));
}

fn split_complex_number_split_complex_number_outer_product(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0.x) * other.g0 + self_.g0 * vec2<f32>(other.g0.x) * vec2<f32>(0.0, 1.0));
}

fn split_complex_number_split_complex_number_inner_product(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0.x) * other.g0 + vec2<f32>(self_.g0.y) * other.g0.yx);
}

fn split_complex_number_split_complex_number_left_contraction(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0.x) * other.g0 + self_.g0.yx * other.g0.yx * vec2<f32>(1.0, 0.0));
}

fn split_complex_number_split_complex_number_right_contraction(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return SplitComplexNumber(vec2<f32>(self_.g0.y) * other.g0.yx + vec2<f32>(self_.g0.x) * vec2<f32>(other.g0.x) * vec2<f32>(1.0, 0.0));
}

fn split_complex_number_split_complex_number_scalar_product(self_: SplitComplexNumber, other: SplitComplexNumber) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y);
}

fn split_complex_number_squared_magnitude(self_: SplitComplexNumber) -> Scalar  {
    return split_complex_number_split_complex_number_scalar_product(self_, split_complex_number_reversal(self_));
}

fn split_complex_number_magnitude(self_: SplitComplexNumber) -> Scalar  {
    return Scalar(sqrt(split_complex_number_squared_magnitude(self_).g0));
}

fn split_complex_number_scale(self_: SplitComplexNumber, other: f32) -> SplitComplexNumber  {
    return split_complex_number_scalar_geometric_product(self_, Scalar(other));
}

fn split_complex_number_signum(self_: SplitComplexNumber) -> SplitComplexNumber  {
    return split_complex_number_scalar_geometric_product(self_, Scalar(1.0 / split_complex_number_magnitude(self_).g0));
}

fn split_complex_number_inverse(self_: SplitComplexNumber) -> SplitComplexNumber  {
    return split_complex_number_scalar_geometric_product(split_complex_number_reversal(self_), Scalar(1.0 / split_complex_number_squared_magnitude(self_).g0));
}

fn scalar_powi(self_: Scalar, exponent: int) -> Scalar  {
    if (exponent == 0) {
        return scalar_one();
    }
    let x: Scalar = select(self_, scalar_inverse(self_), exponent < 0);
    let y: Scalar = scalar_one();
    let n: int = abs(exponent);
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

fn scalar_split_complex_number_geometric_quotient(self_: Scalar, other: SplitComplexNumber) -> SplitComplexNumber  {
    return scalar_split_complex_number_geometric_product(self_, split_complex_number_inverse(other));
}

fn scalar_split_complex_number_transformation(self_: Scalar, other: SplitComplexNumber) -> SplitComplexNumber  {
    return split_complex_number_scalar_geometric_product(scalar_split_complex_number_geometric_product(self_, other), scalar_reversal(self_));
}

fn split_complex_number_scalar_geometric_quotient(self_: SplitComplexNumber, other: Scalar) -> SplitComplexNumber  {
    return split_complex_number_scalar_geometric_product(self_, scalar_inverse(other));
}

fn split_complex_number_scalar_transformation(self_: SplitComplexNumber, other: Scalar) -> Scalar  {
    return split_complex_number_scalar_into(split_complex_number_split_complex_number_geometric_product(split_complex_number_scalar_geometric_product(self_, other), split_complex_number_reversal(self_)));
}

fn split_complex_number_powi(self_: SplitComplexNumber, exponent: int) -> SplitComplexNumber  {
    if (exponent == 0) {
        return split_complex_number_one();
    }
    let x: SplitComplexNumber = select(self_, split_complex_number_inverse(self_), exponent < 0);
    let y: SplitComplexNumber = split_complex_number_one();
    let n: int = abs(exponent);
    while (1 < n) {
        if ((n & 1) == 1) {
            let y = split_complex_number_split_complex_number_geometric_product(x, y);
        }
        let x = split_complex_number_split_complex_number_geometric_product(x, x);
        let n = n >> 1;
    }
    return split_complex_number_split_complex_number_geometric_product(x, y);
}

fn split_complex_number_split_complex_number_geometric_quotient(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return split_complex_number_split_complex_number_geometric_product(self_, split_complex_number_inverse(other));
}

fn split_complex_number_split_complex_number_transformation(self_: SplitComplexNumber, other: SplitComplexNumber) -> SplitComplexNumber  {
    return split_complex_number_split_complex_number_geometric_product(split_complex_number_split_complex_number_geometric_product(self_, other), split_complex_number_reversal(self_));
}

