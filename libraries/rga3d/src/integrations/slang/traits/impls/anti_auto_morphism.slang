extension AntiScalar: AntiAutoMorphism {
    associatedtype Output = AntiScalar;
    func anti_auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> AntiScalar {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> AntiScalar& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension DualNum: AntiAutoMorphism {
    associatedtype Output = DualNum;
    func anti_auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> DualNum {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> DualNum& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Flector: AntiAutoMorphism {
    associatedtype Output = Flector;
    func anti_auto_morphism() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> Flector {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> Flector& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Horizon: AntiAutoMorphism {
    associatedtype Output = Horizon;
    func anti_auto_morphism() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> Horizon {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> Horizon& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Line: AntiAutoMorphism {
    associatedtype Output = Line;
    func anti_auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> Line {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> Line& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Motor: AntiAutoMorphism {
    associatedtype Output = Motor;
    func anti_auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> Motor {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> Motor& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension MultiVector: AntiAutoMorphism {
    associatedtype Output = MultiVector;
    func anti_auto_morphism() -> Output {
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
    func operator/(_rhs: AntiAutoMorphismInfix) -> MultiVector {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> MultiVector& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Origin: AntiAutoMorphism {
    associatedtype Output = Origin;
    func anti_auto_morphism() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> Origin {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> Origin& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Plane: AntiAutoMorphism {
    associatedtype Output = Plane;
    func anti_auto_morphism() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> Plane {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> Plane& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Point: AntiAutoMorphism {
    associatedtype Output = Point;
    func anti_auto_morphism() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> Point {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> Point& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Scalar: AntiAutoMorphism {
    associatedtype Output = Scalar;
    func anti_auto_morphism() -> Output {
        return this;
    }
    func operator/(_rhs: AntiAutoMorphismInfix) -> Scalar {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismInfix& _rhs) -> Scalar& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
