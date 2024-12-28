public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiScalar {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.conjugation();
    }
}
public extension AntiScalar: Conjugation {
    public typedef AntiScalar ConjugationOutput;
    public func conjugation() -> AntiScalar {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> DualNum {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.conjugation();
    }
}
public extension DualNum: Conjugation {
    public typedef DualNum ConjugationOutput;
    public func conjugation() -> DualNum {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Flector {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Flector& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.conjugation();
    }
}
public extension Flector: Conjugation {
    public typedef Flector ConjugationOutput;
    public func conjugation() -> Flector {
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
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Horizon {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.conjugation();
    }
}
public extension Horizon: Conjugation {
    public typedef Horizon ConjugationOutput;
    public func conjugation() -> Horizon {
        return this;
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Line {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Line& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.conjugation();
    }
}
public extension Line: Conjugation {
    public typedef Line ConjugationOutput;
    public func conjugation() -> Line {
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
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Motor {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Motor& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.conjugation();
    }
}
public extension Motor: Conjugation {
    public typedef Motor ConjugationOutput;
    public func conjugation() -> Motor {
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
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> MultiVector {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.conjugation();
    }
}
public extension MultiVector: Conjugation {
    public typedef MultiVector ConjugationOutput;
    public func conjugation() -> MultiVector {
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
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Origin {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Origin& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.conjugation();
    }
}
public extension Origin: Conjugation {
    public typedef Origin ConjugationOutput;
    public func conjugation() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Plane {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Plane& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.conjugation();
    }
}
public extension Plane: Conjugation {
    public typedef Plane ConjugationOutput;
    public func conjugation() -> Plane {
        return this;
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Point {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Point& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.conjugation();
    }
}
public extension Point: Conjugation {
    public typedef Point ConjugationOutput;
    public func conjugation() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Scalar {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.conjugation();
    }
}
public extension Scalar: Conjugation {
    public typedef Scalar ConjugationOutput;
    public func conjugation() -> Scalar {
        return this;
    }
}
