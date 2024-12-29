implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<AntiScalar>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(DualNum other) {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.scalar, 2) * this.e1234
        );
    }
}
public extension AntiScalar {
    public Motor anti_project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Horizon other) {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Line other) {
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
public extension AntiScalar {
    public Motor anti_project_orthogonally_onto(Motor other) {
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
public extension AntiScalar {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float2(1.0, this.e1234 * right_anti_dual.e1234) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * right_anti_dual.group4.xyz), 0.0)
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
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Plane other) {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Point other) {
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
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Scalar other) {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.scalar, 2) * this.e1234
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public AntiProjectOrthogonallyOntoInfixPartial<DualNum> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<DualNum>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public DualNum operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension DualNum {
    public DualNum anti_project_orthogonally_onto(DualNum other) {
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
public extension DualNum {
    public Motor anti_project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(Horizon other) {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(Line other) {
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
public extension DualNum {
    public Motor anti_project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.scalar * right_anti_dual.e1234) + (this.e1234 * right_anti_dual.scalar))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
public extension DualNum {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float2((this.scalar * right_anti_dual.e1234) + (this.e1234 * right_anti_dual.scalar), this.e1234 * right_anti_dual.e1234), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
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
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(Plane other) {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(Point other) {
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
public extension DualNum {
    public DualNum anti_project_orthogonally_onto(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * float2(right_anti_dual.e1234 * this.scalar, right_anti_dual.e1234 * this.e1234)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public AntiProjectOrthogonallyOntoInfixPartial<Flector> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<Flector>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Flector operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(DualNum other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * this.group1.zxy) - (right_anti_dual.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (this.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
public extension Flector {
    public Horizon anti_project_orthogonally_onto(Horizon other) {
        return Horizon.from_groups(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
public extension Flector {
    public Plane anti_project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e31) + (this.e321 * right_anti_dual.e41), (this.e423 * right_anti_dual.e12) + (this.e321 * right_anti_dual.e42), (this.e431 * right_anti_dual.e23) + (this.e321 * right_anti_dual.e43), -(this.e431 * right_anti_dual.e42) - (this.e412 * right_anti_dual.e43)) - (this.group1.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41))
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e31) + (this.e321 * right_anti_dual.e41), (this.e423 * right_anti_dual.e12) + (this.e321 * right_anti_dual.e42), (this.e431 * right_anti_dual.e23) + (this.e321 * right_anti_dual.e43), -(this.e431 * right_anti_dual.e42) - (this.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * this.group0) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Flector {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float2((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321) - (this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e31) + (this.e321 * right_anti_dual.e41), (this.e423 * right_anti_dual.e12) + (this.e321 * right_anti_dual.e42), (this.e431 * right_anti_dual.e23) + (this.e321 * right_anti_dual.e43), -(this.e431 * right_anti_dual.e42) - (this.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * this.group0) - (this.group1.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            (this.group1.zxy * right_anti_dual.group4.yzx) - (this.group1.yzx * right_anti_dual.group4.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group1.xyz) - (float3(this.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group1
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
public extension Flector {
    public Plane anti_project_orthogonally_onto(Plane other) {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(Point other) {
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * right_anti_dual.e423 * -1.0, this.e321 * right_anti_dual.e431 * -1.0, this.e321 * right_anti_dual.e412 * -1.0, (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321)) + (right_anti_dual.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group1
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
    public AntiProjectOrthogonallyOntoInfixPartial<Horizon> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<Horizon>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Horizon operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension Horizon {
    public Horizon anti_project_orthogonally_onto(DualNum other) {
        return Horizon.from_groups(
            /* e321 */
            pow(other.scalar, 2) * this.e321
        );
    }
}
public extension Horizon {
    public Flector anti_project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4) * float4(-1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
public extension Horizon {
    public Horizon anti_project_orthogonally_onto(Horizon other) {
        return Horizon.from_groups(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
public extension Horizon {
    public Plane anti_project_orthogonally_onto(Line other) {
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
public extension Horizon {
    public Flector anti_project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_anti_dual.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.e1234)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(anti_wedge.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Horizon {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float2(this.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_anti_dual.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * right_anti_dual.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.e1234)
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
public extension Horizon {
    public Plane anti_project_orthogonally_onto(Plane other) {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
public extension Horizon {
    public Plane anti_project_orthogonally_onto(Point other) {
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
public extension Horizon {
    public Horizon anti_project_orthogonally_onto(Scalar other) {
        return Horizon.from_groups(
            /* e321 */
            pow(other.scalar, 2) * this.e321
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public AntiProjectOrthogonallyOntoInfixPartial<Line> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<Line>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Line operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension Line {
    public Line anti_project_orthogonally_onto(DualNum other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group1.xyz
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group1.xyz
        );
    }
}
public extension Line {
    public Motor anti_project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e31) + (right_anti_dual.e321 * this.e41), (right_anti_dual.e423 * this.e12) + (right_anti_dual.e321 * this.e42), (right_anti_dual.e431 * this.e23) + (right_anti_dual.e321 * this.e43), -(right_anti_dual.e431 * this.e42) - (right_anti_dual.e412 * this.e43)) - (right_anti_dual.group1.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * anti_wedge.e1, other.e4 * anti_wedge.e2, other.e4 * anti_wedge.e3, -(other.e431 * anti_wedge.e2) - (other.e412 * anti_wedge.e3) - (other.e321 * anti_wedge.e4)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.yzx * anti_wedge.group0.zxy) - (other.group0.zxy * anti_wedge.group0.yzx)), 0.0)
        );
    }
}
public extension Line {
    public Line anti_project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group1.xyz
        );
    }
}
public extension Line {
    public Motor anti_project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
public extension Line {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float2(-(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e321) + (this.e31 * right_anti_dual.e412), (this.e42 * right_anti_dual.e321) + (this.e12 * right_anti_dual.e423), (this.e43 * right_anti_dual.e321) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) - (right_anti_dual.group4.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group1.xyz, 
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
public extension Line {
    public Line anti_project_orthogonally_onto(Point other) {
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e321) + (this.e31 * right_anti_dual.e412), (this.e42 * right_anti_dual.e321) + (this.e12 * right_anti_dual.e423), (this.e43 * right_anti_dual.e321) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge.group0.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)
        );
    }
}
public extension Line {
    public Line anti_project_orthogonally_onto(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group1.xyz
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
    public AntiProjectOrthogonallyOntoInfixPartial<Motor> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<Motor>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Motor operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(DualNum other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * this.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)) * float4(anti_wedge.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e31) + (right_anti_dual.e321 * this.e41), (right_anti_dual.e423 * this.e12) + (right_anti_dual.e321 * this.e42), (right_anti_dual.e431 * this.e23) + (right_anti_dual.e321 * this.e43), -(right_anti_dual.e431 * this.e42) - (right_anti_dual.e412 * this.e43)) + (float4(this.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
public extension Motor {
    public AntiScalar anti_project_orthogonally_onto(Horizon other) {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(anti_wedge.scalar) * other.group0.xyz), -(other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(anti_wedge.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * this.group0.xyz) + (float3(this.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * this.group1) + (float4(this.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
public extension Motor {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float2((this.e1234 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e1234) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), this.e1234 * right_anti_dual.e1234), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * right_anti_dual.e1) + (this.e31 * right_anti_dual.e412), (this.e1234 * right_anti_dual.e2) + (this.e12 * right_anti_dual.e423), (this.e1234 * right_anti_dual.e3) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) + (this.group0 * float4(right_anti_dual.group4.www, right_anti_dual.e4)) - (right_anti_dual.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.e1234) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_anti_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
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
public extension Motor {
    public AntiScalar anti_project_orthogonally_onto(Plane other) {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(other.e321, 2) * this.e1234
        );
    }
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(Point other) {
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e321) + (this.e31 * right_anti_dual.e412), (this.e42 * right_anti_dual.e321) + (this.e12 * right_anti_dual.e423), (this.e43 * right_anti_dual.e321) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.e4 * other.e1 * -1.0, anti_wedge.e4 * other.e2 * -1.0, anti_wedge.e4 * other.e3 * -1.0, (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)) + (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * this.group1
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
    public AntiProjectOrthogonallyOntoInfixPartial<MultiVector> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<MultiVector>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(DualNum other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual.e1234) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group4
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
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e31) + (right_anti_dual.e321 * this.e41), (right_anti_dual.e423 * this.e12) + (right_anti_dual.e321 * this.e42), (right_anti_dual.e431 * this.e23) + (right_anti_dual.e321 * this.e43), -(right_anti_dual.e431 * this.e42) - (right_anti_dual.e412 * this.e43)) + (float4(this.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (right_anti_dual.group1.yzx * this.group4.zxy) - (right_anti_dual.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_anti_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
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
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Horizon other) {
        Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * right_anti_dual.e4), 
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
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) - (this.group4.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group0.xyz, 
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
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual.e1234 * this.scalar) + (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), right_anti_dual.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e1234 * this.e1) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e1234 * this.e2) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e1234 * this.e3) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) + (right_anti_dual.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group2.xyz) + (float3(this.e1234) * right_anti_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * this.group3.xyz) + (float3(this.e1234) * right_anti_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group4
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
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float2((right_anti_dual.scalar * this.e1234) + (right_anti_dual.e1234 * this.scalar) + (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), right_anti_dual.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412) + (right_anti_dual.e412 * this.e31) + (right_anti_dual.e321 * this.e41), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423) + (right_anti_dual.e423 * this.e12) + (right_anti_dual.e321 * this.e42), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431) + (right_anti_dual.e431 * this.e23) + (right_anti_dual.e321 * this.e43), -(right_anti_dual.e43 * this.e412) - (right_anti_dual.e423 * this.e41) - (right_anti_dual.e431 * this.e42) - (right_anti_dual.e412 * this.e43)) + (float4(right_anti_dual.e1234) * this.group1) + (float4(this.e1234) * right_anti_dual.group1) - (this.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((this.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group2.xyz) + (float3(this.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * this.group4.zxy) - (right_anti_dual.group4.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * this.group3.xyz) + (float3(right_anti_dual.e321) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(this.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * this.group4) + (float4(this.e1234) * right_anti_dual.group4)
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
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Plane other) {
        Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * right_anti_dual.e4), 
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
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Point other) {
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e321) + (this.e31 * right_anti_dual.e412), (this.e42 * right_anti_dual.e321) + (this.e12 * right_anti_dual.e423), (this.e43 * right_anti_dual.e321) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * right_anti_dual.group0.yzx) - (this.group4.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
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
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual.e1234) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group4
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
    public AntiProjectOrthogonallyOntoInfixPartial<Origin> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<Origin>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public Origin operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension Origin {
    public Origin anti_project_orthogonally_onto(DualNum other) {
        return Origin.from_groups(
            /* e4 */
            pow(other.scalar, 2) * this.e4
        );
    }
}
public extension Origin {
    public Flector anti_project_orthogonally_onto(Motor other) {
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
public extension Origin {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
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
public extension Origin {
    public Origin anti_project_orthogonally_onto(Scalar other) {
        return Origin.from_groups(
            /* e4 */
            pow(other.scalar, 2) * this.e4
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public AntiProjectOrthogonallyOntoInfixPartial<Plane> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<Plane>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(DualNum other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_anti_dual.e1234 * this.e423, right_anti_dual.e1234 * this.e431, right_anti_dual.e1234 * this.e412, right_anti_dual.e1234 * this.e321)
        );
    }
}
public extension Plane {
    public Flector anti_project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e321 * this.e423, right_anti_dual.e321 * this.e431, right_anti_dual.e321 * this.e412, -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (this.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
public extension Plane {
    public Horizon anti_project_orthogonally_onto(Horizon other) {
        return Horizon.from_groups(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41))
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Plane {
    public Flector anti_project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Plane {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float2(-(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            (right_anti_dual.group4.yzx * this.group0.zxy) - (right_anti_dual.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group0
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
public extension Plane {
    public Plane anti_project_orthogonally_onto(Plane other) {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(Point other) {
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            (right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.xyz.zxy, anti_wedge.e23))
        );
    }
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_anti_dual.e1234 * this.e423, right_anti_dual.e1234 * this.e431, right_anti_dual.e1234 * this.e412, right_anti_dual.e1234 * this.e321)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public AntiProjectOrthogonallyOntoInfixPartial<Point> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<Point>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Point operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension Point {
    public Point anti_project_orthogonally_onto(DualNum other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_anti_dual.e1234 * this.e1, right_anti_dual.e1234 * this.e2, right_anti_dual.e1234 * this.e3, right_anti_dual.e1234 * this.e4)
        );
    }
}
public extension Point {
    public Flector anti_project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group1
        );
    }
}
public extension Point {
    public Flector anti_project_orthogonally_onto(Motor other) {
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
public extension Point {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float2((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0, 
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
public extension Point {
    public Point anti_project_orthogonally_onto(Point other) {
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)) * other.group0
        );
    }
}
public extension Point {
    public Point anti_project_orthogonally_onto(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_anti_dual.e1234 * this.e1, right_anti_dual.e1234 * this.e2, right_anti_dual.e1234 * this.e3, right_anti_dual.e1234 * this.e4)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public AntiProjectOrthogonallyOntoInfixPartial<Scalar> operator /(AntiProjectOrthogonallyOntoInfix rhs) {
        return AntiProjectOrthogonallyOntoInfixPartial<Scalar>(this);
    }
}
public extension AntiProjectOrthogonallyOntoInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public DualNum operator /(DualNum rhs) {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
public extension Scalar {
    public DualNum anti_project_orthogonally_onto(DualNum other) {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar {
    public Motor anti_project_orthogonally_onto(Motor other) {
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
public extension Scalar {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
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
public extension Scalar {
    public Scalar anti_project_orthogonally_onto(Scalar other) {
        return Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
