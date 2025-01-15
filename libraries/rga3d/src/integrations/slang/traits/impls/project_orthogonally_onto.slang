implementing rga3d;
using traits;
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<DualNum> operator /(DualNum lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Motor project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Horizon other) {
        return Scalar(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Line other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(this.scalar) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Motor project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * right_anti_dual.e1234) + (this.e1234 * right_anti_dual.scalar)) * float4(right_anti_dual.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public MultiVector project_orthogonally_onto(MultiVector other) {
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
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.scalar * right_anti_dual.e1234) + (this.e1234 * right_anti_dual.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Plane other) {
        return Scalar(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Point other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Scalar(
            /* scalar */
            (wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Flector> operator /(Flector lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            (this.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (right_anti_dual.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)), 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(wedge.e1234) * other.group0) - (other.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Horizon other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * wedge.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * wedge.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Point project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * this.group0.yzx), 0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), 0.0) + float4((float3(right_anti_dual.scalar) * this.group1.xyz), 0.0) + float4((this.group0.zxy * right_anti_dual.group0.yzx), 0.0) - float4((this.group0.yzx * right_anti_dual.group0.zxy), 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(wedge.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
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
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321) - (this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (this.group0.yzx * right_anti_dual.group1.zxy) - (this.group0.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_anti_dual.group3.xyz), 0.0) + float4((float3(right_anti_dual.scalar) * this.group1.xyz), 0.0) + float4((right_anti_dual.group2.xyz.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Plane other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * other.e321) + (wedge.e31 * other.e412), (wedge.e42 * other.e321) + (wedge.e12 * other.e423), (wedge.e43 * other.e321) + (wedge.e23 * other.e431), -(wedge.e42 * other.e431) - (wedge.e43 * other.e412)) - (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Point project_orthogonally_onto(Point other) {
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321)) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Horizon> operator /(Horizon lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Horizon {
    public Flector project_orthogonally_onto(Flector other) {
        AntiScalar wedge = AntiScalar(
            /* e1234 */
            float4(float3(0.0), other.e321 * -1.0)[3] * this.e321 * -1.0
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(ProjectOrthogonallyOntoInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Horizon {
    public Horizon project_orthogonally_onto(Horizon other) {
        return Horizon(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Horizon {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        AntiScalar wedge = AntiScalar(
            /* e1234 */
            float4(float3(0.0), other.e321 * -1.0)[3] * this.e321 * -1.0
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Plane operator /(ProjectOrthogonallyOntoInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Horizon {
    public Plane project_orthogonally_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Line> operator /(Line lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((right_anti_dual.e3 * this.e42) + (right_anti_dual.e4 * this.e23), (right_anti_dual.e1 * this.e43) + (right_anti_dual.e4 * this.e31), (right_anti_dual.e2 * this.e41) + (right_anti_dual.e4 * this.e12), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group0.yzx) - (other.group1.yzx * wedge.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * wedge.e423 * -1.0, other.e321 * wedge.e431 * -1.0, other.e321 * wedge.e412 * -1.0, (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321)) + (wedge.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(Horizon other) {
        Origin right_anti_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * float4(this.e23 * right_anti_dual.e4, this.e31 * right_anti_dual.e4, this.e12 * right_anti_dual.e4, 0.0).xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e1234 */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return Line(
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge.e1234) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e1234 */
            -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(wedge.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(wedge.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public MultiVector project_orthogonally_onto(MultiVector other) {
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
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * right_anti_dual.e3) + (this.e23 * right_anti_dual.e4), (this.e43 * right_anti_dual.e1) + (this.e31 * right_anti_dual.e4), (this.e41 * right_anti_dual.e2) + (this.e12 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (right_anti_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(Plane other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0)
        );
        return Line(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge.group0.yzx) - (other.group0.yzx * wedge.group0.zxy), 
            /* e23, e31, e12 */
            (float3(wedge.e321) * other.group0.xyz) - (float3(other.e321) * wedge.group0.xyz)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Motor> operator /(Motor lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual.e4 * this.e23) + (right_anti_dual.e423 * this.scalar), (right_anti_dual.e4 * this.e31) + (right_anti_dual.e431 * this.scalar), (right_anti_dual.e4 * this.e12) + (right_anti_dual.e412 * this.scalar), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + float4((right_anti_dual.group0.zxy * this.group0.yzx), right_anti_dual.e321 * this.scalar) - (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Horizon other) {
        Origin right_anti_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_anti_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(wedge.group1.xyz, wedge.e4) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e1234) * other.group1.xyz), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            (float4(right_anti_dual.scalar) * this.group0) + (float4(this.scalar) * right_anti_dual.group0) + float4(float3(0.0), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
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
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.e1234 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e1234) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            (float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e423), (this.e31 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e431), (this.e12 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e412), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + float4((this.group0.yzx * right_anti_dual.group1.zxy), this.scalar * right_anti_dual.e321) - (right_anti_dual.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Plane other) {
        Origin right_anti_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_anti_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge.group1.yzx * other.group0.zxy) - (wedge.group1.zxy * other.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge.e321 * other.e423, wedge.e321 * other.e431, wedge.e321 * other.e412, -(wedge.e2 * other.e431) - (wedge.e3 * other.e412) - (wedge.e4 * other.e321)) - (other.group0.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Scalar project_orthogonally_onto(Point other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Scalar(
            /* scalar */
            (wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<MultiVector> operator /(MultiVector lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual.group0.zxy * this.group1.yzx) - (right_anti_dual.group0.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual.e3 * this.e42) + (right_anti_dual.e4 * this.e23), (right_anti_dual.e1 * this.e43) + (right_anti_dual.e4 * this.e31), (right_anti_dual.e2 * this.e41) + (right_anti_dual.e4 * this.e12), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1) - (right_anti_dual.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Horizon other) {
        Origin right_anti_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(1.0, this.e321 * right_anti_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * this.group3.xyz), 0.0)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), 0.0)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual.e1234 * this.scalar) + (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.scalar) * this.group4.xyz), 0.0) + float4((float3(this.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.yzx * this.group1.zxy), 0.0) - float4((right_anti_dual.group0.zxy * this.group1.yzx), 0.0)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
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
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual.scalar * this.e1234) + (right_anti_dual.e1234 * this.scalar) + (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.scalar) * this.group3.xyz) + (float3(this.scalar) * right_anti_dual.group3.xyz) + (right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual.e3 * this.e42) + (right_anti_dual.e4 * this.e23) + (right_anti_dual.e42 * this.e3) + (right_anti_dual.e23 * this.e4), (right_anti_dual.e1 * this.e43) + (right_anti_dual.e4 * this.e31) + (right_anti_dual.e43 * this.e1) + (right_anti_dual.e31 * this.e4), (right_anti_dual.e2 * this.e41) + (right_anti_dual.e4 * this.e12) + (right_anti_dual.e41 * this.e2) + (right_anti_dual.e12 * this.e4), -(right_anti_dual.e1 * this.e23) - (right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12) - (right_anti_dual.e12 * this.e3)) + (float4(right_anti_dual.scalar) * this.group4) + (float4(this.scalar) * right_anti_dual.group4) - (this.group1.yzxx * float4(right_anti_dual.group2.xyz.zxy, right_anti_dual.e23)) - float4((this.group2.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e31 * this.e2)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Plane other) {
        Origin right_anti_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(1.0, this.e321 * right_anti_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * this.group3.xyz), 0.0)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Point other) {
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Origin> operator /(Origin lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Origin {
    public Point project_orthogonally_onto(Flector other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e431 * wedge.e42) - (other.e412 * wedge.e43)) - (other.group1.yzxx * float4(wedge.group1.xyz.zxy, wedge.e41))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Origin {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return MultiVector(
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
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Plane> operator /(Plane lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Flector project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Horizon project_orthogonally_onto(Horizon other) {
        return Horizon(
            /* e321 */
            pow(other.e321, 2) * this.e321
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public MultiVector project_orthogonally_onto(MultiVector other) {
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
        AntiScalar wedge = AntiScalar(
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Plane operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane project_orthogonally_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Point> operator /(Point lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Flector project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e4 * this.e1 * -1.0, right_anti_dual.e4 * this.e2 * -1.0, right_anti_dual.e4 * this.e3 * -1.0, (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)) + (this.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e423)), 
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)), 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e31) + (other.e321 * wedge.e41), (other.e423 * wedge.e12) + (other.e321 * wedge.e42), (other.e431 * wedge.e23) + (other.e321 * wedge.e43), -(other.e431 * wedge.e42) - (other.e412 * wedge.e43)) + (float4(wedge.e1234) * other.group0) - (other.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Point project_orthogonally_onto(Horizon other) {
        Origin right_anti_dual = Origin(
            /* e4 */
            other.e321 * -1.0
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * float3(right_anti_dual.e4 * this.e1 * -1.0, right_anti_dual.e4 * this.e2 * -1.0, right_anti_dual.e4 * this.e3 * -1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Point project_orthogonally_onto(Line other) {
        Line right_anti_dual = Line(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * this.group0.yzx), 0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Flector project_orthogonally_onto(Motor other) {
        Motor right_anti_dual = Motor(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual.group0.zxy * this.group0.yzx), 0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e321) + (other.e31 * wedge.e412), (other.e42 * wedge.e321) + (other.e12 * wedge.e423), (other.e43 * wedge.e321) + (other.e23 * wedge.e431), -(other.e42 * wedge.e431) - (other.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(wedge.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public MultiVector project_orthogonally_onto(MultiVector other) {
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
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * right_anti_dual.group3.xyz), 0.0) + float4((right_anti_dual.group2.xyz.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Point project_orthogonally_onto(Plane other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * other.e321) + (wedge.e31 * other.e412), (wedge.e42 * other.e321) + (wedge.e12 * other.e423), (wedge.e43 * other.e321) + (wedge.e23 * other.e431), -(wedge.e42 * other.e431) - (wedge.e43 * other.e412)) - (other.group0.yzxx * float4(wedge.group1.xyz.zxy, wedge.e41))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Point project_orthogonally_onto(Point other) {
        Plane right_anti_dual = Plane(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Point(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Scalar> operator /(Scalar lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Motor project_orthogonally_onto(Flector other) {
        Flector right_anti_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e431) + (other.e3 * wedge.e412) + (other.e4 * wedge.e321) - (other.e431 * wedge.e2) - (other.e412 * wedge.e3) - (other.e321 * wedge.e4)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Horizon other) {
        return Scalar(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Line other) {
        Line wedge = Line(
            /* e41, e42, e43 */
            float3(this.scalar) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Motor project_orthogonally_onto(Motor other) {
        Motor wedge = Motor(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * other.group0.xyz)), other.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * wedge.group1) + (float4(wedge.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * wedge.e23) - (other.e42 * wedge.e31) - (other.e43 * wedge.e12) - (other.e23 * wedge.e41) - (other.e31 * wedge.e42) - (other.e12 * wedge.e43))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public MultiVector project_orthogonally_onto(MultiVector other) {
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
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(1.0, right_anti_dual.e1234 * this.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.scalar) * right_anti_dual.group4.xyz), 0.0)
        );
        return MultiVector(
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
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Plane other) {
        return Scalar(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Point other) {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        );
        return Scalar(
            /* scalar */
            (wedge.e423 * other.e1) + (wedge.e431 * other.e2) + (wedge.e412 * other.e3) + (wedge.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
