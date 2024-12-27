extension AntiScalar {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<AntiScalar> {
        project_via_origin_onto_partial(this)
    }
}
extension AntiScalar: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Motor {
AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * 0.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(wedge.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(wedge.e1234) * other.group1
        );
    }
}
extension AntiScalar: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * 0.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(wedge.e1234) * other.group0, 
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group2, 
            /* e23, e31, e12 */
            float3(wedge.e1234) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group4
        );
    }
}
extension DualNum {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<DualNum> {
        project_via_origin_onto_partial(this)
    }
}
extension DualNum: ProjectViaOriginOnto<DualNum> {
    func project_via_origin_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0
        );
    }
}
extension DualNum: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> Motor {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * right_dual.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
extension DualNum: ProjectViaOriginOnto<Horizon> {
    func project_via_origin_onto(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            powi(other.e321, 2) * this.scalar
        );
    }
}
extension DualNum: ProjectViaOriginOnto<Line> {
    func project_via_origin_onto(other: Line) -> Scalar {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group1
        );
        return Scalar.from_groups(
            /* scalar */
            -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43)
        );
    }
}
extension DualNum: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Motor {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e41, right_dual.e42, right_dual.e43, 1.0) * float4(this.group0.xx, this.scalar, (this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar)), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_dual.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
extension DualNum: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * right_dual.scalar, (this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group2, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group3, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * right_dual.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group3) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
extension DualNum: ProjectViaOriginOnto<Plane> {
    func project_via_origin_onto(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            powi(other.e321, 2) * this.scalar
        );
    }
}
extension DualNum: ProjectViaOriginOnto<Point> {
    func project_via_origin_onto(other: Point) -> Scalar {
Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4)
        );
    }
}
extension DualNum: ProjectViaOriginOnto<Scalar> {
    func project_via_origin_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) * this.scalar
        );
    }
}
extension Flector {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<Flector> {
        project_via_origin_onto_partial(this)
    }
}
extension Flector: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> Flector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (this.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)) - (right_dual.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy)), 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(wedge.e1234) * other.group0) - (other.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
extension Flector: ProjectViaOriginOnto<Horizon> {
    func project_via_origin_onto(other: Horizon) -> Flector {
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(wedge.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * wedge.e1234)
        );
    }
}
extension Flector: ProjectViaOriginOnto<Line> {
    func project_via_origin_onto(other: Line) -> Point {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1), 0.0) + float4((right_dual.group0.yzx * this.group0.zxy), 0.0) - float4((right_dual.group0.zxy * this.group0.yzx), 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41))
        );
    }
}
extension Flector: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Flector {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), 0.0) + float4((float3(right_dual.scalar) * this.group1.xyz), 0.0) + float4((this.group0.zxy * right_dual.group0.yzx), 0.0) - float4((this.group0.yzx * right_dual.group0.zxy), 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * other.e31) + (wedge.e321 * other.e41), (wedge.e423 * other.e12) + (wedge.e321 * other.e42), (wedge.e431 * other.e23) + (wedge.e321 * other.e43), -(wedge.e431 * other.e42) - (wedge.e412 * other.e43)) + (float4(other.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * wedge.group1
        );
    }
}
extension Flector: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (this.group0.yzx * right_dual.group1.zxy) - (this.group0.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group3), 0.0) + float4((float3(right_dual.scalar) * this.group1.xyz), 0.0) + float4((right_dual.group2.yzx * this.group0.zxy), 0.0) - float4((right_dual.group2.zxy * this.group0.yzx), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group3) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
extension Flector: ProjectViaOriginOnto<Plane> {
    func project_via_origin_onto(other: Plane) -> Flector {
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * other.e321) + (wedge.e31 * other.e412), (wedge.e42 * other.e321) + (wedge.e12 * other.e423), (wedge.e43 * other.e321) + (wedge.e23 * other.e431), -(wedge.e42 * other.e431) - (wedge.e43 * other.e412)) - (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group0
        );
    }
}
extension Flector: ProjectViaOriginOnto<Point> {
    func project_via_origin_onto(other: Point) -> Point {
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321)) * other.group0
        );
    }
}
extension Horizon {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<Horizon> {
        project_via_origin_onto_partial(this)
    }
}
extension Horizon: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> Flector {
AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            float4(float3(0.0), other.e321 * -1.0)[3] * this.e321 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
extension Horizon: ProjectViaOriginOnto<Horizon> {
    func project_via_origin_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            powi(other.e321, 2) * this.e321
        );
    }
}
extension Horizon: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Flector {
Horizon wedge = Horizon.from_groups(
            /* e321 */
            this.e321 * 0.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e321, wedge.e321, wedge.e321, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge.e321 * other.e1234)
        );
    }
}
extension Horizon: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e321 * right_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_dual.scalar)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group3) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
extension Horizon: ProjectViaOriginOnto<Plane> {
    func project_via_origin_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
extension Line {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<Line> {
        project_via_origin_onto_partial(this)
    }
}
extension Line: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> Motor {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((right_dual.e3 * this.e42) + (right_dual.e4 * this.e23), (right_dual.e1 * this.e43) + (right_dual.e4 * this.e31), (right_dual.e2 * this.e41) + (right_dual.e4 * this.e12), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group0.yzx) - (other.group1.yzx * wedge.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * wedge.e423 * -1.0, other.e321 * wedge.e431 * -1.0, other.e321 * wedge.e412 * -1.0, (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321)) + (wedge.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
extension Line: ProjectViaOriginOnto<Horizon> {
    func project_via_origin_onto(other: Horizon) -> Line {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * float4(this.e23 * right_dual.e4, this.e31 * right_dual.e4, this.e12 * right_dual.e4, 0.0).xyz * float3(-1.0)
        );
    }
}
extension Line: ProjectViaOriginOnto<Line> {
    func project_via_origin_onto(other: Line) -> Line {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0, 
            /* e23, e31, e12 */
            float3(wedge.e1234) * other.group1
        );
    }
}
extension Line: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Motor {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual.scalar, right_dual.scalar, right_dual.scalar, 1.0) * float4(this.group0, -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group1, 0.0) * float4(right_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
extension Line: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0, 
            /* e23, e31, e12 */
            float3(right_dual.scalar) * this.group1, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * right_dual.e3) + (this.e23 * right_dual.e4), (this.e43 * right_dual.e1) + (this.e31 * right_dual.e4), (this.e41 * right_dual.e2) + (this.e12 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (right_dual.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group3) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
extension Line: ProjectViaOriginOnto<Plane> {
    func project_via_origin_onto(other: Plane) -> Line {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(this.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge.group0.yzx) - (other.group0.yzx * wedge.group0.zxy), 
            /* e23, e31, e12 */
            (float3(wedge.e321) * other.group0.xyz) - (float3(other.e321) * wedge.group0.xyz)
        );
    }
}
extension Motor {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<Motor> {
        project_via_origin_onto_partial(this)
    }
}
extension Motor: ProjectViaOriginOnto<DualNum> {
    func project_via_origin_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0
        );
    }
}
extension Motor: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> Motor {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_dual.e4 * this.e23) + (right_dual.e423 * this.scalar), (right_dual.e4 * this.e31) + (right_dual.e431 * this.scalar), (right_dual.e4 * this.e12) + (right_dual.e412 * this.scalar), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + float4((right_dual.group0.zxy * this.group0.yzx), right_dual.e321 * this.scalar) - (right_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
extension Motor: ProjectViaOriginOnto<Horizon> {
    func project_via_origin_onto(other: Horizon) -> Motor {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_dual.e4), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(wedge.group1.xyz, wedge.e4) * float4(-1.0)
        );
    }
}
extension Motor: ProjectViaOriginOnto<Line> {
    func project_via_origin_onto(other: Line) -> Motor {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar, this.scalar, this.scalar, 1.0) * float4(right_dual.group0, -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(right_dual.group1, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge.e1234, wedge.e1234, wedge.e1234, 1.0) * float4(other.group1, -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
extension Motor: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Motor {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(right_dual.scalar) * this.group0) + (float4(this.scalar) * right_dual.group0) + float4(float3(0.0), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_dual.scalar * this.scalar)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
extension Motor: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * right_dual.scalar, (this.e1234 * right_dual.scalar) + (this.scalar * right_dual.e1234) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e41, e42, e43 */
            (float3(this.scalar) * right_dual.group2) + (float3(right_dual.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * right_dual.e4) + (this.scalar * right_dual.e423), (this.e31 * right_dual.e4) + (this.scalar * right_dual.e431), (this.e12 * right_dual.e4) + (this.scalar * right_dual.e412), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + float4((this.group0.yzx * right_dual.group1.zxy), this.scalar * right_dual.e321) - (right_dual.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group3) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
extension Motor: ProjectViaOriginOnto<Plane> {
    func project_via_origin_onto(other: Plane) -> Motor {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_dual.e4), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((wedge.group1.yzx * other.group0.zxy) - (wedge.group1.zxy * other.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge.e321 * other.e423, wedge.e321 * other.e431, wedge.e321 * other.e412, -(wedge.e2 * other.e431) - (wedge.e3 * other.e412) - (wedge.e4 * other.e321)) - (other.group0.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
extension Motor: ProjectViaOriginOnto<Point> {
    func project_via_origin_onto(other: Point) -> Scalar {
Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4)
        );
    }
}
extension Motor: ProjectViaOriginOnto<Scalar> {
    func project_via_origin_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) * this.scalar
        );
    }
}
extension MultiVector {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<MultiVector> {
        project_via_origin_onto_partial(this)
    }
}
extension MultiVector: ProjectViaOriginOnto<DualNum> {
    func project_via_origin_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0
        );
    }
}
extension MultiVector: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> MultiVector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (right_dual.group0.zxy * this.group1.yzx) - (right_dual.group0.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((right_dual.e3 * this.e42) + (right_dual.e4 * this.e23), (right_dual.e1 * this.e43) + (right_dual.e4 * this.e31), (right_dual.e2 * this.e41) + (right_dual.e4 * this.e12), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + (float4(this.scalar) * right_dual.group1) - (right_dual.group0.yzxx * float4(this.group2.zxy, this.e23))
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(wedge.e1234) * other.group0) - (other.group1.yzxx * float4(wedge.group3.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (other.group1.zxy * wedge.group4.yzx) - (other.group1.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(wedge.e321) * other.group1.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
extension MultiVector: ProjectViaOriginOnto<Horizon> {
    func project_via_origin_onto(other: Horizon) -> MultiVector {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e321 * right_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_dual.e4), 
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(this.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321 * wedge.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(wedge.group2, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * wedge.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * wedge.e1234)
        );
    }
}
extension MultiVector: ProjectViaOriginOnto<Line> {
    func project_via_origin_onto(other: Line) -> MultiVector {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group1, 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1), 0.0) + float4((right_dual.group0.yzx * this.group1.zxy), 0.0) - float4((right_dual.group0.zxy * this.group1.yzx), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0, 
            /* e23, e31, e12 */
            float3(wedge.e1234) * other.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> MultiVector {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_dual.scalar * this.scalar, (right_dual.e1234 * this.scalar) + (right_dual.scalar * this.e1234) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e41, e42, e43 */
            (float3(right_dual.scalar) * this.group2) + (float3(this.scalar) * right_dual.group0.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual.scalar) * this.group4.xyz), 0.0) + float4((float3(this.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.yzx * this.group1.zxy), 0.0) - float4((right_dual.group0.zxy * this.group1.yzx), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e1234 * wedge.scalar) + (other.scalar * wedge.e1234) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e1234 * wedge.e1) + (other.e31 * wedge.e412), (other.e1234 * wedge.e2) + (other.e12 * wedge.e423), (other.e1234 * wedge.e3) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) + (other.group0 * float4(wedge.group4.www, wedge.e4)) - (wedge.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * wedge.group4
        );
    }
}
extension MultiVector: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_dual.scalar * this.scalar, (right_dual.scalar * this.e1234) + (right_dual.e1234 * this.scalar) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(right_dual.scalar) * this.group1) + (float4(this.scalar) * right_dual.group1), 
            /* e41, e42, e43 */
            (float3(right_dual.scalar) * this.group2) + (float3(this.scalar) * right_dual.group2) + (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual.scalar) * this.group3) + (float3(this.scalar) * right_dual.group3) + (right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((right_dual.e3 * this.e42) + (right_dual.e4 * this.e23) + (right_dual.e42 * this.e3) + (right_dual.e23 * this.e4), (right_dual.e1 * this.e43) + (right_dual.e4 * this.e31) + (right_dual.e43 * this.e1) + (right_dual.e31 * this.e4), (right_dual.e2 * this.e41) + (right_dual.e4 * this.e12) + (right_dual.e41 * this.e2) + (right_dual.e12 * this.e4), -(right_dual.e1 * this.e23) - (right_dual.e2 * this.e31) - (right_dual.e3 * this.e12) - (right_dual.e12 * this.e3)) + (float4(right_dual.scalar) * this.group4) + (float4(this.scalar) * right_dual.group4) - (this.group1.yzxx * float4(right_dual.group2.zxy, right_dual.e23)) - float4((this.group2.zxy * right_dual.group1.yzx), right_dual.e31 * this.e2)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group3) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
