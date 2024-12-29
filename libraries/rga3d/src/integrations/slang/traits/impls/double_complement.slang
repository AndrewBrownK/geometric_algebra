implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.double_complement();
    }
}
public extension AntiScalar {
    public AntiScalar double_complement() {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.double_complement();
    }
}
public extension DualNum {
    public DualNum double_complement() {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.double_complement();
    }
}
public extension Flector {
    public Flector double_complement() {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Horizon operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.double_complement();
    }
}
public extension Horizon {
    public Horizon double_complement() {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.double_complement();
    }
}
public extension Line {
    public Line double_complement() {
        return this;
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.double_complement();
    }
}
public extension Motor {
    public Motor double_complement() {
        return this;
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.double_complement();
    }
}
public extension MultiVector {
    public MultiVector double_complement() {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.double_complement();
    }
}
public extension Origin {
    public Origin double_complement() {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.double_complement();
    }
}
public extension Plane {
    public Plane double_complement() {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.double_complement();
    }
}
public extension Point {
    public Point double_complement() {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.double_complement();
    }
}
public extension Scalar {
    public Scalar double_complement() {
        return this;
    }
}
