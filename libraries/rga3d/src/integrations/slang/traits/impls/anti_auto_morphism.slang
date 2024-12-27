extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiScalar {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiScalar: AntiAutoMorphism {
    associatedtype Output = AntiScalar;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> DualNum {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension DualNum: AntiAutoMorphism {
    associatedtype Output = DualNum;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Flector {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Flector& {
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
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Horizon {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Horizon: AntiAutoMorphism {
    associatedtype Output = Horizon;
    func anti_auto_morphism() -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Line {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Line& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Line: AntiAutoMorphism {
    associatedtype Output = Line;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Motor {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Motor& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Motor: AntiAutoMorphism {
    associatedtype Output = Motor;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> MultiVector {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension MultiVector: AntiAutoMorphism {
    associatedtype Output = MultiVector;
    func anti_auto_morphism() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Origin {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Origin& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Origin: AntiAutoMorphism {
    associatedtype Output = Origin;
    func anti_auto_morphism() -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Plane {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Plane& {
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
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Point {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Point& {
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
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Scalar {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Scalar: AntiAutoMorphism {
    associatedtype Output = Scalar;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
