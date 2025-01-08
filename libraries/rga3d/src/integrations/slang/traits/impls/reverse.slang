implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.reverse();
    }
}
public extension AntiScalar {
    public AntiScalar reverse() {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.reverse();
    }
}
public extension DualNum {
    public DualNum reverse() {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.reverse();
    }
}
public extension Flector {
    public Flector reverse() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Horizon operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.reverse();
    }
}
public extension Horizon {
    public Horizon reverse() {
        return Horizon(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.reverse();
    }
}
public extension Line {
    public Line reverse() {
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
    public Motor operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.reverse();
    }
}
public extension Motor {
    public Motor reverse() {
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
    public MultiVector operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.reverse();
    }
}
public extension MultiVector {
    public MultiVector reverse() {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.reverse();
    }
}
public extension Origin {
    public Origin reverse() {
        return this;
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.reverse();
    }
}
public extension Plane {
    public Plane reverse() {
        return Plane(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.reverse();
    }
}
public extension Point {
    public Point reverse() {
        return this;
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.reverse();
    }
}
public extension Scalar {
    public Scalar reverse() {
        return this;
    }
}
