using traits::FlatBulkNormSquared;
using traits::SquareRoot;
extension Flector {
    func div(_rhs: bulk_norm) -> 
Scalar {
        this.bulk_norm()
    }
}
extension Flector: BulkNorm {
    func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Line {
    func div(_rhs: bulk_norm) -> 
Scalar {
        this.bulk_norm()
    }
}
extension Line: BulkNorm {
    func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Motor {
    func div(_rhs: bulk_norm) -> 
Scalar {
        this.bulk_norm()
    }
}
extension Motor: BulkNorm {
    func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension MultiVector {
    func div(_rhs: bulk_norm) -> 
Scalar {
        this.bulk_norm()
    }
}
extension MultiVector: BulkNorm {
    func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Point {
    func div(_rhs: bulk_norm) -> 
Scalar {
        this.bulk_norm()
    }
}
extension Point: BulkNorm {
    func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
