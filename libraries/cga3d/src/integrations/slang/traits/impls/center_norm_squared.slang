using traits::FlatWeightNormSquared;
using traits::RightAntiDual;
using traits::RoundBulkNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: CenterNormSquaredPrefixOrPostfix) -> Scalar {
        return this.center_norm_squared();
    }
}
extension AntiCircleRotor: CenterNormSquared {
    associatedtype Output = Scalar;
    func center_norm_squared() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: CenterNormSquaredPrefixOrPostfix) -> Scalar {
        return this.center_norm_squared();
    }
}
extension AntiDipoleInversion: CenterNormSquared {
    associatedtype Output = Scalar;
    func center_norm_squared() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: CenterNormSquaredPrefixOrPostfix) -> Scalar {
        return this.center_norm_squared();
    }
}
extension Circle: CenterNormSquared {
    associatedtype Output = Scalar;
    func center_norm_squared() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: CenterNormSquaredPrefixOrPostfix) -> Scalar {
        return this.center_norm_squared();
    }
}
extension CircleRotor: CenterNormSquared {
    associatedtype Output = Scalar;
    func center_norm_squared() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: CenterNormSquaredPrefixOrPostfix) -> Scalar {
        return this.center_norm_squared();
    }
}
extension Dipole: CenterNormSquared {
    associatedtype Output = Scalar;
    func center_norm_squared() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: CenterNormSquaredPrefixOrPostfix) -> Scalar {
        return this.center_norm_squared();
    }
}
extension DipoleInversion: CenterNormSquared {
    associatedtype Output = Scalar;
    func center_norm_squared() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: CenterNormSquaredPrefixOrPostfix) -> Scalar {
        return this.center_norm_squared();
    }
}
extension MultiVector: CenterNormSquared {
    associatedtype Output = Scalar;
    func center_norm_squared() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: CenterNormSquaredPrefixOrPostfix) -> Scalar {
        return this.center_norm_squared();
    }
}
extension VersorEven: CenterNormSquared {
    associatedtype Output = Scalar;
    func center_norm_squared() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: CenterNormSquaredPrefixOrPostfix) -> Scalar {
        return this.center_norm_squared();
    }
}
extension VersorOdd: CenterNormSquared {
    associatedtype Output = Scalar;
    func center_norm_squared() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar, 0.0, 0.0, 0.0)
        );
    }
}
