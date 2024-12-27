using traits::FlatBulkNormSquared;
using traits::SquareRoot;
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
extension Flector: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
extension Line: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
extension Motor: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
extension MultiVector: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
extension Point: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
