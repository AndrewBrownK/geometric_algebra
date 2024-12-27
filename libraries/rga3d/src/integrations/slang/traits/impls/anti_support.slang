extension DualNum {
    func div(_rhs: anti_support) -> 
Horizon {
        this.anti_support()
    }
}
extension DualNum: AntiSupport {
    func anti_support() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.scalar
        );
    }
}
extension Flector {
    func div(_rhs: anti_support) -> 
Motor {
        this.anti_support()
    }
}
extension Flector: AntiSupport {
    func anti_support() -> Motor {
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
}
extension Horizon {
    func div(_rhs: anti_support) -> 
Scalar {
        this.anti_support()
    }
}
extension Horizon: AntiSupport {
    func anti_support() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321
        );
    }
}
extension Line {
    func div(_rhs: anti_support) -> 
Point {
        this.anti_support()
    }
}
extension Line: AntiSupport {
    func anti_support() -> Point {
Horizon right_complement = Horizon.from_groups(
            /* e321 */
            1.0
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_complement.e321, right_complement.e321, right_complement.e321, 0.0) * float4(float3(this.e23 * -1.0, this.e31 * -1.0, this.e12 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor {
    func div(_rhs: anti_support) -> 
Flector {
        this.anti_support()
    }
}
extension Motor: AntiSupport {
    func anti_support() -> Flector {
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
}
extension MultiVector {
    func div(_rhs: anti_support) -> 
MultiVector {
        this.anti_support()
    }
}
extension MultiVector {
    func div=(const anti_support& _rhs) -> MultiVector& {
        this = this.anti_support()
        return *this;
    }
}
extension MultiVector: AntiSupport {
    func anti_support() -> MultiVector {
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
}
extension Plane {
    func div(_rhs: anti_support) -> 
Scalar {
        this.anti_support()
    }
}
extension Plane: AntiSupport {
    func anti_support() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321
        );
    }
}
extension Point {
    func div(_rhs: anti_support) -> 
Line {
        this.anti_support()
    }
}
extension Point: AntiSupport {
    func anti_support() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float4(this.e1, this.e2, this.e3, 0.0).xyz * float3(-1.0)
        );
    }
}
extension Scalar {
    func div(_rhs: anti_support) -> 
Horizon {
        this.anti_support()
    }
}
extension Scalar: AntiSupport {
    func anti_support() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.scalar
        );
    }
}
