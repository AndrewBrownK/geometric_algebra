extension AntiScalar: AutoMorphism {
    associatedtype Output = AntiScalar;
    func auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AutoMorphismInfix) -> AntiScalar {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> AntiScalar& {
        this = this.auto_morphism();
        return *this;
    }
}
extension DualNum: AutoMorphism {
    associatedtype Output = DualNum;
    func auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AutoMorphismInfix) -> DualNum {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> DualNum& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Flector: AutoMorphism {
    associatedtype Output = Flector;
    func auto_morphism() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
    func operator/(_rhs: AutoMorphismInfix) -> Flector {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> Flector& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Horizon: AutoMorphism {
    associatedtype Output = Horizon;
    func auto_morphism() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
    func operator/(_rhs: AutoMorphismInfix) -> Horizon {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> Horizon& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Line: AutoMorphism {
    associatedtype Output = Line;
    func auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AutoMorphismInfix) -> Line {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> Line& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Motor: AutoMorphism {
    associatedtype Output = Motor;
    func auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AutoMorphismInfix) -> Motor {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> Motor& {
        this = this.auto_morphism();
        return *this;
    }
}
extension MultiVector: AutoMorphism {
    associatedtype Output = MultiVector;
    func auto_morphism() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2, 
            /* e23, e31, e12 */
            this.group3, 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
    func operator/(_rhs: AutoMorphismInfix) -> MultiVector {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> MultiVector& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Origin: AutoMorphism {
    associatedtype Output = Origin;
    func auto_morphism() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
    func operator/(_rhs: AutoMorphismInfix) -> Origin {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> Origin& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Plane: AutoMorphism {
    associatedtype Output = Plane;
    func auto_morphism() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: AutoMorphismInfix) -> Plane {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> Plane& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Point: AutoMorphism {
    associatedtype Output = Point;
    func auto_morphism() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: AutoMorphismInfix) -> Point {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> Point& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Scalar: AutoMorphism {
    associatedtype Output = Scalar;
    func auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AutoMorphismInfix) -> Scalar {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismInfix& _rhs) -> Scalar& {
        this = this.auto_morphism();
        return *this;
    }
}
