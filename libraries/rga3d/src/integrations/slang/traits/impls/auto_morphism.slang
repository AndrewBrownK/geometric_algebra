extension AntiScalar {
    func div(_rhs: auto_morphism) -> 
AntiScalar {
        this.auto_morphism()
    }
}
extension AntiScalar {
    func div=(const auto_morphism& _rhs) -> AntiScalar& {
        this = this.auto_morphism()
        return *this;
    }
}
extension AntiScalar: AutoMorphism {
    func auto_morphism(self) -> AntiScalar {
        return self;
    }
}
extension DualNum {
    func div(_rhs: auto_morphism) -> 
DualNum {
        this.auto_morphism()
    }
}
extension DualNum {
    func div=(const auto_morphism& _rhs) -> DualNum& {
        this = this.auto_morphism()
        return *this;
    }
}
extension DualNum: AutoMorphism {
    func auto_morphism(self) -> DualNum {
        return self;
    }
}
extension Flector {
    func div(_rhs: auto_morphism) -> 
Flector {
        this.auto_morphism()
    }
}
extension Flector {
    func div=(const auto_morphism& _rhs) -> Flector& {
        this = this.auto_morphism()
        return *this;
    }
}
extension Flector: AutoMorphism {
    func auto_morphism(self) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            self.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            self.group1() * float4(-1.0)
        );
    }
}
extension Horizon {
    func div(_rhs: auto_morphism) -> 
Horizon {
        this.auto_morphism()
    }
}
extension Horizon {
    func div=(const auto_morphism& _rhs) -> Horizon& {
        this = this.auto_morphism()
        return *this;
    }
}
extension Horizon: AutoMorphism {
    func auto_morphism(self) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321] * -1.0
        );
    }
}
extension Line {
    func div(_rhs: auto_morphism) -> 
Line {
        this.auto_morphism()
    }
}
extension Line {
    func div=(const auto_morphism& _rhs) -> Line& {
        this = this.auto_morphism()
        return *this;
    }
}
extension Line: AutoMorphism {
    func auto_morphism(self) -> Line {
        return self;
    }
}
extension Motor {
    func div(_rhs: auto_morphism) -> 
Motor {
        this.auto_morphism()
    }
}
extension Motor {
    func div=(const auto_morphism& _rhs) -> Motor& {
        this = this.auto_morphism()
        return *this;
    }
}
extension Motor: AutoMorphism {
    func auto_morphism(self) -> Motor {
        return self;
    }
}
extension MultiVector {
    func div(_rhs: auto_morphism) -> 
MultiVector {
        this.auto_morphism()
    }
}
extension MultiVector {
    func div=(const auto_morphism& _rhs) -> MultiVector& {
        this = this.auto_morphism()
        return *this;
    }
}
extension MultiVector: AutoMorphism {
    func auto_morphism(self) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() * float4(-1.0)
        );
    }
}
extension Origin {
    func div(_rhs: auto_morphism) -> 
Origin {
        this.auto_morphism()
    }
}
extension Origin {
    func div=(const auto_morphism& _rhs) -> Origin& {
        this = this.auto_morphism()
        return *this;
    }
}
extension Origin: AutoMorphism {
    func auto_morphism(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] * -1.0
        );
    }
}
extension Plane {
    func div(_rhs: auto_morphism) -> 
Plane {
        this.auto_morphism()
    }
}
extension Plane {
    func div=(const auto_morphism& _rhs) -> Plane& {
        this = this.auto_morphism()
        return *this;
    }
}
extension Plane: AutoMorphism {
    func auto_morphism(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() * float4(-1.0)
        );
    }
}
extension Point {
    func div(_rhs: auto_morphism) -> 
Point {
        this.auto_morphism()
    }
}
extension Point {
    func div=(const auto_morphism& _rhs) -> Point& {
        this = this.auto_morphism()
        return *this;
    }
}
extension Point: AutoMorphism {
    func auto_morphism(self) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            self.group0() * float4(-1.0)
        );
    }
}
extension Scalar {
    func div(_rhs: auto_morphism) -> 
Scalar {
        this.auto_morphism()
    }
}
extension Scalar {
    func div=(const auto_morphism& _rhs) -> Scalar& {
        this = this.auto_morphism()
        return *this;
    }
}
extension Scalar: AutoMorphism {
    func auto_morphism(self) -> Scalar {
        return self;
    }
}
