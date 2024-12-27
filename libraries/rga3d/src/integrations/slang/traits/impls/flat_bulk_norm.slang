using traits::FlatBulkNormSquared;
using traits::SquareRoot;
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Flector: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Line: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Motor: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension MultiVector: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Point: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
