implementing rga3d;
using traits;
// Fancy postfix trick
public DualNum operator /(Flector lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public DualNum operator /(NormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.norm();
}
public extension Flector {
    public DualNum norm() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Line lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public DualNum operator /(NormPrefixOrPostfix lhs, Line rhs) {
    return rhs.norm();
}
public extension Line {
    public DualNum norm() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Motor lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public DualNum operator /(NormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.norm();
}
public extension Motor {
    public DualNum norm() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(MultiVector lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public DualNum operator /(NormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.norm();
}
public extension MultiVector {
    public DualNum norm() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Point lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public DualNum operator /(NormPrefixOrPostfix lhs, Point rhs) {
    return rhs.norm();
}
public extension Point {
    public DualNum norm() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
