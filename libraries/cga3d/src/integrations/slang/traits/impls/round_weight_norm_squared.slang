using traits::AntiDotProduct;
using traits::RoundWeight;
using traits::Wedge;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension AntiCircleRotor: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        Line round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension AntiDipoleInversion: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        Flector round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension Circle: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        Plane round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension CircleRotor: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        Plane round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension Dipole: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        Line round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension DipoleInversion: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        Motor round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension MultiVector: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        MultiVector round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension RoundPoint: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        FlatPoint round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension Sphere: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        AntiScalar round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension VersorEven: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        Flector round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm_squared();
    }
}
extension VersorOdd: RoundWeightNormSquared {
    associatedtype Output = AntiScalar;
    func round_weight_norm_squared() -> Output {
        Motor round_weight_carrier = this.round_weight().wedge(DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
