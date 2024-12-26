extension DualNum {
    func div(_rhs: inverse) -> 
DualNum {
        this.inverse()
    }
}
extension DualNum {
    func div=(const inverse& _rhs) -> DualNum& {
        this = this.inverse()
        return *this;
    }
}
extension DualNum: Inverse {
    func inverse(self) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(self[scalar], -2)) * self.group0()
        );
    }
}
extension Flector {
    func div(_rhs: inverse) -> 
Flector {
        this.inverse()
    }
}
extension Flector {
    func div=(const inverse& _rhs) -> Flector& {
        this = this.inverse()
        return *this;
    }
}
extension Flector: Inverse {
    func inverse(self) -> Flector {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            self.group0(), 
            /* e423, e431, e412, e321 */
            self.group1() * float4(-1.0)
        );
Scalar other = Scalar::from_groups(
            /* scalar */
            powi(self[e1], 2) + powi(self[e2], 2) + powi(self[e3], 2) + powi(self[e321], 2)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * reverse.group1()
        );
    }
}
extension Horizon {
    func div(_rhs: inverse) -> 
Horizon {
        this.inverse()
    }
}
extension Horizon {
    func div=(const inverse& _rhs) -> Horizon& {
        this = this.inverse()
        return *this;
    }
}
extension Horizon: Inverse {
    func inverse(self) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            1.0/self[e321] * -1.0
        );
    }
}
extension Line {
    func div(_rhs: inverse) -> 
Line {
        this.inverse()
    }
}
extension Line {
    func div=(const inverse& _rhs) -> Line& {
        this = this.inverse()
        return *this;
    }
}
extension Line: Inverse {
    func inverse(self) -> Line {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            self.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group1() * float3(-1.0)
        );
Scalar other = Scalar::from_groups(
            /* scalar */
            powi(self[e23], 2) + powi(self[e31], 2) + powi(self[e12], 2)
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * reverse.group1()
        );
    }
}
extension Motor {
    func div(_rhs: inverse) -> 
Motor {
        this.inverse()
    }
}
extension Motor {
    func div=(const inverse& _rhs) -> Motor& {
        this = this.inverse()
        return *this;
    }
}
extension Motor: Inverse {
    func inverse(self) -> Motor {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            self.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other = Scalar::from_groups(
            /* scalar */
            powi(self[e23], 2) + powi(self[e31], 2) + powi(self[e12], 2) + powi(self[scalar], 2)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * reverse.group1()
        );
    }
}
extension MultiVector {
    func div(_rhs: inverse) -> 
MultiVector {
        this.inverse()
    }
}
extension MultiVector {
    func div=(const inverse& _rhs) -> MultiVector& {
        this = this.inverse()
        return *this;
    }
}
extension MultiVector: Inverse {
    func inverse(self) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group4() * float4(-1.0)
        );
Scalar other = Scalar::from_groups(
            /* scalar */
            powi(self[scalar], 2) + powi(self[e1], 2) + powi(self[e2], 2) + powi(self[e3], 2) + powi(self[e23], 2) + powi(self[e31], 2) + powi(self[e12], 2) + powi(self[e321], 2)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * reverse.group4()
        );
    }
}
extension Plane {
    func div(_rhs: inverse) -> 
Plane {
        this.inverse()
    }
}
extension Plane {
    func div=(const inverse& _rhs) -> Plane& {
        this = this.inverse()
        return *this;
    }
}
extension Plane: Inverse {
    func inverse(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(self[e321], -2)) * float4(self[e423] * -1.0, self[e431] * -1.0, self[e412] * -1.0, self[e321] * -1.0)
        );
    }
}
extension Point {
    func div(_rhs: inverse) -> 
Point {
        this.inverse()
    }
}
extension Point {
    func div=(const inverse& _rhs) -> Point& {
        this = this.inverse()
        return *this;
    }
}
extension Point: Inverse {
    func inverse(self) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(self[e1], 2) + powi(self[e2], 2) + powi(self[e3], 2)) * self.group0()
        );
    }
}
extension Scalar {
    func div(_rhs: inverse) -> 
Scalar {
        this.inverse()
    }
}
extension Scalar {
    func div=(const inverse& _rhs) -> Scalar& {
        this = this.inverse()
        return *this;
    }
}
extension Scalar: Inverse {
    func inverse(self) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            1.0/self[scalar]
        );
    }
}
