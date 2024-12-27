extension AntiScalar: Conjugation {
    associatedtype Output = AntiScalar;
    func conjugation() -> Output {
        return this;
    }
    func operator/(_rhs: ConjugationInfix) -> AntiScalar {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> AntiScalar& {
        this = this.conjugation();
        return *this;
    }
}
extension DualNum: Conjugation {
    associatedtype Output = DualNum;
    func conjugation() -> Output {
        return this;
    }
    func operator/(_rhs: ConjugationInfix) -> DualNum {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> DualNum& {
        this = this.conjugation();
        return *this;
    }
}
extension Flector: Conjugation {
    associatedtype Output = Flector;
    func conjugation() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
    func operator/(_rhs: ConjugationInfix) -> Flector {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> Flector& {
        this = this.conjugation();
        return *this;
    }
}
extension Horizon: Conjugation {
    associatedtype Output = Horizon;
    func conjugation() -> Output {
        return this;
    }
    func operator/(_rhs: ConjugationInfix) -> Horizon {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> Horizon& {
        this = this.conjugation();
        return *this;
    }
}
extension Line: Conjugation {
    associatedtype Output = Line;
    func conjugation() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1 * float3(-1.0)
        );
    }
    func operator/(_rhs: ConjugationInfix) -> Line {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> Line& {
        this = this.conjugation();
        return *this;
    }
}
extension Motor: Conjugation {
    associatedtype Output = Motor;
    func conjugation() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
    func operator/(_rhs: ConjugationInfix) -> Motor {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> Motor& {
        this = this.conjugation();
        return *this;
    }
}
extension MultiVector: Conjugation {
    associatedtype Output = MultiVector;
    func conjugation() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
    func operator/(_rhs: ConjugationInfix) -> MultiVector {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> MultiVector& {
        this = this.conjugation();
        return *this;
    }
}
extension Origin: Conjugation {
    associatedtype Output = Origin;
    func conjugation() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
    func operator/(_rhs: ConjugationInfix) -> Origin {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> Origin& {
        this = this.conjugation();
        return *this;
    }
}
extension Plane: Conjugation {
    associatedtype Output = Plane;
    func conjugation() -> Output {
        return this;
    }
    func operator/(_rhs: ConjugationInfix) -> Plane {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> Plane& {
        this = this.conjugation();
        return *this;
    }
}
extension Point: Conjugation {
    associatedtype Output = Point;
    func conjugation() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: ConjugationInfix) -> Point {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> Point& {
        this = this.conjugation();
        return *this;
    }
}
extension Scalar: Conjugation {
    associatedtype Output = Scalar;
    func conjugation() -> Output {
        return this;
    }
    func operator/(_rhs: ConjugationInfix) -> Scalar {
        return this.conjugation();
    }
    func operator/=(const ConjugationInfix& _rhs) -> Scalar& {
        this = this.conjugation();
        return *this;
    }
}
