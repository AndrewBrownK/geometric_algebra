extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<AntiScalar> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Flector> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Flector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * right_dual.group1.xyz), 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Horizon> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Line> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Line) -> Output {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(-(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, this.e1234 * right_dual.e1234) * float2(0.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_dual.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * right_dual.group4.xyz), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Plane> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Point> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Point) -> Output {
        Plane anti_wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz), 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<DualNum> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension DualNum: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = DualNum;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        DualNum anti_wedge = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar)), 0.0, 0.0)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Flector> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Flector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Horizon> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Line> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Line) -> Output {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(-(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * right_dual.group0, 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar), this.e1234 * right_dual.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_dual.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Plane> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Point> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Point) -> Output {
        Plane anti_wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = DualNum;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar) * float2(right_dual.e1234 * this.scalar, right_dual.e1234 * this.e1234), 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Flector> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension Flector: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Flector> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Flector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1.yzx * this.group1.zxy) - (right_dual.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_dual.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)) - (this.group1.wwwx * float4(right_dual.group1.xyz, right_dual.e1))
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Horizon> {
    associatedtype Output = Horizon;
    func anti_project_via_horizon_onto(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Line> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: Line) -> Output {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e31) + (this.e321 * right_dual.e41), (this.e423 * right_dual.e12) + (this.e321 * right_dual.e42), (this.e431 * right_dual.e23) + (this.e321 * right_dual.e43), -(this.e431 * right_dual.e42) - (this.e412 * right_dual.e43)) - (this.group1.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e41))
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e31) + (this.e321 * right_dual.e41), (this.e423 * right_dual.e12) + (this.e321 * right_dual.e42), (this.e431 * right_dual.e23) + (this.e321 * right_dual.e43), -(this.e431 * right_dual.e42) - (this.e412 * right_dual.e43)) + (float4(right_dual.e1234) * this.group0) - (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e31) + (this.e321 * right_dual.e41), (this.e423 * right_dual.e12) + (this.e321 * right_dual.e42), (this.e431 * right_dual.e23) + (this.e321 * right_dual.e43), -(this.e431 * right_dual.e42) - (this.e412 * right_dual.e43)) + (float4(right_dual.e1234) * this.group0) - (this.group1.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)), 
            /* e41, e42, e43, 0 */
            float4((this.group1.zxy * right_dual.group4.yzx) - (this.group1.yzx * right_dual.group4.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e321) * this.group1.xyz) - (float3(this.e321) * right_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Plane> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Point> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Point) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * right_dual.group0.yzx) - (this.group1.yzx * right_dual.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * right_dual.e423 * -1.0, this.e321 * right_dual.e431 * -1.0, this.e321 * right_dual.e412 * -1.0, (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321)) + (right_dual.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Horizon> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension Horizon: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = Horizon;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Flector> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Flector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(right_dual.group1.xyz, right_dual.e4) * float4(-1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Horizon> {
    associatedtype Output = Horizon;
    func anti_project_via_horizon_onto(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Line> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: Line) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0)), 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_dual.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_dual.e1234)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(anti_wedge.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_dual.group2.xyz), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * right_dual.group4.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_dual.e1234)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Plane> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Point> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: Point) -> Output {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * other.group0.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.xyz.zxy, anti_wedge.e23))
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = Horizon;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Line> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension Line: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = Line;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e1234) * this.group1.xyz, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * anti_wedge.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * anti_wedge.group1.xyz, 0.0)
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<Flector> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Flector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) - (right_dual.group1.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * anti_wedge.e1, other.e4 * anti_wedge.e2, other.e4 * anti_wedge.e3, -(other.e431 * anti_wedge.e2) - (other.e412 * anti_wedge.e3) - (other.e321 * anti_wedge.e4)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.yzx * anti_wedge.group0.zxy) - (other.group0.zxy * anti_wedge.group0.yzx)), 0.0)
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<Line> {
    associatedtype Output = Line;
    func anti_project_via_horizon_onto(other: Line) -> Output {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(anti_wedge.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(anti_wedge.scalar) * other.group1.xyz, 0.0)
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e1234) * this.group1.xyz), -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group4.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e1234) * this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<Point> {
    associatedtype Output = Line;
    func anti_project_via_horizon_onto(other: Point) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(other.e4) * anti_wedge.group0.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy), 0.0)
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = Line;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e1234) * this.group1.xyz, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * anti_wedge.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * anti_wedge.group1.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Motor> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension Motor: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(right_dual.e1234) * this.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)) * float4(anti_wedge.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Flector> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Flector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) + (float4(this.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Horizon> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Line> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Line) -> Output {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(anti_wedge.scalar) * other.group0.xyz), -(other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(anti_wedge.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual.e1234) * this.group0.xyz) + (float3(this.e1234) * right_dual.group0.xyz)), right_dual.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_dual.e1234) * this.group1) + (float4(this.e1234) * right_dual.group1) + float4(float3(0.0), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e1234 * right_dual.scalar) + (this.scalar * right_dual.e1234) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), this.e1234 * right_dual.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * right_dual.e1) + (this.e31 * right_dual.e412), (this.e1234 * right_dual.e2) + (this.e12 * right_dual.e423), (this.e1234 * right_dual.e3) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) + (this.group0 * float4(right_dual.group4.www, right_dual.e4)) - (right_dual.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group2.xyz) + (float3(right_dual.e1234) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * right_dual.group3.xyz) + (float3(right_dual.e1234) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Plane> {
    associatedtype Output = AntiScalar;
    func anti_project_via_horizon_onto(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Point> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Point) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.e4 * other.e1 * -1.0, anti_wedge.e4 * other.e2 * -1.0, anti_wedge.e4 * other.e3 * -1.0, (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)) + (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(right_dual.e1234) * this.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<MultiVector> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(right_dual.e1234) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e1234) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e1234) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * anti_wedge.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * anti_wedge.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * anti_wedge.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group4
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Flector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: Flector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) + (float4(this.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((right_dual.group1.yzx * this.group4.zxy) - (right_dual.group1.zxy * this.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_dual.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (other.e1 * anti_wedge.e423) + (other.e2 * anti_wedge.e431) + (other.e3 * anti_wedge.e412) + (other.e4 * anti_wedge.e321) - (other.e423 * anti_wedge.e1) - (other.e431 * anti_wedge.e2) - (other.e412 * anti_wedge.e3) - (other.e321 * anti_wedge.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e41, e42, e43, 0 */
            float4((float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group0.yzx * anti_wedge.group1.zxy) - (other.group0.zxy * anti_wedge.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * anti_wedge.e42) + (other.e4 * anti_wedge.e23), (other.e1 * anti_wedge.e43) + (other.e4 * anti_wedge.e31), (other.e2 * anti_wedge.e41) + (other.e4 * anti_wedge.e12), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + (float4(anti_wedge.scalar) * other.group1) - (other.group0.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23))
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Horizon> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: Horizon) -> Output {
        Origin right_dual = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * right_dual.e4), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, other.e321 * anti_wedge.e4) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * anti_wedge.scalar)
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Line> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: Line) -> Output {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group4.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e41)), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(float3(anti_wedge.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(anti_wedge.scalar) * other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group1.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_dual.e1234 * this.scalar) + (right_dual.scalar * this.e1234) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), right_dual.e1234 * this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e1234 * this.e1) + (right_dual.e31 * this.e412), (right_dual.e1234 * this.e2) + (right_dual.e12 * this.e423), (right_dual.e1234 * this.e3) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) + (right_dual.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group2.xyz) + (float3(this.e1234) * right_dual.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e1234) * this.group3.xyz) + (float3(this.e1234) * right_dual.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * anti_wedge.scalar, (other.e1234 * anti_wedge.scalar) + (other.scalar * anti_wedge.e1234) - (other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * anti_wedge.group2.xyz) + (float3(anti_wedge.scalar) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * anti_wedge.group3.xyz) + (float3(anti_wedge.scalar) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e23 * anti_wedge.e4) + (other.scalar * anti_wedge.e423), (other.e31 * anti_wedge.e4) + (other.scalar * anti_wedge.e431), (other.e12 * anti_wedge.e4) + (other.scalar * anti_wedge.e412), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) + float4((other.group0.yzx * anti_wedge.group1.zxy), other.scalar * anti_wedge.e321) - (anti_wedge.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_dual.scalar * this.e1234) + (right_dual.e1234 * this.scalar) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), right_dual.e1234 * this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412) + (right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423) + (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431) + (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e43 * this.e412) - (right_dual.e423 * this.e41) - (right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) + (float4(right_dual.e1234) * this.group1) + (float4(this.e1234) * right_dual.group1) - (this.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((this.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * this.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group2.xyz) + (float3(this.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * this.group4.zxy) - (right_dual.group4.zxy * this.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e1234) * this.group3.xyz) + (float3(right_dual.e321) * this.group4.xyz) + (float3(this.e1234) * right_dual.group3.xyz) - (float3(this.e321) * right_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * this.group4) + (float4(this.e1234) * right_dual.group4)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Plane> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: Plane) -> Output {
        Origin right_dual = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * right_dual.e4), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group0
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Point> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: Point) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((this.group4.zxy * right_dual.group0.yzx) - (this.group4.yzx * right_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_dual.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e41, e42, e43, 0 */
            float4((float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((anti_wedge.group1.zxy * other.group0.yzx) - (anti_wedge.group1.yzx * other.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23))
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(right_dual.e1234) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e1234) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e1234) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar) * anti_wedge.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * anti_wedge.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * anti_wedge.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group4
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Origin> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension Origin: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = Origin;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Origin anti_wedge = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4((other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.e4, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * anti_wedge.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * other.group1.xyz), 0.0)
        );
    }
}
extension Origin: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        Origin anti_wedge = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.scalar * this.e4, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, other.e321 * anti_wedge.e4) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * anti_wedge.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(anti_wedge.e4) * other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * other.group3.xyz), 0.0)
        );
    }
}
extension Origin: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = Origin;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Plane> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension Plane: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e423, right_dual.e1234 * this.e431, right_dual.e1234 * this.e412, right_dual.e1234 * this.e321)
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Flector> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Flector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_dual.e321 * this.e423, right_dual.e321 * this.e431, right_dual.e321 * this.e412, -(right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)) - (this.group0.wwwx * float4(right_dual.group1.xyz, right_dual.e1))
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Horizon> {
    associatedtype Output = Horizon;
    func anti_project_via_horizon_onto(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Line> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: Line) -> Output {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e41))
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)), 
            /* e41, e42, e43, 0 */
            float4((right_dual.group4.yzx * this.group0.zxy) - (right_dual.group4.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Plane> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Point> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: Point) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_dual.group0.xyz), 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.xyz.zxy, anti_wedge.e23))
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = Plane;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e423, right_dual.e1234 * this.e431, right_dual.e1234 * this.e412, right_dual.e1234 * this.e321)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Point> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension Point: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = Point;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e1, right_dual.e1234 * this.e2, right_dual.e1234 * this.e3, right_dual.e1234 * this.e4)
        );
    }
}
extension Point: AntiProjectViaHorizonOnto<Flector> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Flector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group1
        );
    }
}
extension Point: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Point: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Point: AntiProjectViaHorizonOnto<Point> {
    associatedtype Output = Point;
    func anti_project_via_horizon_onto(other: Point) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)) * other.group0
        );
    }
}
extension Point: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = Point;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e1, right_dual.e1234 * this.e2, right_dual.e1234 * this.e3, right_dual.e1234 * this.e4)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Scalar> {
        return AntiProjectViaHorizonOntoInfixPartial { a: this };
    }
}
extension Scalar: AntiProjectViaHorizonOnto<DualNum> {
    associatedtype Output = DualNum;
    func anti_project_via_horizon_onto(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * this.scalar) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiProjectViaHorizonOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_via_horizon_onto(other: Motor) -> Output {
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.scalar, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(anti_wedge.scalar) * other.group1
        );
    }
}
extension Scalar: AntiProjectViaHorizonOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_via_horizon_onto(other: MultiVector) -> Output {
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(anti_wedge.scalar) * other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(anti_wedge.scalar) * other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(anti_wedge.scalar) * other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group4
        );
    }
}
extension Scalar: AntiProjectViaHorizonOnto<Scalar> {
    associatedtype Output = Scalar;
    func anti_project_via_horizon_onto(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
