implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<AntiScalar> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<AntiScalar>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension AntiScalar {
    public AntiScalar geometric_anti_quotient(AntiScalar other) {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 / (other.e1234)
        );
    }
}
public extension AntiScalar {
    public DualNum geometric_anti_quotient(DualNum other) {
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, -2)
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * float2(other_2.e1234 * other.scalar, other_2.e1234 * other.e1234)
        );
    }
}
public extension AntiScalar {
    public Flector geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * geometric_anti_product.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * geometric_anti_product.group1
        );
    }
}
public extension AntiScalar {
    public Line geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1.xyz
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_anti_product.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * geometric_anti_product.group1.xyz
        );
    }
}
public extension AntiScalar {
    public Motor geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * geometric_anti_product.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * geometric_anti_product.group1
        );
    }
}
public extension AntiScalar {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * geometric_anti_product.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * geometric_anti_product.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_anti_product.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * geometric_anti_product.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * geometric_anti_product.group4
        );
    }
}
public extension AntiScalar {
    public Origin geometric_anti_quotient(Origin other) {
        return Origin.from_groups(
            /* e4 */
            this.e1234 / (other.e4) * -1.0
        );
    }
}
public extension AntiScalar {
    public Plane geometric_anti_quotient(Plane other) {
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * float4(other_2.e1234 * other.e423, other_2.e1234 * other.e431, other_2.e1234 * other.e412, other_2.e1234 * other.e321)
        );
    }
}
public extension AntiScalar {
    public Point geometric_anti_quotient(Point other) {
        Point anti_reverse = Point.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, -2)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * float4(other_2.e1234 * anti_reverse.e1, other_2.e1234 * anti_reverse.e2, other_2.e1234 * anti_reverse.e3, other_2.e1234 * anti_reverse.e4)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<DualNum> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<DualNum>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public DualNum operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension DualNum {
    public DualNum geometric_anti_quotient(AntiScalar other) {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0/other.e1234) * this.group0.xy
        );
    }
}
public extension DualNum {
    public DualNum geometric_anti_quotient(DualNum other) {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.e1234, -2)) * other.group0.xy
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.scalar * this.e1234) + (geometric_anti_product.e1234 * this.scalar), geometric_anti_product.e1234 * this.e1234)
        );
    }
}
public extension DualNum {
    public Flector geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * geometric_anti_product.group1.xyz) + (float3(this.e1234) * geometric_anti_product.group0.xyz)), this.e1234 * geometric_anti_product.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * geometric_anti_product.e4) + (this.e1234 * geometric_anti_product.e321)) * float4(geometric_anti_product.group1.xyz, 1.0)
        );
    }
}
public extension DualNum {
    public Line geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1.xyz
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_anti_product.group0.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_anti_product.group0.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz)
        );
    }
}
public extension DualNum {
    public Motor geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * geometric_anti_product.group0, 
            /* e23, e31, e12, scalar */
            (float4(this.scalar) * geometric_anti_product.group0) + (float4(this.e1234) * geometric_anti_product.group1)
        );
    }
}
public extension DualNum {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * geometric_anti_product.e1234) + (this.e1234 * geometric_anti_product.scalar), this.e1234 * geometric_anti_product.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * geometric_anti_product.group4.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz)), this.e1234 * geometric_anti_product.e4), 
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_anti_product.group2.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_anti_product.group2.xyz) + (float3(this.e1234) * geometric_anti_product.group3.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * geometric_anti_product.e4) + (this.e1234 * geometric_anti_product.e321)) * float4(geometric_anti_product.group4.xyz, 1.0)
        );
    }
}
public extension DualNum {
    public Flector geometric_anti_quotient(Origin other) {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_anti_product.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * geometric_anti_product.e4)
        );
    }
}
public extension DualNum {
    public Flector geometric_anti_quotient(Plane other) {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(geometric_anti_product.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * geometric_anti_product.group0
        );
    }
}
public extension DualNum {
    public Flector geometric_anti_quotient(Point other) {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * geometric_anti_product.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * geometric_anti_product.e4)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<Flector> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<Flector>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Flector operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension Flector {
    public Flector geometric_anti_quotient(AntiScalar other) {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/other.e1234
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
public extension Flector {
    public Flector geometric_anti_quotient(DualNum other) {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.e1234, -2)) * other.group0.xy
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group0.xyz) - (float3(geometric_anti_product.scalar) * this.group1.xyz)), geometric_anti_product.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.group0.xy.yy, geometric_anti_product.e1234, (geometric_anti_product.e1234 * this.e321) - (geometric_anti_product.scalar * this.e4)) * float4(this.group1.xyz, 1.0)
        );
    }
}
public extension Flector {
    public Motor geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(geometric_anti_product.e423 * this.e4) - (geometric_anti_product.e412 * this.e431), -(geometric_anti_product.e423 * this.e412) - (geometric_anti_product.e431 * this.e4), -(geometric_anti_product.e431 * this.e423) - (geometric_anti_product.e412 * this.e4), (geometric_anti_product.e431 * this.e431) + (geometric_anti_product.e412 * this.e412)) + (geometric_anti_product.group1.yzxx * this.group1.zxyx) - (float4(geometric_anti_product.e4) * float4(this.group1.xyz, this.e4)), 
            /* e23, e31, e12, scalar */
            (float4(geometric_anti_product.e321) * float4(this.group1.xyz, this.e4)) + (float4(this.e4, this.e412, this.e423, this.e1) * float4(geometric_anti_product.group0.xxy, geometric_anti_product.e423)) + (float4(this.e431, this.e4, this.e4, this.e2) * float4(geometric_anti_product.group0.zyz, geometric_anti_product.e431)) + (geometric_anti_product.group1.yzxz * this.group0.zxyz) - (float4(this.e2, this.e321, this.e321, this.e321) * float4(geometric_anti_product.group1.zyz, geometric_anti_product.e4)) - (float4(this.e321, this.e3, this.e1, this.e412) * float4(geometric_anti_product.group1.xxy, geometric_anti_product.e3)) - (geometric_anti_product.group0.yzxx * this.group1.zxyx) - (geometric_anti_product.group0.wwwy * float4(this.group0.xyz, this.e431))
        );
    }
}
public extension Flector {
    public Flector geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1.xyz
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * geometric_anti_product.e43) + (this.e412 * geometric_anti_product.e31) + (this.e321 * geometric_anti_product.e41), (this.e3 * geometric_anti_product.e41) + (this.e423 * geometric_anti_product.e12) + (this.e321 * geometric_anti_product.e42), (this.e1 * geometric_anti_product.e42) + (this.e431 * geometric_anti_product.e23) + (this.e321 * geometric_anti_product.e43), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(geometric_anti_product.group1.xyz.xxy, geometric_anti_product.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(geometric_anti_product.group1.xyz.zyz, geometric_anti_product.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group0.zxy), this.e423 * geometric_anti_product.e41), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(geometric_anti_product.group0.xyz.xxy, geometric_anti_product.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(geometric_anti_product.group0.xyz.zyz, geometric_anti_product.e31)) + float4(float3(0.0), (this.e412 * geometric_anti_product.e12) - (this.e2 * geometric_anti_product.e42) - (this.e3 * geometric_anti_product.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group1.zxy), this.e1 * geometric_anti_product.e41)
        );
    }
}
public extension Flector {
    public Flector geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * geometric_anti_product.e43) + (this.e412 * geometric_anti_product.e31) + (this.e321 * geometric_anti_product.e41) - (this.e431 * geometric_anti_product.e12), (this.e3 * geometric_anti_product.e41) + (this.e423 * geometric_anti_product.e12) + (this.e321 * geometric_anti_product.e42) - (this.e412 * geometric_anti_product.e23), (this.e3 * geometric_anti_product.e1234) + (this.e431 * geometric_anti_product.e23) + (this.e321 * geometric_anti_product.e43) - (this.e412 * geometric_anti_product.scalar), 0.0) + (this.group0.xyxw * geometric_anti_product.group0.wwyw) - (this.group1.xyxz * float4(geometric_anti_product.group1.wwy, geometric_anti_product.e43)) - (geometric_anti_product.group0.yzxx * float4(this.group0.zxy, this.e423)) - float4((this.group0.www * geometric_anti_product.group1.xyz), this.e431 * geometric_anti_product.e42), 
            /* e423, e431, e412, e321 */
            (this.group1.xyxy * float4(geometric_anti_product.group0.wwy, geometric_anti_product.e31)) + (this.group1.yzzz * float4(geometric_anti_product.group0.zxw, geometric_anti_product.e12)) + float4(float3(0.0), (this.e321 * geometric_anti_product.e1234) - (this.e2 * geometric_anti_product.e42) - (this.e3 * geometric_anti_product.e43) - (this.e4 * geometric_anti_product.scalar)) + float4((this.group0.www * geometric_anti_product.group0.xyz), this.e423 * geometric_anti_product.e23) - (geometric_anti_product.group0.yzxx * float4(this.group1.zxy, this.e1))
        );
    }
}
public extension Flector {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e4 * geometric_anti_product.e321) - (this.e431 * geometric_anti_product.e2) - (this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4), 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * geometric_anti_product.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((this.e2 * geometric_anti_product.e43) + (this.e412 * geometric_anti_product.e31) + (this.e321 * geometric_anti_product.e41) - (this.e431 * geometric_anti_product.e12), (this.e3 * geometric_anti_product.e41) + (this.e423 * geometric_anti_product.e12) + (this.e321 * geometric_anti_product.e42) - (this.e4 * geometric_anti_product.e31), (this.e1 * geometric_anti_product.e42) + (this.e431 * geometric_anti_product.e23) + (this.e321 * geometric_anti_product.e43) - (this.e4 * geometric_anti_product.e12), 0.0) + (float4(geometric_anti_product.e1234) * this.group0) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(geometric_anti_product.group3.xyz.xxy, geometric_anti_product.e43)) - (this.group1.xyzx * float4(geometric_anti_product.group0.xy.xx, geometric_anti_product.scalar, geometric_anti_product.e41)) - float4((geometric_anti_product.group2.xyz.yzx * this.group0.zxy), this.e431 * geometric_anti_product.e42), 
            /* e41, e42, e43 */
            (this.group1.zxy * geometric_anti_product.group4.yzx) - (float3(this.e4) * geometric_anti_product.group4.xyz) - (float3(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e431) * this.group1.xyx) - (float3(geometric_anti_product.e412, geometric_anti_product.e423, geometric_anti_product.e4) * this.group1.yzz), 
            /* e23, e31, e12 */
            (float3(this.e4) * geometric_anti_product.group1.xyz) + (float3(geometric_anti_product.e3, geometric_anti_product.e1, geometric_anti_product.e321) * this.group1.yzz) + (float3(geometric_anti_product.e321, geometric_anti_product.e321, geometric_anti_product.e2) * this.group1.xyx) + (this.group0.zxy * geometric_anti_product.group4.yzx) - (float3(this.e321) * geometric_anti_product.group4.xyz) - (float3(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e431) * this.group0.xyx) - (float3(geometric_anti_product.e412, geometric_anti_product.e423, geometric_anti_product.e4) * this.group0.yzz) - (this.group1.zxy * geometric_anti_product.group1.yzx), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product.e1234) * this.group1) + (float4(this.e4, this.e412, this.e423, this.e423) * float4(geometric_anti_product.group2.xyz.xxy, geometric_anti_product.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(geometric_anti_product.group2.xyz.zyz, geometric_anti_product.e31)) + float4(float3(0.0), (this.e412 * geometric_anti_product.e12) - (this.e1 * geometric_anti_product.e41) - (this.e2 * geometric_anti_product.e42) - (this.e3 * geometric_anti_product.e43)) - float4((geometric_anti_product.group2.xyz.yzx * this.group1.zxy), this.e4 * geometric_anti_product.scalar)
        );
    }
}
public extension Flector {
    public Motor geometric_anti_quotient(Origin other) {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
public extension Flector {
    public Motor geometric_anti_quotient(Plane other) {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e4 * geometric_anti_product.e423) - (this.e431 * geometric_anti_product.e412), -(this.e4 * geometric_anti_product.e431) - (this.e412 * geometric_anti_product.e423), -(this.e4 * geometric_anti_product.e412) - (this.e423 * geometric_anti_product.e431), (this.e431 * geometric_anti_product.e431) + (this.e412 * geometric_anti_product.e412)) + (this.group1.zxyx * geometric_anti_product.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(this.e2 * geometric_anti_product.e412) - (this.e321 * geometric_anti_product.e423), -(this.e3 * geometric_anti_product.e423) - (this.e321 * geometric_anti_product.e431), -(this.e1 * geometric_anti_product.e431) - (this.e321 * geometric_anti_product.e412), (this.e3 * geometric_anti_product.e412) + (this.e4 * geometric_anti_product.e321)) + (this.group0.zxyx * geometric_anti_product.group0.yzxx) + (geometric_anti_product.group0.wwwy * float4(this.group1.xyz, this.e2))
        );
    }
}
public extension Flector {
    public Motor geometric_anti_quotient(Point other) {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((this.e4 * geometric_anti_product.e1) + (this.e431 * geometric_anti_product.e3), (this.e4 * geometric_anti_product.e2) + (this.e412 * geometric_anti_product.e1), (this.e4 * geometric_anti_product.e3) + (this.e423 * geometric_anti_product.e2), -(this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4)) - (this.group1.zxyy * geometric_anti_product.group0.yzxy) - (geometric_anti_product.group0.wwwx * float4(this.group0.xyz, this.e423))
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<Horizon> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<Horizon>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Horizon operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension Horizon {
    public Horizon geometric_anti_quotient(AntiScalar other) {
        return Horizon.from_groups(
            /* e321 */
            this.e321 / (other.e1234)
        );
    }
}
public extension Horizon {
    public Horizon geometric_anti_quotient(DualNum other) {
        return Horizon.from_groups(
            /* e321 */
            this.e321 / (other.e1234)
        );
    }
}
public extension Horizon {
    public Motor geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e4) * float4(-1.0)
        );
    }
}
public extension Horizon {
    public Point geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * float3(other_2.e1234 * anti_reverse.e41, other_2.e1234 * anti_reverse.e42, other_2.e1234 * anti_reverse.e43)), 0.0)
        );
    }
}
public extension Horizon {
    public Flector geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_anti_product.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * geometric_anti_product.e1234)
        );
    }
}
public extension Horizon {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * geometric_anti_product.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_anti_product.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * geometric_anti_product.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * geometric_anti_product.e1234)
        );
    }
}
public extension Horizon {
    public Scalar geometric_anti_quotient(Origin other) {
        return Scalar.from_groups(
            /* scalar */
            this.e321 / (other.e4)
        );
    }
}
public extension Horizon {
    public Line geometric_anti_quotient(Plane other) {
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * float4(other_2.e1234 * other.e423, other_2.e1234 * other.e431, other_2.e1234 * other.e412, other_2.e1234 * other.e321).xyz * float3(-1.0)
        );
    }
}
public extension Horizon {
    public Scalar geometric_anti_quotient(Point other) {
        return Scalar.from_groups(
            /* scalar */
            this.e321 / (other.e4)
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<Line> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<Line>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Line operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension Line {
    public Line geometric_anti_quotient(AntiScalar other) {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/other.e1234
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group1.xyz
        );
    }
}
public extension Line {
    public Line geometric_anti_quotient(DualNum other) {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.e1234, -2)) * other.group0.xy
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group0.xyz) + (float3(geometric_anti_product.e1234) * this.group1.xyz)
        );
    }
}
public extension Line {
    public Flector geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e3 * this.e42) + (geometric_anti_product.e4 * this.e23) + (geometric_anti_product.e412 * this.e31) + (geometric_anti_product.e321 * this.e41), (geometric_anti_product.e1 * this.e43) + (geometric_anti_product.e4 * this.e31) + (geometric_anti_product.e423 * this.e12) + (geometric_anti_product.e321 * this.e42), (geometric_anti_product.e2 * this.e41) + (geometric_anti_product.e4 * this.e12) + (geometric_anti_product.e431 * this.e23) + (geometric_anti_product.e321 * this.e43), geometric_anti_product.e412 * this.e43 * -1.0) - (geometric_anti_product.group1.yzxy * float4(this.group1.xyz.zxy, this.e42)) - float4((this.group0.xyz.zxy * geometric_anti_product.group0.yzx), geometric_anti_product.e423 * this.e41), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product.e4 * this.e41) + (geometric_anti_product.e412 * this.e42), (geometric_anti_product.e4 * this.e42) + (geometric_anti_product.e423 * this.e43), (geometric_anti_product.e4 * this.e43) + (geometric_anti_product.e431 * this.e41), -(geometric_anti_product.e2 * this.e42) - (geometric_anti_product.e3 * this.e43) - (geometric_anti_product.e423 * this.e23) - (geometric_anti_product.e431 * this.e31) - (geometric_anti_product.e412 * this.e12)) - float4((this.group0.xyz.zxy * geometric_anti_product.group1.yzx), geometric_anti_product.e1 * this.e41)
        );
    }
}
public extension Line {
    public Motor geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1.xyz
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e43 * this.e42, geometric_anti_product.e41 * this.e43, geometric_anti_product.e42 * this.e41, -(geometric_anti_product.e42 * this.e42) - (geometric_anti_product.e43 * this.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group0.xyz.zxy), geometric_anti_product.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product.e43 * this.e31) + (geometric_anti_product.e12 * this.e42), (geometric_anti_product.e41 * this.e12) + (geometric_anti_product.e23 * this.e43), (geometric_anti_product.e42 * this.e23) + (geometric_anti_product.e31 * this.e41), -(geometric_anti_product.e43 * this.e12) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group1.xyz.zxy), geometric_anti_product.e41 * this.e23) - float4((geometric_anti_product.group1.xyz.yzx * this.group0.xyz.zxy), geometric_anti_product.e42 * this.e31)
        );
    }
}
public extension Line {
    public Motor geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * geometric_anti_product.e1234) + (this.e42 * geometric_anti_product.e43), (this.e42 * geometric_anti_product.e1234) + (this.e43 * geometric_anti_product.e41), (this.e41 * geometric_anti_product.e42) + (this.e43 * geometric_anti_product.e1234), -(this.e42 * geometric_anti_product.e42) - (this.e43 * geometric_anti_product.e43)) - (geometric_anti_product.group0.yzxx * float4(this.group0.xyz.zxy, this.e41)), 
            /* e23, e31, e12, scalar */
            float4((this.e41 * geometric_anti_product.scalar) + (this.e42 * geometric_anti_product.e12) + (this.e23 * geometric_anti_product.e1234) + (this.e31 * geometric_anti_product.e43), (this.e42 * geometric_anti_product.scalar) + (this.e43 * geometric_anti_product.e23) + (this.e31 * geometric_anti_product.e1234) + (this.e12 * geometric_anti_product.e41), (this.e41 * geometric_anti_product.e31) + (this.e43 * geometric_anti_product.scalar) + (this.e23 * geometric_anti_product.e42) + (this.e12 * geometric_anti_product.e1234), -(this.e43 * geometric_anti_product.e12) - (this.e23 * geometric_anti_product.e41) - (this.e31 * geometric_anti_product.e42) - (this.e12 * geometric_anti_product.e43)) - (geometric_anti_product.group1.yzxx * float4(this.group0.xyz.zxy, this.e41)) - float4((this.group1.xyz.zxy * geometric_anti_product.group0.yzx), this.e42 * geometric_anti_product.e31)
        );
    }
}
public extension Line {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e23 * geometric_anti_product.e41) - (this.e31 * geometric_anti_product.e42) - (this.e12 * geometric_anti_product.e43), 0.0) - (float2(this.e41) * float2(geometric_anti_product.e23, geometric_anti_product.e41)) - (float2(this.e42) * float2(geometric_anti_product.e31, geometric_anti_product.e42)) - (float2(this.e43) * float2(geometric_anti_product.e12, geometric_anti_product.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * geometric_anti_product.e321) + (this.e42 * geometric_anti_product.e3) + (this.e23 * geometric_anti_product.e4) + (this.e31 * geometric_anti_product.e412), (this.e42 * geometric_anti_product.e321) + (this.e43 * geometric_anti_product.e1) + (this.e31 * geometric_anti_product.e4) + (this.e12 * geometric_anti_product.e423), (this.e41 * geometric_anti_product.e2) + (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431) + (this.e12 * geometric_anti_product.e4), this.e43 * geometric_anti_product.e412 * -1.0) - (geometric_anti_product.group4.yzxy * float4(this.group1.xyz.zxy, this.e42)) - float4((this.group0.xyz.zxy * geometric_anti_product.group1.yzx), this.e41 * geometric_anti_product.e423), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product.e1234) * this.group0.xyz) + (this.group0.xyz.yzx * geometric_anti_product.group2.xyz.zxy) - (this.group0.xyz.zxy * geometric_anti_product.group2.xyz.yzx), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group0.xyz) + (float3(geometric_anti_product.e1234) * this.group1.xyz) + (this.group0.xyz.yzx * geometric_anti_product.group3.xyz.zxy) + (this.group1.xyz.yzx * geometric_anti_product.group2.xyz.zxy) - (this.group0.xyz.zxy * geometric_anti_product.group3.xyz.yzx) - (this.group1.xyz.zxy * geometric_anti_product.group2.xyz.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e41 * geometric_anti_product.e4) + (this.e42 * geometric_anti_product.e412), (this.e42 * geometric_anti_product.e4) + (this.e43 * geometric_anti_product.e423), (this.e41 * geometric_anti_product.e431) + (this.e43 * geometric_anti_product.e4), -(this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3) - (this.e23 * geometric_anti_product.e423) - (this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) - float4((this.group0.xyz.zxy * geometric_anti_product.group4.yzx), this.e41 * geometric_anti_product.e1)
        );
    }
}
public extension Line {
    public Flector geometric_anti_quotient(Origin other) {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_anti_product.e4) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product.e4) * this.group0.xyz), 0.0)
        );
    }
}
public extension Line {
    public Flector geometric_anti_quotient(Plane other) {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * geometric_anti_product.e321) + (this.e31 * geometric_anti_product.e412), (this.e42 * geometric_anti_product.e321) + (this.e12 * geometric_anti_product.e423), (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431), -(this.e42 * geometric_anti_product.e431) - (this.e43 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * geometric_anti_product.e412, this.e43 * geometric_anti_product.e423, this.e41 * geometric_anti_product.e431, -(this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Line {
    public Flector geometric_anti_quotient(Point other) {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_anti_product.e4) * this.group1.xyz), 0.0) + float4((this.group0.xyz.yzx * geometric_anti_product.group0.zxy), 0.0) - float4((this.group0.xyz.zxy * geometric_anti_product.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product.e4) * this.group0.xyz), -(this.e41 * geometric_anti_product.e1) - (this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3))
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<Motor> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<Motor>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension Motor {
    public Motor geometric_anti_quotient(AntiScalar other) {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/other.e1234
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
public extension Motor {
    public Motor geometric_anti_quotient(DualNum other) {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.e1234, -2)) * other.group0.xy
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(geometric_anti_product.scalar) * this.group0) + (float4(geometric_anti_product.e1234) * this.group1)
        );
    }
}
public extension Motor {
    public Flector geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e3 * this.e42) + (geometric_anti_product.e4 * this.e23) + (geometric_anti_product.e423 * this.scalar) + (geometric_anti_product.e412 * this.e31) + (geometric_anti_product.e321 * this.e41), (geometric_anti_product.e2 * this.e1234) + (geometric_anti_product.e4 * this.e31) + (geometric_anti_product.e423 * this.e12) + (geometric_anti_product.e431 * this.scalar) + (geometric_anti_product.e321 * this.e42), (geometric_anti_product.e3 * this.e1234) + (geometric_anti_product.e4 * this.e12) + (geometric_anti_product.e431 * this.e23) + (geometric_anti_product.e412 * this.scalar) + (geometric_anti_product.e321 * this.e43), geometric_anti_product.e412 * this.e43 * -1.0) + (geometric_anti_product.group0.xxyw * this.group0.wzxw) - (geometric_anti_product.group1.yzxy * float4(this.group1.zxy, this.e42)) - (this.group0.zxyx * float4(geometric_anti_product.group0.yzx, geometric_anti_product.e423)), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e412 * this.e42, geometric_anti_product.e431 * this.e1234, geometric_anti_product.e412 * this.e1234, -(geometric_anti_product.e2 * this.e42) - (geometric_anti_product.e3 * this.e43) - (geometric_anti_product.e423 * this.e23) - (geometric_anti_product.e431 * this.e31) - (geometric_anti_product.e412 * this.e12)) + (float4(geometric_anti_product.e4) * float4(this.group0.xyz, this.scalar)) + (geometric_anti_product.group1.xxyw * this.group0.wzxw) - (this.group0.zxyx * float4(geometric_anti_product.group1.yzx, geometric_anti_product.e1))
        );
    }
}
public extension Motor {
    public Motor geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1.xyz
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_anti_product.e41 * this.e1234) + (geometric_anti_product.e43 * this.e42), (geometric_anti_product.e41 * this.e43) + (geometric_anti_product.e42 * this.e1234), (geometric_anti_product.e42 * this.e41) + (geometric_anti_product.e43 * this.e1234), -(geometric_anti_product.e42 * this.e42) - (geometric_anti_product.e43 * this.e43)) - (this.group0.zxyx * float4(geometric_anti_product.group0.xyz.yzx, geometric_anti_product.e41)), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product.e41 * this.scalar) + (geometric_anti_product.e43 * this.e31) + (geometric_anti_product.e23 * this.e1234) + (geometric_anti_product.e12 * this.e42), (geometric_anti_product.e41 * this.e12) + (geometric_anti_product.e42 * this.scalar) + (geometric_anti_product.e23 * this.e43) + (geometric_anti_product.e31 * this.e1234), (geometric_anti_product.e42 * this.e23) + (geometric_anti_product.e43 * this.scalar) + (geometric_anti_product.e31 * this.e41) + (geometric_anti_product.e12 * this.e1234), -(geometric_anti_product.e43 * this.e12) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43)) - (this.group1.zxyx * float4(geometric_anti_product.group0.xyz.yzx, geometric_anti_product.e41)) - float4((geometric_anti_product.group1.xyz.yzx * this.group0.zxy), geometric_anti_product.e42 * this.e31)
        );
    }
}
public extension Motor {
    public Motor geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_anti_product.e43 * this.e42) + (geometric_anti_product.e1234 * this.e41), (geometric_anti_product.e42 * this.e1234) + (geometric_anti_product.e1234 * this.e42), (geometric_anti_product.e43 * this.e1234) + (geometric_anti_product.e1234 * this.e43), -(geometric_anti_product.e42 * this.e42) - (geometric_anti_product.e43 * this.e43)) + (geometric_anti_product.group0.xxyw * this.group0.wzxw) - (geometric_anti_product.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product.e1234 * this.e23) + (geometric_anti_product.e23 * this.e1234) + (geometric_anti_product.e12 * this.e42) + (geometric_anti_product.scalar * this.e41), (geometric_anti_product.e1234 * this.e31) + (geometric_anti_product.e23 * this.e43) + (geometric_anti_product.e31 * this.e1234) + (geometric_anti_product.scalar * this.e42), (geometric_anti_product.e1234 * this.e12) + (geometric_anti_product.e31 * this.e41) + (geometric_anti_product.e12 * this.e1234) + (geometric_anti_product.scalar * this.e43), -(geometric_anti_product.e43 * this.e12) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43)) + (geometric_anti_product.group0.xxyw * this.group1.wzxw) + float4((geometric_anti_product.group0.zyz * this.group1.yww), geometric_anti_product.scalar * this.e1234) - (geometric_anti_product.group0.yzxx * this.group1.zxyx) - float4((geometric_anti_product.group1.yzx * this.group0.zxy), geometric_anti_product.e42 * this.e31)
        );
    }
}
public extension Motor {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * geometric_anti_product.e1234) - (this.e41 * geometric_anti_product.e23) - (this.e42 * geometric_anti_product.e31) - (this.e43 * geometric_anti_product.e12), 0.0) + (float2(this.e1234) * geometric_anti_product.group0.xy) - (float2(geometric_anti_product.e41) * float2(this.e23, this.e41)) - (float2(geometric_anti_product.e42) * float2(this.e31, this.e42)) - (float2(geometric_anti_product.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e42 * geometric_anti_product.e3) + (this.e1234 * geometric_anti_product.e1) + (this.e23 * geometric_anti_product.e4) + (this.e31 * geometric_anti_product.e412) + (this.scalar * geometric_anti_product.e423), (this.e43 * geometric_anti_product.e1) + (this.e1234 * geometric_anti_product.e2) + (this.e31 * geometric_anti_product.e4) + (this.e12 * geometric_anti_product.e423) + (this.scalar * geometric_anti_product.e431), (this.e43 * geometric_anti_product.e321) + (this.e1234 * geometric_anti_product.e3) + (this.e23 * geometric_anti_product.e431) + (this.e12 * geometric_anti_product.e4) + (this.scalar * geometric_anti_product.e412), this.e43 * geometric_anti_product.e412 * -1.0) + (this.group0.xyxw * float4(geometric_anti_product.group4.ww, geometric_anti_product.e2, geometric_anti_product.e4)) - (this.group0.zxyx * float4(geometric_anti_product.group1.yzx, geometric_anti_product.e423)) - (geometric_anti_product.group4.yzxy * float4(this.group1.zxy, this.e42)), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product.e1234) * this.group0.xyz) + (geometric_anti_product.group2.xyz.xxy * this.group0.wzx) + (geometric_anti_product.group2.xyz.zyz * this.group0.yww) - (geometric_anti_product.group2.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group0.xyz) + (float3(geometric_anti_product.e1234) * this.group1.xyz) + (geometric_anti_product.group2.xyz.xxy * this.group1.wzx) + (geometric_anti_product.group2.xyz.zyz * this.group1.yww) + (geometric_anti_product.group3.xyz.xxy * this.group0.wzx) + (geometric_anti_product.group3.xyz.zyz * this.group0.yww) - (geometric_anti_product.group2.xyz.yzx * this.group1.zxy) - (geometric_anti_product.group3.xyz.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * geometric_anti_product.e423, this.e1234 * geometric_anti_product.e431, this.e1234 * geometric_anti_product.e412, -(this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3) - (this.e23 * geometric_anti_product.e423) - (this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) + (this.group0.xyxw * float4(geometric_anti_product.group1.ww, geometric_anti_product.e431, geometric_anti_product.e321)) + (float4(geometric_anti_product.group4.zx, geometric_anti_product.e4, geometric_anti_product.e4) * float4(this.group0.yzz, this.scalar)) - (this.group0.zxyx * float4(geometric_anti_product.group4.yzx, geometric_anti_product.e1))
        );
    }
}
public extension Motor {
    public Flector geometric_anti_quotient(Origin other) {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e4) * float4(this.group1.xyz, this.e1234), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4) * float4(this.group0.xyz, this.scalar)
        );
    }
}
public extension Motor {
    public Flector geometric_anti_quotient(Plane other) {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * geometric_anti_product.e321) + (this.e31 * geometric_anti_product.e412) + (this.scalar * geometric_anti_product.e423), (this.e42 * geometric_anti_product.e321) + (this.e12 * geometric_anti_product.e423) + (this.scalar * geometric_anti_product.e431), (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431) + (this.scalar * geometric_anti_product.e412), -(this.e42 * geometric_anti_product.e431) - (this.e43 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * geometric_anti_product.e423, this.e1234 * geometric_anti_product.e431, this.e1234 * geometric_anti_product.e412, -(this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) + (this.group0.yzxw * geometric_anti_product.group0.zxyw) - (geometric_anti_product.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension Motor {
    public Flector geometric_anti_quotient(Point other) {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * geometric_anti_product.group0.xyz) + (float3(geometric_anti_product.e4) * this.group1.xyz) + (this.group0.yzx * geometric_anti_product.group0.zxy) - (this.group0.zxy * geometric_anti_product.group0.yzx)), this.e1234 * geometric_anti_product.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product.e4) * this.group0.xyz), (this.scalar * geometric_anti_product.e4) - (this.e41 * geometric_anti_product.e1) - (this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3))
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<MultiVector> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<MultiVector>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(AntiScalar other) {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/other.e1234
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product.e1234) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(DualNum other) {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.e1234, -2)) * other.group0.xy
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.scalar * this.e1234) + (geometric_anti_product.e1234 * this.scalar), geometric_anti_product.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group1.xyz) - (float3(geometric_anti_product.scalar) * this.group4.xyz)), geometric_anti_product.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group2.xyz) + (float3(geometric_anti_product.e1234) * this.group3.xyz), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.group0.xy.yy, geometric_anti_product.e1234, (geometric_anti_product.e1234 * this.e321) - (geometric_anti_product.scalar * this.e4)) * float4(this.group4.xyz, 1.0)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.e321 * this.e4) - (geometric_anti_product.e2 * this.e431) - (geometric_anti_product.e3 * this.e412) - (geometric_anti_product.e4 * this.e321), 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * geometric_anti_product.group0.xw), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e1 * this.e1234) + (geometric_anti_product.e3 * this.e42) + (geometric_anti_product.e4 * this.e23) + (geometric_anti_product.e412 * this.e31) + (geometric_anti_product.e321 * this.e41), (geometric_anti_product.e1 * this.e43) + (geometric_anti_product.e2 * this.e1234) + (geometric_anti_product.e4 * this.e31) + (geometric_anti_product.e423 * this.e12) + (geometric_anti_product.e321 * this.e42), (geometric_anti_product.e2 * this.e41) + (geometric_anti_product.e3 * this.e1234) + (geometric_anti_product.e4 * this.e12) + (geometric_anti_product.e431 * this.e23) + (geometric_anti_product.e321 * this.e43), geometric_anti_product.e412 * this.e43 * -1.0) + (float4(this.group0.xy.xx, this.scalar, this.e1234) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e4)) - (geometric_anti_product.group1.yzxy * float4(this.group3.xyz.zxy, this.e42)) - float4((this.group2.xyz.zxy * geometric_anti_product.group0.yzx), geometric_anti_product.e423 * this.e41), 
            /* e41, e42, e43 */
            (geometric_anti_product.group1.yzx * this.group4.zxy) - (float3(geometric_anti_product.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group1.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group1.zyz), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.e321) * this.group4.xyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group0.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group0.zyz) + (geometric_anti_product.group1.yzx * this.group1.zxy) - (float3(geometric_anti_product.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * geometric_anti_product.group1.zyz) - (float3(this.e321, this.e3, this.e1) * geometric_anti_product.group1.xxy) - (geometric_anti_product.group0.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e412 * this.e42, geometric_anti_product.e423 * this.e43, geometric_anti_product.e4 * this.e43, -(geometric_anti_product.e2 * this.e42) - (geometric_anti_product.e3 * this.e43) - (geometric_anti_product.e423 * this.e23) - (geometric_anti_product.e431 * this.e31) - (geometric_anti_product.e412 * this.e12)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e4)) + (float4(geometric_anti_product.group0.ww, geometric_anti_product.e431, geometric_anti_product.e321) * float4(this.group2.xyz.xyx, this.e1234)) - float4((this.group2.xyz.zxy * geometric_anti_product.group1.yzx), geometric_anti_product.e1 * this.e41)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1.xyz
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43), 0.0) - (float2(geometric_anti_product.e41) * float2(this.e23, this.e41)) - (float2(geometric_anti_product.e42) * float2(this.e31, this.e42)) - (float2(geometric_anti_product.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e43 * this.e2) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e41 * this.e3) + (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e42 * this.e1) + (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(geometric_anti_product.group1.xyz.xxy, geometric_anti_product.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(geometric_anti_product.group1.xyz.zyz, geometric_anti_product.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group1.zxy), geometric_anti_product.e41 * this.e423), 
            /* e41, e42, e43 */
            (float3(this.e1234) * geometric_anti_product.group0.xyz) + (geometric_anti_product.group0.xyz.zxy * this.group2.xyz.yzx) - (geometric_anti_product.group0.xyz.yzx * this.group2.xyz.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_anti_product.group0.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz) + (geometric_anti_product.group0.xyz.zxy * this.group3.xyz.yzx) + (geometric_anti_product.group1.xyz.zxy * this.group2.xyz.yzx) - (geometric_anti_product.group0.xyz.yzx * this.group3.xyz.zxy) - (geometric_anti_product.group1.xyz.yzx * this.group2.xyz.zxy), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(geometric_anti_product.group0.xyz.xxy, geometric_anti_product.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(geometric_anti_product.group0.xyz.zyz, geometric_anti_product.e31)) + float4(float3(0.0), (geometric_anti_product.e12 * this.e412) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3)) - float4((geometric_anti_product.group0.xyz.yzx * this.group4.zxy), geometric_anti_product.e41 * this.e1)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.scalar * this.e1234) - (geometric_anti_product.e41 * this.e23) - (geometric_anti_product.e42 * this.e31) - (geometric_anti_product.e43 * this.e12), 0.0) + (float2(geometric_anti_product.e1234) * this.group0.xy) - (float2(this.e41) * float2(geometric_anti_product.e23, geometric_anti_product.e41)) - (float2(this.e42) * float2(geometric_anti_product.e31, geometric_anti_product.e42)) - (float2(this.e43) * float2(geometric_anti_product.e12, geometric_anti_product.e43)), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e43 * this.e2) + (geometric_anti_product.e1234 * this.e1) + (geometric_anti_product.e31 * this.e412) - (geometric_anti_product.scalar * this.e423), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e1234 * this.e2) + (geometric_anti_product.e12 * this.e423) - (geometric_anti_product.scalar * this.e431), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e1234 * this.e3) + (geometric_anti_product.e23 * this.e431) - (geometric_anti_product.scalar * this.e412), 0.0) + (float4(this.e321, this.e3, this.e1, this.e4) * geometric_anti_product.group0.xxyw) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(geometric_anti_product.group1.xxy, geometric_anti_product.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(geometric_anti_product.group1.zyz, geometric_anti_product.e43)) - (geometric_anti_product.group0.yzxx * float4(this.group1.zxy, this.e423)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * geometric_anti_product.group0.xyz) + (this.group2.xyz.xyx * geometric_anti_product.group0.wwy) + (this.group2.xyz.yzz * geometric_anti_product.group0.zxw) - (this.group2.xyz.zxy * geometric_anti_product.group0.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_anti_product.group0.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz) + (this.group2.xyz.xyx * geometric_anti_product.group1.wwy) + (this.group2.xyz.yzz * geometric_anti_product.group1.zxw) + (this.group3.xyz.xyx * geometric_anti_product.group0.wwy) + (this.group3.xyz.yzz * geometric_anti_product.group0.zxw) - (this.group2.xyz.zxy * geometric_anti_product.group1.yzx) - (this.group3.xyz.zxy * geometric_anti_product.group0.yzx), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e321) * geometric_anti_product.group0.xxyw) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(geometric_anti_product.group0.zyz, geometric_anti_product.e23)) + (this.group4.xyzy * float4(geometric_anti_product.group0.www, geometric_anti_product.e31)) + float4(float3(0.0), (geometric_anti_product.e12 * this.e412) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3) - (geometric_anti_product.scalar * this.e4)) - (geometric_anti_product.group0.yzxx * float4(this.group4.zxy, this.e1))
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.e1234 * this.scalar) + (geometric_anti_product.e321 * this.e4) - (geometric_anti_product.e2 * this.e431) - (geometric_anti_product.e3 * this.e412) - (geometric_anti_product.e4 * this.e321) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43), 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * geometric_anti_product.group0.xy) - (float2(geometric_anti_product.e41) * float2(this.e23, this.e41)) - (float2(geometric_anti_product.e42) * float2(this.e31, this.e42)) - (float2(geometric_anti_product.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * geometric_anti_product.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e1 * this.e1234) + (geometric_anti_product.e3 * this.e42) + (geometric_anti_product.e4 * this.e23) + (geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e43 * this.e2) + (geometric_anti_product.e31 * this.e412) + (geometric_anti_product.e412 * this.e31) + (geometric_anti_product.e321 * this.e41), (geometric_anti_product.e1 * this.e43) + (geometric_anti_product.e2 * this.e1234) + (geometric_anti_product.e4 * this.e31) + (geometric_anti_product.e41 * this.e3) + (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423) + (geometric_anti_product.e423 * this.e12) + (geometric_anti_product.e321 * this.e42), (geometric_anti_product.e2 * this.e41) + (geometric_anti_product.e3 * this.e1234) + (geometric_anti_product.e4 * this.e12) + (geometric_anti_product.e42 * this.e1) + (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431) + (geometric_anti_product.e431 * this.e23) + (geometric_anti_product.e321 * this.e43), 0.0) + (float4(geometric_anti_product.e1234) * this.group1) + (float4(this.group0.xy.xx, this.scalar, this.e1234) * float4(geometric_anti_product.group4.xyz, geometric_anti_product.e4)) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(geometric_anti_product.group3.xyz.xxy, geometric_anti_product.e43)) - (float4(this.e431, this.e4, this.e4, geometric_anti_product.e423) * float4(geometric_anti_product.group3.xyz.zyz, this.e41)) - (geometric_anti_product.group4.yzxz * float4(this.group3.xyz.zxy, this.e43)) - (this.group4.xyzx * float4(geometric_anti_product.group0.xy.xx, geometric_anti_product.scalar, geometric_anti_product.e41)) - float4((geometric_anti_product.group2.xyz.yzx * this.group1.zxy), geometric_anti_product.e42 * this.e431) - float4((this.group2.xyz.zxy * geometric_anti_product.group1.yzx), geometric_anti_product.e431 * this.e42), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product.e1234) * this.group2.xyz) + (float3(this.e1234) * geometric_anti_product.group2.xyz) + (geometric_anti_product.group2.xyz.zxy * this.group2.xyz.yzx) + (geometric_anti_product.group4.yzx * this.group4.zxy) - (float3(geometric_anti_product.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group4.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group4.zyz) - (geometric_anti_product.group2.xyz.yzx * this.group2.xyz.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group2.xyz) + (float3(geometric_anti_product.e1234) * this.group3.xyz) + (float3(geometric_anti_product.e321) * this.group4.xyz) + (float3(this.scalar) * geometric_anti_product.group2.xyz) + (float3(this.e1234) * geometric_anti_product.group3.xyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group1.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group1.zyz) + (geometric_anti_product.group2.xyz.zxy * this.group3.xyz.yzx) + (geometric_anti_product.group3.xyz.zxy * this.group2.xyz.yzx) + (geometric_anti_product.group4.yzx * this.group1.zxy) - (float3(geometric_anti_product.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * geometric_anti_product.group4.zyz) - (float3(this.e321, this.e3, this.e1) * geometric_anti_product.group4.xxy) - (geometric_anti_product.group2.xyz.yzx * this.group3.xyz.zxy) - (geometric_anti_product.group3.xyz.yzx * this.group2.xyz.zxy) - (geometric_anti_product.group1.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, geometric_anti_product.e321) * float4(geometric_anti_product.group2.xyz.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(geometric_anti_product.group2.xyz.zyz, geometric_anti_product.e23)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(geometric_anti_product.group4.xyz, geometric_anti_product.e4)) + (float4(geometric_anti_product.group1.ww, geometric_anti_product.e431, this.e431) * float4(this.group2.xyz.xyx, geometric_anti_product.e31)) + (float4(geometric_anti_product.group4.zx, geometric_anti_product.e4, this.e412) * float4(this.group2.xyz.yzz, geometric_anti_product.e12)) + float4(float3(0.0), -(geometric_anti_product.e1 * this.e41) - (geometric_anti_product.e2 * this.e42) - (geometric_anti_product.e3 * this.e43) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3) - (geometric_anti_product.e423 * this.e23) - (geometric_anti_product.e431 * this.e31) - (geometric_anti_product.e412 * this.e12)) - float4((geometric_anti_product.group2.xyz.yzx * this.group4.zxy), geometric_anti_product.scalar * this.e4) - float4((this.group2.xyz.zxy * geometric_anti_product.group4.yzx), geometric_anti_product.e41 * this.e1)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Origin other) {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product.e4) * float2(this.e321, this.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e4) * float4(this.group3.xyz, this.e1234), 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e4) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4) * float4(this.group2.xyz, this.scalar)
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Plane other) {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)) * other.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e4 * geometric_anti_product.e321, 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)), 
            /* e1, e2, e3, e4 */
            float4((this.scalar * geometric_anti_product.e423) + (this.e41 * geometric_anti_product.e321) + (this.e31 * geometric_anti_product.e412), (this.scalar * geometric_anti_product.e431) + (this.e42 * geometric_anti_product.e321) + (this.e12 * geometric_anti_product.e423), (this.scalar * geometric_anti_product.e412) + (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431), -(this.e42 * geometric_anti_product.e431) - (this.e43 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * geometric_anti_product.group0.yzx) - (float3(this.e4) * geometric_anti_product.group0.xyz) - (this.group4.yzx * geometric_anti_product.group0.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.e321) * this.group4.xyz) + (this.group1.zxy * geometric_anti_product.group0.yzx) - (float3(this.e321) * geometric_anti_product.group0.xyz) - (this.group1.yzx * geometric_anti_product.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * geometric_anti_product.e412, this.e43 * geometric_anti_product.e423, this.e41 * geometric_anti_product.e431, -(this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) + (float4(this.e1234) * geometric_anti_product.group0) - (geometric_anti_product.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
    }
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Point other) {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e423 * geometric_anti_product.e1) - (this.e431 * geometric_anti_product.e2) - (this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4), this.e4 * geometric_anti_product.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * geometric_anti_product.group0.xyz) + (float3(geometric_anti_product.e4) * this.group3.xyz) + (this.group2.xyz.yzx * geometric_anti_product.group0.zxy) - (this.group2.xyz.zxy * geometric_anti_product.group0.yzx)), this.e1234 * geometric_anti_product.e4), 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * geometric_anti_product.group0.xyz) + (this.group4.yzx * geometric_anti_product.group0.zxy) - (float3(geometric_anti_product.e4) * this.group1.xyz) - (this.group4.zxy * geometric_anti_product.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product.e4) * this.group2.xyz), (this.scalar * geometric_anti_product.e4) - (this.e41 * geometric_anti_product.e1) - (this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3))
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<Origin> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<Origin>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public Origin operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension Origin {
    public Origin geometric_anti_quotient(AntiScalar other) {
        return Origin.from_groups(
            /* e4 */
            this.e4 / (other.e1234)
        );
    }
}
public extension Origin {
    public Flector geometric_anti_quotient(DualNum other) {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.e1234, -2)) * other.group0.xy
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_anti_product.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product.scalar * this.e4 * -1.0)
        );
    }
}
public extension Origin {
    public Motor geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e4) * float4(geometric_anti_product.group0.xyz, geometric_anti_product.e321)
        );
    }
}
public extension Origin {
    public Flector geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1.xyz
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e4) * geometric_anti_product.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * geometric_anti_product.group0.xyz), 0.0)
        );
    }
}
public extension Origin {
    public Flector geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(geometric_anti_product.group0.xyz, geometric_anti_product.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension Origin {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e4) * float2(geometric_anti_product.e321, geometric_anti_product.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(geometric_anti_product.group3.xyz, geometric_anti_product.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            float3(this.e4) * geometric_anti_product.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e4) * geometric_anti_product.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(geometric_anti_product.group2.xyz, geometric_anti_product.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension Origin {
    public AntiScalar geometric_anti_quotient(Origin other) {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 / (other.e4)
        );
    }
}
public extension Origin {
    public Motor geometric_anti_quotient(Plane other) {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e4) * geometric_anti_product.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e4 * geometric_anti_product.e321)
        );
    }
}
public extension Origin {
    public Motor geometric_anti_quotient(Point other) {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 * geometric_anti_product.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e4) * geometric_anti_product.group0.xyz), 0.0)
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<Plane> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<Plane>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension Plane {
    public Plane geometric_anti_quotient(AntiScalar other) {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/other.e1234) * this.group0
        );
    }
}
public extension Plane {
    public Flector geometric_anti_quotient(DualNum other) {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.e1234, -2)) * other.group0.xy
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.group0.xy.xx, geometric_anti_product.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group0
        );
    }
}
public extension Plane {
    public Motor geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(geometric_anti_product.e4 * this.e423) - (geometric_anti_product.e412 * this.e431), -(geometric_anti_product.e4 * this.e431) - (geometric_anti_product.e423 * this.e412), -(geometric_anti_product.e4 * this.e412) - (geometric_anti_product.e431 * this.e423), (geometric_anti_product.e431 * this.e431) + (geometric_anti_product.e412 * this.e412)) + (geometric_anti_product.group1.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product.e3 * this.e431) + (geometric_anti_product.e321 * this.e423), (geometric_anti_product.e1 * this.e412) + (geometric_anti_product.e321 * this.e431), (geometric_anti_product.e2 * this.e423) + (geometric_anti_product.e321 * this.e412), -(geometric_anti_product.e3 * this.e412) - (geometric_anti_product.e4 * this.e321)) - (geometric_anti_product.group0.yzxx * this.group0.zxyx) - (this.group0.wwwy * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e2))
        );
    }
}
public extension Plane {
    public Flector geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1.xyz
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), -(geometric_anti_product.e42 * this.e431) - (geometric_anti_product.e43 * this.e412)) - (this.group0.yzxx * float4(geometric_anti_product.group1.xyz.zxy, geometric_anti_product.e41)), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e42 * this.e412 * -1.0, geometric_anti_product.e43 * this.e423 * -1.0, geometric_anti_product.e41 * this.e431 * -1.0, (geometric_anti_product.e31 * this.e431) + (geometric_anti_product.e12 * this.e412)) + (this.group0.yzxx * float4(geometric_anti_product.group0.xyz.zxy, geometric_anti_product.e23))
        );
    }
}
public extension Plane {
    public Flector geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), geometric_anti_product.e43 * this.e412 * -1.0) - (this.group0.xyzy * float4(geometric_anti_product.group1.www, geometric_anti_product.e42)) - (this.group0.yzxx * float4(geometric_anti_product.group1.zxy, geometric_anti_product.e41)), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e42 * this.e412 * -1.0, geometric_anti_product.e43 * this.e423 * -1.0, geometric_anti_product.e41 * this.e431 * -1.0, (geometric_anti_product.e31 * this.e431) + (geometric_anti_product.e12 * this.e412)) + (geometric_anti_product.group0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(geometric_anti_product.group0.www, geometric_anti_product.e23))
        );
    }
}
public extension Plane {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(geometric_anti_product.e1 * this.e423) - (geometric_anti_product.e2 * this.e431) - (geometric_anti_product.e3 * this.e412) - (geometric_anti_product.e4 * this.e321), (geometric_anti_product.e423 * this.e423) + (geometric_anti_product.e431 * this.e431) + (geometric_anti_product.e412 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), geometric_anti_product.e43 * this.e412 * -1.0) - (this.group0.xyzx * float4(geometric_anti_product.group0.xy.xx, geometric_anti_product.scalar, geometric_anti_product.e41)) - (this.group0.yzxy * float4(geometric_anti_product.group3.xyz.zxy, geometric_anti_product.e42)), 
            /* e41, e42, e43 */
            (geometric_anti_product.group4.yzx * this.group0.zxy) - (float3(geometric_anti_product.e4) * this.group0.xyz) - (geometric_anti_product.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.e321) * this.group0.xyz) + (geometric_anti_product.group1.zxy * this.group0.yzx) - (float3(this.e321) * geometric_anti_product.group4.xyz) - (geometric_anti_product.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e42 * this.e412 * -1.0, geometric_anti_product.e43 * this.e423 * -1.0, geometric_anti_product.e41 * this.e431 * -1.0, (geometric_anti_product.e31 * this.e431) + (geometric_anti_product.e12 * this.e412)) + (float4(geometric_anti_product.e1234) * this.group0) + (this.group0.yzxx * float4(geometric_anti_product.group2.xyz.zxy, geometric_anti_product.e23))
        );
    }
}
public extension Plane {
    public Motor geometric_anti_quotient(Origin other) {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(geometric_anti_product.e4) * this.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_anti_product.e4 * this.e321 * -1.0)
        );
    }
}
public extension Plane {
    public Motor geometric_anti_quotient(Plane other) {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e412 * this.e431 * -1.0, geometric_anti_product.e423 * this.e412 * -1.0, geometric_anti_product.e431 * this.e423 * -1.0, (geometric_anti_product.e431 * this.e431) + (geometric_anti_product.e412 * this.e412)) + (geometric_anti_product.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(geometric_anti_product.e321) * this.group0.xyz) - (float3(this.e321) * geometric_anti_product.group0.xyz)), 0.0)
        );
    }
}
public extension Plane {
    public Motor geometric_anti_quotient(Point other) {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(geometric_anti_product.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e431 * geometric_anti_product.e3, this.e412 * geometric_anti_product.e1, this.e423 * geometric_anti_product.e2, -(this.e431 * geometric_anti_product.e2) - (this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4)) - (this.group0.zxyx * geometric_anti_product.group0.yzxx)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<Point> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<Point>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Point operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension Point {
    public Point geometric_anti_quotient(AntiScalar other) {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/other.e1234) * this.group0
        );
    }
}
public extension Point {
    public Flector geometric_anti_quotient(DualNum other) {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.e1234, -2)) * other.group0.xy
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product.scalar * this.e4 * -1.0)
        );
    }
}
public extension Point {
    public Motor geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(-(geometric_anti_product.e4 * this.e1) - (geometric_anti_product.e412 * this.e2), -(geometric_anti_product.e4 * this.e2) - (geometric_anti_product.e423 * this.e3), -(geometric_anti_product.e4 * this.e3) - (geometric_anti_product.e431 * this.e1), (geometric_anti_product.e412 * this.e3) + (geometric_anti_product.e321 * this.e4)) + (geometric_anti_product.group1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(geometric_anti_product.group0.xyz, geometric_anti_product.e423))
        );
    }
}
public extension Point {
    public Flector geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1.xyz
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.group0.xyz.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * geometric_anti_product.group1.xyz), 0.0) - float4((geometric_anti_product.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * geometric_anti_product.group0.xyz), -(geometric_anti_product.e41 * this.e1) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3))
        );
    }
}
public extension Point {
    public Flector geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group0.xyz) + (geometric_anti_product.group0.zxy * this.group0.yzx) - (float3(this.e4) * geometric_anti_product.group1.xyz) - (geometric_anti_product.group0.yzx * this.group0.zxy)), geometric_anti_product.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * geometric_anti_product.group0.xyz), -(geometric_anti_product.e41 * this.e1) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3) - (geometric_anti_product.scalar * this.e4))
        );
    }
}
public extension Point {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.e423 * this.e1) + (geometric_anti_product.e431 * this.e2) + (geometric_anti_product.e412 * this.e3) + (geometric_anti_product.e321 * this.e4), geometric_anti_product.e4 * this.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group0.xyz) + (geometric_anti_product.group2.xyz.zxy * this.group0.yzx) - (float3(this.e4) * geometric_anti_product.group3.xyz) - (geometric_anti_product.group2.xyz.yzx * this.group0.zxy)), geometric_anti_product.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * geometric_anti_product.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * geometric_anti_product.group1.xyz) + (geometric_anti_product.group4.yzx * this.group0.zxy) - (float3(geometric_anti_product.e4) * this.group0.xyz) - (geometric_anti_product.group4.zxy * this.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * geometric_anti_product.group2.xyz), -(geometric_anti_product.scalar * this.e4) - (geometric_anti_product.e41 * this.e1) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3))
        );
    }
}
public extension Point {
    public Motor geometric_anti_quotient(Origin other) {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_anti_product.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(geometric_anti_product.e4) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Point {
    public Motor geometric_anti_quotient(Plane other) {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(geometric_anti_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product.e412 * this.e2 * -1.0, geometric_anti_product.e423 * this.e3 * -1.0, geometric_anti_product.e431 * this.e1 * -1.0, (geometric_anti_product.e431 * this.e2) + (geometric_anti_product.e412 * this.e3) + (geometric_anti_product.e321 * this.e4)) + (geometric_anti_product.group0.yzxx * this.group0.zxyx)
        );
    }
}
public extension Point {
    public Motor geometric_anti_quotient(Point other) {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_anti_product.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * geometric_anti_product.group0.xyz) - (float3(geometric_anti_product.e4) * this.group0.xyz)), 0.0)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public GeometricAntiQuotientInfixPartial<Scalar> operator /(GeometricAntiQuotientInfix rhs) {
        return GeometricAntiQuotientInfixPartial<Scalar>(this);
    }
}
public extension GeometricAntiQuotientInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiScalar rhs) {
        return this.a.geometric_anti_quotient(rhs);
    }
}
public extension Scalar {
    public Scalar geometric_anti_quotient(AntiScalar other) {
        return Scalar.from_groups(
            /* scalar */
            this.scalar / (other.e1234)
        );
    }
}
public extension Scalar {
    public Scalar geometric_anti_quotient(DualNum other) {
        return Scalar.from_groups(
            /* scalar */
            this.scalar / (other.e1234)
        );
    }
}
public extension Scalar {
    public Flector geometric_anti_quotient(Flector other) {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e4, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * geometric_anti_product.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product.e4 * this.scalar)
        );
    }
}
public extension Scalar {
    public Line geometric_anti_quotient(Line other) {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * float3(other_2.e1234 * anti_reverse.e41, other_2.e1234 * anti_reverse.e42, other_2.e1234 * anti_reverse.e43)
        );
    }
}
public extension Scalar {
    public Motor geometric_anti_quotient(Motor other) {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e1234, 2)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * float4(other_2.e1234 * anti_reverse.e41, other_2.e1234 * anti_reverse.e42, other_2.e1234 * anti_reverse.e43, other_2.e1234 * anti_reverse.e1234)
        );
    }
}
public extension Scalar {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e1234, 2) + pow(other.e4, 2) + pow(other.e41, 2) + pow(other.e42, 2) + pow(other.e43, 2) + pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product.e1234 * this.scalar, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * geometric_anti_product.group4.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_anti_product.group2.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product.e4 * this.scalar)
        );
    }
}
public extension Scalar {
    public Horizon geometric_anti_quotient(Origin other) {
        return Horizon.from_groups(
            /* e321 */
            this.scalar / (other.e4) * -1.0
        );
    }
}
public extension Scalar {
    public Point geometric_anti_quotient(Plane other) {
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            pow(other.e423, 2) + pow(other.e431, 2) + pow(other.e412, 2)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * float4(other_2.e1234 * other.e423, other_2.e1234 * other.e431, other_2.e1234 * other.e412, other_2.e1234 * other.e321).xyz), 0.0)
        );
    }
}
public extension Scalar {
    public Horizon geometric_anti_quotient(Point other) {
        return Horizon.from_groups(
            /* e321 */
            this.scalar / (other.e4) * -1.0
        );
    }
}
