implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<AntiScalar> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<AntiScalar>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(DualNum rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension AntiScalar {
    public Motor reject_via_origin_from(Flector other) {
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)) + float4(float3(0.0), (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)) - (right_dual.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.yzx * right_dual.group0.zxy) - (anti_wedge.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Line other) {
        Line anti_wedge = Line(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar(
            /* e1234 */
            -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)
        );
    }
}
public extension AntiScalar {
    public Motor reject_via_origin_from(Motor other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_dual.group0) + (float4(right_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension AntiScalar {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Plane other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Point other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return AntiScalar(
            /* e1234 */
            (right_dual.e423 * anti_wedge.e1) + (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4)
        );
    }
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<DualNum> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<DualNum>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(DualNum rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            (pow(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
public extension DualNum {
    public Motor reject_via_origin_from(Flector other) {
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)) + float4(float3(0.0), (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)) - (right_dual.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.yzx * right_dual.group0.zxy) - (anti_wedge.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Line other) {
        Line anti_wedge = Line(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar(
            /* e1234 */
            -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)
        );
    }
}
public extension DualNum {
    public Motor reject_via_origin_from(Motor other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group1.xyz, 1.0)
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_dual.group0) + (float4(right_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Plane other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Point other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return AntiScalar(
            /* e1234 */
            (right_dual.e423 * anti_wedge.e1) + (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4)
        );
    }
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<Flector> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<Flector>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Flector operator /(Flector rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension Flector {
    public Flector reject_via_origin_from(Flector other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (other.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group1.wwwx * float4(other.group1.xyz, other.e1))
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_dual.e4 * anti_wedge.e23) + (right_dual.e423 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e31) + (right_dual.e431 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e12) + (right_dual.e412 * anti_wedge.scalar), -(right_dual.e2 * anti_wedge.e31) - (right_dual.e3 * anti_wedge.e12)) + float4((right_dual.group0.zxy * anti_wedge.group0.yzx), right_dual.e321 * anti_wedge.scalar) - (right_dual.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
public extension Flector {
    public Flector reject_via_origin_from(Horizon other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
        Origin right_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual.e4) * anti_wedge.group1.xyz), 0.0)
        );
    }
}
public extension Flector {
    public Plane reject_via_origin_from(Line other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.xyz.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_dual.group0.xyz.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
public extension Flector {
    public Plane reject_via_origin_from(Motor other) {
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group1.xyz), 0.0) + float4((anti_wedge.group0.zxy * right_dual.group0.yzx), 0.0) - float4((anti_wedge.group0.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
public extension Flector {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            (this.group1.zxy * other.group4.yzx) - (this.group1.yzx * other.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group1.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
public extension Flector {
    public Flector reject_via_origin_from(Plane other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
        Origin right_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual.e4) * anti_wedge.group1.xyz), 0.0)
        );
    }
}
public extension Flector {
    public Plane reject_via_origin_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<Horizon> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<Horizon>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Flector operator /(Flector rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension Horizon {
    public Flector reject_via_origin_from(Flector other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_dual.e4 * anti_wedge.e23) + (right_dual.e423 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e31) + (right_dual.e431 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e12) + (right_dual.e412 * anti_wedge.scalar), -(right_dual.e2 * anti_wedge.e31) - (right_dual.e3 * anti_wedge.e12)) + float4((right_dual.group0.zxy * anti_wedge.group0.yzx), right_dual.e321 * anti_wedge.scalar) - (right_dual.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
public extension Horizon {
    public Plane reject_via_origin_from(Line other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.xyz.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_dual.group0.xyz.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
public extension Horizon {
    public Plane reject_via_origin_from(Motor other) {
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group1.xyz), 0.0) + float4((anti_wedge.group0.zxy * right_dual.group0.yzx), 0.0) - float4((anti_wedge.group0.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
public extension Horizon {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
public extension Horizon {
    public Plane reject_via_origin_from(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0)), 0.0)
        );
    }
}
public extension Horizon {
    public Plane reject_via_origin_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<Line> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<Line>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Motor operator /(Flector rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension Line {
    public Motor reject_via_origin_from(Flector other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) - (other.group1.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(right_dual.e4 * anti_wedge.e1 * -1.0, right_dual.e4 * anti_wedge.e2 * -1.0, right_dual.e4 * anti_wedge.e3 * -1.0, (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4)) + (anti_wedge.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)), 
            /* e23, e31, e12, scalar */
            float4(((right_dual.group0.zxy * anti_wedge.group0.yzx) - (right_dual.group0.yzx * anti_wedge.group0.zxy)), 0.0)
        );
    }
}
public extension Line {
    public Line reject_via_origin_from(Horizon other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * float4(other.e321 * this.e41, other.e321 * this.e42, other.e321 * this.e43, 0.0).xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Line {
    public Line reject_via_origin_from(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Line {
    public Motor reject_via_origin_from(Motor other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43))
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_dual.group0) + (float4(right_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Line {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2(-(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group4.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
public extension Line {
    public Line reject_via_origin_from(Plane other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * float4((this.e41 * other.e321) + (this.e31 * other.e412) - (this.e12 * other.e431), (this.e42 * other.e321) + (this.e12 * other.e423) - (this.e23 * other.e412), (this.e43 * other.e321) + (this.e23 * other.e431) - (this.e31 * other.e423), -(this.e41 * other.e423) - (this.e42 * other.e431) - (this.e43 * other.e412)).xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<Motor> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<Motor>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(DualNum rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension Motor {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            (pow(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
public extension Motor {
    public Motor reject_via_origin_from(Flector other) {
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e423)) + float4(float3(0.0), (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)) - (right_dual.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.yzx * right_dual.group0.zxy) - (anti_wedge.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
public extension Motor {
    public Motor reject_via_origin_from(Horizon other) {
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(anti_wedge.group0.xyz, anti_wedge.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Motor {
    public Motor reject_via_origin_from(Line other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(anti_wedge.scalar) * right_dual.group0.xyz), -(right_dual.e41 * anti_wedge.e23) - (right_dual.e42 * anti_wedge.e31) - (right_dual.e43 * anti_wedge.e12) - (right_dual.e23 * anti_wedge.e41) - (right_dual.e31 * anti_wedge.e42) - (right_dual.e12 * anti_wedge.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Motor {
    public Motor reject_via_origin_from(Motor other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e1234) * other.group0.xyz)), other.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_dual.group0) + (float4(right_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Motor {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2((this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * other.e1) + (this.e31 * other.e412), (this.e1234 * other.e2) + (this.e12 * other.e423), (this.e1234 * other.e3) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) + (this.group0 * float4(other.group4.www, other.e4)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group2.xyz) + (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group3.xyz) + (float3(other.e1234) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
public extension Motor {
    public Motor reject_via_origin_from(Plane other) {
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(anti_wedge.group0.xyz, anti_wedge.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Motor {
    public AntiScalar reject_via_origin_from(Point other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return AntiScalar(
            /* e1234 */
            (right_dual.e423 * anti_wedge.e1) + (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4)
        );
    }
}
public extension Motor {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<MultiVector> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<MultiVector>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(DualNum rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension MultiVector {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            (pow(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Flector other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (other.group1.yzx * this.group4.zxy) - (other.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (right_dual.e423 * anti_wedge.e1) + (right_dual.e431 * anti_wedge.e2) + (right_dual.e412 * anti_wedge.e3) + (right_dual.e321 * anti_wedge.e4) - (right_dual.e1 * anti_wedge.e423) - (right_dual.e2 * anti_wedge.e431) - (right_dual.e3 * anti_wedge.e412) - (right_dual.e4 * anti_wedge.e321)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e41, e42, e43 */
            (float3(anti_wedge.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (right_dual.group0.zxy * anti_wedge.group1.yzx) - (right_dual.group0.yzx * anti_wedge.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((right_dual.e3 * anti_wedge.e42) + (right_dual.e4 * anti_wedge.e23), (right_dual.e1 * anti_wedge.e43) + (right_dual.e4 * anti_wedge.e31), (right_dual.e2 * anti_wedge.e41) + (right_dual.e4 * anti_wedge.e12), -(right_dual.e2 * anti_wedge.e31) - (right_dual.e3 * anti_wedge.e12)) + (float4(anti_wedge.scalar) * right_dual.group1) - (right_dual.group0.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23))
        );
    }
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Horizon other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group4.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
        Origin right_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, anti_wedge.e321 * right_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_dual.e4), 
            /* e41, e42, e43 */
            float3(right_dual.e4) * anti_wedge.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual.e4) * anti_wedge.group3.xyz), 0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Line other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group4.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(right_dual.e41 * anti_wedge.e23) - (right_dual.e42 * anti_wedge.e31) - (right_dual.e43 * anti_wedge.e12) - (right_dual.e23 * anti_wedge.e41) - (right_dual.e31 * anti_wedge.e42) - (right_dual.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.xyz.yzx * anti_wedge.group1.zxy), 0.0) - float4((right_dual.group0.xyz.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Motor other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2((other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e1234 * this.e1) + (other.e31 * this.e412), (other.e1234 * this.e2) + (other.e12 * this.e423), (other.e1234 * this.e3) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) + (other.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3.xyz) + (float3(this.e1234) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (right_dual.e1234 * anti_wedge.scalar) + (right_dual.scalar * anti_wedge.e1234) - (right_dual.e41 * anti_wedge.e23) - (right_dual.e42 * anti_wedge.e31) - (right_dual.e43 * anti_wedge.e12) - (right_dual.e23 * anti_wedge.e41) - (right_dual.e31 * anti_wedge.e42) - (right_dual.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (float3(right_dual.scalar) * anti_wedge.group2.xyz) + (float3(anti_wedge.scalar) * right_dual.group0.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.yzx * anti_wedge.group1.zxy), 0.0) - float4((right_dual.group0.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e43 * this.e412) - (other.e423 * this.e41) - (other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) - (this.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((this.group3.xyz.zxy * other.group4.yzx), other.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group2.xyz) + (other.group4.yzx * this.group4.zxy) - (other.group4.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.e1234) * other.group3.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e1234) * other.group4)
        );
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Plane other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * other.group0.yzx) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
        Origin right_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, anti_wedge.e321 * right_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_dual.e4), 
            /* e41, e42, e43 */
            float3(right_dual.e4) * anti_wedge.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual.e4) * anti_wedge.group3.xyz), 0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Point other) {
        MultiVector anti_wedge = MultiVector(
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
        Plane right_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.group0.xy.xx, anti_wedge.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension MultiVector {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<Origin> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<Origin>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public Flector operator /(Flector rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension Origin {
    public Flector reject_via_origin_from(Flector other) {
        Scalar anti_wedge = Scalar(
            /* scalar */
            other.e321 * this.e4
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_dual.group1.xyz), 0.0)
        );
    }
}
public extension Origin {
    public Origin reject_via_origin_from(Horizon other) {
        return Origin(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
public extension Origin {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_dual.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_dual.group3.xyz), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(right_dual.e4) * anti_wedge.group3.xyz), 0.0) + float4((anti_wedge.group2.xyz.yzx * right_dual.group1.zxy), 0.0) + float4((right_dual.group2.xyz.yzx * anti_wedge.group1.zxy), 0.0) - float4((anti_wedge.group2.xyz.zxy * right_dual.group1.yzx), 0.0) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
public extension Origin {
    public Origin reject_via_origin_from(Plane other) {
        return Origin(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<Plane> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<Plane>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Flector operator /(Flector rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension Plane {
    public Flector reject_via_origin_from(Flector other) {
        Motor anti_wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_dual.e4 * anti_wedge.e23) + (right_dual.e423 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e31) + (right_dual.e431 * anti_wedge.scalar), (right_dual.e4 * anti_wedge.e12) + (right_dual.e412 * anti_wedge.scalar), -(right_dual.e2 * anti_wedge.e31) - (right_dual.e3 * anti_wedge.e12)) + float4((right_dual.group0.zxy * anti_wedge.group0.yzx), right_dual.e321 * anti_wedge.scalar) - (right_dual.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
public extension Plane {
    public Plane reject_via_origin_from(Horizon other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412)), 0.0)
        );
    }
}
public extension Plane {
    public Plane reject_via_origin_from(Line other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
        Line right_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.xyz.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_dual.group0.xyz.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
public extension Plane {
    public Plane reject_via_origin_from(Motor other) {
        Flector anti_wedge = Flector(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group1.xyz), 0.0) + float4((anti_wedge.group0.zxy * right_dual.group0.yzx), 0.0) - float4((anti_wedge.group0.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
public extension Plane {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
            /* scalar, e1234 */
            float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_dual.e42) + (anti_wedge.e4 * right_dual.e23) + (anti_wedge.e42 * right_dual.e3) + (anti_wedge.e23 * right_dual.e4), (anti_wedge.e1 * right_dual.e43) + (anti_wedge.e4 * right_dual.e31) + (anti_wedge.e43 * right_dual.e1) + (anti_wedge.e31 * right_dual.e4), (anti_wedge.e2 * right_dual.e41) + (anti_wedge.e4 * right_dual.e12) + (anti_wedge.e41 * right_dual.e2) + (anti_wedge.e12 * right_dual.e4), -(anti_wedge.e1 * right_dual.e23) - (anti_wedge.e2 * right_dual.e31) - (anti_wedge.e3 * right_dual.e12) - (anti_wedge.e12 * right_dual.e3)) + (float4(anti_wedge.scalar) * right_dual.group4) + (float4(right_dual.scalar) * anti_wedge.group4) - (right_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_dual.e2)
        );
    }
}
public extension Plane {
    public Plane reject_via_origin_from(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3((other.e321 * this.e423) - (other.e423 * this.e321), (other.e321 * this.e431) - (other.e431 * this.e321), (other.e321 * this.e412) - (other.e412 * this.e321))), 0.0)
        );
    }
}
public extension Plane {
    public Plane reject_via_origin_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<Point> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<Point>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Flector operator /(Flector rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension Point {
    public Flector reject_via_origin_from(Flector other) {
        Scalar anti_wedge = Scalar(
            /* scalar */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_dual.group1.xyz), 0.0)
        );
    }
}
public extension Point {
    public Origin reject_via_origin_from(Horizon other) {
        return Origin(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
public extension Point {
    public Plane reject_via_origin_from(Motor other) {
        Point anti_wedge = Point(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_dual.group0.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
public extension Point {
    public MultiVector reject_via_origin_from(MultiVector other) {
        MultiVector anti_wedge = MultiVector(
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
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_dual.e1234) + (anti_wedge.e1234 * right_dual.scalar) + (anti_wedge.e1 * right_dual.e423) + (anti_wedge.e2 * right_dual.e431) + (anti_wedge.e3 * right_dual.e412) + (anti_wedge.e4 * right_dual.e321) - (anti_wedge.e41 * right_dual.e23) - (anti_wedge.e42 * right_dual.e31) - (anti_wedge.e43 * right_dual.e12) - (anti_wedge.e23 * right_dual.e41) - (anti_wedge.e31 * right_dual.e42) - (anti_wedge.e12 * right_dual.e43) - (anti_wedge.e423 * right_dual.e1) - (anti_wedge.e431 * right_dual.e2) - (anti_wedge.e412 * right_dual.e3) - (anti_wedge.e321 * right_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_dual.group1) + (float4(right_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_dual.group2.xyz) + (float3(anti_wedge.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_dual.group1.zxy) - (anti_wedge.group1.zxy * right_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_dual.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_dual.group3.xyz), 0.0) + float4((float3(right_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(right_dual.e4) * anti_wedge.group3.xyz), 0.0) + float4((anti_wedge.group2.xyz.yzx * right_dual.group1.zxy), 0.0) + float4((right_dual.group2.xyz.yzx * anti_wedge.group1.zxy), 0.0) - float4((anti_wedge.group2.xyz.zxy * right_dual.group1.yzx), 0.0) - float4((right_dual.group2.xyz.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
public extension Point {
    public Origin reject_via_origin_from(Plane other) {
        return Origin(
            /* e4 */
            -(pow(other.e321, 2) * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public RejectViaOriginFromInfixPartial<Scalar> operator /(RejectViaOriginFromInfix rhs) {
        return RejectViaOriginFromInfixPartial<Scalar>(this);
    }
}
public extension RejectViaOriginFromInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(DualNum rhs) {
        return this.a.reject_via_origin_from(rhs);
    }
}
public extension Scalar {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.scalar * other.e1234 * this.scalar
        );
    }
}
public extension Scalar {
    public Motor reject_via_origin_from(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234 * this.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Scalar {
    public MultiVector reject_via_origin_from(MultiVector other) {
        Scalar anti_wedge = Scalar(
            /* scalar */
            other.e1234 * this.scalar
        );
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
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, right_dual.e1234 * anti_wedge.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_dual.group1, 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * right_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_dual.group4.xyz), 0.0)
        );
    }
}
