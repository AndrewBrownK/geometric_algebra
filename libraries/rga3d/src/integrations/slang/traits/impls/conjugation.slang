implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
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
    public DualNum operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> DualNum& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
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
    public Flector operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> Flector& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
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
    public Horizon operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> Horizon& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
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
    public Line operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> Line& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
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
    public Motor operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> Motor& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
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
    public MultiVector operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> MultiVector& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
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
    public Origin operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> Origin& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
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
    public Plane operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> Plane& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
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
    public Point operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> Point& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
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
    public Scalar operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConjugationPrefixOrPostfix& rhs) -> Scalar& {
        this = this.conjugation();
        return *this;
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.conjugation();
    }
}
public extension Scalar: Conjugation {
    public typedef Scalar ConjugationOutput;
    public func conjugation() -> Scalar {
        return this;
    }
}
