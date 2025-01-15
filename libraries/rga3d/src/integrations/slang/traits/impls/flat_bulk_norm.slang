implementing rga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(Flector lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_bulk_norm();
}
public extension Flector {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_bulk_norm();
}
public extension Line {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_bulk_norm();
}
public extension Motor {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_bulk_norm();
}
public extension MultiVector {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Point lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Point rhs) {
    return rhs.flat_bulk_norm();
}
public extension Point {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
