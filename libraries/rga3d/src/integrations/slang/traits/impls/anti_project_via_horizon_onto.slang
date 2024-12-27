extension AntiScalar {
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<AntiScalar> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.scalar, 2) * this.e1234
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Flector> {
    func anti_project_via_horizon_onto(other: Flector) -> Motor {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_dual.group1
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
    func anti_project_via_horizon_onto(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Line> {
    func anti_project_via_horizon_onto(other: Line) -> AntiScalar {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_dual.group1
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> Motor {
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
            float4(this.e1234) * right_dual.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), anti_wedge.scalar * other.scalar)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<MultiVector> {
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * right_dual.group0, 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group2, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_dual.group3, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_dual.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Plane> {
    func anti_project_via_horizon_onto(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> AntiScalar {
Plane anti_wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * float4(other.e1, other.e2, other.e3, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)
        );
    }
}
extension AntiScalar: AntiProjectViaHorizonOnto<Scalar> {
    func anti_project_via_horizon_onto(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.scalar, 2) * this.e1234
        );
    }
}
extension DualNum {
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<DualNum> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension DualNum: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> DualNum {
DualNum anti_wedge = DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar))
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Flector> {
    func anti_project_via_horizon_onto(other: Flector) -> Motor {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_dual.group1
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
    func anti_project_via_horizon_onto(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Line> {
    func anti_project_via_horizon_onto(other: Line) -> AntiScalar {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_dual.group1
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> Motor {
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
            float4(right_dual.e23, right_dual.e31, right_dual.e12, 1.0) * float4(this.group0.yy, this.e1234, (this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar))
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
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar), this.e1234 * right_dual.e1234), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group2, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_dual.group3, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_dual.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Plane> {
    func anti_project_via_horizon_onto(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> AntiScalar {
Plane anti_wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * float4(other.e1, other.e2, other.e3, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)
        );
    }
}
extension DualNum: AntiProjectViaHorizonOnto<Scalar> {
    func anti_project_via_horizon_onto(other: Scalar) -> DualNum {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * float2(right_dual.e1234 * this.scalar, right_dual.e1234 * this.e1234)
        );
    }
}
extension Flector {
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<Flector> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension Flector: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> Flector {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
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
    func anti_project_via_horizon_onto(other: Flector) -> Flector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
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
    func anti_project_via_horizon_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            powi(other.e321, 2) * this.e321
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Line> {
    func anti_project_via_horizon_onto(other: Line) -> Plane {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e31) + (this.e321 * right_dual.e41), (this.e423 * right_dual.e12) + (this.e321 * right_dual.e42), (this.e431 * right_dual.e23) + (this.e321 * right_dual.e43), -(this.e431 * right_dual.e42) - (this.e412 * right_dual.e43)) - (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41))
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> Flector {
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
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e31) + (this.e321 * right_dual.e41), (this.e423 * right_dual.e12) + (this.e321 * right_dual.e42), (this.e431 * right_dual.e23) + (this.e321 * right_dual.e43), -(this.e431 * right_dual.e42) - (this.e412 * right_dual.e43)) + (float4(right_dual.e1234) * this.group0) - (this.group1.yzxx * float4(right_dual.group3.zxy, right_dual.e41)), 
            /* e41, e42, e43 */
            (this.group1.zxy * right_dual.group4.yzx) - (this.group1.yzx * right_dual.group4.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * this.group1.xyz) - (float3(this.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Plane> {
    func anti_project_via_horizon_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
extension Flector: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> Flector {
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
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
    func anti_project_via_horizon_onto(other: Scalar) -> Flector {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
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
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<Horizon> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension Horizon: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            powi(other.scalar, 2) * this.e321
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Flector> {
    func anti_project_via_horizon_onto(other: Flector) -> Flector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
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
    func anti_project_via_horizon_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            powi(other.e321, 2) * this.e321
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Line> {
    func anti_project_via_horizon_onto(other: Line) -> Plane {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> Flector {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_dual.e1234)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<MultiVector> {
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(right_dual.group2, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * right_dual.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_dual.e1234)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Plane> {
    func anti_project_via_horizon_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> Plane {
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * float4(other.e1, other.e2, other.e3, 0.0).xyz * float3(-1.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Horizon: AntiProjectViaHorizonOnto<Scalar> {
    func anti_project_via_horizon_onto(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            powi(other.scalar, 2) * this.e321
        );
    }
}
extension Line {
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<Line> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension Line: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> Line {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group1
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group1
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<Flector> {
    func anti_project_via_horizon_onto(other: Flector) -> Motor {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e41))
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
    func anti_project_via_horizon_onto(other: Line) -> Line {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group1
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> Motor {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_dual.e1234, right_dual.e1234, right_dual.e1234, 1.0) * float4(this.group1, -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43))
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
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> Line {
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41))
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge.group0.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)
        );
    }
}
extension Line: AntiProjectViaHorizonOnto<Scalar> {
    func anti_project_via_horizon_onto(other: Scalar) -> Line {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group1
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group1
        );
    }
}
extension Motor {
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<Motor> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension Motor: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> Motor {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(right_dual.e1234) * this.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.e41, anti_wedge.e42, anti_wedge.e43, 1.0) * float4(other.group0.xx, other.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Flector> {
    func anti_project_via_horizon_onto(other: Flector) -> Motor {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) + (float4(this.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_dual.group1
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
    func anti_project_via_horizon_onto(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Line> {
    func anti_project_via_horizon_onto(other: Line) -> Motor {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_dual.group1, -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.scalar, anti_wedge.scalar, anti_wedge.scalar, 1.0) * float4(other.group0, -(other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1, 0.0) * float4(anti_wedge.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> Motor {
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
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1234 * right_dual.scalar) + (this.scalar * right_dual.e1234) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), this.e1234 * right_dual.e1234), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * right_dual.e1) + (this.e31 * right_dual.e412), (this.e1234 * right_dual.e2) + (this.e12 * right_dual.e423), (this.e1234 * right_dual.e3) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) + (this.group0 * float4(right_dual.group4.www, right_dual.e4)) - (right_dual.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group2) + (float3(right_dual.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_dual.group3) + (float3(right_dual.e1234) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_dual.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Plane> {
    func anti_project_via_horizon_onto(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension Motor: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> Motor {
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_dual.group0
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
    func anti_project_via_horizon_onto(other: Scalar) -> Motor {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
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
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<MultiVector> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> MultiVector {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_dual.e1234) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group2, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * anti_wedge.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group2, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group3, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group4
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Flector> {
    func anti_project_via_horizon_onto(other: Flector) -> MultiVector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) + (float4(this.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (right_dual.group1.yzx * this.group4.zxy) - (right_dual.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_dual.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e1 * anti_wedge.e423) + (other.e2 * anti_wedge.e431) + (other.e3 * anti_wedge.e412) + (other.e4 * anti_wedge.e321) - (other.e423 * anti_wedge.e1) - (other.e431 * anti_wedge.e2) - (other.e412 * anti_wedge.e3) - (other.e321 * anti_wedge.e4)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.yzx * anti_wedge.group1.zxy) - (other.group0.zxy * anti_wedge.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * anti_wedge.e42) + (other.e4 * anti_wedge.e23), (other.e1 * anti_wedge.e43) + (other.e4 * anti_wedge.e31), (other.e2 * anti_wedge.e41) + (other.e4 * anti_wedge.e12), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + (float4(anti_wedge.scalar) * other.group1) - (other.group0.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23))
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Horizon> {
    func anti_project_via_horizon_onto(other: Horizon) -> MultiVector {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * right_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e321 * anti_wedge.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * anti_wedge.scalar)
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Line> {
    func anti_project_via_horizon_onto(other: Line) -> MultiVector {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group4.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_dual.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group1, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> MultiVector {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.e1234 * this.scalar) + (right_dual.scalar * this.e1234) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), right_dual.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e1234 * this.e1) + (right_dual.e31 * this.e412), (right_dual.e1234 * this.e2) + (right_dual.e12 * this.e423), (right_dual.e1234 * this.e3) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) + (right_dual.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group2) + (float3(this.e1234) * right_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * this.group3) + (float3(this.e1234) * right_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * anti_wedge.scalar, (other.e1234 * anti_wedge.scalar) + (other.scalar * anti_wedge.e1234) - (other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43 */
            (float3(other.scalar) * anti_wedge.group2) + (float3(anti_wedge.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * anti_wedge.group3) + (float3(anti_wedge.scalar) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4((other.e23 * anti_wedge.e4) + (other.scalar * anti_wedge.e423), (other.e31 * anti_wedge.e4) + (other.scalar * anti_wedge.e431), (other.e12 * anti_wedge.e4) + (other.scalar * anti_wedge.e412), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) + float4((other.group0.yzx * anti_wedge.group1.zxy), other.scalar * anti_wedge.e321) - (anti_wedge.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<MultiVector> {
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * this.e1234) + (right_dual.e1234 * this.scalar) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), right_dual.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412) + (right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423) + (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431) + (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e43 * this.e412) - (right_dual.e423 * this.e41) - (right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) + (float4(right_dual.e1234) * this.group1) + (float4(this.e1234) * right_dual.group1) - (this.group4.yzxx * float4(right_dual.group3.zxy, right_dual.e41)) - float4((this.group3.zxy * right_dual.group4.yzx), right_dual.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group2) + (float3(this.e1234) * right_dual.group2) + (right_dual.group4.yzx * this.group4.zxy) - (right_dual.group4.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * this.group3) + (float3(right_dual.e321) * this.group4.xyz) + (float3(this.e1234) * right_dual.group3) - (float3(this.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * this.group4) + (float4(this.e1234) * right_dual.group4)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Plane> {
    func anti_project_via_horizon_onto(other: Plane) -> MultiVector {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * right_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group0
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> MultiVector {
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * right_dual.group0.yzx) - (this.group4.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_dual.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_dual.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge.group1.zxy * other.group0.yzx) - (anti_wedge.group1.yzx * other.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23))
        );
    }
}
extension MultiVector: AntiProjectViaHorizonOnto<Scalar> {
    func anti_project_via_horizon_onto(other: Scalar) -> MultiVector {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_dual.e1234) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group2, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * anti_wedge.group0, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group2, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group3, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group4
        );
    }
}
extension Origin {
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<Origin> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension Origin: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            powi(other.scalar, 2) * this.e4
        );
    }
}
extension Origin: AntiProjectViaHorizonOnto<Flector> {
    func anti_project_via_horizon_onto(other: Flector) -> Flector {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            this.e4 * 0.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group1
        );
    }
}
extension Origin: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> Flector {
Origin anti_wedge = Origin.from_groups(
            /* e4 */
            (other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.e4
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * anti_wedge.e4), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.e4, anti_wedge.e4, anti_wedge.e4, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: AntiProjectViaHorizonOnto<MultiVector> {
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_dual.e321 * this.e4, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_dual.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Origin: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4 * 0.0) * other.group0
        );
    }
}
extension Origin: AntiProjectViaHorizonOnto<Scalar> {
    func anti_project_via_horizon_onto(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            powi(other.scalar, 2) * this.e4
        );
    }
}
extension Plane {
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<Plane> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension Plane: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> Plane {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e423, right_dual.e1234 * this.e431, right_dual.e1234 * this.e412, right_dual.e1234 * this.e321)
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Flector> {
    func anti_project_via_horizon_onto(other: Flector) -> Flector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
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
    func anti_project_via_horizon_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            powi(other.e321, 2) * this.e321
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Line> {
    func anti_project_via_horizon_onto(other: Line) -> Plane {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41))
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> Flector {
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
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_dual.group3.zxy, right_dual.e41)), 
            /* e41, e42, e43 */
            (right_dual.group4.yzx * this.group0.zxy) - (right_dual.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Plane> {
    func anti_project_via_horizon_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> Plane {
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            (right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_dual.group0.xyz)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Plane: AntiProjectViaHorizonOnto<Scalar> {
    func anti_project_via_horizon_onto(other: Scalar) -> Plane {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e423, right_dual.e1234 * this.e431, right_dual.e1234 * this.e412, right_dual.e1234 * this.e321)
        );
    }
}
extension Point {
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<Point> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension Point: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> Point {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e1, right_dual.e1234 * this.e2, right_dual.e1234 * this.e3, right_dual.e1234 * this.e4)
        );
    }
}
extension Point: AntiProjectViaHorizonOnto<Flector> {
    func anti_project_via_horizon_onto(other: Flector) -> Flector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)
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
    func anti_project_via_horizon_onto(other: Motor) -> Flector {
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
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Point: AntiProjectViaHorizonOnto<Point> {
    func anti_project_via_horizon_onto(other: Point) -> Point {
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)) * other.group0
        );
    }
}
extension Point: AntiProjectViaHorizonOnto<Scalar> {
    func anti_project_via_horizon_onto(other: Scalar) -> Point {
AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e1, right_dual.e1234 * this.e2, right_dual.e1234 * this.e3, right_dual.e1234 * this.e4)
        );
    }
}
extension Scalar {
    func div(_rhs: anti_project_via_horizon_onto) -> anti_project_via_horizon_onto_partial<Scalar> {
        anti_project_via_horizon_onto_partial(this)
    }
}
extension Scalar: AntiProjectViaHorizonOnto<DualNum> {
    func anti_project_via_horizon_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0
        );
    }
}
extension Scalar: AntiProjectViaHorizonOnto<Motor> {
    func anti_project_via_horizon_onto(other: Motor) -> Motor {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            (other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.scalar
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
    func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar) * other.group0, 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group2, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group4
        );
    }
}
extension Scalar: AntiProjectViaHorizonOnto<Scalar> {
    func anti_project_via_horizon_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) * this.scalar
        );
    }
}
