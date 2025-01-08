implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiScalar> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiScalar>(this);
    }
}
public extension WedgeInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(DualNum rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiScalar {
    public AntiScalar wedge(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension AntiScalar {
    public AntiScalar wedge(Motor other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension AntiScalar {
    public AntiScalar wedge(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension AntiScalar {
    public AntiScalar wedge(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<DualNum> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<DualNum>(this);
    }
}
public extension WedgeInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.wedge(rhs);
    }
}
public extension DualNum {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension DualNum {
    public DualNum wedge(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
public extension DualNum {
    public Flector wedge(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension DualNum {
    public Horizon wedge(Horizon other) {
        return Horizon(
            /* e321 */
            this.scalar * other.e321
        );
    }
}
public extension DualNum {
    public Line wedge(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension DualNum {
    public Motor wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
public extension DualNum {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension DualNum {
    public Origin wedge(Origin other) {
        return Origin(
            /* e4 */
            this.scalar * other.e4
        );
    }
}
public extension DualNum {
    public Plane wedge(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension DualNum {
    public Point wedge(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension DualNum {
    public DualNum wedge(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Flector> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Flector>(this);
    }
}
public extension WedgeInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Flector operator /(DualNum rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Flector {
    public Flector wedge(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Flector {
    public Motor wedge(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
public extension Flector {
    public AntiScalar wedge(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
public extension Flector {
    public Plane wedge(Line other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Flector {
    public Flector wedge(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e4 * other.e23) + (this.e423 * other.scalar), (this.e4 * other.e31) + (this.e431 * other.scalar), (this.e4 * other.e12) + (this.e412 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group0.zxy * other.group0.yzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Flector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension Flector {
    public Motor wedge(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Flector {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)
        );
    }
}
public extension Flector {
    public Motor wedge(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * other.e1, this.e4 * other.e2, this.e4 * other.e3, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public Flector wedge(Scalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Horizon> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Horizon>(this);
    }
}
public extension WedgeInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Horizon operator /(DualNum rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Horizon {
    public Horizon wedge(DualNum other) {
        return Horizon(
            /* e321 */
            other.scalar * this.e321
        );
    }
}
public extension Horizon {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
public extension Horizon {
    public Horizon wedge(Motor other) {
        return Horizon(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
public extension Horizon {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension Horizon {
    public AntiScalar wedge(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Horizon {
    public AntiScalar wedge(Point other) {
        return AntiScalar(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Horizon {
    public Horizon wedge(Scalar other) {
        return Horizon(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Line> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Line>(this);
    }
}
public extension WedgeInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Line operator /(DualNum rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Line {
    public Line wedge(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension Line {
    public Plane wedge(Flector other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Line {
    public AntiScalar wedge(Line other) {
        return AntiScalar(
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension Line {
    public Motor wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension Line {
    public Plane wedge(Origin other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
public extension Line {
    public Plane wedge(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Line {
    public Line wedge(Scalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Motor> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Motor>(this);
    }
}
public extension WedgeInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Motor {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension Motor {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Motor {
    public Flector wedge(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar), (other.e4 * this.e31) + (other.e431 * this.scalar), (other.e4 * this.e12) + (other.e412 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension Motor {
    public Horizon wedge(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
public extension Motor {
    public Motor wedge(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public Motor wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.scalar)
        );
    }
}
public extension Motor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension Motor {
    public Flector wedge(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
public extension Motor {
    public Plane wedge(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Motor {
    public Flector wedge(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension Motor {
    public Motor wedge(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<MultiVector> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<MultiVector>(this);
    }
}
public extension WedgeInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.wedge(rhs);
    }
}
public extension MultiVector {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(DualNum other) {
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector wedge(Flector other) {
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector wedge(Horizon other) {
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector wedge(Line other) {
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector wedge(Motor other) {
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector wedge(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, this.e321 * other.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group3.xyz), 0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(Plane other) {
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector wedge(Point other) {
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector wedge(Scalar other) {
        return MultiVector(
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
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Origin> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Origin>(this);
    }
}
public extension WedgeInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public Origin operator /(DualNum rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Origin {
    public Origin wedge(DualNum other) {
        return Origin(
            /* e4 */
            other.scalar * this.e4
        );
    }
}
public extension Origin {
    public Motor wedge(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Origin {
    public AntiScalar wedge(Horizon other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
public extension Origin {
    public Plane wedge(Line other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
public extension Origin {
    public Flector wedge(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
public extension Origin {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension Origin {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
public extension Origin {
    public Line wedge(Point other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Origin {
    public Origin wedge(Scalar other) {
        return Origin(
            /* e4 */
            this.e4 * other.scalar
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Plane> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Plane>(this);
    }
}
public extension WedgeInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(DualNum rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Plane {
    public Plane wedge(DualNum other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
    }
}
public extension Plane {
    public Plane wedge(Motor other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension Plane {
    public AntiScalar wedge(Origin other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
public extension Plane {
    public AntiScalar wedge(Point other) {
        return AntiScalar(
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension Plane {
    public Plane wedge(Scalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Point> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Point>(this);
    }
}
public extension WedgeInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Point operator /(DualNum rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Point {
    public Point wedge(DualNum other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Point {
    public Motor wedge(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * this.e1 * -1.0, other.e4 * this.e2 * -1.0, other.e4 * this.e3 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
public extension Point {
    public AntiScalar wedge(Horizon other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
public extension Point {
    public Plane wedge(Line other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Point {
    public Flector wedge(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Point {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension Point {
    public Line wedge(Origin other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Point {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension Point {
    public Line wedge(Point other) {
        return Line(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
    }
}
public extension Point {
    public Point wedge(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Scalar> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Scalar>(this);
    }
}
public extension WedgeInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Scalar {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar {
    public DualNum wedge(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar {
    public Flector wedge(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar {
    public Horizon wedge(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
public extension Scalar {
    public Line wedge(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension Scalar {
    public Motor wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
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
    }
}
public extension Scalar {
    public Origin wedge(Origin other) {
        return Origin(
            /* e4 */
            other.e4 * this.scalar
        );
    }
}
public extension Scalar {
    public Plane wedge(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar {
    public Point wedge(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar {
    public Scalar wedge(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
