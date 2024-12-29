implementing rga3d;
using traits;
public extension Flector {
    // Fancy postfix trick
    public DualNum operator /(NormSquaredPrefixOrPostfix rhs)  {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(Flector rhs) {
        return rhs.norm_squared();
    }
}
public extension Flector {
    public DualNum norm_squared() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public DualNum operator /(NormSquaredPrefixOrPostfix rhs)  {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(Line rhs) {
        return rhs.norm_squared();
    }
}
public extension Line {
    public DualNum norm_squared() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public DualNum operator /(NormSquaredPrefixOrPostfix rhs)  {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(Motor rhs) {
        return rhs.norm_squared();
    }
}
public extension Motor {
    public DualNum norm_squared() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public DualNum operator /(NormSquaredPrefixOrPostfix rhs)  {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(MultiVector rhs) {
        return rhs.norm_squared();
    }
}
public extension MultiVector {
    public DualNum norm_squared() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public DualNum operator /(NormSquaredPrefixOrPostfix rhs)  {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(Point rhs) {
        return rhs.norm_squared();
    }
}
public extension Point {
    public DualNum norm_squared() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
