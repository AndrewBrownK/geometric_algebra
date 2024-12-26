extension AntiScalar {
    func div(_rhs: anti_inverse) -> 
AntiScalar {
        this.anti_inverse()
    }
}
extension AntiScalar {
    func div=(const anti_inverse& _rhs) -> AntiScalar& {
        this = this.anti_inverse()
        return *this;
    }
}
extension AntiScalar: AntiInverse {
    func anti_inverse(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            1.0/self[e1234]
        );
    }
}
extension DualNum {
    func div(_rhs: anti_inverse) -> 
DualNum {
        this.anti_inverse()
    }
}
extension DualNum {
    func div=(const anti_inverse& _rhs) -> DualNum& {
        this = this.anti_inverse()
        return *this;
    }
}
extension DualNum: AntiInverse {
    func anti_inverse(self) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(self[e1234], -2)) * self.group0()
        );
    }
}
extension Flector {
    func div(_rhs: anti_inverse) -> 
Flector {
        this.anti_inverse()
    }
}
extension Flector {
    func div=(const anti_inverse& _rhs) -> Flector& {
        this = this.anti_inverse()
        return *this;
    }
}
extension Flector: AntiInverse {
    func anti_inverse(self) -> Flector {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            self.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            self.group1()
        );
AntiScalar other = AntiScalar::from_groups(
            /* e1234 */
            powi(self[e4], 2) + powi(self[e423], 2) + powi(self[e431], 2) + powi(self[e412], 2)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * anti_reverse.group1()
        );
    }
}
extension Line {
    func div(_rhs: anti_inverse) -> 
Line {
        this.anti_inverse()
    }
}
extension Line {
    func div=(const anti_inverse& _rhs) -> Line& {
        this = this.anti_inverse()
        return *this;
    }
}
extension Line: AntiInverse {
    func anti_inverse(self) -> Line {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            self.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group1() * float3(-1.0)
        );
AntiScalar other = AntiScalar::from_groups(
            /* e1234 */
            powi(self[e41], 2) + powi(self[e42], 2) + powi(self[e43], 2)
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other[e1234]) * anti_reverse.group1()
        );
    }
}
extension Motor {
    func div(_rhs: anti_inverse) -> 
Motor {
        this.anti_inverse()
    }
}
extension Motor {
    func div=(const anti_inverse& _rhs) -> Motor& {
        this = this.anti_inverse()
        return *this;
    }
}
extension Motor: AntiInverse {
    func anti_inverse(self) -> Motor {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            self.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other = AntiScalar::from_groups(
            /* e1234 */
            powi(self[e41], 2) + powi(self[e42], 2) + powi(self[e43], 2) + powi(self[e1234], 2)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[e1234]) * anti_reverse.group1()
        );
    }
}
extension MultiVector {
    func div(_rhs: anti_inverse) -> 
MultiVector {
        this.anti_inverse()
    }
}
extension MultiVector {
    func div=(const anti_inverse& _rhs) -> MultiVector& {
        this = this.anti_inverse()
        return *this;
    }
}
extension MultiVector: AntiInverse {
    func anti_inverse(self) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
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
AntiScalar other = AntiScalar::from_groups(
            /* e1234 */
            powi(self[e1234], 2) + powi(self[e4], 2) + powi(self[e41], 2) + powi(self[e42], 2) + powi(self[e43], 2) + powi(self[e423], 2) + powi(self[e431], 2) + powi(self[e412], 2)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * anti_reverse.group4()
        );
    }
}
extension Origin {
    func div(_rhs: anti_inverse) -> 
Origin {
        this.anti_inverse()
    }
}
extension Origin {
    func div=(const anti_inverse& _rhs) -> Origin& {
        this = this.anti_inverse()
        return *this;
    }
}
extension Origin: AntiInverse {
    func anti_inverse(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            1.0/self[e4] * -1.0
        );
    }
}
extension Plane {
    func div(_rhs: anti_inverse) -> 
Plane {
        this.anti_inverse()
    }
}
extension Plane {
    func div=(const anti_inverse& _rhs) -> Plane& {
        this = this.anti_inverse()
        return *this;
    }
}
extension Plane: AntiInverse {
    func anti_inverse(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(self[e423], 2) + powi(self[e431], 2) + powi(self[e412], 2)) * self.group0()
        );
    }
}
extension Point {
    func div(_rhs: anti_inverse) -> 
Point {
        this.anti_inverse()
    }
}
extension Point {
    func div=(const anti_inverse& _rhs) -> Point& {
        this = this.anti_inverse()
        return *this;
    }
}
extension Point: AntiInverse {
    func anti_inverse(self) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(self[e4], -2)) * float4(self[e1] * -1.0, self[e2] * -1.0, self[e3] * -1.0, self[e4] * -1.0)
        );
    }
}
