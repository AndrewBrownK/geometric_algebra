extension DualNum {
    func div(_rhs: right_anti_dual) -> 
AntiScalar {
        this.right_anti_dual()
    }
}
extension DualNum: RightAntiDual {
    func right_anti_dual(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[scalar]
        );
    }
}
extension Flector {
    func div(_rhs: right_anti_dual) -> 
Flector {
        this.right_anti_dual()
    }
}
extension Flector {
    func div=(const right_anti_dual& _rhs) -> Flector& {
        this = this.right_anti_dual()
        return *this;
    }
}
extension Flector: RightAntiDual {
    func right_anti_dual(self) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(self[e1], self[e2], self[e3], 0.0)
        );
    }
}
extension Horizon {
    func div(_rhs: right_anti_dual) -> 
Origin {
        this.right_anti_dual()
    }
}
extension Horizon: RightAntiDual {
    func right_anti_dual(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e321] * -1.0
        );
    }
}
extension Line {
    func div(_rhs: right_anti_dual) -> 
Line {
        this.right_anti_dual()
    }
}
extension Line {
    func div=(const right_anti_dual& _rhs) -> Line& {
        this = this.right_anti_dual()
        return *this;
    }
}
extension Line: RightAntiDual {
    func right_anti_dual(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            self.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Motor {
    func div(_rhs: right_anti_dual) -> 
Motor {
        this.right_anti_dual()
    }
}
extension Motor {
    func div=(const right_anti_dual& _rhs) -> Motor& {
        this = this.right_anti_dual()
        return *this;
    }
}
extension Motor: RightAntiDual {
    func right_anti_dual(self) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: right_anti_dual) -> 
MultiVector {
        this.right_anti_dual()
    }
}
extension MultiVector {
    func div=(const right_anti_dual& _rhs) -> MultiVector& {
        this = this.right_anti_dual()
        return *this;
    }
}
extension MultiVector: RightAntiDual {
    func right_anti_dual(self) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e321] * -1.0), 
            /* e41, e42, e43 */
            self.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e1], self[e2], self[e3], 0.0)
        );
    }
}
extension Plane {
    func div(_rhs: right_anti_dual) -> 
Origin {
        this.right_anti_dual()
    }
}
extension Plane: RightAntiDual {
    func right_anti_dual(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e321] * -1.0
        );
    }
}
extension Point {
    func div(_rhs: right_anti_dual) -> 
Plane {
        this.right_anti_dual()
    }
}
extension Point: RightAntiDual {
    func right_anti_dual(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1], self[e2], self[e3], 0.0)
        );
    }
}
extension Scalar {
    func div(_rhs: right_anti_dual) -> 
AntiScalar {
        this.right_anti_dual()
    }
}
extension Scalar: RightAntiDual {
    func right_anti_dual(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[scalar]
        );
    }
}
