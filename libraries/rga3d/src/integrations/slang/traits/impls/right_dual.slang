extension DualNum {
    func div(_rhs: right_dual) -> 
AntiScalar {
        this.right_dual()
    }
}
extension DualNum: RightDual {
    func right_dual() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}
extension Flector {
    func div(_rhs: right_dual) -> 
Flector {
        this.right_dual()
    }
}
extension Flector {
    func div=(const right_dual& _rhs) -> Flector& {
        this = this.right_dual()
        return *this;
    }
}
extension Flector: RightDual {
    func right_dual() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
}
extension Horizon {
    func div(_rhs: right_dual) -> 
Origin {
        this.right_dual()
    }
}
extension Horizon: RightDual {
    func right_dual() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
extension Line {
    func div(_rhs: right_dual) -> 
Line {
        this.right_dual()
    }
}
extension Line {
    func div=(const right_dual& _rhs) -> Line& {
        this = this.right_dual()
        return *this;
    }
}
extension Line: RightDual {
    func right_dual() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Motor {
    func div(_rhs: right_dual) -> 
Motor {
        this.right_dual()
    }
}
extension Motor {
    func div=(const right_dual& _rhs) -> Motor& {
        this = this.right_dual()
        return *this;
    }
}
extension Motor: RightDual {
    func right_dual() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: right_dual) -> 
MultiVector {
        this.right_dual()
    }
}
extension MultiVector {
    func div=(const right_dual& _rhs) -> MultiVector& {
        this = this.right_dual()
        return *this;
    }
}
extension MultiVector: RightDual {
    func right_dual() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e41, e42, e43 */
            this.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
}
extension Plane {
    func div(_rhs: right_dual) -> 
Origin {
        this.right_dual()
    }
}
extension Plane: RightDual {
    func right_dual() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
extension Point {
    func div(_rhs: right_dual) -> 
Plane {
        this.right_dual()
    }
}
extension Point: RightDual {
    func right_dual() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
}
extension Scalar {
    func div(_rhs: right_dual) -> 
AntiScalar {
        this.right_dual()
    }
}
extension Scalar: RightDual {
    func right_dual() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}
