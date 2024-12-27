extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiScalar {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiScalar: AutoMorphism {
    associatedtype Output = AntiScalar;
    func auto_morphism() -> Output {
        return this;
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> DualNum {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.auto_morphism();
        return *this;
    }
}
extension DualNum: AutoMorphism {
    associatedtype Output = DualNum;
    func auto_morphism() -> Output {
        return this;
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Flector {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Flector& {
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
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Horizon {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Horizon& {
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
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Line {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Line& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Line: AutoMorphism {
    associatedtype Output = Line;
    func auto_morphism() -> Output {
        return this;
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Motor {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Motor& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Motor: AutoMorphism {
    associatedtype Output = Motor;
    func auto_morphism() -> Output {
        return this;
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> MultiVector {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> MultiVector& {
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
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Origin {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Origin& {
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
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Plane {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Plane& {
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
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Point {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Point& {
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
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Scalar {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Scalar: AutoMorphism {
    associatedtype Output = Scalar;
    func auto_morphism() -> Output {
        return this;
    }
}
