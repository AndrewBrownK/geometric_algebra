implementing rga3d;
using traits;
public extension Flector {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Flector rhs) {
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
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Line rhs) {
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
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Motor rhs) {
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
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
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
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Point rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension Point: FlatBulkNorm {
    public typedef Scalar FlatBulkNormOutput;
    public func flat_bulk_norm() -> Scalar {
        return this.flat_bulk_norm_squared().square_root();
    }
}
