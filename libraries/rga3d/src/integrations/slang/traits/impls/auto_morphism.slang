public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiScalar {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> DualNum {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Flector {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Flector& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Horizon {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Line {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Line& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Motor {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Motor& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> MultiVector {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Origin {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Origin& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Plane {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Plane& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Point {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Point& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Scalar {
        return this.auto_morphism();
    }
    // Fancy postfix self-assign
    public func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.auto_morphism();
        return *this;
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.auto_morphism();
    }
}
public extension Scalar: AutoMorphism {
    public typedef Scalar AutoMorphismOutput;
    public func auto_morphism() -> Scalar {
        return this;
    }
}
