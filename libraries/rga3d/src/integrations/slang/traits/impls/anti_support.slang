extension DualNum: AntiSupport {
    associatedtype Output = Horizon;
    func anti_support() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.scalar
        );
    }
    func operator/(_rhs: AntiSupportInfix) -> Horizon {
        return this.anti_support();
    }
}
extension Flector: AntiSupport {
    associatedtype Output = Motor;
    func anti_support() -> Output {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(right_dual.group1.xyz, right_dual.e4) * float4(-1.0)
        );
    }
    func operator/(_rhs: AntiSupportInfix) -> Motor {
        return this.anti_support();
    }
}
extension Horizon: AntiSupport {
    associatedtype Output = Scalar;
    func anti_support() -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e321
        );
    }
    func operator/(_rhs: AntiSupportInfix) -> Scalar {
        return this.anti_support();
    }
}
extension Line: AntiSupport {
    associatedtype Output = Point;
    func anti_support() -> Output {
Horizon right_complement = Horizon.from_groups(
            /* e321 */
            1.0
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_complement.e321, right_complement.e321, right_complement.e321, 0.0) * float4(float3(this.e23 * -1.0, this.e31 * -1.0, this.e12 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
    func operator/(_rhs: AntiSupportInfix) -> Point {
        return this.anti_support();
    }
}
extension Motor: AntiSupport {
    associatedtype Output = Flector;
    func anti_support() -> Output {
Horizon right_complement = Horizon.from_groups(
            /* e321 */
            1.0
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_complement.e321, right_complement.e321, right_complement.e321, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), right_complement.e321 * right_dual.e1234)
        );
    }
    func operator/(_rhs: AntiSupportInfix) -> Flector {
        return this.anti_support();
    }
}
extension MultiVector: AntiSupport {
    associatedtype Output = MultiVector;
    func anti_support() -> Output {
Horizon right_complement = Horizon.from_groups(
            /* e321 */
            1.0
        );
MultiVector right_dual = MultiVector.from_groups(
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_complement.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_complement.e321, right_complement.e321, right_complement.e321, 0.0) * float4(right_dual.group2, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_complement.e321) * right_dual.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), right_complement.e321 * right_dual.e1234)
        );
    }
    func operator/(_rhs: AntiSupportInfix) -> MultiVector {
        return this.anti_support();
    }
    func operator/=(const AntiSupportInfix& _rhs) -> MultiVector& {
        this = this.anti_support();
        return *this;
    }
}
extension Plane: AntiSupport {
    associatedtype Output = Scalar;
    func anti_support() -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e321
        );
    }
    func operator/(_rhs: AntiSupportInfix) -> Scalar {
        return this.anti_support();
    }
}
extension Point: AntiSupport {
    associatedtype Output = Line;
    func anti_support() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float4(this.e1, this.e2, this.e3, 0.0).xyz * float3(-1.0)
        );
    }
    func operator/(_rhs: AntiSupportInfix) -> Line {
        return this.anti_support();
    }
}
extension Scalar: AntiSupport {
    associatedtype Output = Horizon;
    func anti_support() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.scalar
        );
    }
    func operator/(_rhs: AntiSupportInfix) -> Horizon {
        return this.anti_support();
    }
}
