public extension DualNum {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<DualNum> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> DualNum.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension DualNum: ProjectViaOriginOnto<DualNum> {
    public typedef DualNum ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
public extension DualNum: ProjectViaOriginOnto<Flector> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> Motor {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
public extension DualNum: ProjectViaOriginOnto<Horizon> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension DualNum: ProjectViaOriginOnto<Line> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Line) -> Scalar {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43)
        );
    }
}
public extension DualNum: ProjectViaOriginOnto<Motor> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Motor) -> Motor {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar)) * float4(right_dual.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
public extension DualNum: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((wedge.group3.xyz.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * other.group2.xyz) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * other.group3.xyz) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
public extension DualNum: ProjectViaOriginOnto<Plane> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension DualNum: ProjectViaOriginOnto<Point> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Point) -> Scalar {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4)
        );
    }
}
public extension DualNum: ProjectViaOriginOnto<Scalar> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<Flector> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Flector.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension Flector: ProjectViaOriginOnto<Flector> {
    public typedef Flector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> Flector {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
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
public extension Flector: ProjectViaOriginOnto<Horizon> {
    public typedef Flector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Horizon) -> Flector {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * wedge.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * wedge.e1234)
        );
    }
}
public extension Flector: ProjectViaOriginOnto<Line> {
    public typedef Point ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Line) -> Point {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.xyz.yzx * this.group0.zxy), 0.0) - float4((right_dual.group0.xyz.zxy * this.group0.yzx), 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
    }
}
public extension Flector: ProjectViaOriginOnto<Motor> {
    public typedef Flector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Motor) -> Flector {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), 0.0) + float4((float3(right_dual.scalar) * this.group1.xyz), 0.0) + float4((this.group0.zxy * right_dual.group0.yzx), 0.0) - float4((this.group0.yzx * right_dual.group0.zxy), 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(wedge.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Flector: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (this.group0.yzx * right_dual.group1.zxy) - (this.group0.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group3.xyz), 0.0) + float4((float3(right_dual.scalar) * this.group1.xyz), 0.0) + float4((right_dual.group2.xyz.yzx * this.group0.zxy), 0.0) - float4((right_dual.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((wedge.group3.xyz.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * other.group2.xyz) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * other.group3.xyz) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
public extension Flector: ProjectViaOriginOnto<Plane> {
    public typedef Flector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Plane) -> Flector {
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
public extension Flector: ProjectViaOriginOnto<Point> {
    public typedef Point ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Point) -> Point {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321)) * other.group0
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<Horizon> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Horizon.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension Horizon: ProjectViaOriginOnto<Flector> {
    public typedef Flector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> Flector {
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
public extension Horizon: ProjectViaOriginOnto<Horizon> {
    public typedef Horizon ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
public extension Horizon: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            float4(float3(0.0), other.e321 * -1.0)[3] * this.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(wedge.e1234) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(wedge.e1234) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group4
        );
    }
}
public extension Horizon: ProjectViaOriginOnto<Plane> {
    public typedef Plane ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<Line> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<Line> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Line.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension Line: ProjectViaOriginOnto<Flector> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> Motor {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((right_dual.e3 * this.e42) + (right_dual.e4 * this.e23), (right_dual.e1 * this.e43) + (right_dual.e4 * this.e31), (right_dual.e2 * this.e41) + (right_dual.e4 * this.e12), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group0.yzx) - (other.group1.yzx * wedge.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * wedge.e423 * -1.0, other.e321 * wedge.e431 * -1.0, other.e321 * wedge.e412 * -1.0, (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321)) + (wedge.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
public extension Line: ProjectViaOriginOnto<Horizon> {
    public typedef Line ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Horizon) -> Line {
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
public extension Line: ProjectViaOriginOnto<Line> {
    public typedef Line ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Line) -> Line {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge.e1234) * other.group1.xyz
        );
    }
}
public extension Line: ProjectViaOriginOnto<Motor> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Motor) -> Motor {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(wedge.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(wedge.e1234) * other.group1
        );
    }
}
public extension Line: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * right_dual.e3) + (this.e23 * right_dual.e4), (this.e43 * right_dual.e1) + (this.e31 * right_dual.e4), (this.e41 * right_dual.e2) + (this.e12 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (right_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((wedge.group3.xyz.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * other.group2.xyz) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * other.group3.xyz) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
public extension Line: ProjectViaOriginOnto<Plane> {
    public typedef Line ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Plane) -> Line {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge.group0.yzx) - (other.group0.yzx * wedge.group0.zxy), 
            /* e23, e31, e12 */
            (float3(wedge.e321) * other.group0.xyz) - (float3(other.e321) * wedge.group0.xyz)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<Motor> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Motor.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension Motor: ProjectViaOriginOnto<DualNum> {
    public typedef DualNum ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
public extension Motor: ProjectViaOriginOnto<Flector> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> Motor {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
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
public extension Motor: ProjectViaOriginOnto<Horizon> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Horizon) -> Motor {
        Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(wedge.group1.xyz, wedge.e4) * float4(-1.0)
        );
    }
}
public extension Motor: ProjectViaOriginOnto<Line> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Line) -> Motor {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.scalar) * right_dual.group0.xyz), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e1234) * other.group1.xyz), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
public extension Motor: ProjectViaOriginOnto<Motor> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Motor) -> Motor {
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
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
public extension Motor: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e1234 * right_dual.scalar) + (this.scalar * right_dual.e1234) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e41, e42, e43 */
            (float3(this.scalar) * right_dual.group2.xyz) + (float3(right_dual.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * right_dual.e4) + (this.scalar * right_dual.e423), (this.e31 * right_dual.e4) + (this.scalar * right_dual.e431), (this.e12 * right_dual.e4) + (this.scalar * right_dual.e412), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + float4((this.group0.yzx * right_dual.group1.zxy), this.scalar * right_dual.e321) - (right_dual.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((wedge.group3.xyz.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * other.group2.xyz) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * other.group3.xyz) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
public extension Motor: ProjectViaOriginOnto<Plane> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Plane) -> Motor {
        Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((wedge.group1.yzx * other.group0.zxy) - (wedge.group1.zxy * other.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge.e321 * other.e423, wedge.e321 * other.e431, wedge.e321 * other.e412, -(wedge.e2 * other.e431) - (wedge.e3 * other.e412) - (wedge.e4 * other.e321)) - (other.group0.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
public extension Motor: ProjectViaOriginOnto<Point> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Point) -> Scalar {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4)
        );
    }
}
public extension Motor: ProjectViaOriginOnto<Scalar> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<MultiVector> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> MultiVector.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension MultiVector: ProjectViaOriginOnto<DualNum> {
    public typedef DualNum ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
public extension MultiVector: ProjectViaOriginOnto<Flector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> MultiVector {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
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
            float4((right_dual.e3 * this.e42) + (right_dual.e4 * this.e23), (right_dual.e1 * this.e43) + (right_dual.e4 * this.e31), (right_dual.e2 * this.e41) + (right_dual.e4 * this.e12), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + (float4(this.scalar) * right_dual.group1) - (right_dual.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(wedge.e1234) * other.group0) - (other.group1.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (other.group1.zxy * wedge.group4.yzx) - (other.group1.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(wedge.e321) * other.group1.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
public extension MultiVector: ProjectViaOriginOnto<Horizon> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Horizon) -> MultiVector {
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
            float4((float3(right_dual.e4) * this.group3.xyz), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321 * wedge.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * wedge.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * wedge.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * wedge.e1234)
        );
    }
}
public extension MultiVector: ProjectViaOriginOnto<Line> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Line) -> MultiVector {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.xyz.yzx * this.group1.zxy), 0.0) - float4((right_dual.group0.xyz.zxy * this.group1.yzx), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group4.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge.e1234) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: ProjectViaOriginOnto<Motor> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Motor) -> MultiVector {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_dual.e1234 * this.scalar) + (right_dual.scalar * this.e1234) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (float3(right_dual.scalar) * this.group2.xyz) + (float3(this.scalar) * right_dual.group0.xyz), 
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
            (float3(other.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(wedge.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension MultiVector: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_dual.scalar * this.e1234) + (right_dual.e1234 * this.scalar) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(right_dual.scalar) * this.group1) + (float4(this.scalar) * right_dual.group1), 
            /* e41, e42, e43 */
            (float3(right_dual.scalar) * this.group2.xyz) + (float3(this.scalar) * right_dual.group2.xyz) + (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual.scalar) * this.group3.xyz) + (float3(this.scalar) * right_dual.group3.xyz) + (right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((right_dual.e3 * this.e42) + (right_dual.e4 * this.e23) + (right_dual.e42 * this.e3) + (right_dual.e23 * this.e4), (right_dual.e1 * this.e43) + (right_dual.e4 * this.e31) + (right_dual.e43 * this.e1) + (right_dual.e31 * this.e4), (right_dual.e2 * this.e41) + (right_dual.e4 * this.e12) + (right_dual.e41 * this.e2) + (right_dual.e12 * this.e4), -(right_dual.e1 * this.e23) - (right_dual.e2 * this.e31) - (right_dual.e3 * this.e12) - (right_dual.e12 * this.e3)) + (float4(right_dual.scalar) * this.group4) + (float4(this.scalar) * right_dual.group4) - (this.group1.yzxx * float4(right_dual.group2.xyz.zxy, right_dual.e23)) - float4((this.group2.xyz.zxy * right_dual.group1.yzx), right_dual.e31 * this.e2)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((wedge.group3.xyz.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * other.group2.xyz) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * other.group3.xyz) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
public extension MultiVector: ProjectViaOriginOnto<Plane> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Plane) -> MultiVector {
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
            float4((float3(right_dual.e4) * this.group3.xyz), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(wedge.e1 * other.e423) - (wedge.e2 * other.e431) - (wedge.e3 * other.e412) - (wedge.e4 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * other.e321) + (wedge.e31 * other.e412), (wedge.e42 * other.e321) + (wedge.e12 * other.e423), (wedge.e43 * other.e321) + (wedge.e23 * other.e431), -(wedge.e42 * other.e431) - (wedge.e43 * other.e412)) - (other.group0.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (wedge.group4.yzx * other.group0.zxy) - (wedge.group4.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (float3(wedge.e321) * other.group0.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group0
        );
    }
}
public extension MultiVector: ProjectViaOriginOnto<Point> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Point) -> MultiVector {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
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
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
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
public extension MultiVector: ProjectViaOriginOnto<Scalar> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<Origin> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Origin.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension Origin: ProjectViaOriginOnto<Flector> {
    public typedef Point ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> Point {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e431 * wedge.e42) - (other.e412 * wedge.e43)) - (other.group1.yzxx * float4(wedge.group1.xyz.zxy, wedge.e41))
        );
    }
}
public extension Origin: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(wedge.e41 * other.e23) - (wedge.e42 * other.e31) - (wedge.e43 * other.e12) - (wedge.e23 * other.e41) - (wedge.e31 * other.e42) - (wedge.e12 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * other.e321) + (wedge.e31 * other.e412), (wedge.e42 * other.e321) + (wedge.e12 * other.e423), (wedge.e43 * other.e321) + (wedge.e23 * other.e431), -(wedge.e42 * other.e431) - (wedge.e43 * other.e412)) - (other.group4.yzxx * float4(wedge.group1.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            float3(other.e1234) * wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<Plane> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Plane.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension Plane: ProjectViaOriginOnto<Flector> {
    public typedef Flector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> Flector {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
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
public extension Plane: ProjectViaOriginOnto<Horizon> {
    public typedef Horizon ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
public extension Plane: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            -(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(wedge.e1234) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(wedge.e1234) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group4
        );
    }
}
public extension Plane: ProjectViaOriginOnto<Plane> {
    public typedef Plane ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<Point> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<Point> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Point.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension Point: ProjectViaOriginOnto<Flector> {
    public typedef Flector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> Flector {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
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
public extension Point: ProjectViaOriginOnto<Horizon> {
    public typedef Point ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Horizon) -> Point {
        Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * float3(right_dual.e4 * this.e1 * -1.0, right_dual.e4 * this.e2 * -1.0, right_dual.e4 * this.e3 * -1.0)), 0.0)
        );
    }
}
public extension Point: ProjectViaOriginOnto<Line> {
    public typedef Point ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Line) -> Point {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.xyz.yzx * this.group0.zxy), 0.0) - float4((right_dual.group0.xyz.zxy * this.group0.yzx), 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
    }
}
public extension Point: ProjectViaOriginOnto<Motor> {
    public typedef Flector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Motor) -> Flector {
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.yzx * this.group0.zxy), 0.0) - float4((right_dual.group0.zxy * this.group0.yzx), 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(wedge.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Point: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_dual.group3.xyz), 0.0) + float4((right_dual.group2.xyz.yzx * this.group0.zxy), 0.0) - float4((right_dual.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((wedge.group3.xyz.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * other.group2.xyz) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * other.group3.xyz) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
public extension Point: ProjectViaOriginOnto<Plane> {
    public typedef Point ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Plane) -> Point {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * other.e321) + (wedge.e31 * other.e412), (wedge.e42 * other.e321) + (wedge.e12 * other.e423), (wedge.e43 * other.e321) + (wedge.e23 * other.e431), -(wedge.e42 * other.e431) - (wedge.e43 * other.e412)) - (other.group0.yzxx * float4(wedge.group1.xyz.zxy, wedge.e41))
        );
    }
}
public extension Point: ProjectViaOriginOnto<Point> {
    public typedef Point ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Point) -> Point {
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
public extension Scalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: ProjectViaOriginOntoInfix) -> ProjectViaOriginOntoInfixPartial<Scalar> {
        return ProjectViaOriginOntoInfixPartial(this);
    }
}
public extension ProjectViaOriginOntoInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Scalar.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
public extension Scalar: ProjectViaOriginOnto<DualNum> {
    public typedef DualNum ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar: ProjectViaOriginOnto<Flector> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Flector) -> Motor {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(this.scalar) * right_dual.group1.xyz), 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
