using traits::DotProduct;
using traits::RoundBulk;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension AntiCircleRotor: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiMotor round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension AntiDipoleInversion: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiFlector round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension AntiDualNum: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        Scalar round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension AntiFlatPoint: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiFlatPoint round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension AntiFlector: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiFlector round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension AntiLine: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiLine round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension AntiMotor: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiMotor round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension AntiPlane: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiPlane round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension Circle: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiFlatPoint round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension CircleRotor: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiFlatPoint round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension Dipole: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiLine round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension DipoleInversion: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiLine round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension MultiVector: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        MultiVector round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension RoundPoint: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiPlane round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
    func operator/=(const RoundBulkNormSquaredPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.round_bulk_norm_squared();
        return *this;
    }
}
extension Scalar: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        Scalar round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension VersorEven: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiFlector round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm_squared();
    }
}
extension VersorOdd: RoundBulkNormSquared {
    associatedtype Output = Scalar;
    func round_bulk_norm_squared() -> Output {
        AntiMotor round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
