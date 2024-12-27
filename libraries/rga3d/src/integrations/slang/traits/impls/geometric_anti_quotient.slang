extension AntiScalar {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<AntiScalar> {
        geometric_anti_quotient_partial(this)
    }
}
extension AntiScalar: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 / (other.e1234)
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> DualNum {
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, -2)
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * float2(other_2.e1234 * other.scalar, other_2.e1234 * other.e1234)
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Flector {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
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
extension AntiScalar: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Line {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_anti_product.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * geometric_anti_product.group1
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Motor {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
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
extension AntiScalar: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * geometric_anti_product.group0, 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * geometric_anti_product.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_anti_product.group2, 
            /* e23, e31, e12 */
            float3(this.e1234) * geometric_anti_product.group3, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * geometric_anti_product.group4
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 / (other.e4) * -1.0
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Plane {
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * float4(other_2.e1234 * other.e423, other_2.e1234 * other.e431, other_2.e1234 * other.e412, other_2.e1234 * other.e321)
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Point {
Point anti_reverse = Point.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, -2)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * float4(other_2.e1234 * anti_reverse.e1, other_2.e1234 * anti_reverse.e2, other_2.e1234 * anti_reverse.e3, other_2.e1234 * anti_reverse.e4)
        );
    }
}
extension DualNum {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<DualNum> {
        geometric_anti_quotient_partial(this)
    }
}
extension DualNum: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0/other.e1234) * this.group0
        );
    }
}
extension DualNum: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> DualNum {
DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.e1234, -2)) * other.group0
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.scalar * this.e1234) + (geometric_anti_product.e1234 * this.scalar), geometric_anti_product.e1234 * this.e1234)
        );
    }
}
extension DualNum: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Flector {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
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
            float4(geometric_anti_product.e423, geometric_anti_product.e431, geometric_anti_product.e412, 1.0) * float4(this.group0.yy, this.e1234, (this.scalar * geometric_anti_product.e4) + (this.e1234 * geometric_anti_product.e321))
        );
    }
}
extension DualNum: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Line {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_anti_product.group0, 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_anti_product.group0) + (float3(this.e1234) * geometric_anti_product.group1)
        );
    }
}
extension DualNum: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Motor {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
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
extension DualNum: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * geometric_anti_product.e1234) + (this.e1234 * geometric_anti_product.scalar), this.e1234 * geometric_anti_product.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * geometric_anti_product.group4.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz)), this.e1234 * geometric_anti_product.e4), 
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_anti_product.group2, 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_anti_product.group2) + (float3(this.e1234) * geometric_anti_product.group3), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e423, geometric_anti_product.e431, geometric_anti_product.e412, 1.0) * float4(this.group0.yy, this.e1234, (this.scalar * geometric_anti_product.e4) + (this.e1234 * geometric_anti_product.e321))
        );
    }
}
extension DualNum: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> Flector {
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
extension DualNum: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Flector {
Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(geometric_anti_product.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * geometric_anti_product.group0
        );
    }
}
extension DualNum: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Flector {
Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * geometric_anti_product.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * geometric_anti_product.e4)
        );
    }
}
extension Flector {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Flector> {
        geometric_anti_quotient_partial(this)
    }
}
extension Flector: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> Flector {
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
extension Flector: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> Flector {
DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.e1234, -2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group0.xyz) - (float3(geometric_anti_product.scalar) * this.group1.xyz)), geometric_anti_product.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 1.0) * float4(geometric_anti_product.group0.yy, geometric_anti_product.e1234, (geometric_anti_product.e1234 * this.e321) - (geometric_anti_product.scalar * this.e4))
        );
    }
}
extension Flector: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Motor {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
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
extension Flector: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Flector {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * geometric_anti_product.e43) + (this.e412 * geometric_anti_product.e31) + (this.e321 * geometric_anti_product.e41), (this.e3 * geometric_anti_product.e41) + (this.e423 * geometric_anti_product.e12) + (this.e321 * geometric_anti_product.e42), (this.e1 * geometric_anti_product.e42) + (this.e431 * geometric_anti_product.e23) + (this.e321 * geometric_anti_product.e43), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(geometric_anti_product.group1.xxy, geometric_anti_product.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(geometric_anti_product.group1.zyz, geometric_anti_product.e43)) - float4((geometric_anti_product.group0.yzx * this.group0.zxy), this.e423 * geometric_anti_product.e41), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(geometric_anti_product.group0.xxy, geometric_anti_product.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(geometric_anti_product.group0.zyz, geometric_anti_product.e31)) + float4(float3(0.0), (this.e412 * geometric_anti_product.e12) - (this.e2 * geometric_anti_product.e42) - (this.e3 * geometric_anti_product.e43)) - float4((geometric_anti_product.group0.yzx * this.group1.zxy), this.e1 * geometric_anti_product.e41)
        );
    }
}
extension Flector: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Flector {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
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
extension Flector: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e4 * geometric_anti_product.e321) - (this.e431 * geometric_anti_product.e2) - (this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4), 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * geometric_anti_product.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((this.e2 * geometric_anti_product.e43) + (this.e412 * geometric_anti_product.e31) + (this.e321 * geometric_anti_product.e41) - (this.e431 * geometric_anti_product.e12), (this.e3 * geometric_anti_product.e41) + (this.e423 * geometric_anti_product.e12) + (this.e321 * geometric_anti_product.e42) - (this.e4 * geometric_anti_product.e31), (this.e1 * geometric_anti_product.e42) + (this.e431 * geometric_anti_product.e23) + (this.e321 * geometric_anti_product.e43) - (this.e4 * geometric_anti_product.e12), 0.0) + (float4(geometric_anti_product.e1234) * this.group0) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(geometric_anti_product.group3.xxy, geometric_anti_product.e43)) - (this.group1.xyzx * float4(geometric_anti_product.group0.xx, geometric_anti_product.scalar, geometric_anti_product.e41)) - float4((geometric_anti_product.group2.yzx * this.group0.zxy), this.e431 * geometric_anti_product.e42), 
            /* e41, e42, e43 */
            (this.group1.zxy * geometric_anti_product.group4.yzx) - (float3(this.e4) * geometric_anti_product.group4.xyz) - (float3(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e431) * this.group1.xyx) - (float3(geometric_anti_product.e412, geometric_anti_product.e423, geometric_anti_product.e4) * this.group1.yzz), 
            /* e23, e31, e12 */
            (float3(this.e4) * geometric_anti_product.group1.xyz) + (float3(geometric_anti_product.e3, geometric_anti_product.e1, geometric_anti_product.e321) * this.group1.yzz) + (float3(geometric_anti_product.e321, geometric_anti_product.e321, geometric_anti_product.e2) * this.group1.xyx) + (this.group0.zxy * geometric_anti_product.group4.yzx) - (float3(this.e321) * geometric_anti_product.group4.xyz) - (float3(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e431) * this.group0.xyx) - (float3(geometric_anti_product.e412, geometric_anti_product.e423, geometric_anti_product.e4) * this.group0.yzz) - (this.group1.zxy * geometric_anti_product.group1.yzx), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product.e1234) * this.group1) + (float4(this.e4, this.e412, this.e423, this.e423) * float4(geometric_anti_product.group2.xxy, geometric_anti_product.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(geometric_anti_product.group2.zyz, geometric_anti_product.e31)) + float4(float3(0.0), (this.e412 * geometric_anti_product.e12) - (this.e1 * geometric_anti_product.e41) - (this.e2 * geometric_anti_product.e42) - (this.e3 * geometric_anti_product.e43)) - float4((geometric_anti_product.group2.yzx * this.group1.zxy), this.e4 * geometric_anti_product.scalar)
        );
    }
}
extension Flector: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> Motor {
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
extension Flector: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Motor {
Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e4 * geometric_anti_product.e423) - (this.e431 * geometric_anti_product.e412), -(this.e4 * geometric_anti_product.e431) - (this.e412 * geometric_anti_product.e423), -(this.e4 * geometric_anti_product.e412) - (this.e423 * geometric_anti_product.e431), (this.e431 * geometric_anti_product.e431) + (this.e412 * geometric_anti_product.e412)) + (this.group1.zxyx * geometric_anti_product.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(this.e2 * geometric_anti_product.e412) - (this.e321 * geometric_anti_product.e423), -(this.e3 * geometric_anti_product.e423) - (this.e321 * geometric_anti_product.e431), -(this.e1 * geometric_anti_product.e431) - (this.e321 * geometric_anti_product.e412), (this.e3 * geometric_anti_product.e412) + (this.e4 * geometric_anti_product.e321)) + (this.group0.zxyx * geometric_anti_product.group0.yzxx) + (geometric_anti_product.group0.wwwy * float4(this.group1.xyz, this.e2))
        );
    }
}
extension Flector: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Motor {
Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((this.e4 * geometric_anti_product.e1) + (this.e431 * geometric_anti_product.e3), (this.e4 * geometric_anti_product.e2) + (this.e412 * geometric_anti_product.e1), (this.e4 * geometric_anti_product.e3) + (this.e423 * geometric_anti_product.e2), -(this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4)) - (this.group1.zxyy * geometric_anti_product.group0.yzxy) - (geometric_anti_product.group0.wwwx * float4(this.group0.xyz, this.e423))
        );
    }
}
extension Horizon {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Horizon> {
        geometric_anti_quotient_partial(this)
    }
}
extension Horizon: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 / (other.e1234)
        );
    }
}
extension Horizon: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 / (other.e1234)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Motor {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
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
extension Horizon: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Point {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(float3(other_2.e1234 * anti_reverse.e41, other_2.e1234 * anti_reverse.e42, other_2.e1234 * anti_reverse.e43), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Flector {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
        );
Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(geometric_anti_product.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * geometric_anti_product.e1234)
        );
    }
}
extension Horizon: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * geometric_anti_product.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(geometric_anti_product.group2, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * geometric_anti_product.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * geometric_anti_product.e1234)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 / (other.e4)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Line {
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * float4(other_2.e1234 * other.e423, other_2.e1234 * other.e431, other_2.e1234 * other.e412, other_2.e1234 * other.e321).xyz * float3(-1.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 / (other.e4)
        );
    }
}
extension Line {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Line> {
        geometric_anti_quotient_partial(this)
    }
}
extension Line: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> Line {
AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/other.e1234
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group1
        );
    }
}
extension Line: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> Line {
DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.e1234, -2)) * other.group0
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group0) + (float3(geometric_anti_product.e1234) * this.group1)
        );
    }
}
extension Line: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Flector {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e3 * this.e42) + (geometric_anti_product.e4 * this.e23) + (geometric_anti_product.e412 * this.e31) + (geometric_anti_product.e321 * this.e41), (geometric_anti_product.e1 * this.e43) + (geometric_anti_product.e4 * this.e31) + (geometric_anti_product.e423 * this.e12) + (geometric_anti_product.e321 * this.e42), (geometric_anti_product.e2 * this.e41) + (geometric_anti_product.e4 * this.e12) + (geometric_anti_product.e431 * this.e23) + (geometric_anti_product.e321 * this.e43), geometric_anti_product.e412 * this.e43 * -1.0) - (geometric_anti_product.group1.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group0.zxy * geometric_anti_product.group0.yzx), geometric_anti_product.e423 * this.e41), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product.e4 * this.e41) + (geometric_anti_product.e412 * this.e42), (geometric_anti_product.e4 * this.e42) + (geometric_anti_product.e423 * this.e43), (geometric_anti_product.e4 * this.e43) + (geometric_anti_product.e431 * this.e41), -(geometric_anti_product.e2 * this.e42) - (geometric_anti_product.e3 * this.e43) - (geometric_anti_product.e423 * this.e23) - (geometric_anti_product.e431 * this.e31) - (geometric_anti_product.e412 * this.e12)) - float4((this.group0.zxy * geometric_anti_product.group1.yzx), geometric_anti_product.e1 * this.e41)
        );
    }
}
extension Line: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Motor {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e43 * this.e42, geometric_anti_product.e41 * this.e43, geometric_anti_product.e42 * this.e41, -(geometric_anti_product.e42 * this.e42) - (geometric_anti_product.e43 * this.e43)) - float4((geometric_anti_product.group0.yzx * this.group0.zxy), geometric_anti_product.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product.e43 * this.e31) + (geometric_anti_product.e12 * this.e42), (geometric_anti_product.e41 * this.e12) + (geometric_anti_product.e23 * this.e43), (geometric_anti_product.e42 * this.e23) + (geometric_anti_product.e31 * this.e41), -(geometric_anti_product.e43 * this.e12) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43)) - float4((geometric_anti_product.group0.yzx * this.group1.zxy), geometric_anti_product.e41 * this.e23) - float4((geometric_anti_product.group1.yzx * this.group0.zxy), geometric_anti_product.e42 * this.e31)
        );
    }
}
extension Line: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Motor {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
        );
Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * geometric_anti_product.e1234) + (this.e42 * geometric_anti_product.e43), (this.e42 * geometric_anti_product.e1234) + (this.e43 * geometric_anti_product.e41), (this.e41 * geometric_anti_product.e42) + (this.e43 * geometric_anti_product.e1234), -(this.e42 * geometric_anti_product.e42) - (this.e43 * geometric_anti_product.e43)) - (geometric_anti_product.group0.yzxx * float4(this.group0.zxy, this.e41)), 
            /* e23, e31, e12, scalar */
            float4((this.e41 * geometric_anti_product.scalar) + (this.e42 * geometric_anti_product.e12) + (this.e23 * geometric_anti_product.e1234) + (this.e31 * geometric_anti_product.e43), (this.e42 * geometric_anti_product.scalar) + (this.e43 * geometric_anti_product.e23) + (this.e31 * geometric_anti_product.e1234) + (this.e12 * geometric_anti_product.e41), (this.e41 * geometric_anti_product.e31) + (this.e43 * geometric_anti_product.scalar) + (this.e23 * geometric_anti_product.e42) + (this.e12 * geometric_anti_product.e1234), -(this.e43 * geometric_anti_product.e12) - (this.e23 * geometric_anti_product.e41) - (this.e31 * geometric_anti_product.e42) - (this.e12 * geometric_anti_product.e43)) - (geometric_anti_product.group1.yzxx * float4(this.group0.zxy, this.e41)) - float4((this.group1.zxy * geometric_anti_product.group0.yzx), this.e42 * geometric_anti_product.e31)
        );
    }
}
extension Line: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e23 * geometric_anti_product.e41) - (this.e31 * geometric_anti_product.e42) - (this.e12 * geometric_anti_product.e43), 0.0) - (float2(this.e41) * float2(geometric_anti_product.e23, geometric_anti_product.e41)) - (float2(this.e42) * float2(geometric_anti_product.e31, geometric_anti_product.e42)) - (float2(this.e43) * float2(geometric_anti_product.e12, geometric_anti_product.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * geometric_anti_product.e321) + (this.e42 * geometric_anti_product.e3) + (this.e23 * geometric_anti_product.e4) + (this.e31 * geometric_anti_product.e412), (this.e42 * geometric_anti_product.e321) + (this.e43 * geometric_anti_product.e1) + (this.e31 * geometric_anti_product.e4) + (this.e12 * geometric_anti_product.e423), (this.e41 * geometric_anti_product.e2) + (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431) + (this.e12 * geometric_anti_product.e4), this.e43 * geometric_anti_product.e412 * -1.0) - (geometric_anti_product.group4.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group0.zxy * geometric_anti_product.group1.yzx), this.e41 * geometric_anti_product.e423), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product.e1234) * this.group0) + (this.group0.yzx * geometric_anti_product.group2.zxy) - (this.group0.zxy * geometric_anti_product.group2.yzx), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group0) + (float3(geometric_anti_product.e1234) * this.group1) + (this.group0.yzx * geometric_anti_product.group3.zxy) + (this.group1.yzx * geometric_anti_product.group2.zxy) - (this.group0.zxy * geometric_anti_product.group3.yzx) - (this.group1.zxy * geometric_anti_product.group2.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e41 * geometric_anti_product.e4) + (this.e42 * geometric_anti_product.e412), (this.e42 * geometric_anti_product.e4) + (this.e43 * geometric_anti_product.e423), (this.e41 * geometric_anti_product.e431) + (this.e43 * geometric_anti_product.e4), -(this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3) - (this.e23 * geometric_anti_product.e423) - (this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) - float4((this.group0.zxy * geometric_anti_product.group4.yzx), this.e41 * geometric_anti_product.e1)
        );
    }
}
extension Line: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> Flector {
Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 0.0) * float4(this.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 0.0) * float4(this.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Flector {
Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * geometric_anti_product.e321) + (this.e31 * geometric_anti_product.e412), (this.e42 * geometric_anti_product.e321) + (this.e12 * geometric_anti_product.e423), (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431), -(this.e42 * geometric_anti_product.e431) - (this.e43 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * geometric_anti_product.e412, this.e43 * geometric_anti_product.e423, this.e41 * geometric_anti_product.e431, -(this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Line: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Flector {
Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_anti_product.e4) * this.group1), 0.0) + float4((this.group0.yzx * geometric_anti_product.group0.zxy), 0.0) - float4((this.group0.zxy * geometric_anti_product.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 1.0) * float4(this.group0, -(this.e41 * geometric_anti_product.e1) - (this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3))
        );
    }
}
extension Motor {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Motor> {
        geometric_anti_quotient_partial(this)
    }
}
extension Motor: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> Motor {
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
extension Motor: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> Motor {
DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.e1234, -2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(geometric_anti_product.scalar) * this.group0) + (float4(geometric_anti_product.e1234) * this.group1)
        );
    }
}
extension Motor: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Flector {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
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
extension Motor: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Motor {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_anti_product.e41 * this.e1234) + (geometric_anti_product.e43 * this.e42), (geometric_anti_product.e41 * this.e43) + (geometric_anti_product.e42 * this.e1234), (geometric_anti_product.e42 * this.e41) + (geometric_anti_product.e43 * this.e1234), -(geometric_anti_product.e42 * this.e42) - (geometric_anti_product.e43 * this.e43)) - (this.group0.zxyx * float4(geometric_anti_product.group0.yzx, geometric_anti_product.e41)), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product.e41 * this.scalar) + (geometric_anti_product.e43 * this.e31) + (geometric_anti_product.e23 * this.e1234) + (geometric_anti_product.e12 * this.e42), (geometric_anti_product.e41 * this.e12) + (geometric_anti_product.e42 * this.scalar) + (geometric_anti_product.e23 * this.e43) + (geometric_anti_product.e31 * this.e1234), (geometric_anti_product.e42 * this.e23) + (geometric_anti_product.e43 * this.scalar) + (geometric_anti_product.e31 * this.e41) + (geometric_anti_product.e12 * this.e1234), -(geometric_anti_product.e43 * this.e12) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43)) - (this.group1.zxyx * float4(geometric_anti_product.group0.yzx, geometric_anti_product.e41)) - float4((geometric_anti_product.group1.yzx * this.group0.zxy), geometric_anti_product.e42 * this.e31)
        );
    }
}
extension Motor: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Motor {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
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
extension Motor: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * geometric_anti_product.e1234) - (this.e41 * geometric_anti_product.e23) - (this.e42 * geometric_anti_product.e31) - (this.e43 * geometric_anti_product.e12), 0.0) + (float2(this.e1234) * geometric_anti_product.group0) - (float2(geometric_anti_product.e41) * float2(this.e23, this.e41)) - (float2(geometric_anti_product.e42) * float2(this.e31, this.e42)) - (float2(geometric_anti_product.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e42 * geometric_anti_product.e3) + (this.e1234 * geometric_anti_product.e1) + (this.e23 * geometric_anti_product.e4) + (this.e31 * geometric_anti_product.e412) + (this.scalar * geometric_anti_product.e423), (this.e43 * geometric_anti_product.e1) + (this.e1234 * geometric_anti_product.e2) + (this.e31 * geometric_anti_product.e4) + (this.e12 * geometric_anti_product.e423) + (this.scalar * geometric_anti_product.e431), (this.e43 * geometric_anti_product.e321) + (this.e1234 * geometric_anti_product.e3) + (this.e23 * geometric_anti_product.e431) + (this.e12 * geometric_anti_product.e4) + (this.scalar * geometric_anti_product.e412), this.e43 * geometric_anti_product.e412 * -1.0) + (this.group0.xyxw * float4(geometric_anti_product.group4.ww, geometric_anti_product.e2, geometric_anti_product.e4)) - (this.group0.zxyx * float4(geometric_anti_product.group1.yzx, geometric_anti_product.e423)) - (geometric_anti_product.group4.yzxy * float4(this.group1.zxy, this.e42)), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product.e1234) * this.group0.xyz) + (geometric_anti_product.group2.xxy * this.group0.wzx) + (geometric_anti_product.group2.zyz * this.group0.yww) - (geometric_anti_product.group2.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group0.xyz) + (float3(geometric_anti_product.e1234) * this.group1.xyz) + (geometric_anti_product.group2.xxy * this.group1.wzx) + (geometric_anti_product.group2.zyz * this.group1.yww) + (geometric_anti_product.group3.xxy * this.group0.wzx) + (geometric_anti_product.group3.zyz * this.group0.yww) - (geometric_anti_product.group2.yzx * this.group1.zxy) - (geometric_anti_product.group3.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * geometric_anti_product.e423, this.e1234 * geometric_anti_product.e431, this.e1234 * geometric_anti_product.e412, -(this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3) - (this.e23 * geometric_anti_product.e423) - (this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) + (this.group0.xyxw * float4(geometric_anti_product.group1.ww, geometric_anti_product.e431, geometric_anti_product.e321)) + (float4(geometric_anti_product.group4.zx, geometric_anti_product.e4, geometric_anti_product.e4) * float4(this.group0.yzz, this.scalar)) - (this.group0.zxyx * float4(geometric_anti_product.group4.yzx, geometric_anti_product.e1))
        );
    }
}
extension Motor: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> Flector {
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
extension Motor: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Flector {
Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * geometric_anti_product.e321) + (this.e31 * geometric_anti_product.e412) + (this.scalar * geometric_anti_product.e423), (this.e42 * geometric_anti_product.e321) + (this.e12 * geometric_anti_product.e423) + (this.scalar * geometric_anti_product.e431), (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431) + (this.scalar * geometric_anti_product.e412), -(this.e42 * geometric_anti_product.e431) - (this.e43 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * geometric_anti_product.e423, this.e1234 * geometric_anti_product.e431, this.e1234 * geometric_anti_product.e412, -(this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) + (this.group0.yzxw * geometric_anti_product.group0.zxyw) - (geometric_anti_product.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Motor: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Flector {
Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * geometric_anti_product.group0.xyz) + (float3(geometric_anti_product.e4) * this.group1.xyz) + (this.group0.yzx * geometric_anti_product.group0.zxy) - (this.group0.zxy * geometric_anti_product.group0.yzx)), this.e1234 * geometric_anti_product.e4), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 1.0) * float4(this.group0.xyz, (this.scalar * geometric_anti_product.e4) - (this.e41 * geometric_anti_product.e1) - (this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3))
        );
    }
}
extension MultiVector {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<MultiVector> {
        geometric_anti_quotient_partial(this)
    }
}
extension MultiVector: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> MultiVector {
AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/other.e1234
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product.e1234) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group2, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group4
        );
    }
}
extension MultiVector: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> MultiVector {
DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.e1234, -2)) * other.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.scalar * this.e1234) + (geometric_anti_product.e1234 * this.scalar), geometric_anti_product.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group1.xyz) - (float3(geometric_anti_product.scalar) * this.group4.xyz)), geometric_anti_product.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group2, 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group2) + (float3(geometric_anti_product.e1234) * this.group3), 
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 1.0) * float4(geometric_anti_product.group0.yy, geometric_anti_product.e1234, (geometric_anti_product.e1234 * this.e321) - (geometric_anti_product.scalar * this.e4))
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> MultiVector {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
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
            float4((geometric_anti_product.e1 * this.e1234) + (geometric_anti_product.e3 * this.e42) + (geometric_anti_product.e4 * this.e23) + (geometric_anti_product.e412 * this.e31) + (geometric_anti_product.e321 * this.e41), (geometric_anti_product.e1 * this.e43) + (geometric_anti_product.e2 * this.e1234) + (geometric_anti_product.e4 * this.e31) + (geometric_anti_product.e423 * this.e12) + (geometric_anti_product.e321 * this.e42), (geometric_anti_product.e2 * this.e41) + (geometric_anti_product.e3 * this.e1234) + (geometric_anti_product.e4 * this.e12) + (geometric_anti_product.e431 * this.e23) + (geometric_anti_product.e321 * this.e43), geometric_anti_product.e412 * this.e43 * -1.0) + (float4(this.group0.xx, this.scalar, this.e1234) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e4)) - (geometric_anti_product.group1.yzxy * float4(this.group3.zxy, this.e42)) - float4((this.group2.zxy * geometric_anti_product.group0.yzx), geometric_anti_product.e423 * this.e41), 
            /* e41, e42, e43 */
            (geometric_anti_product.group1.yzx * this.group4.zxy) - (float3(geometric_anti_product.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group1.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group1.zyz), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.e321) * this.group4.xyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group0.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group0.zyz) + (geometric_anti_product.group1.yzx * this.group1.zxy) - (float3(geometric_anti_product.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * geometric_anti_product.group1.zyz) - (float3(this.e321, this.e3, this.e1) * geometric_anti_product.group1.xxy) - (geometric_anti_product.group0.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e412 * this.e42, geometric_anti_product.e423 * this.e43, geometric_anti_product.e4 * this.e43, -(geometric_anti_product.e2 * this.e42) - (geometric_anti_product.e3 * this.e43) - (geometric_anti_product.e423 * this.e23) - (geometric_anti_product.e431 * this.e31) - (geometric_anti_product.e412 * this.e12)) + (float4(this.group0.yy, this.e1234, this.scalar) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e4)) + (float4(geometric_anti_product.group0.ww, geometric_anti_product.e431, geometric_anti_product.e321) * float4(this.group2.xyx, this.e1234)) - float4((this.group2.zxy * geometric_anti_product.group1.yzx), geometric_anti_product.e1 * this.e41)
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> MultiVector {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43), 0.0) - (float2(geometric_anti_product.e41) * float2(this.e23, this.e41)) - (float2(geometric_anti_product.e42) * float2(this.e31, this.e42)) - (float2(geometric_anti_product.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e43 * this.e2) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e41 * this.e3) + (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e42 * this.e1) + (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(geometric_anti_product.group1.xxy, geometric_anti_product.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(geometric_anti_product.group1.zyz, geometric_anti_product.e43)) - float4((geometric_anti_product.group0.yzx * this.group1.zxy), geometric_anti_product.e41 * this.e423), 
            /* e41, e42, e43 */
            (float3(this.e1234) * geometric_anti_product.group0) + (geometric_anti_product.group0.zxy * this.group2.yzx) - (geometric_anti_product.group0.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_anti_product.group0) + (float3(this.e1234) * geometric_anti_product.group1) + (geometric_anti_product.group0.zxy * this.group3.yzx) + (geometric_anti_product.group1.zxy * this.group2.yzx) - (geometric_anti_product.group0.yzx * this.group3.zxy) - (geometric_anti_product.group1.yzx * this.group2.zxy), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(geometric_anti_product.group0.xxy, geometric_anti_product.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(geometric_anti_product.group0.zyz, geometric_anti_product.e31)) + float4(float3(0.0), (geometric_anti_product.e12 * this.e412) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3)) - float4((geometric_anti_product.group0.yzx * this.group4.zxy), geometric_anti_product.e41 * this.e1)
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> MultiVector {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
        );
Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.scalar * this.e1234) - (geometric_anti_product.e41 * this.e23) - (geometric_anti_product.e42 * this.e31) - (geometric_anti_product.e43 * this.e12), 0.0) + (float2(geometric_anti_product.e1234) * this.group0) - (float2(this.e41) * float2(geometric_anti_product.e23, geometric_anti_product.e41)) - (float2(this.e42) * float2(geometric_anti_product.e31, geometric_anti_product.e42)) - (float2(this.e43) * float2(geometric_anti_product.e12, geometric_anti_product.e43)), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e43 * this.e2) + (geometric_anti_product.e1234 * this.e1) + (geometric_anti_product.e31 * this.e412) - (geometric_anti_product.scalar * this.e423), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e1234 * this.e2) + (geometric_anti_product.e12 * this.e423) - (geometric_anti_product.scalar * this.e431), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e1234 * this.e3) + (geometric_anti_product.e23 * this.e431) - (geometric_anti_product.scalar * this.e412), 0.0) + (float4(this.e321, this.e3, this.e1, this.e4) * geometric_anti_product.group0.xxyw) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(geometric_anti_product.group1.xxy, geometric_anti_product.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(geometric_anti_product.group1.zyz, geometric_anti_product.e43)) - (geometric_anti_product.group0.yzxx * float4(this.group1.zxy, this.e423)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * geometric_anti_product.group0.xyz) + (this.group2.xyx * geometric_anti_product.group0.wwy) + (this.group2.yzz * geometric_anti_product.group0.zxw) - (this.group2.zxy * geometric_anti_product.group0.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_anti_product.group0.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz) + (this.group2.xyx * geometric_anti_product.group1.wwy) + (this.group2.yzz * geometric_anti_product.group1.zxw) + (this.group3.xyx * geometric_anti_product.group0.wwy) + (this.group3.yzz * geometric_anti_product.group0.zxw) - (this.group2.zxy * geometric_anti_product.group1.yzx) - (this.group3.zxy * geometric_anti_product.group0.yzx), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e321) * geometric_anti_product.group0.xxyw) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(geometric_anti_product.group0.zyz, geometric_anti_product.e23)) + (this.group4.xyzy * float4(geometric_anti_product.group0.www, geometric_anti_product.e31)) + float4(float3(0.0), (geometric_anti_product.e12 * this.e412) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3) - (geometric_anti_product.scalar * this.e4)) - (geometric_anti_product.group0.yzxx * float4(this.group4.zxy, this.e1))
        );
    }
}
extension MultiVector: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.e1234 * this.scalar) + (geometric_anti_product.e321 * this.e4) - (geometric_anti_product.e2 * this.e431) - (geometric_anti_product.e3 * this.e412) - (geometric_anti_product.e4 * this.e321) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43), 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * geometric_anti_product.group0) - (float2(geometric_anti_product.e41) * float2(this.e23, this.e41)) - (float2(geometric_anti_product.e42) * float2(this.e31, this.e42)) - (float2(geometric_anti_product.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * geometric_anti_product.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e1 * this.e1234) + (geometric_anti_product.e3 * this.e42) + (geometric_anti_product.e4 * this.e23) + (geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e43 * this.e2) + (geometric_anti_product.e31 * this.e412) + (geometric_anti_product.e412 * this.e31) + (geometric_anti_product.e321 * this.e41), (geometric_anti_product.e1 * this.e43) + (geometric_anti_product.e2 * this.e1234) + (geometric_anti_product.e4 * this.e31) + (geometric_anti_product.e41 * this.e3) + (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423) + (geometric_anti_product.e423 * this.e12) + (geometric_anti_product.e321 * this.e42), (geometric_anti_product.e2 * this.e41) + (geometric_anti_product.e3 * this.e1234) + (geometric_anti_product.e4 * this.e12) + (geometric_anti_product.e42 * this.e1) + (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431) + (geometric_anti_product.e431 * this.e23) + (geometric_anti_product.e321 * this.e43), 0.0) + (float4(geometric_anti_product.e1234) * this.group1) + (float4(this.group0.xx, this.scalar, this.e1234) * float4(geometric_anti_product.group4.xyz, geometric_anti_product.e4)) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(geometric_anti_product.group3.xxy, geometric_anti_product.e43)) - (float4(this.e431, this.e4, this.e4, geometric_anti_product.e423) * float4(geometric_anti_product.group3.zyz, this.e41)) - (geometric_anti_product.group4.yzxz * float4(this.group3.zxy, this.e43)) - (this.group4.xyzx * float4(geometric_anti_product.group0.xx, geometric_anti_product.scalar, geometric_anti_product.e41)) - float4((geometric_anti_product.group2.yzx * this.group1.zxy), geometric_anti_product.e42 * this.e431) - float4((this.group2.zxy * geometric_anti_product.group1.yzx), geometric_anti_product.e431 * this.e42), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product.e1234) * this.group2) + (float3(this.e1234) * geometric_anti_product.group2) + (geometric_anti_product.group2.zxy * this.group2.yzx) + (geometric_anti_product.group4.yzx * this.group4.zxy) - (float3(geometric_anti_product.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group4.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group4.zyz) - (geometric_anti_product.group2.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.scalar) * this.group2) + (float3(geometric_anti_product.e1234) * this.group3) + (float3(geometric_anti_product.e321) * this.group4.xyz) + (float3(this.scalar) * geometric_anti_product.group2) + (float3(this.e1234) * geometric_anti_product.group3) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group1.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group1.zyz) + (geometric_anti_product.group2.zxy * this.group3.yzx) + (geometric_anti_product.group3.zxy * this.group2.yzx) + (geometric_anti_product.group4.yzx * this.group1.zxy) - (float3(geometric_anti_product.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * geometric_anti_product.group4.zyz) - (float3(this.e321, this.e3, this.e1) * geometric_anti_product.group4.xxy) - (geometric_anti_product.group2.yzx * this.group3.zxy) - (geometric_anti_product.group3.yzx * this.group2.zxy) - (geometric_anti_product.group1.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, geometric_anti_product.e321) * float4(geometric_anti_product.group2.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(geometric_anti_product.group2.zyz, geometric_anti_product.e23)) + (float4(this.group0.yy, this.e1234, this.scalar) * float4(geometric_anti_product.group4.xyz, geometric_anti_product.e4)) + (float4(geometric_anti_product.group1.ww, geometric_anti_product.e431, this.e431) * float4(this.group2.xyx, geometric_anti_product.e31)) + (float4(geometric_anti_product.group4.zx, geometric_anti_product.e4, this.e412) * float4(this.group2.yzz, geometric_anti_product.e12)) + float4(float3(0.0), -(geometric_anti_product.e1 * this.e41) - (geometric_anti_product.e2 * this.e42) - (geometric_anti_product.e3 * this.e43) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3) - (geometric_anti_product.e423 * this.e23) - (geometric_anti_product.e431 * this.e31) - (geometric_anti_product.e412 * this.e12)) - float4((geometric_anti_product.group2.yzx * this.group4.zxy), geometric_anti_product.scalar * this.e4) - float4((this.group2.zxy * geometric_anti_product.group4.yzx), geometric_anti_product.e41 * this.e1)
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> MultiVector {
Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product.e4) * float2(this.e321, this.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e4) * float4(this.group3, this.e1234), 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e4) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4) * float4(this.group2, this.scalar)
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> MultiVector {
Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e4 * geometric_anti_product.e321, 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)), 
            /* e1, e2, e3, e4 */
            float4((this.scalar * geometric_anti_product.e423) + (this.e41 * geometric_anti_product.e321) + (this.e31 * geometric_anti_product.e412), (this.scalar * geometric_anti_product.e431) + (this.e42 * geometric_anti_product.e321) + (this.e12 * geometric_anti_product.e423), (this.scalar * geometric_anti_product.e412) + (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431), -(this.e42 * geometric_anti_product.e431) - (this.e43 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * geometric_anti_product.group0.yzx) - (float3(this.e4) * geometric_anti_product.group0.xyz) - (this.group4.yzx * geometric_anti_product.group0.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.e321) * this.group4.xyz) + (this.group1.zxy * geometric_anti_product.group0.yzx) - (float3(this.e321) * geometric_anti_product.group0.xyz) - (this.group1.yzx * geometric_anti_product.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * geometric_anti_product.e412, this.e43 * geometric_anti_product.e423, this.e41 * geometric_anti_product.e431, -(this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) + (float4(this.e1234) * geometric_anti_product.group0) - (geometric_anti_product.group0.yzxx * float4(this.group2.zxy, this.e23))
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> MultiVector {
Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e423 * geometric_anti_product.e1) - (this.e431 * geometric_anti_product.e2) - (this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4), this.e4 * geometric_anti_product.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * geometric_anti_product.group0.xyz) + (float3(geometric_anti_product.e4) * this.group3) + (this.group2.yzx * geometric_anti_product.group0.zxy) - (this.group2.zxy * geometric_anti_product.group0.yzx)), this.e1234 * geometric_anti_product.e4), 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * geometric_anti_product.group0.xyz) + (this.group4.yzx * geometric_anti_product.group0.zxy) - (float3(geometric_anti_product.e4) * this.group1.xyz) - (this.group4.zxy * geometric_anti_product.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 1.0) * float4(this.group2, (this.scalar * geometric_anti_product.e4) - (this.e41 * geometric_anti_product.e1) - (this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3))
        );
    }
}
extension Origin {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Origin> {
        geometric_anti_quotient_partial(this)
    }
}
extension Origin: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 / (other.e1234)
        );
    }
}
extension Origin: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> Flector {
DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.e1234, -2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_anti_product.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product.scalar * this.e4 * -1.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Motor {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
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
extension Origin: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Flector {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(geometric_anti_product.group1, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(geometric_anti_product.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Flector {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
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
extension Origin: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e4) * float2(geometric_anti_product.e321, geometric_anti_product.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(geometric_anti_product.group3, geometric_anti_product.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            float3(this.e4) * geometric_anti_product.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e4) * geometric_anti_product.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(geometric_anti_product.group2, geometric_anti_product.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 / (other.e4)
        );
    }
}
extension Origin: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Motor {
Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(geometric_anti_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e4 * geometric_anti_product.e321)
        );
    }
}
extension Origin: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Motor {
Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 * geometric_anti_product.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(geometric_anti_product.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Plane> {
        geometric_anti_quotient_partial(this)
    }
}
extension Plane: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/other.e1234) * this.group0
        );
    }
}
extension Plane: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> Flector {
DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.e1234, -2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.group0.xx, geometric_anti_product.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group0
        );
    }
}
extension Plane: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Motor {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
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
extension Plane: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Flector {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), -(geometric_anti_product.e42 * this.e431) - (geometric_anti_product.e43 * this.e412)) - (this.group0.yzxx * float4(geometric_anti_product.group1.zxy, geometric_anti_product.e41)), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e42 * this.e412 * -1.0, geometric_anti_product.e43 * this.e423 * -1.0, geometric_anti_product.e41 * this.e431 * -1.0, (geometric_anti_product.e31 * this.e431) + (geometric_anti_product.e12 * this.e412)) + (this.group0.yzxx * float4(geometric_anti_product.group0.zxy, geometric_anti_product.e23))
        );
    }
}
extension Plane: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Flector {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
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
extension Plane: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(geometric_anti_product.e1 * this.e423) - (geometric_anti_product.e2 * this.e431) - (geometric_anti_product.e3 * this.e412) - (geometric_anti_product.e4 * this.e321), (geometric_anti_product.e423 * this.e423) + (geometric_anti_product.e431 * this.e431) + (geometric_anti_product.e412 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), geometric_anti_product.e43 * this.e412 * -1.0) - (this.group0.xyzx * float4(geometric_anti_product.group0.xx, geometric_anti_product.scalar, geometric_anti_product.e41)) - (this.group0.yzxy * float4(geometric_anti_product.group3.zxy, geometric_anti_product.e42)), 
            /* e41, e42, e43 */
            (geometric_anti_product.group4.yzx * this.group0.zxy) - (float3(geometric_anti_product.e4) * this.group0.xyz) - (geometric_anti_product.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product.e321) * this.group0.xyz) + (geometric_anti_product.group1.zxy * this.group0.yzx) - (float3(this.e321) * geometric_anti_product.group4.xyz) - (geometric_anti_product.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e42 * this.e412 * -1.0, geometric_anti_product.e43 * this.e423 * -1.0, geometric_anti_product.e41 * this.e431 * -1.0, (geometric_anti_product.e31 * this.e431) + (geometric_anti_product.e12 * this.e412)) + (float4(geometric_anti_product.e1234) * this.group0) + (this.group0.yzxx * float4(geometric_anti_product.group2.zxy, geometric_anti_product.e23))
        );
    }
}
extension Plane: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> Motor {
Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_anti_product.e4 * this.e321 * -1.0)
        );
    }
}
extension Plane: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Motor {
Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e412 * this.e431 * -1.0, geometric_anti_product.e423 * this.e412 * -1.0, geometric_anti_product.e431 * this.e423 * -1.0, (geometric_anti_product.e431 * this.e431) + (geometric_anti_product.e412 * this.e412)) + (geometric_anti_product.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(geometric_anti_product.e321) * this.group0.xyz) - (float3(this.e321) * geometric_anti_product.group0.xyz)), 0.0)
        );
    }
}
extension Plane: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Motor {
Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(geometric_anti_product.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e431 * geometric_anti_product.e3, this.e412 * geometric_anti_product.e1, this.e423 * geometric_anti_product.e2, -(this.e431 * geometric_anti_product.e2) - (this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4)) - (this.group0.zxyx * geometric_anti_product.group0.yzxx)
        );
    }
}
extension Point {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Point> {
        geometric_anti_quotient_partial(this)
    }
}
extension Point: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/other.e1234) * this.group0
        );
    }
}
extension Point: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> Flector {
DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.e1234, -2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product.scalar * this.e4 * -1.0)
        );
    }
}
extension Point: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Motor {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
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
extension Point: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Flector {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.group0.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * geometric_anti_product.group1), 0.0) - float4((geometric_anti_product.group0.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 1.0) * float4(geometric_anti_product.group0, -(geometric_anti_product.e41 * this.e1) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3))
        );
    }
}
extension Point: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Flector {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
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
            float4(this.e4, this.e4, this.e4, 1.0) * float4(geometric_anti_product.group0.xyz, -(geometric_anti_product.e41 * this.e1) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3) - (geometric_anti_product.scalar * this.e4))
        );
    }
}
extension Point: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product.e423 * this.e1) + (geometric_anti_product.e431 * this.e2) + (geometric_anti_product.e412 * this.e3) + (geometric_anti_product.e321 * this.e4), geometric_anti_product.e4 * this.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group0.xyz) + (geometric_anti_product.group2.zxy * this.group0.yzx) - (float3(this.e4) * geometric_anti_product.group3) - (geometric_anti_product.group2.yzx * this.group0.zxy)), geometric_anti_product.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * geometric_anti_product.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * geometric_anti_product.group1.xyz) + (geometric_anti_product.group4.yzx * this.group0.zxy) - (float3(geometric_anti_product.e4) * this.group0.xyz) - (geometric_anti_product.group4.zxy * this.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 1.0) * float4(geometric_anti_product.group2, -(geometric_anti_product.scalar * this.e4) - (geometric_anti_product.e41 * this.e1) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3))
        );
    }
}
extension Point: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> Motor {
Origin geometric_anti_product = Origin.from_groups(
            /* e4 */
            1.0/other.e4 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_anti_product.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Motor {
Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(geometric_anti_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product.e412 * this.e2 * -1.0, geometric_anti_product.e423 * this.e3 * -1.0, geometric_anti_product.e431 * this.e1 * -1.0, (geometric_anti_product.e431 * this.e2) + (geometric_anti_product.e412 * this.e3) + (geometric_anti_product.e321 * this.e4)) + (geometric_anti_product.group0.yzxx * this.group0.zxyx)
        );
    }
}
extension Point: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Motor {
Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_anti_product.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * geometric_anti_product.group0.xyz) - (float3(geometric_anti_product.e4) * this.group0.xyz)), 0.0)
        );
    }
}
extension Scalar {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Scalar> {
        geometric_anti_quotient_partial(this)
    }
}
extension Scalar: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(other: AntiScalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar / (other.e1234)
        );
    }
}
extension Scalar: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar / (other.e1234)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(other: Flector) -> Flector {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar, this.scalar, this.scalar, 0.0) * float4(geometric_anti_product.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product.e4 * this.scalar)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(other: Line) -> Line {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * float3(other_2.e1234 * anti_reverse.e41, other_2.e1234 * anti_reverse.e42, other_2.e1234 * anti_reverse.e43)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(other: Motor) -> Motor {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * float4(other_2.e1234 * anti_reverse.e41, other_2.e1234 * anti_reverse.e42, other_2.e1234 * anti_reverse.e43, other_2.e1234 * anti_reverse.e1234)
        );
    }
}
extension Scalar: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product.e1234 * this.scalar, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar, this.scalar, this.scalar, 0.0) * float4(geometric_anti_product.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_anti_product.group2, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product.e4 * this.scalar)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(other: Origin) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.scalar / (other.e4) * -1.0
        );
    }
}
extension Scalar: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(other: Plane) -> Point {
AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234 */
            powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar, this.scalar, this.scalar, 0.0) * float4(float4(other_2.e1234 * other.e423, other_2.e1234 * other.e431, other_2.e1234 * other.e412, other_2.e1234 * other.e321).xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(other: Point) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.scalar / (other.e4) * -1.0
        );
    }
}
