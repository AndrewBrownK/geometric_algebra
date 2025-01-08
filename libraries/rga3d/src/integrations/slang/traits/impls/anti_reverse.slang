implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiScalar {
    public AntiScalar anti_reverse() {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.anti_reverse();
    }
}
public extension DualNum {
    public DualNum anti_reverse() {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.anti_reverse();
    }
}
public extension Flector {
    public Flector anti_reverse() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Horizon operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.anti_reverse();
    }
}
public extension Horizon {
    public Horizon anti_reverse() {
        return this;
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.anti_reverse();
    }
}
public extension Line {
    public Line anti_reverse() {
        return Line(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.anti_reverse();
    }
}
public extension Motor {
    public Motor anti_reverse() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.anti_reverse();
    }
}
public extension MultiVector {
    public MultiVector anti_reverse() {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.anti_reverse();
    }
}
public extension Origin {
    public Origin anti_reverse() {
        return Origin(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.anti_reverse();
    }
}
public extension Plane {
    public Plane anti_reverse() {
        return this;
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.anti_reverse();
    }
}
public extension Point {
    public Point anti_reverse() {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.anti_reverse();
    }
}
public extension Scalar {
    public Scalar anti_reverse() {
        return this;
    }
}
