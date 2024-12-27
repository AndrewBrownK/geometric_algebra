using traits::FlatBulkNormSquared;
using traits::FlatWeightNormSquared;
extension Flector {
    func div(_rhs: norm_squared) -> 
DualNum {
        this.norm_squared()
    }
}
extension Flector: NormSquared {
    func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
extension Line {
    func div(_rhs: norm_squared) -> 
DualNum {
        this.norm_squared()
    }
}
extension Line: NormSquared {
    func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
extension Motor {
    func div(_rhs: norm_squared) -> 
DualNum {
        this.norm_squared()
    }
}
extension Motor: NormSquared {
    func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
extension MultiVector {
    func div(_rhs: norm_squared) -> 
DualNum {
        this.norm_squared()
    }
}
extension MultiVector: NormSquared {
    func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
extension Point {
    func div(_rhs: norm_squared) -> 
DualNum {
        this.norm_squared()
    }
}
extension Point: NormSquared {
    func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