public extension Scalar: ProjectViaOriginOnto<Horizon> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension Scalar: ProjectViaOriginOnto<Line> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Line) -> Scalar {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43)
        );
    }
}
public extension Scalar: ProjectViaOriginOnto<Motor> {
    public typedef Motor ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Motor) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
public extension Scalar: ProjectViaOriginOnto<MultiVector> {
    public typedef MultiVector ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: MultiVector) -> MultiVector {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, right_dual.e1234 * this.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.scalar) * right_dual.group4.xyz), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * wedge.e1234) + (other.e1234 * wedge.scalar) + (other.e1 * wedge.e423) + (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43) - (other.e423 * wedge.e1) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4), other.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412) + (other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e42 * wedge.e321) + (other.e12 * wedge.e423) + (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e43 * wedge.e321) + (other.e23 * wedge.e431) + (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e43 * wedge.e412) - (other.e423 * wedge.e41) - (other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) - (wedge.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((wedge.group3.xyz.zxy * other.group4.yzx), other.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * other.group2.xyz) + (other.group4.zxy * wedge.group4.yzx) - (other.group4.yzx * wedge.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * other.group3.xyz) + (float3(wedge.e321) * other.group4.xyz) - (float3(other.e321) * wedge.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * wedge.group4) + (float4(wedge.e1234) * other.group4)
        );
    }
}
public extension Scalar: ProjectViaOriginOnto<Plane> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension Scalar: ProjectViaOriginOnto<Point> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Point) -> Scalar {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4)
        );
    }
}
public extension Scalar: ProjectViaOriginOnto<Scalar> {
    public typedef Scalar ProjectViaOriginOntoOutput;
    public func project_via_origin_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
