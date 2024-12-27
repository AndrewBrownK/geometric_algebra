extension AntiScalar {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<AntiScalar> {
        reject_via_origin_from_partial(this)
    }
}
extension AntiScalar: RejectViaOriginFrom<DualNum> {
    func reject_via_origin_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * powi(other.scalar, 2)
        );
    }
}
extension AntiScalar: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> Motor {
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (anti_wedge.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)) + float4(float3(0.0), (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)) - (right_dual.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.yzx * right_dual.group0.zxy) - (anti_wedge.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiScalar: RejectViaOriginFrom<Horizon> {
    func reject_via_origin_from(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * powi(other.e321, 2)
        );
    }
}
extension AntiScalar: RejectViaOriginFrom<Line> {
    func reject_via_origin_from(other: Line) -> AntiScalar {
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1
        );
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)
        );
    }
}
extension AntiScalar: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Motor {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_dual.group0) + (float4(right_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), anti_wedge.scalar * right_dual.scalar)
        );
    }
}
extension AntiScalar: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0, 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * right_dual.scalar, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
extension AntiScalar: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * powi(other.e321, 2)
        );
    }
}
extension AntiScalar: RejectViaOriginFrom<Point> {
    func reject_via_origin_from(other: Point) -> AntiScalar {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (right_dual.e423 * anti_wedge.e1) + (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4)
        );
    }
}
extension AntiScalar: RejectViaOriginFrom<Scalar> {
    func reject_via_origin_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * powi(other.scalar, 2)
        );
    }
}
extension DualNum {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<DualNum> {
        reject_via_origin_from_partial(this)
    }
}
extension DualNum: RejectViaOriginFrom<DualNum> {
    func reject_via_origin_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (powi(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
extension DualNum: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> Motor {
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (anti_wedge.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)) + float4(float3(0.0), (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)) - (right_dual.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.yzx * right_dual.group0.zxy) - (anti_wedge.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension DualNum: RejectViaOriginFrom<Horizon> {
    func reject_via_origin_from(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * powi(other.e321, 2)
        );
    }
}
extension DualNum: RejectViaOriginFrom<Line> {
    func reject_via_origin_from(other: Line) -> AntiScalar {
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1
        );
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)
        );
    }
}
extension DualNum: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Motor {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e23, other.e31, other.e12, 1.0) * float4(this.group0.yy, this.e1234, (this.scalar * other.e1234) + (this.e1234 * other.scalar))
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_dual.group0) + (float4(right_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension DualNum: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
extension DualNum: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * powi(other.e321, 2)
        );
    }
}
extension DualNum: RejectViaOriginFrom<Point> {
    func reject_via_origin_from(other: Point) -> AntiScalar {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (right_dual.e423 * anti_wedge.e1) + (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4)
        );
    }
}
extension DualNum: RejectViaOriginFrom<Scalar> {
    func reject_via_origin_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * powi(other.scalar, 2)
        );
    }
}
extension Flector {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<Flector> {
        reject_via_origin_from_partial(this)
    }
}
extension Flector: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> Flector {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (other.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group1.wwwx * float4(other.group1.xyz, other.e1))
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_dual.e4 * anti_wedge.e23) + (right_dual.e423 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e31) + (right_dual.e431 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e12) + (right_dual.e412 * anti_wedge.scalar), -(right_dual.e2 * anti_wedge.e31) - (right_dual.e3 * anti_wedge.e12)) + float4((right_dual.group0.zxy * anti_wedge.group0.yzx), right_dual.e321 * anti_wedge.scalar) - (right_dual.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Flector: RejectViaOriginFrom<Horizon> {
    func reject_via_origin_from(other: Horizon) -> Flector {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_dual.e4), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(anti_wedge.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Flector: RejectViaOriginFrom<Line> {
    func reject_via_origin_from(other: Line) -> Plane {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41))
        );
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1), 0.0) + float4((right_dual.group0.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_dual.group0.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
extension Flector: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Plane {
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group1.xyz), 0.0) + float4((anti_wedge.group0.zxy * right_dual.group0.yzx), 0.0) - float4((anti_wedge.group0.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
extension Flector: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (this.group1.zxy * other.group4.yzx) - (this.group1.yzx * other.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group1.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
extension Flector: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> Flector {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_dual.e4), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(anti_wedge.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Flector: RejectViaOriginFrom<Point> {
    func reject_via_origin_from(other: Point) -> Plane {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar, anti_wedge.scalar, anti_wedge.scalar, 0.0) * float4(float4(other.e1, other.e2, other.e3, 0.0).xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<Horizon> {
        reject_via_origin_from_partial(this)
    }
}
extension Horizon: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> Flector {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_dual.e4 * anti_wedge.e23) + (right_dual.e423 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e31) + (right_dual.e431 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e12) + (right_dual.e412 * anti_wedge.scalar), -(right_dual.e2 * anti_wedge.e31) - (right_dual.e3 * anti_wedge.e12)) + float4((right_dual.group0.zxy * anti_wedge.group0.yzx), right_dual.e321 * anti_wedge.scalar) - (right_dual.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Horizon: RejectViaOriginFrom<Line> {
    func reject_via_origin_from(other: Line) -> Plane {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1), 0.0) + float4((right_dual.group0.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_dual.group0.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
extension Horizon: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Flector {
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(anti_wedge.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group1.xyz), 0.0) + float4((anti_wedge.group0.zxy * right_dual.group0.yzx), 0.0) - float4((anti_wedge.group0.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
extension Horizon: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group2, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * right_dual.scalar, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
extension Horizon: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> Plane {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(float3(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: RejectViaOriginFrom<Point> {
    func reject_via_origin_from(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e4 * -1.0) * float4(other.e1, other.e2, other.e3, 0.0)
        );
    }
}
extension Line {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<Line> {
        reject_via_origin_from_partial(this)
    }
}
extension Line: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> Motor {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41))
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e4 * anti_wedge.e1 * -1.0, right_dual.e4 * anti_wedge.e2 * -1.0, right_dual.e4 * anti_wedge.e3 * -1.0, (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4)) + (anti_wedge.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)), 
            /* e23, e31, e12, scalar */
            float4(((right_dual.group0.zxy * anti_wedge.group0.yzx) - (right_dual.group0.yzx * anti_wedge.group0.zxy)), 0.0)
        );
    }
}
extension Line: RejectViaOriginFrom<Horizon> {
    func reject_via_origin_from(other: Horizon) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * float4(other.e321 * this.e41, other.e321 * this.e42, other.e321 * this.e43, 0.0).xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Line: RejectViaOriginFrom<Line> {
    func reject_via_origin_from(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Line: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Motor {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e1234, other.e1234, other.e1234, 1.0) * float4(this.group1, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43))
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_dual.group0) + (float4(right_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Line: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
extension Line: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * float4((this.e41 * other.e321) + (this.e31 * other.e412) - (this.e12 * other.e431), (this.e42 * other.e321) + (this.e12 * other.e423) - (this.e23 * other.e412), (this.e43 * other.e321) + (this.e23 * other.e431) - (this.e31 * other.e423), -(this.e41 * other.e423) - (this.e42 * other.e431) - (this.e43 * other.e412)).xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Motor {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<Motor> {
        reject_via_origin_from_partial(this)
    }
}
extension Motor: RejectViaOriginFrom<DualNum> {
    func reject_via_origin_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (powi(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
extension Motor: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> Motor {
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (anti_wedge.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)) + float4(float3(0.0), (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)) - (right_dual.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.yzx * right_dual.group0.zxy) - (anti_wedge.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension Motor: RejectViaOriginFrom<Horizon> {
    func reject_via_origin_from(other: Horizon) -> Motor {
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(anti_wedge.group0.xyz, anti_wedge.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: RejectViaOriginFrom<Line> {
    func reject_via_origin_from(other: Line) -> Motor {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(other.group1, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.scalar, anti_wedge.scalar, anti_wedge.scalar, 1.0) * float4(right_dual.group0, -(right_dual.e41 * anti_wedge.e23) - (right_dual.e42 * anti_wedge.e31) - (right_dual.e43 * anti_wedge.e12) - (right_dual.e23 * anti_wedge.e41) - (right_dual.e31 * anti_wedge.e42) - (right_dual.e12 * anti_wedge.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Motor {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e1234) * other.group0.xyz)), other.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_dual.group0) + (float4(right_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * other.e1) + (this.e31 * other.e412), (this.e1234 * other.e2) + (this.e12 * other.e423), (this.e1234 * other.e3) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) + (this.group0 * float4(other.group4.www, other.e4)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group2) + (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group3) + (float3(other.e1234) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
extension Motor: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> Motor {
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(anti_wedge.group0.xyz, anti_wedge.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: RejectViaOriginFrom<Point> {
    func reject_via_origin_from(other: Point) -> AntiScalar {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (right_dual.e423 * anti_wedge.e1) + (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4)
        );
    }
}
extension Motor: RejectViaOriginFrom<Scalar> {
    func reject_via_origin_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * powi(other.scalar, 2)
        );
    }
}
extension MultiVector {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<MultiVector> {
        reject_via_origin_from_partial(this)
    }
}
extension MultiVector: RejectViaOriginFrom<DualNum> {
    func reject_via_origin_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (powi(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
extension MultiVector: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (other.group1.yzx * this.group4.zxy) - (other.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_dual.e423 * anti_wedge.e1) + (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4) - (right_dual.e1 * anti_wedge.e423) - (right_dual.e2 * anti_wedge.e431) - (right_dual.e3 * anti_wedge.e412) - (right_dual.e4 * anti_wedge.e321)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e41, e42, e43 */
            (float3(anti_wedge.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (right_dual.group0.zxy * anti_wedge.group1.yzx) - (right_dual.group0.yzx * anti_wedge.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((right_dual.e3 * anti_wedge.e42) + (right_dual.e4 * anti_wedge.e23), (right_dual.e1 * anti_wedge.e43) + (right_dual.e4 * anti_wedge.e31), (right_dual.e2 * anti_wedge.e41) + (right_dual.e4 * anti_wedge.e12), -(right_dual.e2 * anti_wedge.e31) - (right_dual.e3 * anti_wedge.e12)) + (float4(anti_wedge.scalar) * right_dual.group1) - (right_dual.group0.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23))
        );
    }
}
extension MultiVector: RejectViaOriginFrom<Horizon> {
    func reject_via_origin_from(other: Horizon) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group2, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group4.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, anti_wedge.e321 * right_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_dual.e4), 
            /* e41, e42, e43 */
            float3(right_dual.e4) * anti_wedge.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(anti_wedge.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: RejectViaOriginFrom<Line> {
    func reject_via_origin_from(other: Line) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(right_dual.e41 * anti_wedge.e23) - (right_dual.e42 * anti_wedge.e31) - (right_dual.e43 * anti_wedge.e12) - (right_dual.e23 * anti_wedge.e41) - (right_dual.e31 * anti_wedge.e42) - (right_dual.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * right_dual.group0, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1), 0.0) + float4((right_dual.group0.yzx * anti_wedge.group1.zxy), 0.0) - float4((right_dual.group0.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
extension MultiVector: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e1234 * this.e1) + (other.e31 * this.e412), (other.e1234 * this.e2) + (other.e12 * this.e423), (other.e1234 * this.e3) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) + (other.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2) + (float3(this.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3) + (float3(this.e1234) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_dual.e1234 * anti_wedge.scalar) + (right_dual.scalar * anti_wedge.e1234) - (right_dual.e41 * anti_wedge.e23) - (right_dual.e42 * anti_wedge.e31) - (right_dual.e43 * anti_wedge.e12) - (right_dual.e23 * anti_wedge.e41) - (right_dual.e31 * anti_wedge.e42) - (right_dual.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (float3(right_dual.scalar) * anti_wedge.group2) + (float3(anti_wedge.scalar) * right_dual.group0.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.yzx * anti_wedge.group1.zxy), 0.0) - float4((right_dual.group0.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
extension MultiVector: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e43 * this.e412) - (other.e423 * this.e41) - (other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) - (this.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((this.group3.zxy * other.group4.yzx), other.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2) + (float3(this.e1234) * other.group2) + (other.group4.yzx * this.group4.zxy) - (other.group4.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3) + (float3(other.e321) * this.group4.xyz) + (float3(this.e1234) * other.group3) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e1234) * other.group4)
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
extension MultiVector: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * other.group0.yzx) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, anti_wedge.e321 * right_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_dual.e4), 
            /* e41, e42, e43 */
            float3(right_dual.e4) * anti_wedge.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(anti_wedge.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: RejectViaOriginFrom<Point> {
    func reject_via_origin_from(other: Point) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.group0.xx, anti_wedge.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: RejectViaOriginFrom<Scalar> {
    func reject_via_origin_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * powi(other.scalar, 2)
        );
    }
}
extension Origin {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<Origin> {
        reject_via_origin_from_partial(this)
    }
}
extension Origin: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> Flector {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * right_dual.group1
        );
    }
}
extension Origin: RejectViaOriginFrom<Horizon> {
    func reject_via_origin_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            powi(other.e321, 2) * this.e4 * -1.0
        );
    }
}
extension Origin: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Flector {
Origin anti_wedge = Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_dual.scalar * anti_wedge.e4), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.e4, anti_wedge.e4, anti_wedge.e4, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * right_dual.scalar, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_dual.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_dual.group3), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(right_dual.e4) * anti_wedge.group3), 0.0) + float4((anti_wedge.group2.yzx * right_dual.group1.zxy), 0.0) + float4((right_dual.group2.yzx * anti_wedge.group1.zxy), 0.0) - float4((anti_wedge.group2.zxy * right_dual.group1.yzx), 0.0) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
extension Origin: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * powi(other.e321, 2) * -1.0
        );
    }
}
extension Plane {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<Plane> {
        reject_via_origin_from_partial(this)
    }
}
extension Plane: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> Flector {
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_dual.e4 * anti_wedge.e23) + (right_dual.e423 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e31) + (right_dual.e431 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e12) + (right_dual.e412 * anti_wedge.scalar), -(right_dual.e2 * anti_wedge.e31) - (right_dual.e3 * anti_wedge.e12)) + float4((right_dual.group0.zxy * anti_wedge.group0.yzx), right_dual.e321 * anti_wedge.scalar) - (right_dual.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Plane: RejectViaOriginFrom<Horizon> {
    func reject_via_origin_from(other: Horizon) -> Plane {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(float3(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane: RejectViaOriginFrom<Line> {
    func reject_via_origin_from(other: Line) -> Plane {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41))
        );
Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1), 0.0) + float4((right_dual.group0.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_dual.group0.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
extension Plane: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Plane {
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group1.xyz), 0.0) + float4((anti_wedge.group0.zxy * right_dual.group0.yzx), 0.0) - float4((anti_wedge.group0.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
extension Plane: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
extension Plane: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> Plane {
Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(right_dual.e4, right_dual.e4, right_dual.e4, 0.0) * float4(float3((other.e321 * this.e423) - (other.e423 * this.e321), (other.e321 * this.e431) - (other.e431 * this.e321), (other.e321 * this.e412) - (other.e412 * this.e321)), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane: RejectViaOriginFrom<Point> {
    func reject_via_origin_from(other: Point) -> Plane {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar, anti_wedge.scalar, anti_wedge.scalar, 0.0) * float4(float4(other.e1, other.e2, other.e3, 0.0).xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<Point> {
        reject_via_origin_from_partial(this)
    }
}
extension Point: RejectViaOriginFrom<Flector> {
    func reject_via_origin_from(other: Flector) -> Flector {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar, anti_wedge.scalar, anti_wedge.scalar, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: RejectViaOriginFrom<Horizon> {
    func reject_via_origin_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            powi(other.e321, 2) * this.e4 * -1.0
        );
    }
}
extension Point: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Flector {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_dual.group0.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
extension Point: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3) + (float3(right_dual.scalar) * anti_wedge.group3) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_dual.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_dual.group3), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(right_dual.e4) * anti_wedge.group3), 0.0) + float4((anti_wedge.group2.yzx * right_dual.group1.zxy), 0.0) + float4((right_dual.group2.yzx * anti_wedge.group1.zxy), 0.0) - float4((anti_wedge.group2.zxy * right_dual.group1.yzx), 0.0) - float4((right_dual.group2.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
extension Point: RejectViaOriginFrom<Plane> {
    func reject_via_origin_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            -(powi(other.e321, 2) * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
extension Scalar {
    func div(_rhs: reject_via_origin_from) -> reject_via_origin_from_partial<Scalar> {
        reject_via_origin_from_partial(this)
    }
}
extension Scalar: RejectViaOriginFrom<DualNum> {
    func reject_via_origin_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.scalar * other.e1234 * this.scalar
        );
    }
}
extension Scalar: RejectViaOriginFrom<Motor> {
    func reject_via_origin_from(other: Motor) -> Motor {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e23, e31, e12, scalar */
            float4(anti_wedge.scalar) * right_dual.group1
        );
    }
}
extension Scalar: RejectViaOriginFrom<MultiVector> {
    func reject_via_origin_from(other: MultiVector) -> MultiVector {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar) * right_dual.group0, 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * right_dual.group2, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * right_dual.group3, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * right_dual.group4
        );
    }
}
