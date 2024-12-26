extension AntiScalar {
    func div(_rhs: left_complement) -> 
Scalar {
        this.left_complement()
    }
}
extension AntiScalar: LeftComplement {
    func left_complement(self) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e1234]
        );
    }
}
extension DualNum {
    func div(_rhs: left_complement) -> 
DualNum {
        this.left_complement()
    }
}
extension DualNum {
    func div=(const left_complement& _rhs) -> DualNum& {
        this = this.left_complement()
        return *this;
    }
}
extension DualNum: LeftComplement {
    func left_complement(self) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            self.group0().yx
        );
    }
}
extension Flector {
    func div(_rhs: left_complement) -> 
Flector {
        this.left_complement()
    }
}
extension Flector {
    func div=(const left_complement& _rhs) -> Flector& {
        this = this.left_complement()
        return *this;
    }
}
extension Flector: LeftComplement {
    func left_complement(self) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            self.group0() * float4(-1.0)
        );
    }
}
extension Horizon {
    func div(_rhs: left_complement) -> 
Origin {
        this.left_complement()
    }
}
extension Horizon: LeftComplement {
    func left_complement(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e321]
        );
    }
}
extension Line {
    func div(_rhs: left_complement) -> 
Line {
        this.left_complement()
    }
}
extension Line {
    func div=(const left_complement& _rhs) -> Line& {
        this = this.left_complement()
        return *this;
    }
}
extension Line: LeftComplement {
    func left_complement(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            self.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group0() * float3(-1.0)
        );
    }
}
extension Motor {
    func div(_rhs: left_complement) -> 
Motor {
        this.left_complement()
    }
}
extension Motor {
    func div=(const left_complement& _rhs) -> Motor& {
        this = this.left_complement()
        return *this;
    }
}
extension Motor: LeftComplement {
    func left_complement(self) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            self.group0() * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: left_complement) -> 
MultiVector {
        this.left_complement()
    }
}
extension MultiVector {
    func div=(const left_complement& _rhs) -> MultiVector& {
        this = this.left_complement()
        return *this;
    }
}
extension MultiVector: LeftComplement {
    func left_complement(self) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0().yx, 
            /* e1, e2, e3, e4 */
            self.group4(), 
            /* e41, e42, e43 */
            self.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group2() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group1() * float4(-1.0)
        );
    }
}
extension Origin {
    func div(_rhs: left_complement) -> 
Horizon {
        this.left_complement()
    }
}
extension Origin: LeftComplement {
    func left_complement(self) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e4] * -1.0
        );
    }
}
extension Plane {
    func div(_rhs: left_complement) -> 
Point {
        this.left_complement()
    }
}
extension Plane: LeftComplement {
    func left_complement(self) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            self.group0()
        );
    }
}
extension Point {
    func div(_rhs: left_complement) -> 
Plane {
        this.left_complement()
    }
}
extension Point: LeftComplement {
    func left_complement(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() * float4(-1.0)
        );
    }
}
extension Scalar {
    func div(_rhs: left_complement) -> 
AntiScalar {
        this.left_complement()
    }
}
extension Scalar: LeftComplement {
    func left_complement(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[scalar]
        );
    }
}
