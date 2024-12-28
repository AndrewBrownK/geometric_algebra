using traits;
using traits;
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.bulk_norm();
    }
}
public extension Flector: BulkNorm {
    public typedef Scalar BulkNormOutput;
    public func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.bulk_norm();
    }
}
public extension Line: BulkNorm {
    public typedef Scalar BulkNormOutput;
    public func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.bulk_norm();
    }
}
public extension Motor: BulkNorm {
    public typedef Scalar BulkNormOutput;
    public func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.bulk_norm();
    }
}
public extension MultiVector: BulkNorm {
    public typedef Scalar BulkNormOutput;
    public func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: BulkNormPrefixOrPostfix) -> Scalar {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.bulk_norm();
    }
}
public extension Point: BulkNorm {
    public typedef Scalar BulkNormOutput;
    public func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
