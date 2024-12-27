using traits::FlatBulkNormSquared;
using traits::FlatWeightNormSquared;
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
extension Flector: NormSquared {
    associatedtype Output = DualNum;
    func norm_squared() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
extension Line: NormSquared {
    associatedtype Output = DualNum;
    func norm_squared() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
extension Motor: NormSquared {
    associatedtype Output = DualNum;
    func norm_squared() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
extension MultiVector: NormSquared {
    associatedtype Output = DualNum;
    func norm_squared() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
extension Point: NormSquared {
    associatedtype Output = DualNum;
    func norm_squared() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
