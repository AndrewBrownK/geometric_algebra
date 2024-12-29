implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiScalar: AutoMorphism {
    public typedef AntiScalar AutoMorphismOutput;
    public func auto_morphism() -> AntiScalar {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> DualNum& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.auto_morphism();
    }
}
public extension DualNum: AutoMorphism {
    public typedef DualNum AutoMorphismOutput;
    public func auto_morphism() -> DualNum {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> Flector& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.auto_morphism();
    }
}
public extension Flector: AutoMorphism {
    public typedef Flector AutoMorphismOutput;
    public func auto_morphism() -> Flector {
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
    public Horizon operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> Horizon& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.auto_morphism();
    }
}
public extension Horizon: AutoMorphism {
    public typedef Horizon AutoMorphismOutput;
    public func auto_morphism() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> Line& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.auto_morphism();
    }
}
public extension Line: AutoMorphism {
    public typedef Line AutoMorphismOutput;
    public func auto_morphism() -> Line {
        return this;
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> Motor& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.auto_morphism();
    }
}
public extension Motor: AutoMorphism {
    public typedef Motor AutoMorphismOutput;
    public func auto_morphism() -> Motor {
        return this;
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> MultiVector& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.auto_morphism();
    }
}
public extension MultiVector: AutoMorphism {
    public typedef MultiVector AutoMorphismOutput;
    public func auto_morphism() -> MultiVector {
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
    public Origin operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> Origin& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.auto_morphism();
    }
}
public extension Origin: AutoMorphism {
    public typedef Origin AutoMorphismOutput;
    public func auto_morphism() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> Plane& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.auto_morphism();
    }
}
public extension Plane: AutoMorphism {
    public typedef Plane AutoMorphismOutput;
    public func auto_morphism() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> Point& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.auto_morphism();
    }
}
public extension Point: AutoMorphism {
    public typedef Point AutoMorphismOutput;
    public func auto_morphism() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AutoMorphismPrefixOrPostfix& rhs) -> Scalar& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.auto_morphism();
    }
}
public extension Scalar: AutoMorphism {
    public typedef Scalar AutoMorphismOutput;
    public func auto_morphism() -> Scalar {
        return this;
    }
}