extension MultiVector: ProjectViaOriginOnto<Plane> {
    func project_via_origin_onto(other: Plane) -> MultiVector {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e321 * right_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_dual.e4), 
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(this.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(wedge.e1 * other.e423) - (wedge.e2 * other.e431) - (wedge.e3 * other.e412) - (wedge.e4 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * other.e321) + (wedge.e31 * other.e412), (wedge.e42 * other.e321) + (wedge.e12 * other.e423), (wedge.e43 * other.e321) + (wedge.e23 * other.e431), -(wedge.e42 * other.e431) - (wedge.e43 * other.e412)) - (other.group0.yzxx * float4(wedge.group3.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (wedge.group4.yzx * other.group0.zxy) - (wedge.group4.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (float3(wedge.e321) * other.group0.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group0
        );
    }
}
extension MultiVector: ProjectViaOriginOnto<Point> {
    func project_via_origin_onto(other: Point) -> MultiVector {
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * right_dual.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: ProjectViaOriginOnto<Scalar> {
    func project_via_origin_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) * this.scalar
        );
    }
}
extension Origin {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<Origin> {
        project_via_origin_onto_partial(this)
    }
}
extension Origin: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> Flector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(right_dual.group0.xyz, right_dual.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(wedge.e1234) * other.group0) - (other.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
extension Origin: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Flector {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_dual.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * other.e31) + (wedge.e321 * other.e41), (wedge.e423 * other.e12) + (wedge.e321 * other.e42), (wedge.e431 * other.e23) + (wedge.e321 * other.e43), -(wedge.e431 * other.e42) - (wedge.e412 * other.e43)) + (float4(other.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * wedge.group1
        );
    }
}
extension Origin: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, right_dual.e321 * this.e4) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_dual.scalar * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * right_dual.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(right_dual.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: ProjectViaOriginOnto<Point> {
    func project_via_origin_onto(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4 * 0.0) * other.group0
        );
    }
}
extension Plane {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<Plane> {
        project_via_origin_onto_partial(this)
    }
}
extension Plane: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> Flector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            -(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
extension Plane: ProjectViaOriginOnto<Horizon> {
    func project_via_origin_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            powi(other.e321, 2) * this.e321
        );
    }
}
extension Plane: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Plane {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * float4(right_dual.scalar * this.e423, right_dual.scalar * this.e431, right_dual.scalar * this.e412, right_dual.scalar * this.e321)
        );
    }
}
extension Plane: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.scalar) * this.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group3) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
extension Plane: ProjectViaOriginOnto<Plane> {
    func project_via_origin_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
extension Point {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<Point> {
        project_via_origin_onto_partial(this)
    }
}
extension Point: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> Flector {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e4 * this.e1 * -1.0, right_dual.e4 * this.e2 * -1.0, right_dual.e4 * this.e3 * -1.0, (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)) + (this.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)), 
            /* e23, e31, e12, scalar */
            float4(((right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy)), 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(wedge.e1234) * other.group0) - (other.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
extension Point: ProjectViaOriginOnto<Horizon> {
    func project_via_origin_onto(other: Horizon) -> Point {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(float3(right_dual.e4 * this.e1 * -1.0, right_dual.e4 * this.e2 * -1.0, right_dual.e4 * this.e3 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: ProjectViaOriginOnto<Line> {
    func project_via_origin_onto(other: Line) -> Point {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1), 0.0) + float4((right_dual.group0.yzx * this.group0.zxy), 0.0) - float4((right_dual.group0.zxy * this.group0.yzx), 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41))
        );
    }
}
extension Point: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Flector {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.yzx * this.group0.zxy), 0.0) - float4((right_dual.group0.zxy * this.group0.yzx), 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * other.e31) + (wedge.e321 * other.e41), (wedge.e423 * other.e12) + (wedge.e321 * other.e42), (wedge.e431 * other.e23) + (wedge.e321 * other.e43), -(wedge.e431 * other.e42) - (wedge.e412 * other.e43)) + (float4(other.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * wedge.group1
        );
    }
}
extension Point: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group3), 0.0) + float4((right_dual.group2.yzx * this.group0.zxy), 0.0) - float4((right_dual.group2.zxy * this.group0.yzx), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group3) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
extension Point: ProjectViaOriginOnto<Plane> {
    func project_via_origin_onto(other: Plane) -> Point {
Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * other.e321) + (wedge.e31 * other.e412), (wedge.e42 * other.e321) + (wedge.e12 * other.e423), (wedge.e43 * other.e321) + (wedge.e23 * other.e431), -(wedge.e42 * other.e431) - (wedge.e43 * other.e412)) - (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e41))
        );
    }
}
extension Point: ProjectViaOriginOnto<Point> {
    func project_via_origin_onto(other: Point) -> Point {
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
extension Scalar {
    func div(_rhs: project_via_origin_onto) -> project_via_origin_onto_partial<Scalar> {
        project_via_origin_onto_partial(this)
    }
}
extension Scalar: ProjectViaOriginOnto<DualNum> {
    func project_via_origin_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0
        );
    }
}
extension Scalar: ProjectViaOriginOnto<Flector> {
    func project_via_origin_onto(other: Flector) -> Motor {
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * right_dual.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
extension Scalar: ProjectViaOriginOnto<Horizon> {
    func project_via_origin_onto(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            powi(other.e321, 2) * this.scalar
        );
    }
}
extension Scalar: ProjectViaOriginOnto<Line> {
    func project_via_origin_onto(other: Line) -> Scalar {
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group1
        );
        return Scalar.from_groups(
            /* scalar */
            -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43)
        );
    }
}
extension Scalar: ProjectViaOriginOnto<Motor> {
    func project_via_origin_onto(other: Motor) -> Motor {
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * right_dual.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_dual.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
extension Scalar: ProjectViaOriginOnto<MultiVector> {
    func project_via_origin_onto(other: MultiVector) -> MultiVector {
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
MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * right_dual.group0, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group2, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group3, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * right_dual.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((wedge.group3.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2) + (float3(wedge.e1234) * other.group2) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3) + (float3(wedge.e1234) * other.group3) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
extension Scalar: ProjectViaOriginOnto<Plane> {
    func project_via_origin_onto(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            powi(other.e321, 2) * this.scalar
        );
    }
}
extension Scalar: ProjectViaOriginOnto<Point> {
    func project_via_origin_onto(other: Point) -> Scalar {
Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4)
        );
    }
}
extension Scalar: ProjectViaOriginOnto<Scalar> {
    func project_via_origin_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) * this.scalar
        );
    }
}
