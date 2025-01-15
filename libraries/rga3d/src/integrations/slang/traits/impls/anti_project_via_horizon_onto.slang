implementing rga3d;
using traits;
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(DualNum other) {
        return AntiScalar(
            /* e1234 */
            pow(other.scalar, 2) * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_via_horizon_onto(Flector other) {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * right_dual.group1.xyz), 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Horizon other) {
        return AntiScalar(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Line other) {
        Line anti_wedge = Line(
            /* e41, e42, e43 */
            float3(this.e1234) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar(
            /* e1234 */
            -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_via_horizon_onto(Motor other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        return AntiScalar(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Point other) {
        Plane anti_wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz), 0.0)
        );
        return AntiScalar(
            /* e1234 */
            (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Scalar other) {
        return AntiScalar(
            /* e1234 */
            pow(other.scalar, 2) * this.e1234
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<DualNum> operator /(DualNum lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public DualNum anti_project_via_horizon_onto(DualNum other) {
        DualNum anti_wedge = DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy
        );
        return DualNum(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_via_horizon_onto(Flector other) {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Horizon other) {
        return AntiScalar(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Line other) {
        Line anti_wedge = Line(
            /* e41, e42, e43 */
            float3(this.e1234) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar(
            /* e1234 */
            -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_via_horizon_onto(Motor other) {
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * right_dual.group0, 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar))
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        return AntiScalar(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Point other) {
        Plane anti_wedge = Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiScalar(
            /* e1234 */
            (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public DualNum anti_project_via_horizon_onto(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * float2(right_dual.e1234 * this.scalar, right_dual.e1234 * this.e1234)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Flector> operator /(Flector lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(DualNum other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group1
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Flector other) {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1.yzx * this.group1.zxy) - (right_dual.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_dual.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)) - (this.group1.wwwx * float4(right_dual.group1.xyz, right_dual.e1))
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Horizon anti_project_via_horizon_onto(Horizon other) {
        return Horizon(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Plane anti_project_via_horizon_onto(Line other) {
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e31) + (this.e321 * right_dual.e41), (this.e423 * right_dual.e12) + (this.e321 * right_dual.e42), (this.e431 * right_dual.e23) + (this.e321 * right_dual.e43), -(this.e431 * right_dual.e42) - (this.e412 * right_dual.e43)) - (this.group1.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e41))
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Motor other) {
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e31) + (this.e321 * right_dual.e41), (this.e423 * right_dual.e12) + (this.e321 * right_dual.e42), (this.e431 * right_dual.e23) + (this.e321 * right_dual.e43), -(this.e431 * right_dual.e42) - (this.e412 * right_dual.e43)) + (float4(right_dual.e1234) * this.group0) - (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group1
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Plane anti_project_via_horizon_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Point other) {
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * right_dual.group0.yzx) - (this.group1.yzx * right_dual.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * right_dual.e423 * -1.0, this.e321 * right_dual.e431 * -1.0, this.e321 * right_dual.e412 * -1.0, (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321)) + (right_dual.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group1
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Horizon> operator /(Horizon lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Horizon anti_project_via_horizon_onto(DualNum other) {
        return Horizon(
            /* e321 */
            pow(other.scalar, 2) * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Flector anti_project_via_horizon_onto(Flector other) {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(right_dual.group1.xyz, right_dual.e4) * float4(-1.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Horizon anti_project_via_horizon_onto(Horizon other) {
        return Horizon(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Plane anti_project_via_horizon_onto(Line other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0)), 0.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Flector anti_project_via_horizon_onto(Motor other) {
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_dual.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_dual.e1234)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(anti_wedge.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Plane anti_project_via_horizon_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Plane anti_project_via_horizon_onto(Point other) {
        Line anti_wedge = Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.xyz.zxy, anti_wedge.e23))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Horizon anti_project_via_horizon_onto(Scalar other) {
        return Horizon(
            /* e321 */
            pow(other.scalar, 2) * this.e321
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Line> operator /(Line lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(DualNum other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        Line anti_wedge = Line(
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group1.xyz
        );
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Motor anti_project_via_horizon_onto(Flector other) {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) - (right_dual.group1.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * anti_wedge.e1, other.e4 * anti_wedge.e2, other.e4 * anti_wedge.e3, -(other.e431 * anti_wedge.e2) - (other.e412 * anti_wedge.e3) - (other.e321 * anti_wedge.e4)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.yzx * anti_wedge.group0.zxy) - (other.group0.zxy * anti_wedge.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(Line other) {
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Scalar anti_wedge = Scalar(
            /* scalar */
            -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
        return Line(
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Motor anti_project_via_horizon_onto(Motor other) {
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e1234) * this.group1.xyz), -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43))
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(Point other) {
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
        return Line(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge.group0.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        Line anti_wedge = Line(
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group1.xyz
        );
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group1.xyz
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Motor> operator /(Motor lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(DualNum other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(right_dual.e1234) * this.group1
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)) * float4(anti_wedge.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Flector other) {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e31) + (right_dual.e321 * this.e41), (right_dual.e423 * this.e12) + (right_dual.e321 * this.e42), (right_dual.e431 * this.e23) + (right_dual.e321 * this.e43), -(right_dual.e431 * this.e42) - (right_dual.e412 * this.e43)) + (float4(this.e1234) * right_dual.group0) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_via_horizon_onto(Horizon other) {
        return AntiScalar(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Line other) {
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43))
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(anti_wedge.scalar) * other.group0.xyz), -(other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(anti_wedge.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Motor other) {
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual.e1234) * this.group0.xyz) + (float3(this.e1234) * right_dual.group0.xyz)), right_dual.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_dual.e1234) * this.group1) + (float4(this.e1234) * right_dual.group1) + float4(float3(0.0), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43))
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        return AntiScalar(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Point other) {
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e321) + (this.e31 * right_dual.e412), (this.e42 * right_dual.e321) + (this.e12 * right_dual.e423), (this.e43 * right_dual.e321) + (this.e23 * right_dual.e431), -(this.e42 * right_dual.e431) - (this.e43 * right_dual.e412)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.e4 * other.e1 * -1.0, anti_wedge.e4 * other.e2 * -1.0, anti_wedge.e4 * other.e3 * -1.0, (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)) + (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(right_dual.e1234) * this.group1
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<MultiVector> operator /(MultiVector lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(DualNum other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Flector other) {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Horizon other) {
        Origin right_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Line other) {
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Motor other) {
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Plane other) {
        Origin right_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Point other) {
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Origin> operator /(Origin lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(AntiProjectViaHorizonOntoInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Origin {
    public Origin anti_project_via_horizon_onto(DualNum other) {
        return Origin(
            /* e4 */
            pow(other.scalar, 2) * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Origin {
    public Flector anti_project_via_horizon_onto(Motor other) {
        Origin anti_wedge = Origin(
            /* e4 */
            (other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.e4
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * anti_wedge.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Origin {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        Origin anti_wedge = Origin(
            /* e4 */
            other.scalar * this.e4
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Origin operator /(AntiProjectViaHorizonOntoInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Origin {
    public Origin anti_project_via_horizon_onto(Scalar other) {
        return Origin(
            /* e4 */
            pow(other.scalar, 2) * this.e4
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Plane> operator /(Plane lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(DualNum other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e423, right_dual.e1234 * this.e431, right_dual.e1234 * this.e412, right_dual.e1234 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Flector anti_project_via_horizon_onto(Flector other) {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_dual.e321 * this.e423, right_dual.e321 * this.e431, right_dual.e321 * this.e412, -(right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)) - (this.group0.wwwx * float4(right_dual.group1.xyz, right_dual.e1))
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Horizon anti_project_via_horizon_onto(Horizon other) {
        return Horizon(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(Line other) {
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e41))
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Flector anti_project_via_horizon_onto(Motor other) {
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((right_dual.e41 * this.e321) + (right_dual.e31 * this.e412), (right_dual.e42 * this.e321) + (right_dual.e12 * this.e423), (right_dual.e43 * this.e321) + (right_dual.e23 * this.e431), -(right_dual.e42 * this.e431) - (right_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_dual.e1234) * this.group0
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(Point other) {
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Line anti_wedge = Line(
            /* e41, e42, e43 */
            (right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_dual.group0.xyz)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.xyz.zxy, anti_wedge.e23))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e423, right_dual.e1234 * this.e431, right_dual.e1234 * this.e412, right_dual.e1234 * this.e321)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Point> operator /(Point lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Point anti_project_via_horizon_onto(DualNum other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e1, right_dual.e1234 * this.e2, right_dual.e1234 * this.e3, right_dual.e1234 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Flector anti_project_via_horizon_onto(Flector other) {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Scalar anti_wedge = Scalar(
            /* scalar */
            (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Flector anti_project_via_horizon_onto(Motor other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
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
// Fancy infix trick (second half)
public Point operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Point anti_project_via_horizon_onto(Point other) {
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Point anti_project_via_horizon_onto(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_dual.e1234 * this.e1, right_dual.e1234 * this.e2, right_dual.e1234 * this.e3, right_dual.e1234 * this.e4)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Scalar> operator /(Scalar lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public DualNum anti_project_via_horizon_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public Motor anti_project_via_horizon_onto(Motor other) {
        Scalar anti_wedge = Scalar(
            /* scalar */
            (other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.scalar
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(anti_wedge.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        Scalar anti_wedge = Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Scalar operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public Scalar anti_project_via_horizon_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
