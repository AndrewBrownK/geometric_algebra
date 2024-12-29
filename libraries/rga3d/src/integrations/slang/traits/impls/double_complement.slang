implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.double_complement();
    }
}
public extension AntiScalar: DoubleComplement {
    public typedef AntiScalar DoubleComplementOutput;
    public func double_complement() -> AntiScalar {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> DualNum& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.double_complement();
    }
}
public extension DualNum: DoubleComplement {
    public typedef DualNum DoubleComplementOutput;
    public func double_complement() -> DualNum {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> Flector& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.double_complement();
    }
}
public extension Flector: DoubleComplement {
    public typedef Flector DoubleComplementOutput;
    public func double_complement() -> Flector {
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
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> Horizon& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.double_complement();
    }
}
public extension Horizon: DoubleComplement {
    public typedef Horizon DoubleComplementOutput;
    public func double_complement() -> Horizon {
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
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> Line& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.double_complement();
    }
}
public extension Line: DoubleComplement {
    public typedef Line DoubleComplementOutput;
    public func double_complement() -> Line {
        return this;
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> Motor& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.double_complement();
    }
}
public extension Motor: DoubleComplement {
    public typedef Motor DoubleComplementOutput;
    public func double_complement() -> Motor {
        return this;
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(DoubleComplementPrefixOrPostfix rhs)  {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> MultiVector& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.double_complement();
    }
}
public extension MultiVector: DoubleComplement {
    public typedef MultiVector DoubleComplementOutput;
    public func double_complement() -> MultiVector {
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
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> Origin& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.double_complement();
    }
}
public extension Origin: DoubleComplement {
    public typedef Origin DoubleComplementOutput;
    public func double_complement() -> Origin {
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
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> Plane& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.double_complement();
    }
}
public extension Plane: DoubleComplement {
    public typedef Plane DoubleComplementOutput;
    public func double_complement() -> Plane {
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
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> Point& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.double_complement();
    }
}
public extension Point: DoubleComplement {
    public typedef Point DoubleComplementOutput;
    public func double_complement() -> Point {
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
    // Fancy postfix self-assign
    public func operator /=(const DoubleComplementPrefixOrPostfix& rhs) -> Scalar& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.double_complement();
    }
}
public extension Scalar: DoubleComplement {
    public typedef Scalar DoubleComplementOutput;
    public func double_complement() -> Scalar {
        return this;
    }
}
