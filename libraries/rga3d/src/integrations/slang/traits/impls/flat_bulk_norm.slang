using traits;
using traits;
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.flat_bulk_norm();
    }
}
public extension Flector: FlatBulkNorm {
    public typedef Scalar FlatBulkNormOutput;
    public func flat_bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.flat_bulk_norm();
    }
}
public extension Line: FlatBulkNorm {
    public typedef Scalar FlatBulkNormOutput;
    public func flat_bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.flat_bulk_norm();
    }
}
public extension Motor: FlatBulkNorm {
    public typedef Scalar FlatBulkNormOutput;
    public func flat_bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.flat_bulk_norm();
    }
}
public extension MultiVector: FlatBulkNorm {
    public typedef Scalar FlatBulkNormOutput;
    public func flat_bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.flat_bulk_norm();
    }
}
public extension Point: FlatBulkNorm {
    public typedef Scalar FlatBulkNormOutput;
    public func flat_bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
