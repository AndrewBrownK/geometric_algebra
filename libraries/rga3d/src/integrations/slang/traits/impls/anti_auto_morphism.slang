implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension AntiScalar: AntiAutoMorphism {
    public typedef AntiScalar AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> AntiScalar {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> DualNum& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension DualNum: AntiAutoMorphism {
    public typedef DualNum AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> DualNum {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> Flector& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension Flector: AntiAutoMorphism {
    public typedef Flector AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> Flector {
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
    public Horizon operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> Horizon& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension Horizon: AntiAutoMorphism {
    public typedef Horizon AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> Line& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension Line: AntiAutoMorphism {
    public typedef Line AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> Line {
        return this;
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> Motor& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension Motor: AntiAutoMorphism {
    public typedef Motor AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> Motor {
        return this;
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> MultiVector& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension MultiVector: AntiAutoMorphism {
    public typedef MultiVector AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> MultiVector {
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
    public Origin operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> Origin& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension Origin: AntiAutoMorphism {
    public typedef Origin AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> Plane& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension Plane: AntiAutoMorphism {
    public typedef Plane AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> Point& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension Point: AntiAutoMorphism {
    public typedef Point AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(AntiAutoMorphismPrefixOrPostfix rhs)  {
        return this.anti_auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiAutoMorphismPrefixOrPostfix& rhs) -> Scalar& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
public extension AntiAutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.anti_auto_morphism();
    }
}
public extension Scalar: AntiAutoMorphism {
    public typedef Scalar AntiAutoMorphismOutput;
    public func anti_auto_morphism() -> Scalar {
        return this;
    }
}
