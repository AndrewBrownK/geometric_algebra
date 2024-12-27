using traits::FlatBulkNormSquared;
using traits::SquareRoot;
extension Flector: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: FlatBulkNormInfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Line: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: FlatBulkNormInfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Motor: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: FlatBulkNormInfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension MultiVector: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: FlatBulkNormInfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Point: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: FlatBulkNormInfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
