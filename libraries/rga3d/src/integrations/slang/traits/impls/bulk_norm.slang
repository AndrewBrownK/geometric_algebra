using traits::FlatBulkNormSquared;
using traits::SquareRoot;
extension Flector: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: BulkNormInfix) -> Scalar {
        return this.bulk_norm();
    }
}
extension Line: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: BulkNormInfix) -> Scalar {
        return this.bulk_norm();
    }
}
extension Motor: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: BulkNormInfix) -> Scalar {
        return this.bulk_norm();
    }
}
extension MultiVector: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: BulkNormInfix) -> Scalar {
        return this.bulk_norm();
    }
}
extension Point: BulkNorm {
    associatedtype Output = Scalar;
    func bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
    func operator/(_rhs: BulkNormInfix) -> Scalar {
        return this.bulk_norm();
    }
}
