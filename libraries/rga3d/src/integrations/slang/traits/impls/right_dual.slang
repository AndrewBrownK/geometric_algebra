extension DualNum: RightDual {
    associatedtype Output = AntiScalar;
    func right_dual() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
    func operator/(_rhs: RightDualInfix) -> AntiScalar {
        return this.right_dual();
    }
}
extension Flector: RightDual {
    associatedtype Output = Flector;
    func right_dual() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
    func operator/(_rhs: RightDualInfix) -> Flector {
        return this.right_dual();
    }
    func operator/=(const RightDualInfix& _rhs) -> Flector& {
        this = this.right_dual();
        return *this;
    }
}
extension Horizon: RightDual {
    associatedtype Output = Origin;
    func right_dual() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
    func operator/(_rhs: RightDualInfix) -> Origin {
        return this.right_dual();
    }
}
extension Line: RightDual {
    associatedtype Output = Line;
    func right_dual() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
    func operator/(_rhs: RightDualInfix) -> Line {
        return this.right_dual();
    }
    func operator/=(const RightDualInfix& _rhs) -> Line& {
        this = this.right_dual();
        return *this;
    }
}
extension Motor: RightDual {
    associatedtype Output = Motor;
    func right_dual() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
    func operator/(_rhs: RightDualInfix) -> Motor {
        return this.right_dual();
    }
    func operator/=(const RightDualInfix& _rhs) -> Motor& {
        this = this.right_dual();
        return *this;
    }
}
extension MultiVector: RightDual {
    associatedtype Output = MultiVector;
    func right_dual() -> Output {
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
    func operator/(_rhs: RightDualInfix) -> MultiVector {
        return this.right_dual();
    }
    func operator/=(const RightDualInfix& _rhs) -> MultiVector& {
        this = this.right_dual();
        return *this;
    }
}
extension Plane: RightDual {
    associatedtype Output = Origin;
    func right_dual() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
    func operator/(_rhs: RightDualInfix) -> Origin {
        return this.right_dual();
    }
}
extension Point: RightDual {
    associatedtype Output = Plane;
    func right_dual() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
    func operator/(_rhs: RightDualInfix) -> Plane {
        return this.right_dual();
    }
}
extension Scalar: RightDual {
    associatedtype Output = AntiScalar;
    func right_dual() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
    func operator/(_rhs: RightDualInfix) -> AntiScalar {
        return this.right_dual();
    }
}
