extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> AntiScalar {
        return this.right_dual();
    }
}
extension DualNum: RightDual {
    associatedtype Output = AntiScalar;
    func right_dual() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Flector {
        return this.right_dual();
    }
    func operator/=(const RightDualPrefixOrPostfix& _rhs) -> Flector& {
        this = this.right_dual();
        return *this;
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
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Origin {
        return this.right_dual();
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
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Line {
        return this.right_dual();
    }
    func operator/=(const RightDualPrefixOrPostfix& _rhs) -> Line& {
        this = this.right_dual();
        return *this;
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
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Motor {
        return this.right_dual();
    }
    func operator/=(const RightDualPrefixOrPostfix& _rhs) -> Motor& {
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
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> MultiVector {
        return this.right_dual();
    }
    func operator/=(const RightDualPrefixOrPostfix& _rhs) -> MultiVector& {
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
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Origin {
        return this.right_dual();
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
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Plane {
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
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> AntiScalar {
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
}
