extension AntiScalar {
    func div(_rhs: double_complement) -> 
AntiScalar {
        this.double_complement()
    }
}
extension AntiScalar {
    func div=(const double_complement& _rhs) -> AntiScalar& {
        this = this.double_complement()
        return *this;
    }
}
extension AntiScalar: DoubleComplement {
    func double_complement(self) -> AntiScalar {
        return self;
    }
}
extension DualNum {
    func div(_rhs: double_complement) -> 
DualNum {
        this.double_complement()
    }
}
extension DualNum {
    func div=(const double_complement& _rhs) -> DualNum& {
        this = this.double_complement()
        return *this;
    }
}
extension DualNum: DoubleComplement {
    func double_complement(self) -> DualNum {
        return self;
    }
}
extension Flector {
    func div(_rhs: double_complement) -> 
Flector {
        this.double_complement()
    }
}
extension Flector {
    func div=(const double_complement& _rhs) -> Flector& {
        this = this.double_complement()
        return *this;
    }
}
extension Flector: DoubleComplement {
    func double_complement(self) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            self.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            self.group1() * float4(-1.0)
        );
    }
}
extension Horizon {
    func div(_rhs: double_complement) -> 
Horizon {
        this.double_complement()
    }
}
extension Horizon {
    func div=(const double_complement& _rhs) -> Horizon& {
        this = this.double_complement()
        return *this;
    }
}
extension Horizon: DoubleComplement {
    func double_complement(self) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321] * -1.0
        );
    }
}
extension Line {
    func div(_rhs: double_complement) -> 
Line {
        this.double_complement()
    }
}
extension Line {
    func div=(const double_complement& _rhs) -> Line& {
        this = this.double_complement()
        return *this;
    }
}
extension Line: DoubleComplement {
    func double_complement(self) -> Line {
        return self;
    }
}
extension Motor {
    func div(_rhs: double_complement) -> 
Motor {
        this.double_complement()
    }
}
extension Motor {
    func div=(const double_complement& _rhs) -> Motor& {
        this = this.double_complement()
        return *this;
    }
}
extension Motor: DoubleComplement {
    func double_complement(self) -> Motor {
        return self;
    }
}
extension MultiVector {
    func div(_rhs: double_complement) -> 
MultiVector {
        this.double_complement()
    }
}
extension MultiVector {
    func div=(const double_complement& _rhs) -> MultiVector& {
        this = this.double_complement()
        return *this;
    }
}
extension MultiVector: DoubleComplement {
    func double_complement(self) -> MultiVector {
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
    func div(_rhs: double_complement) -> 
Origin {
        this.double_complement()
    }
}
extension Origin {
    func div=(const double_complement& _rhs) -> Origin& {
        this = this.double_complement()
        return *this;
    }
}
extension Origin: DoubleComplement {
    func double_complement(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] * -1.0
        );
    }
}
extension Plane {
    func div(_rhs: double_complement) -> 
Plane {
        this.double_complement()
    }
}
extension Plane {
    func div=(const double_complement& _rhs) -> Plane& {
        this = this.double_complement()
        return *this;
    }
}
extension Plane: DoubleComplement {
    func double_complement(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() * float4(-1.0)
        );
    }
}
extension Point {
    func div(_rhs: double_complement) -> 
Point {
        this.double_complement()
    }
}
extension Point {
    func div=(const double_complement& _rhs) -> Point& {
        this = this.double_complement()
        return *this;
    }
}
extension Point: DoubleComplement {
    func double_complement(self) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            self.group0() * float4(-1.0)
        );
    }
}
extension Scalar {
    func div(_rhs: double_complement) -> 
Scalar {
        this.double_complement()
    }
}
extension Scalar {
    func div=(const double_complement& _rhs) -> Scalar& {
        this = this.double_complement()
        return *this;
    }
}
extension Scalar: DoubleComplement {
    func double_complement(self) -> Scalar {
        return self;
    }
}
