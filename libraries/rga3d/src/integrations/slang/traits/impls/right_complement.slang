implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public Scalar operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiScalar rhs) {
        return rhs.right_complement();
    }
}
public extension AntiScalar: RightComplement {
    public typedef Scalar RightComplementOutput;
    public func right_complement() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const RightComplementPrefixOrPostfix& rhs) -> DualNum& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.right_complement();
    }
}
public extension DualNum: RightComplement {
    public typedef DualNum RightComplementOutput;
    public func right_complement() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            this.group0.xy.yx
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const RightComplementPrefixOrPostfix& rhs) -> Flector& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.right_complement();
    }
}
public extension Flector: RightComplement {
    public typedef Flector RightComplementOutput;
    public func right_complement() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Origin operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Horizon rhs) {
        return rhs.right_complement();
    }
}
public extension Horizon: RightComplement {
    public typedef Origin RightComplementOutput;
    public func right_complement() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const RightComplementPrefixOrPostfix& rhs) -> Line& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.right_complement();
    }
}
public extension Line: RightComplement {
    public typedef Line RightComplementOutput;
    public func right_complement() -> Line {
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
    public Motor operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const RightComplementPrefixOrPostfix& rhs) -> Motor& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.right_complement();
    }
}
public extension Motor: RightComplement {
    public typedef Motor RightComplementOutput;
    public func right_complement() -> Motor {
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
    public MultiVector operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const RightComplementPrefixOrPostfix& rhs) -> MultiVector& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.right_complement();
    }
}
public extension MultiVector: RightComplement {
    public typedef MultiVector RightComplementOutput;
    public func right_complement() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy.yx, 
            /* e1, e2, e3, e4 */
            this.group4 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group2.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    public Horizon operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Origin rhs) {
        return rhs.right_complement();
    }
}
public extension Origin: RightComplement {
    public typedef Horizon RightComplementOutput;
    public func right_complement() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e4
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Point operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Plane rhs) {
        return rhs.right_complement();
    }
}
public extension Plane: RightComplement {
    public typedef Point RightComplementOutput;
    public func right_complement() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Plane operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Point rhs) {
        return rhs.right_complement();
    }
}
public extension Point: RightComplement {
    public typedef Plane RightComplementOutput;
    public func right_complement() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public AntiScalar operator /(RightComplementPrefixOrPostfix rhs)  {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Scalar rhs) {
        return rhs.right_complement();
    }
}
public extension Scalar: RightComplement {
    public typedef AntiScalar RightComplementOutput;
    public func right_complement() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}
