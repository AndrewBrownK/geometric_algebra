implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiScalar: AntiReverse {
    public typedef AntiScalar AntiReverseOutput;
    public func anti_reverse() -> AntiScalar {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> DualNum& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.anti_reverse();
    }
}
public extension DualNum: AntiReverse {
    public typedef DualNum AntiReverseOutput;
    public func anti_reverse() -> DualNum {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> Flector& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.anti_reverse();
    }
}
public extension Flector: AntiReverse {
    public typedef Flector AntiReverseOutput;
    public func anti_reverse() -> Flector {
        return Flector.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> Horizon& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.anti_reverse();
    }
}
public extension Horizon: AntiReverse {
    public typedef Horizon AntiReverseOutput;
    public func anti_reverse() -> Horizon {
        return this;
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> Line& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.anti_reverse();
    }
}
public extension Line: AntiReverse {
    public typedef Line AntiReverseOutput;
    public func anti_reverse() -> Line {
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
    public Motor operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> Motor& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.anti_reverse();
    }
}
public extension Motor: AntiReverse {
    public typedef Motor AntiReverseOutput;
    public func anti_reverse() -> Motor {
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
    public MultiVector operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> MultiVector& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.anti_reverse();
    }
}
public extension MultiVector: AntiReverse {
    public typedef MultiVector AntiReverseOutput;
    public func anti_reverse() -> MultiVector {
        return MultiVector.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> Origin& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.anti_reverse();
    }
}
public extension Origin: AntiReverse {
    public typedef Origin AntiReverseOutput;
    public func anti_reverse() -> Origin {
        return Origin.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> Plane& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.anti_reverse();
    }
}
public extension Plane: AntiReverse {
    public typedef Plane AntiReverseOutput;
    public func anti_reverse() -> Plane {
        return this;
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> Point& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.anti_reverse();
    }
}
public extension Point: AntiReverse {
    public typedef Point AntiReverseOutput;
    public func anti_reverse() -> Point {
        return Point.from_groups(
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
    // Fancy postfix self-assign
    public func operator /=(const AntiReversePrefixOrPostfix& rhs) -> Scalar& {
        this = this.anti_reverse();
        return *this;
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.anti_reverse();
    }
}
public extension Scalar: AntiReverse {
    public typedef Scalar AntiReverseOutput;
    public func anti_reverse() -> Scalar {
        return this;
    }
}
