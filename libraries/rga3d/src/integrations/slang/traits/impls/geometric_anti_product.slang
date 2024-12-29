implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<AntiScalar> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<AntiScalar>(this);
    }
}
public extension GeometricAntiProductInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension AntiScalar {
    public AntiScalar geometric_anti_product(AntiScalar other) {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension AntiScalar {
    public DualNum geometric_anti_product(DualNum other) {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0.xy
        );
    }
}
public extension AntiScalar {
    public Flector geometric_anti_product(Flector other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
public extension AntiScalar {
    public Horizon geometric_anti_product(Horizon other) {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
public extension AntiScalar {
    public Line geometric_anti_product(Line other) {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
    }
}
public extension AntiScalar {
    public Motor geometric_anti_product(Motor other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
    }
}
public extension AntiScalar {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
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
    }
}
public extension AntiScalar {
    public Origin geometric_anti_product(Origin other) {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
public extension AntiScalar {
    public Plane geometric_anti_product(Plane other) {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension AntiScalar {
    public Point geometric_anti_product(Point other) {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension AntiScalar {
    public Scalar geometric_anti_product(Scalar other) {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<DualNum> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<DualNum>(this);
    }
}
public extension GeometricAntiProductInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public DualNum operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension DualNum {
    public DualNum geometric_anti_product(AntiScalar other) {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0.xy
        );
    }
}
public extension DualNum {
    public DualNum geometric_anti_product(DualNum other) {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234)
        );
    }
}
public extension DualNum {
    public Flector geometric_anti_product(Flector other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(this.e1234) * other.group0.xyz)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321)) * float4(other.group1.xyz, 1.0)
        );
    }
}
public extension DualNum {
    public Horizon geometric_anti_product(Horizon other) {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
public extension DualNum {
    public Line geometric_anti_product(Line other) {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz)
        );
    }
}
public extension DualNum {
    public Motor geometric_anti_product(Motor other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1)
        );
    }
}
public extension DualNum {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group4.xyz) + (float3(this.e1234) * other.group1.xyz)), this.e1234 * other.e4), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321)) * float4(other.group4.xyz, 1.0)
        );
    }
}
public extension DualNum {
    public Flector geometric_anti_product(Origin other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        );
    }
}
public extension DualNum {
    public Flector geometric_anti_product(Plane other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension DualNum {
    public Flector geometric_anti_product(Point other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        );
    }
}
public extension DualNum {
    public Scalar geometric_anti_product(Scalar other) {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Flector> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Flector>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Flector operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Flector {
    public Flector geometric_anti_product(AntiScalar other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
public extension Flector {
    public Flector geometric_anti_product(DualNum other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) - (float3(other.scalar) * this.group1.xyz)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xy.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4)) * float4(this.group1.xyz, 1.0)
        );
    }
}
public extension Flector {
    public Motor geometric_anti_product(Flector other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e423 * this.e4) - (other.e412 * this.e431), -(other.e423 * this.e412) - (other.e431 * this.e4), -(other.e431 * this.e423) - (other.e412 * this.e4), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group1.zxyx) - (float4(other.e4) * float4(this.group1.xyz, this.e4)), 
            /* e23, e31, e12, scalar */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (float4(this.e4, this.e412, this.e423, this.e1) * float4(other.group0.xxy, other.e423)) + (float4(this.e431, this.e4, this.e4, this.e2) * float4(other.group0.zyz, other.e431)) + (other.group1.yzxz * this.group0.zxyz) - (float4(this.e2, this.e321, this.e321, this.e321) * float4(other.group1.zyz, other.e4)) - (float4(this.e321, this.e3, this.e1, this.e412) * float4(other.group1.xxy, other.e3)) - (other.group0.yzxx * this.group1.zxyx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431))
        );
    }
}
public extension Flector {
    public Motor geometric_anti_product(Horizon other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
    }
}
public extension Flector {
    public Flector geometric_anti_product(Line other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xyz.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.xyz.zyz, other.e43)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e423 * other.e41), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xyz.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.xyz.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group0.xyz.yzx * this.group1.zxy), this.e1 * other.e41)
        );
    }
}
public extension Flector {
    public Flector geometric_anti_product(Motor other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e412 * other.e23), (this.e3 * other.e1234) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e412 * other.scalar), 0.0) + (this.group0.xyxw * other.group0.wwyw) - (this.group1.xyxz * float4(other.group1.wwy, other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e423)) - float4((this.group0.www * other.group1.xyz), this.e431 * other.e42), 
            /* e423, e431, e412, e321 */
            (this.group1.xyxy * float4(other.group0.wwy, other.e31)) + (this.group1.yzzz * float4(other.group0.zxw, other.e12)) + float4(float3(0.0), (this.e321 * other.e1234) - (this.e2 * other.e42) - (this.e3 * other.e43) - (this.e4 * other.scalar)) + float4((this.group0.www * other.group0.xyz), this.e423 * other.e23) - (other.group0.yzxx * float4(this.group1.zxy, this.e1))
        );
    }
}
public extension Flector {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e4 * other.e31), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e4 * other.e12), 0.0) + (float4(other.e1234) * this.group0) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(other.group3.xyz.xxy, other.e43)) - (this.group1.xyzx * float4(other.group0.xy.xx, other.scalar, other.e41)) - float4((other.group2.xyz.yzx * this.group0.zxy), this.e431 * other.e42), 
            /* e41, e42, e43 */
            (this.group1.zxy * other.group4.yzx) - (float3(this.e4) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group1.xyx) - (float3(other.e412, other.e423, other.e4) * this.group1.yzz), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.e3, other.e1, other.e321) * this.group1.yzz) + (float3(other.e321, other.e321, other.e2) * this.group1.xyx) + (this.group0.zxy * other.group4.yzx) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group0.xyx) - (float3(other.e412, other.e423, other.e4) * this.group0.yzz) - (this.group1.zxy * other.group1.yzx), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group1) + (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group2.xyz.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group2.xyz.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group2.xyz.yzx * this.group1.zxy), this.e4 * other.scalar)
        );
    }
}
public extension Flector {
    public Motor geometric_anti_product(Origin other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
public extension Flector {
    public Motor geometric_anti_product(Plane other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e4 * other.e423) - (this.e431 * other.e412), -(this.e4 * other.e431) - (this.e412 * other.e423), -(this.e4 * other.e412) - (this.e423 * other.e431), (this.e431 * other.e431) + (this.e412 * other.e412)) + (this.group1.zxyx * other.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(this.e2 * other.e412) - (this.e321 * other.e423), -(this.e3 * other.e423) - (this.e321 * other.e431), -(this.e1 * other.e431) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.zxyx * other.group0.yzxx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2))
        );
    }
}
public extension Flector {
    public Motor geometric_anti_product(Point other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((this.e4 * other.e1) + (this.e431 * other.e3), (this.e4 * other.e2) + (this.e412 * other.e1), (this.e4 * other.e3) + (this.e423 * other.e2), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.zxyy * other.group0.yzxy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423))
        );
    }
}
public extension Flector {
    public Flector geometric_anti_product(Scalar other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Horizon> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Horizon>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Horizon operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Horizon {
    public Horizon geometric_anti_product(AntiScalar other) {
        return Horizon.from_groups(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
public extension Horizon {
    public Horizon geometric_anti_product(DualNum other) {
        return Horizon.from_groups(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
public extension Horizon {
    public Motor geometric_anti_product(Flector other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
    }
}
public extension Horizon {
    public Point geometric_anti_product(Line other) {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
public extension Horizon {
    public Flector geometric_anti_product(Motor other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
public extension Horizon {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
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
    }
}
public extension Horizon {
    public Scalar geometric_anti_product(Origin other) {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Horizon {
    public Line geometric_anti_product(Plane other) {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        );
    }
}
public extension Horizon {
    public Scalar geometric_anti_product(Point other) {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Line> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Line>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Line operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Line {
    public Line geometric_anti_product(AntiScalar other) {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz
        );
    }
}
public extension Line {
    public Line geometric_anti_product(DualNum other) {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz)
        );
    }
}
public extension Line {
    public Flector geometric_anti_product(Flector other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) - (other.group1.yzxy * float4(this.group1.xyz.zxy, this.e42)) - float4((this.group0.xyz.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e41) + (other.e412 * this.e42), (other.e4 * this.e42) + (other.e423 * this.e43), (other.e4 * this.e43) + (other.e431 * this.e41), -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group0.xyz.zxy * other.group1.yzx), other.e1 * this.e41)
        );
    }
}
public extension Line {
    public Point geometric_anti_product(Horizon other) {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0)
        );
    }
}
public extension Line {
    public Motor geometric_anti_product(Line other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e43 * this.e42, other.e41 * this.e43, other.e42 * this.e41, -(other.e42 * this.e42) - (other.e43 * this.e43)) - float4((other.group0.xyz.yzx * this.group0.xyz.zxy), other.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((other.e43 * this.e31) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e23 * this.e43), (other.e42 * this.e23) + (other.e31 * this.e41), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e41 * this.e23) - float4((other.group1.xyz.yzx * this.group0.xyz.zxy), other.e42 * this.e31)
        );
    }
}
public extension Line {
    public Motor geometric_anti_product(Motor other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.e1234) + (this.e42 * other.e43), (this.e42 * other.e1234) + (this.e43 * other.e41), (this.e41 * other.e42) + (this.e43 * other.e1234), -(this.e42 * other.e42) - (this.e43 * other.e43)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e41)), 
            /* e23, e31, e12, scalar */
            float4((this.e41 * other.scalar) + (this.e42 * other.e12) + (this.e23 * other.e1234) + (this.e31 * other.e43), (this.e42 * other.scalar) + (this.e43 * other.e23) + (this.e31 * other.e1234) + (this.e12 * other.e41), (this.e41 * other.e31) + (this.e43 * other.scalar) + (this.e23 * other.e42) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e41)) - float4((this.group1.xyz.zxy * other.group0.yzx), this.e42 * other.e31)
        );
    }
}
public extension Line {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e42 * other.e3) + (this.e23 * other.e4) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e43 * other.e1) + (this.e31 * other.e4) + (this.e12 * other.e423), (this.e41 * other.e2) + (this.e43 * other.e321) + (this.e23 * other.e431) + (this.e12 * other.e4), this.e43 * other.e412 * -1.0) - (other.group4.yzxy * float4(this.group1.xyz.zxy, this.e42)) - float4((this.group0.xyz.zxy * other.group1.yzx), this.e41 * other.e423), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group0.xyz) + (this.group0.xyz.yzx * other.group2.xyz.zxy) - (this.group0.xyz.zxy * other.group2.xyz.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.xyz.zxy) + (this.group1.xyz.yzx * other.group2.xyz.zxy) - (this.group0.xyz.zxy * other.group3.xyz.yzx) - (this.group1.xyz.zxy * other.group2.xyz.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e41 * other.e4) + (this.e42 * other.e412), (this.e42 * other.e4) + (this.e43 * other.e423), (this.e41 * other.e431) + (this.e43 * other.e4), -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group0.xyz.zxy * other.group4.yzx), this.e41 * other.e1)
        );
    }
}
public extension Line {
    public Flector geometric_anti_product(Origin other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), 0.0)
        );
    }
}
public extension Line {
    public Flector geometric_anti_product(Plane other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Line {
    public Flector geometric_anti_product(Point other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.xyz.yzx * other.group0.zxy), 0.0) - float4((this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), -(this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        );
    }
}
public extension Line {
    public Line geometric_anti_product(Scalar other) {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Motor> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Motor>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Motor {
    public Motor geometric_anti_product(AntiScalar other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        );
    }
}
public extension Motor {
    public Motor geometric_anti_product(DualNum other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1)
        );
    }
}
public extension Motor {
    public Flector geometric_anti_product(Flector other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e431 * this.scalar) + (other.e321 * this.e42), (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e412 * this.scalar) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (other.group0.xxyw * this.group0.wzxw) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - (this.group0.zxyx * float4(other.group0.yzx, other.e423)), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e431 * this.e1234, other.e412 * this.e1234, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(other.e4) * float4(this.group0.xyz, this.scalar)) + (other.group1.xxyw * this.group0.wzxw) - (this.group0.zxyx * float4(other.group1.yzx, other.e1))
        );
    }
}
public extension Motor {
    public Flector geometric_anti_product(Horizon other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
public extension Motor {
    public Motor geometric_anti_product(Line other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.e1234) + (other.e43 * this.e42), (other.e41 * this.e43) + (other.e42 * this.e1234), (other.e42 * this.e41) + (other.e43 * this.e1234), -(other.e42 * this.e42) - (other.e43 * this.e43)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e41)), 
            /* e23, e31, e12, scalar */
            float4((other.e41 * this.scalar) + (other.e43 * this.e31) + (other.e23 * this.e1234) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e42 * this.scalar) + (other.e23 * this.e43) + (other.e31 * this.e1234), (other.e42 * this.e23) + (other.e43 * this.scalar) + (other.e31 * this.e41) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.zxyx * float4(other.group0.xyz.yzx, other.e41)) - float4((other.group1.xyz.yzx * this.group0.zxy), other.e42 * this.e31)
        );
    }
}
public extension Motor {
    public Motor geometric_anti_product(Motor other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e43 * this.e42) + (other.e1234 * this.e41), (other.e42 * this.e1234) + (other.e1234 * this.e42), (other.e43 * this.e1234) + (other.e1234 * this.e43), -(other.e42 * this.e42) - (other.e43 * this.e43)) + (other.group0.xxyw * this.group0.wzxw) - (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e12 * this.e42) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e23 * this.e43) + (other.e31 * this.e1234) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e31 * this.e41) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xxyw * this.group1.wzxw) + float4((other.group0.zyz * this.group1.yww), other.scalar * this.e1234) - (other.group0.yzxx * this.group1.zxyx) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        );
    }
}
public extension Motor {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12), 0.0) + (float2(this.e1234) * other.group0.xy) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e42 * other.e3) + (this.e1234 * other.e1) + (this.e23 * other.e4) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e43 * other.e1) + (this.e1234 * other.e2) + (this.e31 * other.e4) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e1234 * other.e3) + (this.e23 * other.e431) + (this.e12 * other.e4) + (this.scalar * other.e412), this.e43 * other.e412 * -1.0) + (this.group0.xyxw * float4(other.group4.ww, other.e2, other.e4)) - (this.group0.zxyx * float4(other.group1.yzx, other.e423)) - (other.group4.yzxy * float4(this.group1.zxy, this.e42)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group0.xyz) + (other.group2.xyz.xxy * this.group0.wzx) + (other.group2.xyz.zyz * this.group0.yww) - (other.group2.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xyz.xxy * this.group1.wzx) + (other.group2.xyz.zyz * this.group1.yww) + (other.group3.xyz.xxy * this.group0.wzx) + (other.group3.xyz.zyz * this.group0.yww) - (other.group2.xyz.yzx * this.group1.zxy) - (other.group3.xyz.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.xyxw * float4(other.group1.ww, other.e431, other.e321)) + (float4(other.group4.zx, other.e4, other.e4) * float4(this.group0.yzz, this.scalar)) - (this.group0.zxyx * float4(other.group4.yzx, other.e1))
        );
    }
}
public extension Motor {
    public Flector geometric_anti_product(Origin other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group1.xyz, this.e1234), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group0.xyz, this.scalar)
        );
    }
}
public extension Motor {
    public Flector geometric_anti_product(Plane other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e42 * other.e321) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e23 * other.e431) + (this.scalar * other.e412), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.yzxw * other.group0.zxyw) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension Motor {
    public Flector geometric_anti_product(Point other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        );
    }
}
public extension Motor {
    public Motor geometric_anti_product(Scalar other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<MultiVector> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<MultiVector>(this);
    }
}
public extension GeometricAntiProductInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(AntiScalar other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(DualNum other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group1.xyz) - (float3(other.scalar) * this.group4.xyz)), other.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xy.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4)) * float4(this.group4.xyz, 1.0)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Flector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group0.xw), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1234) + (other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (float4(this.group0.xy.xx, this.scalar, this.e1234) * float4(other.group1.xyz, other.e4)) - (other.group1.yzxy * float4(this.group3.xyz.zxy, this.e42)) - float4((this.group2.xyz.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e41, e42, e43 */
            (other.group1.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group1.xxy) - (float3(this.e431, this.e4, this.e4) * other.group1.zyz), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e412, this.e423) * other.group0.xxy) + (float3(this.e431, this.e4, this.e4) * other.group0.zyz) + (other.group1.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group1.zyz) - (float3(this.e321, this.e3, this.e1) * other.group1.xxy) - (other.group0.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e423 * this.e43, other.e4 * this.e43, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(other.group1.xyz, other.e4)) + (float4(other.group0.ww, other.e431, other.e321) * float4(this.group2.xyz.xyx, this.e1234)) - float4((this.group2.xyz.zxy * other.group1.yzx), other.e1 * this.e41)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Horizon other) {
        return MultiVector.from_groups(
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
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Line other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412), (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423), (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xyz.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.xyz.zyz, other.e43)) - float4((other.group0.xyz.yzx * this.group1.zxy), other.e41 * this.e423), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group0.xyz) + (other.group0.xyz.zxy * this.group2.xyz.yzx) - (other.group0.xyz.yzx * this.group2.xyz.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.xyz.yzx) + (other.group1.xyz.zxy * this.group2.xyz.yzx) - (other.group0.xyz.yzx * this.group3.xyz.zxy) - (other.group1.xyz.yzx * this.group2.xyz.zxy), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xyz.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.xyz.zyz, other.e31)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3)) - float4((other.group0.xyz.yzx * this.group4.zxy), other.e41 * this.e1)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Motor other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12), 0.0) + (float2(other.e1234) * this.group0.xy) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e43 * this.e2) + (other.e1234 * this.e1) + (other.e31 * this.e412) - (other.scalar * this.e423), (other.e42 * this.e321) + (other.e1234 * this.e2) + (other.e12 * this.e423) - (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) - (other.scalar * this.e412), 0.0) + (float4(this.e321, this.e3, this.e1, this.e4) * other.group0.xxyw) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.zyz, other.e43)) - (other.group0.yzxx * float4(this.group1.zxy, this.e423)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group0.xyz) + (this.group2.xyz.xyx * other.group0.wwy) + (this.group2.xyz.yzz * other.group0.zxw) - (this.group2.xyz.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xyz.xyx * other.group1.wwy) + (this.group2.xyz.yzz * other.group1.zxw) + (this.group3.xyz.xyx * other.group0.wwy) + (this.group3.xyz.yzz * other.group0.zxw) - (this.group2.xyz.zxy * other.group1.yzx) - (this.group3.xyz.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e321) * other.group0.xxyw) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(other.group0.zyz, other.e23)) + (this.group4.xyzy * float4(other.group0.www, other.e31)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4)) - (other.group0.yzxx * float4(this.group4.zxy, this.e1))
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * other.group0.xy) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * other.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1234) + (other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), 0.0) + (float4(other.e1234) * this.group1) + (float4(this.group0.xy.xx, this.scalar, this.e1234) * float4(other.group4.xyz, other.e4)) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(other.group3.xyz.xxy, other.e43)) - (float4(this.e431, this.e4, this.e4, other.e423) * float4(other.group3.xyz.zyz, this.e41)) - (other.group4.yzxz * float4(this.group3.xyz.zxy, this.e43)) - (this.group4.xyzx * float4(other.group0.xy.xx, other.scalar, other.e41)) - float4((other.group2.xyz.yzx * this.group1.zxy), other.e42 * this.e431) - float4((this.group2.xyz.zxy * other.group1.yzx), other.e431 * this.e42), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group2.xyz) + (other.group2.xyz.zxy * this.group2.xyz.yzx) + (other.group4.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group4.xxy) - (float3(this.e431, this.e4, this.e4) * other.group4.zyz) - (other.group2.xyz.yzx * this.group2.xyz.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz) + (float3(this.e4, this.e412, this.e423) * other.group1.xxy) + (float3(this.e431, this.e4, this.e4) * other.group1.zyz) + (other.group2.xyz.zxy * this.group3.xyz.yzx) + (other.group3.xyz.zxy * this.group2.xyz.yzx) + (other.group4.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group4.zyz) - (float3(this.e321, this.e3, this.e1) * other.group4.xxy) - (other.group2.xyz.yzx * this.group3.xyz.zxy) - (other.group3.xyz.yzx * this.group2.xyz.zxy) - (other.group1.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, other.e321) * float4(other.group2.xyz.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(other.group2.xyz.zyz, other.e23)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(other.group4.xyz, other.e4)) + (float4(other.group1.ww, other.e431, this.e431) * float4(this.group2.xyz.xyx, other.e31)) + (float4(other.group4.zx, other.e4, this.e412) * float4(this.group2.xyz.yzz, other.e12)) + float4(float3(0.0), -(other.e1 * this.e41) - (other.e2 * this.e42) - (other.e3 * this.e43) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((other.group2.xyz.yzx * this.group4.zxy), other.scalar * this.e4) - float4((this.group2.xyz.zxy * other.group4.yzx), other.e41 * this.e1)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Origin other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e4) * float2(this.e321, this.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group3.xyz, this.e1234), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group2.xyz, this.scalar)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Plane other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e4 * other.e321, 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)), 
            /* e1, e2, e3, e4 */
            float4((this.scalar * other.e423) + (this.e41 * other.e321) + (this.e31 * other.e412), (this.scalar * other.e431) + (this.e42 * other.e321) + (this.e12 * other.e423), (this.scalar * other.e412) + (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * other.group0.yzx) - (float3(this.e4) * other.group0.xyz) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(this.e321) * other.group0.xyz) - (this.group1.yzx * other.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (float4(this.e1234) * other.group0) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Point other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), this.e4 * other.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group3.xyz) + (this.group2.xyz.yzx * other.group0.zxy) - (this.group2.xyz.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group0.xyz) + (this.group4.yzx * other.group0.zxy) - (float3(other.e4) * this.group1.xyz) - (this.group4.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group2.xyz), (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Scalar other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e1234 * other.scalar, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group4.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group2.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Origin> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Origin>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public Origin operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Origin {
    public Origin geometric_anti_product(AntiScalar other) {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
public extension Origin {
    public Flector geometric_anti_product(DualNum other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        );
    }
}
public extension Origin {
    public Motor geometric_anti_product(Flector other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e4) * float4(other.group0.xyz, other.e321)
        );
    }
}
public extension Origin {
    public Scalar geometric_anti_product(Horizon other) {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
public extension Origin {
    public Flector geometric_anti_product(Line other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e4) * other.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        );
    }
}
public extension Origin {
    public Flector geometric_anti_product(Motor other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group1.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group0.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension Origin {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e4) * float2(other.e321, other.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e4) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group2.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension Origin {
    public AntiScalar geometric_anti_product(Origin other) {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4 * -1.0
        );
    }
}
public extension Origin {
    public Motor geometric_anti_product(Plane other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e4) * other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e4 * other.e321)
        );
    }
}
public extension Origin {
    public Motor geometric_anti_product(Point other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        );
    }
}
public extension Origin {
    public Horizon geometric_anti_product(Scalar other) {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * other.scalar * -1.0
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Plane> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Plane>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Plane {
    public Plane geometric_anti_product(AntiScalar other) {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Plane {
    public Flector geometric_anti_product(DualNum other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Plane {
    public Motor geometric_anti_product(Flector other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e423) - (other.e412 * this.e431), -(other.e4 * this.e431) - (other.e423 * this.e412), -(other.e4 * this.e412) - (other.e431 * this.e423), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e3 * this.e431) + (other.e321 * this.e423), (other.e1 * this.e412) + (other.e321 * this.e431), (other.e2 * this.e423) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.yzxx * this.group0.zxyx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2))
        );
    }
}
public extension Plane {
    public Line geometric_anti_product(Horizon other) {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz
        );
    }
}
public extension Plane {
    public Flector geometric_anti_product(Line other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Plane {
    public Flector geometric_anti_product(Motor other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzy * float4(other.group1.www, other.e42)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (other.group0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(other.group0.www, other.e23))
        );
    }
}
public extension Plane {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzx * float4(other.group0.xy.xx, other.scalar, other.e41)) - (this.group0.yzxy * float4(other.group3.xyz.zxy, other.e42)), 
            /* e41, e42, e43 */
            (other.group4.yzx * this.group0.zxy) - (float3(other.e4) * this.group0.xyz) - (other.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (float3(this.e321) * other.group4.xyz) - (other.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (float4(other.e1234) * this.group0) + (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        );
    }
}
public extension Plane {
    public Motor geometric_anti_product(Origin other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e4 * this.e321 * -1.0)
        );
    }
}
public extension Plane {
    public Motor geometric_anti_product(Plane other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e412 * this.e431 * -1.0, other.e423 * this.e412 * -1.0, other.e431 * this.e423 * -1.0, (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0)
        );
    }
}
public extension Plane {
    public Motor geometric_anti_product(Point other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e431 * other.e3, this.e412 * other.e1, this.e423 * other.e2, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.zxyx * other.group0.yzxx)
        );
    }
}
public extension Plane {
    public Point geometric_anti_product(Scalar other) {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Point> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Point>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Point operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Point {
    public Point geometric_anti_product(AntiScalar other) {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Point {
    public Flector geometric_anti_product(DualNum other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        );
    }
}
public extension Point {
    public Motor geometric_anti_product(Flector other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(-(other.e4 * this.e1) - (other.e412 * this.e2), -(other.e4 * this.e2) - (other.e423 * this.e3), -(other.e4 * this.e3) - (other.e431 * this.e1), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423))
        );
    }
}
public extension Point {
    public Scalar geometric_anti_product(Horizon other) {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
public extension Point {
    public Flector geometric_anti_product(Line other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * other.group1.xyz), 0.0) - float4((other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        );
    }
}
public extension Point {
    public Flector geometric_anti_product(Motor other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group0.zxy * this.group0.yzx) - (float3(this.e4) * other.group1.xyz) - (other.group0.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4))
        );
    }
}
public extension Point {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), other.e4 * this.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group2.xyz.zxy * this.group0.yzx) - (float3(this.e4) * other.group3.xyz) - (other.group2.xyz.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group1.xyz) + (other.group4.yzx * this.group0.zxy) - (float3(other.e4) * this.group0.xyz) - (other.group4.zxy * this.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group2.xyz), -(other.scalar * this.e4) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        );
    }
}
public extension Point {
    public Motor geometric_anti_product(Origin other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Point {
    public Motor geometric_anti_product(Plane other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e412 * this.e2 * -1.0, other.e423 * this.e3 * -1.0, other.e431 * this.e1 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.yzxx * this.group0.zxyx)
        );
    }
}
public extension Point {
    public Motor geometric_anti_product(Point other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0)
        );
    }
}
public extension Point {
    public Horizon geometric_anti_product(Scalar other) {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * other.scalar * -1.0
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Scalar> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Scalar>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Scalar {
    public Scalar geometric_anti_product(AntiScalar other) {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar geometric_anti_product(DualNum other) {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar {
    public Flector geometric_anti_product(Flector other) {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        );
    }
}
public extension Scalar {
    public Line geometric_anti_product(Line other) {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz
        );
    }
}
public extension Scalar {
    public Motor geometric_anti_product(Motor other) {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e1234 * this.scalar, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group4.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group2.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        );
    }
}
public extension Scalar {
    public Horizon geometric_anti_product(Origin other) {
        return Horizon.from_groups(
            /* e321 */
            other.e4 * this.scalar
        );
    }
}
public extension Scalar {
    public Point geometric_anti_product(Plane other) {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        );
    }
}
public extension Scalar {
    public Horizon geometric_anti_product(Point other) {
        return Horizon.from_groups(
            /* e321 */
            other.e4 * this.scalar
        );
    }
}
