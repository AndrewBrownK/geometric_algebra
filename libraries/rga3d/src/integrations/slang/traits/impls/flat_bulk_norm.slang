using traits::FlatBulkNormSquared;
using traits::SquareRoot;
extension Flector {
    func div(_rhs: flat_bulk_norm) -> 
Scalar {
        this.flat_bulk_norm()
    }
}
extension Flector: FlatBulkNorm {
    func flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
extension Line {
    func div(_rhs: flat_bulk_norm) -> 
Scalar {
        this.flat_bulk_norm()
    }
}
extension Line: FlatBulkNorm {
    func flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
extension Motor {
    func div(_rhs: flat_bulk_norm) -> 
Scalar {
        this.flat_bulk_norm()
    }
}
extension Motor: FlatBulkNorm {
    func flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
extension MultiVector {
    func div(_rhs: flat_bulk_norm) -> 
Scalar {
        this.flat_bulk_norm()
    }
}
extension MultiVector: FlatBulkNorm {
    func flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
extension Point {
    func div(_rhs: flat_bulk_norm) -> 
Scalar {
        this.flat_bulk_norm()
    }
}
extension Point: FlatBulkNorm {
    func flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
