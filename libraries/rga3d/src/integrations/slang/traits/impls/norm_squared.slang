implementing rga3d;
using traits;
// Fancy postfix trick
public DualNum operator /(Flector lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public DualNum operator /(NormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.norm_squared();
}
public extension Flector {
    public DualNum norm_squared() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Line lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public DualNum operator /(NormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.norm_squared();
}
public extension Line {
    public DualNum norm_squared() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Motor lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public DualNum operator /(NormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.norm_squared();
}
public extension Motor {
    public DualNum norm_squared() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(MultiVector lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public DualNum operator /(NormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.norm_squared();
}
public extension MultiVector {
    public DualNum norm_squared() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Point lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public DualNum operator /(NormSquaredPrefixOrPostfix lhs, Point rhs) {
    return rhs.norm_squared();
}
public extension Point {
    public DualNum norm_squared() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
