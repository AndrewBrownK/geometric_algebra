implementing rga3d;
using traits;
public extension Flector {
    // Fancy postfix trick
    public Scalar operator /(BulkNormPrefixOrPostfix rhs)  {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Flector rhs) {
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
    public Scalar operator /(BulkNormPrefixOrPostfix rhs)  {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Line rhs) {
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
    public Scalar operator /(BulkNormPrefixOrPostfix rhs)  {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Motor rhs) {
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
    public Scalar operator /(BulkNormPrefixOrPostfix rhs)  {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
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
    public Scalar operator /(BulkNormPrefixOrPostfix rhs)  {
        return this.bulk_norm();
    }
}
public extension BulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Point rhs) {
        return rhs.bulk_norm();
    }
}
public extension Point: BulkNorm {
    public typedef Scalar BulkNormOutput;
    public func bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
