implementing rga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(Flector lhs, BulkNormPrefixOrPostfix rhs)  {
    return lhs.bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.bulk_norm();
}
public extension Flector {
    public Scalar bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, BulkNormPrefixOrPostfix rhs)  {
    return lhs.bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.bulk_norm();
}
public extension Line {
    public Scalar bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, BulkNormPrefixOrPostfix rhs)  {
    return lhs.bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.bulk_norm();
}
public extension Motor {
    public Scalar bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, BulkNormPrefixOrPostfix rhs)  {
    return lhs.bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.bulk_norm();
}
public extension MultiVector {
    public Scalar bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Point lhs, BulkNormPrefixOrPostfix rhs)  {
    return lhs.bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormPrefixOrPostfix lhs, Point rhs) {
    return rhs.bulk_norm();
}
public extension Point {
    public Scalar bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
