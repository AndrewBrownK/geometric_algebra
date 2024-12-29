implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.reverse();
    }
}
public extension AntiScalar: Reverse {
    public typedef AntiScalar ReverseOutput;
    public func reverse() -> AntiScalar {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> DualNum& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.reverse();
    }
}
public extension DualNum: Reverse {
    public typedef DualNum ReverseOutput;
    public func reverse() -> DualNum {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> Flector& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.reverse();
    }
}
public extension Flector: Reverse {
    public typedef Flector ReverseOutput;
    public func reverse() -> Flector {
        return Flector.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> Horizon& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.reverse();
    }
}
public extension Horizon: Reverse {
    public typedef Horizon ReverseOutput;
    public func reverse() -> Horizon {
        return Horizon.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> Line& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.reverse();
    }
}
public extension Line: Reverse {
    public typedef Line ReverseOutput;
    public func reverse() -> Line {
        return Line.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> Motor& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.reverse();
    }
}
public extension Motor: Reverse {
    public typedef Motor ReverseOutput;
    public func reverse() -> Motor {
        return Motor.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> MultiVector& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.reverse();
    }
}
public extension MultiVector: Reverse {
    public typedef MultiVector ReverseOutput;
    public func reverse() -> MultiVector {
        return MultiVector.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> Origin& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.reverse();
    }
}
public extension Origin: Reverse {
    public typedef Origin ReverseOutput;
    public func reverse() -> Origin {
        return this;
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> Plane& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.reverse();
    }
}
public extension Plane: Reverse {
    public typedef Plane ReverseOutput;
    public func reverse() -> Plane {
        return Plane.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> Point& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.reverse();
    }
}
public extension Point: Reverse {
    public typedef Point ReverseOutput;
    public func reverse() -> Point {
        return this;
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(ReversePrefixOrPostfix rhs)  {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const ReversePrefixOrPostfix& rhs) -> Scalar& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.reverse();
    }
}
public extension Scalar: Reverse {
    public typedef Scalar ReverseOutput;
    public func reverse() -> Scalar {
        return this;
    }
}
