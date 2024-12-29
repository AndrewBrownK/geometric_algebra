implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public Scalar operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiScalar rhs) {
        return rhs.left_complement();
    }
}
public extension AntiScalar {
    public Scalar left_complement() {
        return Scalar.from_groups(
            /* scalar */
            this.e1234
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.left_complement();
    }
}
public extension DualNum {
    public DualNum left_complement() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            this.group0.xy.yx
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.left_complement();
    }
}
public extension Flector {
    public Flector left_complement() {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Origin operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Horizon rhs) {
        return rhs.left_complement();
    }
}
public extension Horizon {
    public Origin left_complement() {
        return Origin.from_groups(
            /* e4 */
            this.e321
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.left_complement();
    }
}
public extension Line {
    public Line left_complement() {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.left_complement();
    }
}
public extension Motor {
    public Motor left_complement() {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.left_complement();
    }
}
public extension MultiVector {
    public MultiVector left_complement() {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy.yx, 
            /* e1, e2, e3, e4 */
            this.group4, 
            /* e41, e42, e43 */
            this.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group2.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    public Horizon operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Origin rhs) {
        return rhs.left_complement();
    }
}
public extension Origin {
    public Horizon left_complement() {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Point operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Plane rhs) {
        return rhs.left_complement();
    }
}
public extension Plane {
    public Point left_complement() {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Plane operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Point rhs) {
        return rhs.left_complement();
    }
}
public extension Point {
    public Plane left_complement() {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public AntiScalar operator /(LeftComplementPrefixOrPostfix rhs)  {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Scalar rhs) {
        return rhs.left_complement();
    }
}
public extension Scalar {
    public AntiScalar left_complement() {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}
