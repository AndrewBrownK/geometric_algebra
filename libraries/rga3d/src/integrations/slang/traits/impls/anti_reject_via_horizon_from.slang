implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<AntiScalar> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<AntiScalar>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension AntiScalar: AntiRejectViaHorizonFrom<DualNum> {
    public typedef AntiScalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension AntiScalar: AntiRejectViaHorizonFrom<Motor> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234 * other.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension AntiScalar: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, wedge.e1234 * right_dual.e1234) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(wedge.e1234) * right_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(wedge.e1234) * right_dual.group4.xyz), 0.0)
        );
    }
}
public extension AntiScalar: AntiRejectViaHorizonFrom<Scalar> {
    public typedef AntiScalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<DualNum> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<DualNum>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public DualNum operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension DualNum: AntiRejectViaHorizonFrom<DualNum> {
    public typedef DualNum AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
public extension DualNum: AntiRejectViaHorizonFrom<Flector> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> Motor {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1.yzx * wedge.group1.zxy) - (right_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_dual.group1.xyz, right_dual.e1))
        );
    }
}
public extension DualNum: AntiRejectViaHorizonFrom<Horizon> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
public extension DualNum: AntiRejectViaHorizonFrom<Line> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Line) -> Scalar {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            -(right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43)
        );
    }
}
public extension DualNum: AntiRejectViaHorizonFrom<Motor> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_dual.group0.xyz)), right_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) + float4(float3(0.0), -(right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43))
        );
    }
}
public extension DualNum: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(right_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group3.xyz) - (float3(wedge.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_dual.group4)
        );
    }
}
public extension DualNum: AntiRejectViaHorizonFrom<Plane> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
public extension DualNum: AntiRejectViaHorizonFrom<Point> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> Scalar {
        Point wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4)
        );
    }
}
public extension DualNum: AntiRejectViaHorizonFrom<Scalar> {
    public typedef DualNum AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * float2(this.scalar * other.scalar, this.e1234 * other.scalar)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<Flector> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<Flector>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Flector operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension Flector: AntiRejectViaHorizonFrom<DualNum> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> Flector {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * wedge.group1
        );
    }
}
public extension Flector: AntiRejectViaHorizonFrom<Flector> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> Flector {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Flector: AntiRejectViaHorizonFrom<Horizon> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
public extension Flector: AntiRejectViaHorizonFrom<Line> {
    public typedef Point AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Line) -> Point {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431), -(right_dual.e42 * wedge.e431) - (right_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e41))
        );
    }
}
public extension Flector: AntiRejectViaHorizonFrom<Motor> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Flector {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e4 * other.e23) + (this.e423 * other.scalar), (this.e4 * other.e31) + (this.e431 * other.scalar), (this.e4 * other.e12) + (this.e412 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group0.zxy * other.group0.yzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * right_dual.e31) + (wedge.e321 * right_dual.e41), (wedge.e423 * right_dual.e12) + (wedge.e321 * right_dual.e42), (wedge.e431 * right_dual.e23) + (wedge.e321 * right_dual.e43), -(wedge.e431 * right_dual.e42) - (wedge.e412 * right_dual.e43)) + (float4(right_dual.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * wedge.group1
        );
    }
}
public extension Flector: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(right_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group3.xyz) - (float3(wedge.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_dual.group4)
        );
    }
}
public extension Flector: AntiRejectViaHorizonFrom<Plane> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            -(pow(other.e321, 2) * this.e4) - (this.e1 * other.e423 * other.e321) - (this.e2 * other.e431 * other.e321) - (this.e3 * other.e412 * other.e321)
        );
    }
}
public extension Flector: AntiRejectViaHorizonFrom<Point> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> Flector {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * other.e1, this.e4 * other.e2, this.e4 * other.e3, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_dual.e321) + (wedge.e31 * right_dual.e412), (wedge.e42 * right_dual.e321) + (wedge.e12 * right_dual.e423), (wedge.e43 * right_dual.e321) + (wedge.e23 * right_dual.e431), -(wedge.e42 * right_dual.e431) - (wedge.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Flector: AntiRejectViaHorizonFrom<Scalar> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> Flector {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * wedge.group1
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<Horizon> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<Horizon>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Horizon operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension Horizon: AntiRejectViaHorizonFrom<DualNum> {
    public typedef Horizon AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            pow(other.scalar, 2) * this.e321
        );
    }
}
public extension Horizon: AntiRejectViaHorizonFrom<Flector> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> Flector {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(wedge.e1234) * right_dual.group1.xyz), 0.0)
        );
    }
}
public extension Horizon: AntiRejectViaHorizonFrom<Motor> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Flector {
        Horizon wedge = Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(wedge.e321) * right_dual.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge.e321 * right_dual.e1234)
        );
    }
}
public extension Horizon: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e321 * other.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.scalar)
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(right_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group3.xyz) - (float3(wedge.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_dual.group4)
        );
    }
}
public extension Horizon: AntiRejectViaHorizonFrom<Point> {
    public typedef Plane AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
public extension Horizon: AntiRejectViaHorizonFrom<Scalar> {
    public typedef Horizon AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * pow(other.scalar, 2)
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<Line> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<Line>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Line operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension Line: AntiRejectViaHorizonFrom<DualNum> {
    public typedef Line AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> Line {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(right_dual.e1234) * wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * wedge.group1.xyz
        );
    }
}
public extension Line: AntiRejectViaHorizonFrom<Flector> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> Motor {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1.yzx * wedge.group0.zxy) - (right_dual.group1.zxy * wedge.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_dual.e321 * wedge.e423, right_dual.e321 * wedge.e431, right_dual.e321 * wedge.e412, -(right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321)) - (wedge.group0.wwwx * float4(right_dual.group1.xyz, right_dual.e1))
        );
    }
}
public extension Line: AntiRejectViaHorizonFrom<Line> {
    public typedef Line AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Line: AntiRejectViaHorizonFrom<Motor> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_dual.group0.xyz)), right_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) + float4(float3(0.0), -(right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43))
        );
    }
}
public extension Line: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(right_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group3.xyz) - (float3(wedge.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_dual.group4)
        );
    }
}
public extension Line: AntiRejectViaHorizonFrom<Point> {
    public typedef Line AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> Line {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (right_dual.group0.yzx * wedge.group0.zxy) - (right_dual.group0.zxy * wedge.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * wedge.group0.xyz) - (float3(wedge.e321) * right_dual.group0.xyz)
        );
    }
}
public extension Line: AntiRejectViaHorizonFrom<Scalar> {
    public typedef Line AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> Line {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(right_dual.e1234) * wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * wedge.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<Motor> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<Motor>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Motor operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension Motor: AntiRejectViaHorizonFrom<DualNum> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e1234) * wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(right_dual.e1234) * wedge.group1
        );
    }
}
public extension Motor: AntiRejectViaHorizonFrom<Flector> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> Motor {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar), (other.e4 * this.e31) + (other.e431 * this.scalar), (other.e4 * this.e12) + (other.e412 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1.yzx * wedge.group1.zxy) - (right_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_dual.group1.xyz, right_dual.e1))
        );
    }
}
public extension Motor: AntiRejectViaHorizonFrom<Horizon> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension Motor: AntiRejectViaHorizonFrom<Line> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Line) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43))
        );
    }
}
public extension Motor: AntiRejectViaHorizonFrom<Motor> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.scalar)
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_dual.group0.xyz)), right_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) + float4(float3(0.0), -(right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43))
        );
    }
}
public extension Motor: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group2.xyz) + (float3(other.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * other.e4) + (this.scalar * other.e423), (this.e31 * other.e4) + (this.scalar * other.e431), (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(right_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group3.xyz) - (float3(wedge.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_dual.group4)
        );
    }
}
public extension Motor: AntiRejectViaHorizonFrom<Plane> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
public extension Motor: AntiRejectViaHorizonFrom<Point> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> Motor {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((wedge.group1.zxy * right_dual.group0.yzx) - (wedge.group1.yzx * right_dual.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge.e321 * right_dual.e423 * -1.0, wedge.e321 * right_dual.e431 * -1.0, wedge.e321 * right_dual.e412 * -1.0, (wedge.e2 * right_dual.e431) + (wedge.e3 * right_dual.e412) + (wedge.e4 * right_dual.e321)) + (right_dual.group0.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
public extension Motor: AntiRejectViaHorizonFrom<Scalar> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e1234) * wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(right_dual.e1234) * wedge.group1
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<MultiVector> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<MultiVector>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension MultiVector: AntiRejectViaHorizonFrom<DualNum> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        );
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_dual.e1234) * wedge.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * wedge.group1, 
            /* e41, e42, e43 */
            float3(right_dual.e1234) * wedge.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * wedge.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * wedge.group4
        );
    }
}
public extension MultiVector: AntiRejectViaHorizonFrom<Flector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (right_dual.group1.yzx * wedge.group4.zxy) - (right_dual.group1.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * wedge.group4.xyz) - (float3(wedge.e321) * right_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0.xy.yy, wedge.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension MultiVector: AntiRejectViaHorizonFrom<Horizon> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Horizon) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.scalar)
        );
        Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(wedge.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge.e1234 * right_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiRejectViaHorizonFrom<Line> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Line) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431), -(right_dual.e42 * wedge.e431) - (right_dual.e43 * wedge.e412)) - (wedge.group4.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e41)), 
            /* e41, e42, e43 */
            float3(wedge.e1234) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiRejectViaHorizonFrom<Motor> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4((other.e23 * this.e4) + (other.scalar * this.e423), (other.e31 * this.e4) + (other.scalar * this.e431), (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.yzx * this.group1.zxy), other.scalar * this.e321) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.e1234 * wedge.scalar) + (right_dual.scalar * wedge.e1234) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e1234 * wedge.e1) + (right_dual.e31 * wedge.e412), (right_dual.e1234 * wedge.e2) + (right_dual.e12 * wedge.e423), (right_dual.e1234 * wedge.e3) + (right_dual.e23 * wedge.e431), -(right_dual.e42 * wedge.e431) - (right_dual.e43 * wedge.e412)) + (right_dual.group0 * float4(wedge.group4.www, wedge.e4)) - (wedge.group4.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * right_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * wedge.group4
        );
    }
}
public extension MultiVector: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e42 * this.e3) + (other.e23 * this.e4), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e43 * this.e1) + (other.e31 * this.e4), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group4) + (float4(this.scalar) * other.group4) - (this.group1.yzxx * float4(other.group2.xyz.zxy, other.e23)) - float4((this.group2.xyz.zxy * other.group1.yzx), other.e31 * this.e2)
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(right_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group3.xyz) - (float3(wedge.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_dual.group4)
        );
    }
}
public extension MultiVector: AntiRejectViaHorizonFrom<Plane> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Plane) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
        Origin right_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(wedge.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge.e1234 * right_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiRejectViaHorizonFrom<Point> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((wedge.e1 * right_dual.e423) + (wedge.e2 * right_dual.e431) + (wedge.e3 * right_dual.e412) + (wedge.e4 * right_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_dual.e321) + (wedge.e31 * right_dual.e412), (wedge.e42 * right_dual.e321) + (wedge.e12 * right_dual.e423), (wedge.e43 * right_dual.e321) + (wedge.e23 * right_dual.e431), -(wedge.e42 * right_dual.e431) - (wedge.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (wedge.group4.zxy * right_dual.group0.yzx) - (wedge.group4.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * wedge.group4.xyz) - (float3(wedge.e321) * right_dual.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0.xy.yy, wedge.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension MultiVector: AntiRejectViaHorizonFrom<Scalar> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        );
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_dual.e1234) * wedge.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * wedge.group1, 
            /* e41, e42, e43 */
            float3(right_dual.e1234) * wedge.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * wedge.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * wedge.group4
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<Origin> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<Origin>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public Origin operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension Origin: AntiRejectViaHorizonFrom<DualNum> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.scalar, 2) * this.e4
        );
    }
}
public extension Origin: AntiRejectViaHorizonFrom<Flector> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> Flector {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Origin: AntiRejectViaHorizonFrom<Horizon> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
public extension Origin: AntiRejectViaHorizonFrom<Line> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Line) -> Origin {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Origin.from_groups(
            /* e4 */
            -(right_dual.e41 * wedge.e423) - (right_dual.e42 * wedge.e431) - (right_dual.e43 * wedge.e412)
        );
    }
}
public extension Origin: AntiRejectViaHorizonFrom<Motor> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Flector {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * right_dual.e31) + (wedge.e321 * right_dual.e41), (wedge.e423 * right_dual.e12) + (wedge.e321 * right_dual.e42), (wedge.e431 * right_dual.e23) + (wedge.e321 * right_dual.e43), -(wedge.e431 * right_dual.e42) - (wedge.e412 * right_dual.e43)) + (float4(right_dual.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.www, 0.0) * float4(wedge.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Origin: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group3.xyz), 0.0)
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(right_dual.e1234) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group4.xyz)), 0.0)
        );
    }
}
public extension Origin: AntiRejectViaHorizonFrom<Plane> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
public extension Origin: AntiRejectViaHorizonFrom<Point> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> Origin {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Origin.from_groups(
            /* e4 */
            -(wedge.e41 * right_dual.e423) - (wedge.e42 * right_dual.e431) - (wedge.e43 * right_dual.e412)
        );
    }
}
public extension Origin: AntiRejectViaHorizonFrom<Scalar> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * pow(other.scalar, 2)
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<Plane> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<Plane>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension Plane: AntiRejectViaHorizonFrom<DualNum> {
    public typedef Plane AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(other.scalar * this.e423, other.scalar * this.e431, other.scalar * this.e412, other.scalar * this.e321)
        );
    }
}
public extension Plane: AntiRejectViaHorizonFrom<Flector> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> Flector {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(wedge.e1234) * right_dual.group1.xyz), 0.0)
        );
    }
}
public extension Plane: AntiRejectViaHorizonFrom<Motor> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Flector {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431), -(right_dual.e42 * wedge.e431) - (right_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * wedge.group0
        );
    }
}
public extension Plane: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(right_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group3.xyz) - (float3(wedge.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_dual.group4)
        );
    }
}
public extension Plane: AntiRejectViaHorizonFrom<Point> {
    public typedef Plane AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
public extension Plane: AntiRejectViaHorizonFrom<Scalar> {
    public typedef Plane AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(this.e423 * other.scalar, this.e431 * other.scalar, this.e412 * other.scalar, this.e321 * other.scalar)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<Point> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<Point>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Point operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension Point: AntiRejectViaHorizonFrom<DualNum> {
    public typedef Point AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, other.scalar * this.e4)
        );
    }
}
public extension Point: AntiRejectViaHorizonFrom<Flector> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> Flector {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * this.e1 * -1.0, other.e4 * this.e2 * -1.0, other.e4 * this.e3 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Point: AntiRejectViaHorizonFrom<Horizon> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
public extension Point: AntiRejectViaHorizonFrom<Line> {
    public typedef Point AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Line) -> Point {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431), -(right_dual.e42 * wedge.e431) - (right_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e41))
        );
    }
}
public extension Point: AntiRejectViaHorizonFrom<Motor> {
    public typedef Flector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Flector {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * right_dual.e31) + (wedge.e321 * right_dual.e41), (wedge.e423 * right_dual.e12) + (wedge.e321 * right_dual.e42), (wedge.e431 * right_dual.e23) + (wedge.e321 * right_dual.e43), -(wedge.e431 * right_dual.e42) - (wedge.e412 * right_dual.e43)) + (float4(right_dual.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * wedge.group1
        );
    }
}
public extension Point: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(right_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group3.xyz) - (float3(wedge.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_dual.group4)
        );
    }
}
public extension Point: AntiRejectViaHorizonFrom<Plane> {
    public typedef Origin AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            -(pow(other.e321, 2) * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
public extension Point: AntiRejectViaHorizonFrom<Point> {
    public typedef Point AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> Point {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_dual.e321) + (wedge.e31 * right_dual.e412), (wedge.e42 * right_dual.e321) + (wedge.e12 * right_dual.e423), (wedge.e43 * right_dual.e321) + (wedge.e23 * right_dual.e431), -(wedge.e42 * right_dual.e431) - (wedge.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(wedge.group1.xyz.zxy, wedge.e41))
        );
    }
}
public extension Point: AntiRejectViaHorizonFrom<Scalar> {
    public typedef Point AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.e1 * other.scalar, this.e2 * other.scalar, this.e3 * other.scalar, this.e4 * other.scalar)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public AntiRejectViaHorizonFromInfixPartial<Scalar> operator /(AntiRejectViaHorizonFromInfix rhs) {
        return AntiRejectViaHorizonFromInfixPartial<Scalar>(this);
    }
}
public extension AntiRejectViaHorizonFromInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public DualNum operator /(DualNum rhs) {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
public extension Scalar: AntiRejectViaHorizonFrom<DualNum> {
    public typedef DualNum AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * float2(other.scalar * this.scalar, other.e1234 * this.scalar)
        );
    }
}
public extension Scalar: AntiRejectViaHorizonFrom<Flector> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Flector) -> Motor {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1.yzx * wedge.group1.zxy) - (right_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_dual.group1.xyz, right_dual.e1))
        );
    }
}
public extension Scalar: AntiRejectViaHorizonFrom<Horizon> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension Scalar: AntiRejectViaHorizonFrom<Line> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Line) -> Scalar {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            -(right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43)
        );
    }
}
public extension Scalar: AntiRejectViaHorizonFrom<Motor> {
    public typedef Motor AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Motor) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
        Motor right_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_dual.group0.xyz)), right_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) + float4(float3(0.0), -(right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43))
        );
    }
}
public extension Scalar: AntiRejectViaHorizonFrom<MultiVector> {
    public typedef MultiVector AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
        );
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
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * wedge.e1234) + (right_dual.e1234 * wedge.scalar) + (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4) - (right_dual.e1 * wedge.e423) - (right_dual.e2 * wedge.e431) - (right_dual.e3 * wedge.e412) - (right_dual.e4 * wedge.e321) - (right_dual.e41 * wedge.e23) - (right_dual.e42 * wedge.e31) - (right_dual.e43 * wedge.e12) - (right_dual.e23 * wedge.e41) - (right_dual.e31 * wedge.e42) - (right_dual.e12 * wedge.e43), right_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * wedge.e321) + (right_dual.e31 * wedge.e412) + (right_dual.e412 * wedge.e31) + (right_dual.e321 * wedge.e41), (right_dual.e42 * wedge.e321) + (right_dual.e12 * wedge.e423) + (right_dual.e423 * wedge.e12) + (right_dual.e321 * wedge.e42), (right_dual.e43 * wedge.e321) + (right_dual.e23 * wedge.e431) + (right_dual.e431 * wedge.e23) + (right_dual.e321 * wedge.e43), -(right_dual.e43 * wedge.e412) - (right_dual.e423 * wedge.e41) - (right_dual.e431 * wedge.e42) - (right_dual.e412 * wedge.e43)) + (float4(right_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_dual.group1) - (wedge.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((wedge.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * wedge.group4.zxy) - (right_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * wedge.group3.xyz) + (float3(right_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_dual.group3.xyz) - (float3(wedge.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_dual.group4)
        );
    }
}
public extension Scalar: AntiRejectViaHorizonFrom<Plane> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension Scalar: AntiRejectViaHorizonFrom<Point> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Point) -> Scalar {
        Point wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (right_dual.e423 * wedge.e1) + (right_dual.e431 * wedge.e2) + (right_dual.e412 * wedge.e3) + (right_dual.e321 * wedge.e4)
        );
    }
}
public extension Scalar: AntiRejectViaHorizonFrom<Scalar> {
    public typedef Scalar AntiRejectViaHorizonFromOutput;
    public func anti_reject_via_horizon_from(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
