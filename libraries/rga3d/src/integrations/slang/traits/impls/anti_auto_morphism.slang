extension AntiScalar {
    func div(_rhs: anti_auto_morphism) -> 
AntiScalar {
        this.anti_auto_morphism()
    }
}
extension AntiScalar {
    func div=(const anti_auto_morphism& _rhs) -> AntiScalar& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension AntiScalar: AntiAutoMorphism {
    func anti_auto_morphism() -> AntiScalar {
        return this;
    }
}
extension DualNum {
    func div(_rhs: anti_auto_morphism) -> 
DualNum {
        this.anti_auto_morphism()
    }
}
extension DualNum {
    func div=(const anti_auto_morphism& _rhs) -> DualNum& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension DualNum: AntiAutoMorphism {
    func anti_auto_morphism() -> DualNum {
        return this;
    }
}
extension Flector {
    func div(_rhs: anti_auto_morphism) -> 
Flector {
        this.anti_auto_morphism()
    }
}
extension Flector {
    func div=(const anti_auto_morphism& _rhs) -> Flector& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension Flector: AntiAutoMorphism {
    func anti_auto_morphism() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
extension Horizon {
    func div(_rhs: anti_auto_morphism) -> 
Horizon {
        this.anti_auto_morphism()
    }
}
extension Horizon {
    func div=(const anti_auto_morphism& _rhs) -> Horizon& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension Horizon: AntiAutoMorphism {
    func anti_auto_morphism() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
extension Line {
    func div(_rhs: anti_auto_morphism) -> 
Line {
        this.anti_auto_morphism()
    }
}
extension Line {
    func div=(const anti_auto_morphism& _rhs) -> Line& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension Line: AntiAutoMorphism {
    func anti_auto_morphism() -> Line {
        return this;
    }
}
extension Motor {
    func div(_rhs: anti_auto_morphism) -> 
Motor {
        this.anti_auto_morphism()
    }
}
extension Motor {
    func div=(const anti_auto_morphism& _rhs) -> Motor& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension Motor: AntiAutoMorphism {
    func anti_auto_morphism() -> Motor {
        return this;
    }
}
extension MultiVector {
    func div(_rhs: anti_auto_morphism) -> 
MultiVector {
        this.anti_auto_morphism()
    }
}
extension MultiVector {
    func div=(const anti_auto_morphism& _rhs) -> MultiVector& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension MultiVector: AntiAutoMorphism {
    func anti_auto_morphism() -> MultiVector {
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
    func div(_rhs: anti_auto_morphism) -> 
Origin {
        this.anti_auto_morphism()
    }
}
extension Origin {
    func div=(const anti_auto_morphism& _rhs) -> Origin& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension Origin: AntiAutoMorphism {
    func anti_auto_morphism() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
extension Plane {
    func div(_rhs: anti_auto_morphism) -> 
Plane {
        this.anti_auto_morphism()
    }
}
extension Plane {
    func div=(const anti_auto_morphism& _rhs) -> Plane& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension Plane: AntiAutoMorphism {
    func anti_auto_morphism() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Point {
    func div(_rhs: anti_auto_morphism) -> 
Point {
        this.anti_auto_morphism()
    }
}
extension Point {
    func div=(const anti_auto_morphism& _rhs) -> Point& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension Point: AntiAutoMorphism {
    func anti_auto_morphism() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Scalar {
    func div(_rhs: anti_auto_morphism) -> 
Scalar {
        this.anti_auto_morphism()
    }
}
extension Scalar {
    func div=(const anti_auto_morphism& _rhs) -> Scalar& {
        this = this.anti_auto_morphism()
        return *this;
    }
}
extension Scalar: AntiAutoMorphism {
    func anti_auto_morphism() -> Scalar {
        return this;
    }
}
