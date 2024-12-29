implementing rga3d;
using traits;
public extension Flector {
    // Fancy postfix trick
    public DualNum operator /(NormPrefixOrPostfix rhs)  {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(Flector rhs) {
        return rhs.norm();
    }
}
public extension Flector {
    public DualNum norm() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public DualNum operator /(NormPrefixOrPostfix rhs)  {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(Line rhs) {
        return rhs.norm();
    }
}
public extension Line {
    public DualNum norm() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public DualNum operator /(NormPrefixOrPostfix rhs)  {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(Motor rhs) {
        return rhs.norm();
    }
}
public extension Motor {
    public DualNum norm() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public DualNum operator /(NormPrefixOrPostfix rhs)  {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(MultiVector rhs) {
        return rhs.norm();
    }
}
public extension MultiVector {
    public DualNum norm() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public DualNum operator /(NormPrefixOrPostfix rhs)  {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(Point rhs) {
        return rhs.norm();
    }
}
public extension Point {
    public DualNum norm() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
