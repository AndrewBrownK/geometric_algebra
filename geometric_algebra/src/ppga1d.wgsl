struct Scalar {
    // 1
     g0: f32,
}

struct DualNumber {
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

fn scalar_dual_number_add(self_: Scalar, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) + other.g0);
}

fn scalar_dual_number_sub(self_: Scalar, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) - other.g0);
}

fn scalar_dual_number_geometric_product(self_: Scalar, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_dual_number_regressive_product(self_: Scalar, other: DualNumber) -> Scalar  {
    return Scalar(self_.g0 * other.g0.y);
}

fn scalar_dual_number_outer_product(self_: Scalar, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_dual_number_inner_product(self_: Scalar, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_dual_number_left_contraction(self_: Scalar, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0) * other.g0);
}

fn scalar_dual_number_right_contraction(self_: Scalar, other: DualNumber) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_dual_number_scalar_product(self_: Scalar, other: DualNumber) -> Scalar  {
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

fn dual_number_zero() -> DualNumber  {
    return DualNumber(vec2<f32>(0.0));
}

fn dual_number_one() -> DualNumber  {
    return DualNumber(vec2<f32>(1.0, 0.0));
}

fn dual_number_neg(self_: DualNumber) -> DualNumber  {
    return DualNumber(self_.g0 * vec2<f32>(-1.0));
}

fn dual_number_automorphism(self_: DualNumber) -> DualNumber  {
    return DualNumber(self_.g0);
}

fn dual_number_reversal(self_: DualNumber) -> DualNumber  {
    return DualNumber(self_.g0 * vec2<f32>(1.0, -1.0));
}

fn dual_number_conjugation(self_: DualNumber) -> DualNumber  {
    return DualNumber(self_.g0 * vec2<f32>(1.0, -1.0));
}

fn dual_number_dual(self_: DualNumber) -> DualNumber  {
    return DualNumber(self_.g0.yx);
}

fn dual_number_scalar_into(self_: DualNumber) -> Scalar  {
    return Scalar(self_.g0.x);
}

fn dual_number_scalar_add(self_: DualNumber, other: Scalar) -> DualNumber  {
    return DualNumber(self_.g0 + vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn dual_number_scalar_sub(self_: DualNumber, other: Scalar) -> DualNumber  {
    return DualNumber(self_.g0 - vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn dual_number_scalar_geometric_product(self_: DualNumber, other: Scalar) -> DualNumber  {
    return DualNumber(self_.g0 * vec2<f32>(other.g0));
}

fn dual_number_scalar_regressive_product(self_: DualNumber, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.y * other.g0);
}

fn dual_number_scalar_outer_product(self_: DualNumber, other: Scalar) -> DualNumber  {
    return DualNumber(self_.g0 * vec2<f32>(other.g0));
}

fn dual_number_scalar_inner_product(self_: DualNumber, other: Scalar) -> DualNumber  {
    return DualNumber(self_.g0 * vec2<f32>(other.g0));
}

fn dual_number_scalar_left_contraction(self_: DualNumber, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn dual_number_scalar_right_contraction(self_: DualNumber, other: Scalar) -> DualNumber  {
    return DualNumber(self_.g0 * vec2<f32>(other.g0));
}

fn dual_number_scalar_scalar_product(self_: DualNumber, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn dual_number_dual_number_add(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(self_.g0 + other.g0);
}

fn dual_number_dual_number_sub(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(self_.g0 - other.g0);
}

fn dual_number_dual_number_mul(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(self_.g0 * other.g0);
}

fn dual_number_dual_number_div(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0.x, self_.g0.y) * vec2<f32>(1.0, 1.0) / vec2<f32>(other.g0.x, other.g0.y) * vec2<f32>(1.0, 1.0));
}

fn dual_number_dual_number_geometric_product(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0.x) * other.g0 + self_.g0 * vec2<f32>(other.g0.x) * vec2<f32>(0.0, 1.0));
}

fn dual_number_dual_number_regressive_product(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0.y) * other.g0 + vec2<f32>(self_.g0.x) * other.g0.yx * vec2<f32>(1.0, 0.0));
}

fn dual_number_dual_number_outer_product(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0.x) * other.g0 + self_.g0 * vec2<f32>(other.g0.x) * vec2<f32>(0.0, 1.0));
}

fn dual_number_dual_number_inner_product(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0.x) * other.g0 + self_.g0 * vec2<f32>(other.g0.x) * vec2<f32>(0.0, 1.0));
}

fn dual_number_dual_number_left_contraction(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(vec2<f32>(self_.g0.x) * other.g0);
}

fn dual_number_dual_number_right_contraction(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return DualNumber(self_.g0 * vec2<f32>(other.g0.x));
}

fn dual_number_dual_number_scalar_product(self_: DualNumber, other: DualNumber) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x);
}

fn dual_number_squared_magnitude(self_: DualNumber) -> Scalar  {
    return dual_number_dual_number_scalar_product(self_, dual_number_reversal(self_));
}

fn dual_number_magnitude(self_: DualNumber) -> Scalar  {
    return Scalar(sqrt(dual_number_squared_magnitude(self_).g0));
}

fn dual_number_scale(self_: DualNumber, other: f32) -> DualNumber  {
    return dual_number_scalar_geometric_product(self_, Scalar(other));
}

fn dual_number_signum(self_: DualNumber) -> DualNumber  {
    return dual_number_scalar_geometric_product(self_, Scalar(1.0 / dual_number_magnitude(self_).g0));
}

fn dual_number_inverse(self_: DualNumber) -> DualNumber  {
    return dual_number_scalar_geometric_product(dual_number_reversal(self_), Scalar(1.0 / dual_number_squared_magnitude(self_).g0));
}

fn dual_number_powi(self_: DualNumber, exponent: int) -> DualNumber  {
    if (exponent == 0) {
        return dual_number_one();
    }
    let x: DualNumber = select(self_, dual_number_inverse(self_), exponent < 0);
    let y: DualNumber = dual_number_one();
    let n: int = abs(exponent);
    while (1 < n) {
        if ((n & 1) == 1) {
            let y = dual_number_dual_number_geometric_product(x, y);
        }
        let x = dual_number_dual_number_geometric_product(x, x);
        let n = n >> 1;
    }
    return dual_number_dual_number_geometric_product(x, y);
}

fn dual_number_dual_number_geometric_quotient(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return dual_number_dual_number_geometric_product(self_, dual_number_inverse(other));
}

fn dual_number_dual_number_transformation(self_: DualNumber, other: DualNumber) -> DualNumber  {
    return dual_number_dual_number_geometric_product(dual_number_dual_number_geometric_product(self_, other), dual_number_reversal(self_));
}

fn dual_number_scalar_geometric_quotient(self_: DualNumber, other: Scalar) -> DualNumber  {
    return dual_number_scalar_geometric_product(self_, scalar_inverse(other));
}

fn dual_number_scalar_transformation(self_: DualNumber, other: Scalar) -> Scalar  {
    return dual_number_scalar_into(dual_number_dual_number_geometric_product(dual_number_scalar_geometric_product(self_, other), dual_number_reversal(self_)));
}

fn scalar_dual_number_geometric_quotient(self_: Scalar, other: DualNumber) -> DualNumber  {
    return scalar_dual_number_geometric_product(self_, dual_number_inverse(other));
}

fn scalar_dual_number_transformation(self_: Scalar, other: DualNumber) -> DualNumber  {
    return dual_number_scalar_geometric_product(scalar_dual_number_geometric_product(self_, other), scalar_reversal(self_));
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

