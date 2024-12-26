extension AntiScalar {
    func div(_rhs: conjugation) -> 
AntiScalar {
        this.conjugation()
    }
}
extension AntiScalar {
    func div=(const conjugation& _rhs) -> AntiScalar& {
        this = this.conjugation()
        return *this;
    }
}
extension AntiScalar: Conjugation {
    func conjugation(self) -> AntiScalar {
        return self;
    }
}
extension DualNum {
    func div(_rhs: conjugation) -> 
DualNum {
        this.conjugation()
    }
}
extension DualNum {
    func div=(const conjugation& _rhs) -> DualNum& {
        this = this.conjugation()
        return *this;
    }
}
extension DualNum: Conjugation {
    func conjugation(self) -> DualNum {
        return self;
    }
}
extension Flector {
    func div(_rhs: conjugation) -> 
Flector {
        this.conjugation()
    }
}
extension Flector {
    func div=(const conjugation& _rhs) -> Flector& {
        this = this.conjugation()
        return *this;
    }
}
extension Flector: Conjugation {
    func conjugation(self) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            self.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            self.group1()
        );
    }
}
extension Horizon {
    func div(_rhs: conjugation) -> 
Horizon {
        this.conjugation()
    }
}
extension Horizon {
    func div=(const conjugation& _rhs) -> Horizon& {
        this = this.conjugation()
        return *this;
    }
}
extension Horizon: Conjugation {
    func conjugation(self) -> Horizon {
        return self;
    }
}
extension Line {
    func div(_rhs: conjugation) -> 
Line {
        this.conjugation()
    }
}
extension Line {
    func div=(const conjugation& _rhs) -> Line& {
        this = this.conjugation()
        return *this;
    }
}
extension Line: Conjugation {
    func conjugation(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            self.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group1() * float3(-1.0)
        );
    }
}
extension Motor {
    func div(_rhs: conjugation) -> 
Motor {
        this.conjugation()
    }
}
extension Motor {
    func div=(const conjugation& _rhs) -> Motor& {
        this = this.conjugation()
        return *this;
    }
}
extension Motor: Conjugation {
    func conjugation(self) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            self.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: conjugation) -> 
MultiVector {
        this.conjugation()
    }
}
extension MultiVector {
    func div=(const conjugation& _rhs) -> MultiVector& {
        this = this.conjugation()
        return *this;
    }
}
extension MultiVector: Conjugation {
    func conjugation(self) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension Origin {
    func div(_rhs: conjugation) -> 
Origin {
        this.conjugation()
    }
}
extension Origin {
    func div=(const conjugation& _rhs) -> Origin& {
        this = this.conjugation()
        return *this;
    }
}
extension Origin: Conjugation {
    func conjugation(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] * -1.0
        );
    }
}
extension Plane {
    func div(_rhs: conjugation) -> 
Plane {
        this.conjugation()
    }
}
extension Plane {
    func div=(const conjugation& _rhs) -> Plane& {
        this = this.conjugation()
        return *this;
    }
}
extension Plane: Conjugation {
    func conjugation(self) -> Plane {
        return self;
    }
}
extension Point {
    func div(_rhs: conjugation) -> 
Point {
        this.conjugation()
    }
}
extension Point {
    func div=(const conjugation& _rhs) -> Point& {
        this = this.conjugation()
        return *this;
    }
}
extension Point: Conjugation {
    func conjugation(self) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            self.group0() * float4(-1.0)
        );
    }
}
extension Scalar {
    func div(_rhs: conjugation) -> 
Scalar {
        this.conjugation()
    }
}
extension Scalar {
    func div=(const conjugation& _rhs) -> Scalar& {
        this = this.conjugation()
        return *this;
    }
}
extension Scalar: Conjugation {
    func conjugation(self) -> Scalar {
        return self;
    }
}
