using traits::FlatBulkNorm;
using traits::FlatWeightNorm;
extension Flector {
    func div(_rhs: norm) -> 
DualNum {
        this.norm()
    }
}
extension Flector: Norm {
    func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
extension Line {
    func div(_rhs: norm) -> 
DualNum {
        this.norm()
    }
}
extension Line: Norm {
    func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
extension Motor {
    func div(_rhs: norm) -> 
DualNum {
        this.norm()
    }
}
extension Motor: Norm {
    func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
extension MultiVector {
    func div(_rhs: norm) -> 
DualNum {
        this.norm()
    }
}
extension MultiVector: Norm {
    func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
extension Point {
    func div(_rhs: norm) -> 
DualNum {
        this.norm()
    }
}
extension Point: Norm {
    func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
