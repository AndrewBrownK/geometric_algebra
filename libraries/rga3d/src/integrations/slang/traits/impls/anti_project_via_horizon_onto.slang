public extension AntiScalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<AntiScalar> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> AntiScalar.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension AntiScalar: AntiProjectViaHorizonOnto<DualNum> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.scalar, 2) * this.e1234
        );
    }
}
public extension AntiScalar: AntiProjectViaHorizonOnto<Flector> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Flector) -> Motor {
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
public extension AntiScalar: AntiProjectViaHorizonOnto<Horizon> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension AntiScalar: AntiProjectViaHorizonOnto<Line> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Line) -> AntiScalar {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)
        );
    }
}
public extension AntiScalar: AntiProjectViaHorizonOnto<Motor> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Motor {
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
public extension AntiScalar: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
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
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e1234 * right_dual.e1234) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * right_dual.group4.xyz), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
public extension AntiScalar: AntiProjectViaHorizonOnto<Plane> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension AntiScalar: AntiProjectViaHorizonOnto<Point> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Point) -> AntiScalar {
        Plane anti_wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz), 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)
        );
    }
}
public extension AntiScalar: AntiProjectViaHorizonOnto<Scalar> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.scalar, 2) * this.e1234
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<DualNum> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> DualNum.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension DualNum: AntiProjectViaHorizonOnto<DualNum> {
    public typedef DualNum AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> DualNum {
        DualNum anti_wedge = DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar))
        );
    }
}
public extension DualNum: AntiProjectViaHorizonOnto<Flector> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Flector) -> Motor {
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
public extension DualNum: AntiProjectViaHorizonOnto<Horizon> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension DualNum: AntiProjectViaHorizonOnto<Line> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Line) -> AntiScalar {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)
        );
    }
}
public extension DualNum: AntiProjectViaHorizonOnto<Motor> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Motor {
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
public extension DualNum: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
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
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar), this.e1234 * right_dual.e1234), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
public extension DualNum: AntiProjectViaHorizonOnto<Plane> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension DualNum: AntiProjectViaHorizonOnto<Point> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Point) -> AntiScalar {
        Plane anti_wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)
        );
    }
}
public extension DualNum: AntiProjectViaHorizonOnto<Scalar> {
    public typedef DualNum AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> DualNum {
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
public extension Flector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Flector> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Flector.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension Flector: AntiProjectViaHorizonOnto<DualNum> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> Flector {
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
public extension Flector: AntiProjectViaHorizonOnto<Flector> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Flector) -> Flector {
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
public extension Flector: AntiProjectViaHorizonOnto<Horizon> {
    public typedef Horizon AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
public extension Flector: AntiProjectViaHorizonOnto<Line> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Line) -> Plane {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
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
public extension Flector: AntiProjectViaHorizonOnto<Motor> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Flector {
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
public extension Flector: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
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
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e31) + (this.e321 * right_dual.e41), (this.e423 * right_dual.e12) + (this.e321 * right_dual.e42), (this.e431 * right_dual.e23) + (this.e321 * right_dual.e43), -(this.e431 * right_dual.e42) - (this.e412 * right_dual.e43)) + (float4(right_dual.e1234) * this.group0) - (this.group1.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)), 
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
            (float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
public extension Flector: AntiProjectViaHorizonOnto<Plane> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
public extension Flector: AntiProjectViaHorizonOnto<Point> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Point) -> Flector {
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
public extension Flector: AntiProjectViaHorizonOnto<Scalar> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> Flector {
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
public extension Horizon {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Horizon> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Horizon.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension Horizon: AntiProjectViaHorizonOnto<DualNum> {
    public typedef Horizon AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            pow(other.scalar, 2) * this.e321
        );
    }
}
public extension Horizon: AntiProjectViaHorizonOnto<Flector> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Flector) -> Flector {
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
public extension Horizon: AntiProjectViaHorizonOnto<Horizon> {
    public typedef Horizon AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
public extension Horizon: AntiProjectViaHorizonOnto<Line> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Line) -> Plane {
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
public extension Horizon: AntiProjectViaHorizonOnto<Motor> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Flector {
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
public extension Horizon: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
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
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_dual.group2.xyz), 0.0), 
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
            (float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
public extension Horizon: AntiProjectViaHorizonOnto<Plane> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
public extension Horizon: AntiProjectViaHorizonOnto<Point> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Point) -> Plane {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.xyz.zxy, anti_wedge.e23))
        );
    }
}
public extension Horizon: AntiProjectViaHorizonOnto<Scalar> {
    public typedef Horizon AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            pow(other.scalar, 2) * this.e321
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Line> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<Line> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Line.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension Line: AntiProjectViaHorizonOnto<DualNum> {
    public typedef Line AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> Line {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group1.xyz
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group1.xyz
        );
    }
}
public extension Line: AntiProjectViaHorizonOnto<Flector> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Flector) -> Motor {
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
public extension Line: AntiProjectViaHorizonOnto<Line> {
    public typedef Line AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Line) -> Line {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group1.xyz
        );
    }
}
public extension Line: AntiProjectViaHorizonOnto<Motor> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Motor {
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
public extension Line: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
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
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group4.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
public extension Line: AntiProjectViaHorizonOnto<Point> {
    public typedef Line AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Point) -> Line {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge.group0.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)
        );
    }
}
public extension Line: AntiProjectViaHorizonOnto<Scalar> {
    public typedef Line AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> Line {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group1.xyz
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Motor> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Motor.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension Motor: AntiProjectViaHorizonOnto<DualNum> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> Motor {
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
            float4(other.group0.xy.xx, other.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)) * float4(anti_wedge.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
public extension Motor: AntiProjectViaHorizonOnto<Flector> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Flector) -> Motor {
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
public extension Motor: AntiProjectViaHorizonOnto<Horizon> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension Motor: AntiProjectViaHorizonOnto<Line> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Line) -> Motor {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
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
public extension Motor: AntiProjectViaHorizonOnto<Motor> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Motor {
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
public extension Motor: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
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
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1234 * right_dual.scalar) + (this.scalar * right_dual.e1234) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), this.e1234 * right_dual.e1234), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * right_dual.e1) + (this.e31 * right_dual.e412), (this.e1234 * right_dual.e2) + (this.e12 * right_dual.e423), (this.e1234 * right_dual.e3) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) + (this.group0 * float4(right_dual.group4.www, right_dual.e4)) - (right_dual.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group2.xyz) + (float3(right_dual.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_dual.group3.xyz) + (float3(right_dual.e1234) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
public extension Motor: AntiProjectViaHorizonOnto<Plane> {
    public typedef AntiScalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension Motor: AntiProjectViaHorizonOnto<Point> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Point) -> Motor {
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
public extension Motor: AntiProjectViaHorizonOnto<Scalar> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> Motor {
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
public extension MultiVector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<MultiVector> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> MultiVector.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension MultiVector: AntiProjectViaHorizonOnto<DualNum> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> MultiVector {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_dual.e1234) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * anti_wedge.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group4
        );
    }
}
public extension MultiVector: AntiProjectViaHorizonOnto<Flector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Flector) -> MultiVector {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) + (float4(this.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (right_dual.group1.yzx * this.group4.zxy) - (right_dual.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
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
            float4((other.e3 * anti_wedge.e42) + (other.e4 * anti_wedge.e23), (other.e1 * anti_wedge.e43) + (other.e4 * anti_wedge.e31), (other.e2 * anti_wedge.e41) + (other.e4 * anti_wedge.e12), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + (float4(anti_wedge.scalar) * other.group1) - (other.group0.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23))
        );
    }
}
public extension MultiVector: AntiProjectViaHorizonOnto<Horizon> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Horizon) -> MultiVector {
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
public extension MultiVector: AntiProjectViaHorizonOnto<Line> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Line) -> MultiVector {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group4.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e41)), 
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group1.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension MultiVector: AntiProjectViaHorizonOnto<Motor> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> MultiVector {
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
            (float3(right_dual.e1234) * this.group2.xyz) + (float3(this.e1234) * right_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * this.group3.xyz) + (float3(this.e1234) * right_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * anti_wedge.scalar, (other.e1234 * anti_wedge.scalar) + (other.scalar * anti_wedge.e1234) - (other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43 */
            (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(anti_wedge.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * anti_wedge.group3.xyz) + (float3(anti_wedge.scalar) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4((other.e23 * anti_wedge.e4) + (other.scalar * anti_wedge.e423), (other.e31 * anti_wedge.e4) + (other.scalar * anti_wedge.e431), (other.e12 * anti_wedge.e4) + (other.scalar * anti_wedge.e412), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) + float4((other.group0.yzx * anti_wedge.group1.zxy), other.scalar * anti_wedge.e321) - (anti_wedge.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension MultiVector: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
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
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_dual.scalar * this.e1234) + (right_dual.e1234 * this.scalar) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), right_dual.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412) + (right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423) + (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431) + (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e43 * this.e412) - (right_dual.e423 * this.e41) - (right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) + (float4(right_dual.e1234) * this.group1) + (float4(this.e1234) * right_dual.group1) - (this.group4.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)) - float4((this.group3.xyz.zxy * right_dual.group4.yzx), right_dual.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group2.xyz) + (float3(this.e1234) * right_dual.group2.xyz) + (right_dual.group4.yzx * this.group4.zxy) - (right_dual.group4.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * this.group3.xyz) + (float3(right_dual.e321) * this.group4.xyz) + (float3(this.e1234) * right_dual.group3.xyz) - (float3(this.e321) * right_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual.e1234) * this.group4) + (float4(this.e1234) * right_dual.group4)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
public extension MultiVector: AntiProjectViaHorizonOnto<Plane> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Plane) -> MultiVector {
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
public extension MultiVector: AntiProjectViaHorizonOnto<Point> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Point) -> MultiVector {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * right_dual.group0.yzx) - (this.group4.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_dual.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
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
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23))
        );
    }
}
public extension MultiVector: AntiProjectViaHorizonOnto<Scalar> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> MultiVector {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_dual.e1234) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * anti_wedge.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group4
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Origin> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Origin.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension Origin: AntiProjectViaHorizonOnto<DualNum> {
    public typedef Origin AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.scalar, 2) * this.e4
        );
    }
}
public extension Origin: AntiProjectViaHorizonOnto<Motor> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Flector {
        Origin anti_wedge = Origin.from_groups(
            /* e4 */
            (other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.e4
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * anti_wedge.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * other.group1.xyz), 0.0)
        );
    }
}
public extension Origin: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
        Origin anti_wedge = Origin.from_groups(
            /* e4 */
            other.scalar * this.e4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e321 * anti_wedge.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * anti_wedge.e4), 
            /* e41, e42, e43 */
            float3(anti_wedge.e4) * other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * other.group3.xyz), 0.0)
        );
    }
}
public extension Origin: AntiProjectViaHorizonOnto<Scalar> {
    public typedef Origin AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.scalar, 2) * this.e4
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Plane> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Plane.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension Plane: AntiProjectViaHorizonOnto<DualNum> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> Plane {
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
public extension Plane: AntiProjectViaHorizonOnto<Flector> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Flector) -> Flector {
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
public extension Plane: AntiProjectViaHorizonOnto<Horizon> {
    public typedef Horizon AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
public extension Plane: AntiProjectViaHorizonOnto<Line> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Line) -> Plane {
        Line right_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
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
public extension Plane: AntiProjectViaHorizonOnto<Motor> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Flector {
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
public extension Plane: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
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
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_dual.group3.xyz.zxy, right_dual.e41)), 
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
            (float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
public extension Plane: AntiProjectViaHorizonOnto<Plane> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
public extension Plane: AntiProjectViaHorizonOnto<Point> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Point) -> Plane {
        Plane right_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            (right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_dual.group0.xyz)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.xyz.zxy, anti_wedge.e23))
        );
    }
}
public extension Plane: AntiProjectViaHorizonOnto<Scalar> {
    public typedef Plane AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> Plane {
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
public extension Point {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Point> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<Point> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Point.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension Point: AntiProjectViaHorizonOnto<DualNum> {
    public typedef Point AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> Point {
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
public extension Point: AntiProjectViaHorizonOnto<Flector> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Flector) -> Flector {
        Flector right_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
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
public extension Point: AntiProjectViaHorizonOnto<Motor> {
    public typedef Flector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Flector {
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
public extension Point: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
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
            (float3(anti_wedge.scalar) * other.group2.xyz) + (float3(other.scalar) * anti_wedge.group2.xyz) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3.xyz) + (float3(other.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((other.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
public extension Point: AntiProjectViaHorizonOnto<Point> {
    public typedef Point AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Point) -> Point {
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
public extension Point: AntiProjectViaHorizonOnto<Scalar> {
    public typedef Point AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> Point {
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
public extension Scalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiProjectViaHorizonOntoInfix) -> AntiProjectViaHorizonOntoInfixPartial<Scalar> {
        return AntiProjectViaHorizonOntoInfixPartial(this);
    }
}
public extension AntiProjectViaHorizonOntoInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Scalar.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
public extension Scalar: AntiProjectViaHorizonOnto<DualNum> {
    public typedef DualNum AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar: AntiProjectViaHorizonOnto<Motor> {
    public typedef Motor AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Motor) -> Motor {
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
public extension Scalar: AntiProjectViaHorizonOnto<MultiVector> {
    public typedef MultiVector AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: MultiVector) -> MultiVector {
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group4
        );
    }
}
public extension Scalar: AntiProjectViaHorizonOnto<Scalar> {
    public typedef Scalar AntiProjectViaHorizonOntoOutput;
    public func anti_project_via_horizon_onto(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
