extension AntiScalar {
    func div(_rhs: right_complement) -> 
Scalar {
        this.right_complement()
    }
}
extension AntiScalar: RightComplement {
    func right_complement(self) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e1234]
        );
    }
}
extension DualNum {
    func div(_rhs: right_complement) -> 
DualNum {
        this.right_complement()
    }
}
extension DualNum {
    func div=(const right_complement& _rhs) -> DualNum& {
        this = this.right_complement()
        return *this;
    }
}
extension DualNum: RightComplement {
    func right_complement(self) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            self.group0().yx
        );
    }
}
extension Flector {
    func div(_rhs: right_complement) -> 
Flector {
        this.right_complement()
    }
}
extension Flector {
    func div=(const right_complement& _rhs) -> Flector& {
        this = this.right_complement()
        return *this;
    }
}
extension Flector: RightComplement {
    func right_complement(self) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            self.group1() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Horizon {
    func div(_rhs: right_complement) -> 
Origin {
        this.right_complement()
    }
}
extension Horizon: RightComplement {
    func right_complement(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e321] * -1.0
        );
    }
}
extension Line {
    func div(_rhs: right_complement) -> 
Line {
        this.right_complement()
    }
}
extension Line {
    func div=(const right_complement& _rhs) -> Line& {
        this = this.right_complement()
        return *this;
    }
}
extension Line: RightComplement {
    func right_complement(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            self.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group0() * float3(-1.0)
        );
    }
}
extension Motor {
    func div(_rhs: right_complement) -> 
Motor {
        this.right_complement()
    }
}
extension Motor {
    func div=(const right_complement& _rhs) -> Motor& {
        this = this.right_complement()
        return *this;
    }
}
extension Motor: RightComplement {
    func right_complement(self) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            self.group0() * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: right_complement) -> 
MultiVector {
        this.right_complement()
    }
}
extension MultiVector {
    func div=(const right_complement& _rhs) -> MultiVector& {
        this = this.right_complement()
        return *this;
    }
}
extension MultiVector: RightComplement {
    func right_complement(self) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0().yx, 
            /* e1, e2, e3, e4 */
            self.group4() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group2() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group1()
        );
    }
}
extension Origin {
    func div(_rhs: right_complement) -> 
Horizon {
        this.right_complement()
    }
}
extension Origin: RightComplement {
    func right_complement(self) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e4]
        );
    }
}
extension Plane {
    func div(_rhs: right_complement) -> 
Point {
        this.right_complement()
    }
}
extension Plane: RightComplement {
    func right_complement(self) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            self.group0() * float4(-1.0)
        );
    }
}
extension Point {
    func div(_rhs: right_complement) -> 
Plane {
        this.right_complement()
    }
}
extension Point: RightComplement {
    func right_complement(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Scalar {
    func div(_rhs: right_complement) -> 
AntiScalar {
        this.right_complement()
    }
}
extension Scalar: RightComplement {
    func right_complement(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[scalar]
        );
    }
}
