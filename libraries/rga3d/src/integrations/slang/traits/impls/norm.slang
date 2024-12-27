using traits::FlatBulkNorm;
using traits::FlatWeightNorm;
extension Flector: Norm {
    associatedtype Output = DualNum;
    func norm() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
    func operator/(_rhs: NormInfix) -> DualNum {
        return this.norm();
    }
}
extension Line: Norm {
    associatedtype Output = DualNum;
    func norm() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
    func operator/(_rhs: NormInfix) -> DualNum {
        return this.norm();
    }
}
extension Motor: Norm {
    associatedtype Output = DualNum;
    func norm() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
    func operator/(_rhs: NormInfix) -> DualNum {
        return this.norm();
    }
}
extension MultiVector: Norm {
    associatedtype Output = DualNum;
    func norm() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
    func operator/(_rhs: NormInfix) -> DualNum {
        return this.norm();
    }
}
extension Point: Norm {
    associatedtype Output = DualNum;
    func norm() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
    func operator/(_rhs: NormInfix) -> DualNum {
        return this.norm();
    }
}
