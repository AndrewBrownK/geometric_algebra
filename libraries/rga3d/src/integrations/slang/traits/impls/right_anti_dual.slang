extension DualNum: RightAntiDual {
    associatedtype Output = AntiScalar;
    func right_anti_dual() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
    func operator/(_rhs: RightAntiDualInfix) -> AntiScalar {
        return this.right_anti_dual();
    }
}
extension Flector: RightAntiDual {
    associatedtype Output = Flector;
    func right_anti_dual() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
    func operator/(_rhs: RightAntiDualInfix) -> Flector {
        return this.right_anti_dual();
    }
    func operator/=(const RightAntiDualInfix& _rhs) -> Flector& {
        this = this.right_anti_dual();
        return *this;
    }
}
extension Horizon: RightAntiDual {
    associatedtype Output = Origin;
    func right_anti_dual() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
    func operator/(_rhs: RightAntiDualInfix) -> Origin {
        return this.right_anti_dual();
    }
}
extension Line: RightAntiDual {
    associatedtype Output = Line;
    func right_anti_dual() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
    func operator/(_rhs: RightAntiDualInfix) -> Line {
        return this.right_anti_dual();
    }
    func operator/=(const RightAntiDualInfix& _rhs) -> Line& {
        this = this.right_anti_dual();
        return *this;
    }
}
extension Motor: RightAntiDual {
    associatedtype Output = Motor;
    func right_anti_dual() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
    func operator/(_rhs: RightAntiDualInfix) -> Motor {
        return this.right_anti_dual();
    }
    func operator/=(const RightAntiDualInfix& _rhs) -> Motor& {
        this = this.right_anti_dual();
        return *this;
    }
}
extension MultiVector: RightAntiDual {
    associatedtype Output = MultiVector;
    func right_anti_dual() -> Output {
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
    func operator/(_rhs: RightAntiDualInfix) -> MultiVector {
        return this.right_anti_dual();
    }
    func operator/=(const RightAntiDualInfix& _rhs) -> MultiVector& {
        this = this.right_anti_dual();
        return *this;
    }
}
extension Plane: RightAntiDual {
    associatedtype Output = Origin;
    func right_anti_dual() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
    func operator/(_rhs: RightAntiDualInfix) -> Origin {
        return this.right_anti_dual();
    }
}
extension Point: RightAntiDual {
    associatedtype Output = Plane;
    func right_anti_dual() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
    func operator/(_rhs: RightAntiDualInfix) -> Plane {
        return this.right_anti_dual();
    }
}
extension Scalar: RightAntiDual {
    associatedtype Output = AntiScalar;
    func right_anti_dual() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
    func operator/(_rhs: RightAntiDualInfix) -> AntiScalar {
        return this.right_anti_dual();
    }
}
