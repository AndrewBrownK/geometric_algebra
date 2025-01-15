implementing rga3d;
using traits;
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_reject_orthogonally_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public Motor anti_reject_orthogonally_from(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234 * other.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        AntiScalar wedge = AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar
        );
        MultiVector right_anti_dual = MultiVector(
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
            float2(1.0, wedge.e1234 * right_anti_dual.e1234) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            float3(wedge.e1234) * right_anti_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(wedge.e1234) * right_anti_dual.group4.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_reject_orthogonally_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<DualNum> operator /(DualNum lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_orthogonally_from(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_orthogonally_from(Flector other) {
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group1.zxy) - (right_anti_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_orthogonally_from(Horizon other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_orthogonally_from(Line other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_orthogonally_from(Motor other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_orthogonally_from(Point other) {
        Point wedge = Point(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_orthogonally_from(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * float2(this.scalar * other.scalar, this.e1234 * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Flector> operator /(Flector lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(DualNum other) {
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(Flector other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Origin anti_reject_orthogonally_from(Horizon other) {
        return Origin(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Point anti_reject_orthogonally_from(Line other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(Motor other) {
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e4 * other.e23) + (this.e423 * other.scalar), (this.e4 * other.e31) + (this.e431 * other.scalar), (this.e4 * other.e12) + (this.e412 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group0.zxy * other.group0.yzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * right_anti_dual.e31) + (wedge.e321 * right_anti_dual.e41), (wedge.e423 * right_anti_dual.e12) + (wedge.e321 * right_anti_dual.e42), (wedge.e431 * right_anti_dual.e23) + (wedge.e321 * right_anti_dual.e43), -(wedge.e431 * right_anti_dual.e42) - (wedge.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Origin anti_reject_orthogonally_from(Plane other) {
        return Origin(
            /* e4 */
            -(pow(other.e321, 2) * this.e4) - (this.e1 * other.e423 * other.e321) - (this.e2 * other.e431 * other.e321) - (this.e3 * other.e412 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(Point other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * other.e1, this.e4 * other.e2, this.e4 * other.e3, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_anti_dual.e321) + (wedge.e31 * right_anti_dual.e412), (wedge.e42 * right_anti_dual.e321) + (wedge.e12 * right_anti_dual.e423), (wedge.e43 * right_anti_dual.e321) + (wedge.e23 * right_anti_dual.e431), -(wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(Scalar other) {
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Horizon> operator /(Horizon lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Horizon operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Horizon anti_reject_orthogonally_from(DualNum other) {
        return Horizon(
            /* e321 */
            pow(other.scalar, 2) * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Flector anti_reject_orthogonally_from(Flector other) {
        AntiScalar wedge = AntiScalar(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(wedge.e1234) * right_anti_dual.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Flector anti_reject_orthogonally_from(Motor other) {
        Horizon wedge = Horizon(
            /* e321 */
            this.e321 * other.scalar
        );
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(wedge.e321) * right_anti_dual.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge.e321 * right_anti_dual.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Plane anti_reject_orthogonally_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Horizon anti_reject_orthogonally_from(Scalar other) {
        return Horizon(
            /* e321 */
            this.e321 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Line> operator /(Line lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(DualNum other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
        AntiScalar right_anti_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Line(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * wedge.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor anti_reject_orthogonally_from(Flector other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group0.zxy) - (right_anti_dual.group1.zxy * wedge.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e321 * wedge.e423, right_anti_dual.e321 * wedge.e431, right_anti_dual.e321 * wedge.e412, -(right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor anti_reject_orthogonally_from(Motor other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(Point other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Line(
            /* e41, e42, e43 */
            (right_anti_dual.group0.yzx * wedge.group0.zxy) - (right_anti_dual.group0.zxy * wedge.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * wedge.group0.xyz) - (float3(wedge.e321) * right_anti_dual.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(Scalar other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
        AntiScalar right_anti_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Line(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * wedge.group1.xyz
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Motor> operator /(Motor lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(DualNum other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Flector other) {
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar), (other.e4 * this.e31) + (other.e431 * this.scalar), (other.e4 * this.e12) + (other.e412 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group1.zxy) - (right_anti_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Scalar anti_reject_orthogonally_from(Horizon other) {
        return Scalar(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Line other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Motor other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.scalar)
        );
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Point other) {
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge.group1.zxy * right_anti_dual.group0.yzx) - (wedge.group1.yzx * right_anti_dual.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge.e321 * right_anti_dual.e423 * -1.0, wedge.e321 * right_anti_dual.e431 * -1.0, wedge.e321 * right_anti_dual.e412 * -1.0, (wedge.e2 * right_anti_dual.e431) + (wedge.e3 * right_anti_dual.e412) + (wedge.e4 * right_anti_dual.e321)) + (right_anti_dual.group0.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Scalar other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<MultiVector> operator /(MultiVector lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(DualNum other) {
        MultiVector wedge = MultiVector(
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
        AntiScalar right_anti_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(right_anti_dual.e1234) * wedge.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group1, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * wedge.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * wedge.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Flector other) {
        MultiVector wedge = MultiVector(
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
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (right_anti_dual.group1.yzx * wedge.group4.zxy) - (right_anti_dual.group1.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * wedge.group4.xyz) - (float3(wedge.e321) * right_anti_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0.xy.yy, wedge.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Horizon other) {
        MultiVector wedge = MultiVector(
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
        Origin right_anti_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(wedge.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge.e1234 * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Line other) {
        MultiVector wedge = MultiVector(
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
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group4.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            float3(wedge.e1234) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Motor other) {
        MultiVector wedge = MultiVector(
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
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.scalar * wedge.e1234) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e1234 * wedge.e1) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e1234 * wedge.e2) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e1234 * wedge.e3) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) + (right_anti_dual.group0 * float4(wedge.group4.www, wedge.e4)) - (wedge.group4.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * right_anti_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Plane other) {
        MultiVector wedge = MultiVector(
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
        Origin right_anti_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(wedge.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge.e1234 * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Point other) {
        MultiVector wedge = MultiVector(
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
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge.e1 * right_anti_dual.e423) + (wedge.e2 * right_anti_dual.e431) + (wedge.e3 * right_anti_dual.e412) + (wedge.e4 * right_anti_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_anti_dual.e321) + (wedge.e31 * right_anti_dual.e412), (wedge.e42 * right_anti_dual.e321) + (wedge.e12 * right_anti_dual.e423), (wedge.e43 * right_anti_dual.e321) + (wedge.e23 * right_anti_dual.e431), -(wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (wedge.group4.zxy * right_anti_dual.group0.yzx) - (wedge.group4.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * wedge.group4.xyz) - (float3(wedge.e321) * right_anti_dual.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0.xy.yy, wedge.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Scalar other) {
        MultiVector wedge = MultiVector(
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
        AntiScalar right_anti_dual = AntiScalar(
            /* e1234 */
            other.scalar
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(right_anti_dual.e1234) * wedge.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group1, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * wedge.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * wedge.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group4
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Origin> operator /(Origin lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(DualNum other) {
        return Origin(
            /* e4 */
            pow(other.scalar, 2) * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Flector anti_reject_orthogonally_from(Flector other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Horizon other) {
        return Origin(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Line other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Origin(
            /* e4 */
            -(right_anti_dual.e41 * wedge.e423) - (right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Flector anti_reject_orthogonally_from(Motor other) {
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * right_anti_dual.e31) + (wedge.e321 * right_anti_dual.e41), (wedge.e423 * right_anti_dual.e12) + (wedge.e321 * right_anti_dual.e42), (wedge.e431 * right_anti_dual.e23) + (wedge.e321 * right_anti_dual.e43), -(wedge.e431 * right_anti_dual.e42) - (wedge.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(wedge.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(right_anti_dual.e1234) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group4.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Plane other) {
        return Origin(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Point other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Origin(
            /* e4 */
            -(wedge.e41 * right_anti_dual.e423) - (wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Scalar other) {
        return Origin(
            /* e4 */
            this.e4 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Plane> operator /(Plane lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane anti_reject_orthogonally_from(DualNum other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(other.scalar * this.e423, other.scalar * this.e431, other.scalar * this.e412, other.scalar * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector anti_reject_orthogonally_from(Flector other) {
        AntiScalar wedge = AntiScalar(
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(wedge.e1234) * right_anti_dual.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector anti_reject_orthogonally_from(Motor other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane anti_reject_orthogonally_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane anti_reject_orthogonally_from(Scalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(this.e423 * other.scalar, this.e431 * other.scalar, this.e412 * other.scalar, this.e321 * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Point> operator /(Point lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Point anti_reject_orthogonally_from(DualNum other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, other.scalar * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Flector anti_reject_orthogonally_from(Flector other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * this.e1 * -1.0, other.e4 * this.e2 * -1.0, other.e4 * this.e3 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Origin anti_reject_orthogonally_from(Horizon other) {
        return Origin(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Point anti_reject_orthogonally_from(Line other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Flector anti_reject_orthogonally_from(Motor other) {
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * right_anti_dual.e31) + (wedge.e321 * right_anti_dual.e41), (wedge.e423 * right_anti_dual.e12) + (wedge.e321 * right_anti_dual.e42), (wedge.e431 * right_anti_dual.e23) + (wedge.e321 * right_anti_dual.e43), -(wedge.e431 * right_anti_dual.e42) - (wedge.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Origin anti_reject_orthogonally_from(Plane other) {
        return Origin(
            /* e4 */
            -(pow(other.e321, 2) * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Point anti_reject_orthogonally_from(Point other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_anti_dual.e321) + (wedge.e31 * right_anti_dual.e412), (wedge.e42 * right_anti_dual.e321) + (wedge.e12 * right_anti_dual.e423), (wedge.e43 * right_anti_dual.e321) + (wedge.e23 * right_anti_dual.e431), -(wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(wedge.group1.xyz.zxy, wedge.e41))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Point anti_reject_orthogonally_from(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.e1 * other.scalar, this.e2 * other.scalar, this.e3 * other.scalar, this.e4 * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Scalar> operator /(Scalar lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public DualNum anti_reject_orthogonally_from(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * float2(other.scalar * this.scalar, other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Motor anti_reject_orthogonally_from(Flector other) {
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group1.zxy) - (right_anti_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Horizon other) {
        return Scalar(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Line other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Motor anti_reject_orthogonally_from(Motor other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        MultiVector wedge = MultiVector(
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
        MultiVector right_anti_dual = MultiVector(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        return Scalar(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Point other) {
        Point wedge = Point(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
