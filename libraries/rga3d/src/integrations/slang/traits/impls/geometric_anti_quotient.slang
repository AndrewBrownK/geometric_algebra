extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<AntiScalar> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension AntiScalar: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = AntiScalar;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 / (other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<DualNum> {
    associatedtype Output = DualNum;
    func geometric_anti_quotient(other: DualNum) -> Output {
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, -2), 0.0, 0.0, 0.0)
        );
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e1234) * float2(other_2.e1234 * other.scalar, other_2.e1234 * other.e1234), 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Flector> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = Line;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group1.xyz, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * geometric_anti_product.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * geometric_anti_product.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e1234) * geometric_anti_product.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * geometric_anti_product.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * geometric_anti_product.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * geometric_anti_product.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * geometric_anti_product.group4
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Origin> {
    associatedtype Output = Origin;
    func geometric_anti_quotient(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e1234 / (other.e4) * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Plane> {
    associatedtype Output = Plane;
    func geometric_anti_quotient(other: Plane) -> Output {
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * float4(other_2.e1234 * other.e423, other_2.e1234 * other.e431, other_2.e1234 * other.e412, other_2.e1234 * other.e321)
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Point> {
    associatedtype Output = Point;
    func geometric_anti_quotient(other: Point) -> Output {
        Point anti_reverse = Point.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, -2), 0.0, 0.0, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * float4(other_2.e1234 * anti_reverse.e1, other_2.e1234 * anti_reverse.e2, other_2.e1234 * anti_reverse.e3, other_2.e1234 * anti_reverse.e4)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<DualNum> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension DualNum: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = DualNum;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0/other.e1234) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: GeometricAntiQuotient<DualNum> {
    associatedtype Output = DualNum;
    func geometric_anti_quotient(other: DualNum) -> Output {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.e1234, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((geometric_anti_product.scalar * this.e1234) + (geometric_anti_product.e1234 * this.scalar), geometric_anti_product.e1234 * this.e1234), 0.0, 0.0)
        );
    }
}
extension DualNum: GeometricAntiQuotient<Flector> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
            float4(geometric_anti_product.e423, geometric_anti_product.e431, geometric_anti_product.e412, 1.0) * float4(this.group0.xy.yy, this.e1234, (this.scalar * geometric_anti_product.e4) + (this.e1234 * geometric_anti_product.e321))
        );
    }
}
extension DualNum: GeometricAntiQuotient<Line> {
    associatedtype Output = Line;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group1.xyz, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * geometric_anti_product.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * geometric_anti_product.group0.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz), 0.0)
        );
    }
}
extension DualNum: GeometricAntiQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.scalar * geometric_anti_product.e1234) + (this.e1234 * geometric_anti_product.scalar), this.e1234 * geometric_anti_product.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * geometric_anti_product.group4.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz)), this.e1234 * geometric_anti_product.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * geometric_anti_product.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * geometric_anti_product.group2.xyz) + (float3(this.e1234) * geometric_anti_product.group3.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e423, geometric_anti_product.e431, geometric_anti_product.e412, 1.0) * float4(this.group0.xy.yy, this.e1234, (this.scalar * geometric_anti_product.e4) + (this.e1234 * geometric_anti_product.e321))
        );
    }
}
extension DualNum: GeometricAntiQuotient<Origin> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Origin) -> Output {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0/other.e4 * -1.0, 0.0, 0.0, 0.0)
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
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Plane) -> Output {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(geometric_anti_product.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * geometric_anti_product.group0
        );
    }
}
extension DualNum: GeometricAntiQuotient<Point> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Point) -> Output {
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
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<Flector> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension Flector: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(1.0/other.e1234, 0.0, 0.0, 0.0)
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
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: DualNum) -> Output {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.e1234, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group0.xyz) - (float3(geometric_anti_product.scalar) * this.group1.xyz)), geometric_anti_product.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 1.0) * float4(geometric_anti_product.group0.xy.yy, geometric_anti_product.e1234, (geometric_anti_product.e1234 * this.e321) - (geometric_anti_product.scalar * this.e4))
        );
    }
}
extension Flector: GeometricAntiQuotient<Flector> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group1.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * geometric_anti_product.e43) + (this.e412 * geometric_anti_product.e31) + (this.e321 * geometric_anti_product.e41), (this.e3 * geometric_anti_product.e41) + (this.e423 * geometric_anti_product.e12) + (this.e321 * geometric_anti_product.e42), (this.e1 * geometric_anti_product.e42) + (this.e431 * geometric_anti_product.e23) + (this.e321 * geometric_anti_product.e43), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(geometric_anti_product.group1.xyz.xxy, geometric_anti_product.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(geometric_anti_product.group1.xyz.zyz, geometric_anti_product.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group0.zxy), this.e423 * geometric_anti_product.e41), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(geometric_anti_product.group0.xyz.xxy, geometric_anti_product.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(geometric_anti_product.group0.xyz.zyz, geometric_anti_product.e31)) + float4(float3(0.0), (this.e412 * geometric_anti_product.e12) - (this.e2 * geometric_anti_product.e42) - (this.e3 * geometric_anti_product.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group1.zxy), this.e1 * geometric_anti_product.e41)
        );
    }
}
extension Flector: GeometricAntiQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e4 * geometric_anti_product.e321) - (this.e431 * geometric_anti_product.e2) - (this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4), 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * geometric_anti_product.group1.xw), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e2 * geometric_anti_product.e43) + (this.e412 * geometric_anti_product.e31) + (this.e321 * geometric_anti_product.e41) - (this.e431 * geometric_anti_product.e12), (this.e3 * geometric_anti_product.e41) + (this.e423 * geometric_anti_product.e12) + (this.e321 * geometric_anti_product.e42) - (this.e4 * geometric_anti_product.e31), (this.e1 * geometric_anti_product.e42) + (this.e431 * geometric_anti_product.e23) + (this.e321 * geometric_anti_product.e43) - (this.e4 * geometric_anti_product.e12), 0.0) + (float4(geometric_anti_product.e1234) * this.group0) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(geometric_anti_product.group3.xyz.xxy, geometric_anti_product.e43)) - (this.group1.xyzx * float4(geometric_anti_product.group0.xy.xx, geometric_anti_product.scalar, geometric_anti_product.e41)) - float4((geometric_anti_product.group2.xyz.yzx * this.group0.zxy), this.e431 * geometric_anti_product.e42), 
            /* e41, e42, e43, 0 */
            float4((this.group1.zxy * geometric_anti_product.group4.yzx) - (float3(this.e4) * geometric_anti_product.group4.xyz) - (float3(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e431) * this.group1.xyx) - (float3(geometric_anti_product.e412, geometric_anti_product.e423, geometric_anti_product.e4) * this.group1.yzz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e4) * geometric_anti_product.group1.xyz) + (float3(geometric_anti_product.e3, geometric_anti_product.e1, geometric_anti_product.e321) * this.group1.yzz) + (float3(geometric_anti_product.e321, geometric_anti_product.e321, geometric_anti_product.e2) * this.group1.xyx) + (this.group0.zxy * geometric_anti_product.group4.yzx) - (float3(this.e321) * geometric_anti_product.group4.xyz) - (float3(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e431) * this.group0.xyx) - (float3(geometric_anti_product.e412, geometric_anti_product.e423, geometric_anti_product.e4) * this.group0.yzz) - (this.group1.zxy * geometric_anti_product.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product.e1234) * this.group1) + (float4(this.e4, this.e412, this.e423, this.e423) * float4(geometric_anti_product.group2.xyz.xxy, geometric_anti_product.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(geometric_anti_product.group2.xyz.zyz, geometric_anti_product.e31)) + float4(float3(0.0), (this.e412 * geometric_anti_product.e12) - (this.e1 * geometric_anti_product.e41) - (this.e2 * geometric_anti_product.e42) - (this.e3 * geometric_anti_product.e43)) - float4((geometric_anti_product.group2.xyz.yzx * this.group1.zxy), this.e4 * geometric_anti_product.scalar)
        );
    }
}
extension Flector: GeometricAntiQuotient<Origin> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Origin) -> Output {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0/other.e4 * -1.0, 0.0, 0.0, 0.0)
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Plane) -> Output {
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Point) -> Output {
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
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<Horizon> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension Horizon: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = Horizon;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 / (other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<DualNum> {
    associatedtype Output = Horizon;
    func geometric_anti_quotient(other: DualNum) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 / (other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Flector> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = Point;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(float3(other_2.e1234 * anti_reverse.e41, other_2.e1234 * anti_reverse.e42, other_2.e1234 * anti_reverse.e43), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321 * geometric_anti_product.e4, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(geometric_anti_product.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * geometric_anti_product.group4.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * geometric_anti_product.e1234)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Origin> {
    associatedtype Output = Scalar;
    func geometric_anti_quotient(other: Origin) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 / (other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Plane> {
    associatedtype Output = Line;
    func geometric_anti_quotient(other: Plane) -> Output {
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * float4(other_2.e1234 * other.e423, other_2.e1234 * other.e431, other_2.e1234 * other.e412, other_2.e1234 * other.e321).xyz * float3(-1.0), 0.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Point> {
    associatedtype Output = Scalar;
    func geometric_anti_quotient(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 / (other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<Line> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension Line: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = Line;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(1.0/other.e1234, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: GeometricAntiQuotient<DualNum> {
    associatedtype Output = Line;
    func geometric_anti_quotient(other: DualNum) -> Output {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.e1234, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_anti_product.scalar) * this.group0.xyz) + (float3(geometric_anti_product.e1234) * this.group1.xyz), 0.0)
        );
    }
}
extension Line: GeometricAntiQuotient<Flector> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
extension Line: GeometricAntiQuotient<Line> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group1.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e43 * this.e42, geometric_anti_product.e41 * this.e43, geometric_anti_product.e42 * this.e41, -(geometric_anti_product.e42 * this.e42) - (geometric_anti_product.e43 * this.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group0.xyz.zxy), geometric_anti_product.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product.e43 * this.e31) + (geometric_anti_product.e12 * this.e42), (geometric_anti_product.e41 * this.e12) + (geometric_anti_product.e23 * this.e43), (geometric_anti_product.e42 * this.e23) + (geometric_anti_product.e31 * this.e41), -(geometric_anti_product.e43 * this.e12) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group1.xyz.zxy), geometric_anti_product.e41 * this.e23) - float4((geometric_anti_product.group1.xyz.yzx * this.group0.xyz.zxy), geometric_anti_product.e42 * this.e31)
        );
    }
}
extension Line: GeometricAntiQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
extension Line: GeometricAntiQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(this.e23 * geometric_anti_product.e41) - (this.e31 * geometric_anti_product.e42) - (this.e12 * geometric_anti_product.e43), 0.0) - (float2(this.e41) * float2(geometric_anti_product.e23, geometric_anti_product.e41)) - (float2(this.e42) * float2(geometric_anti_product.e31, geometric_anti_product.e42)) - (float2(this.e43) * float2(geometric_anti_product.e12, geometric_anti_product.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * geometric_anti_product.e321) + (this.e42 * geometric_anti_product.e3) + (this.e23 * geometric_anti_product.e4) + (this.e31 * geometric_anti_product.e412), (this.e42 * geometric_anti_product.e321) + (this.e43 * geometric_anti_product.e1) + (this.e31 * geometric_anti_product.e4) + (this.e12 * geometric_anti_product.e423), (this.e41 * geometric_anti_product.e2) + (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431) + (this.e12 * geometric_anti_product.e4), this.e43 * geometric_anti_product.e412 * -1.0) - (geometric_anti_product.group4.yzxy * float4(this.group1.xyz.zxy, this.e42)) - float4((this.group0.xyz.zxy * geometric_anti_product.group1.yzx), this.e41 * geometric_anti_product.e423), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_anti_product.e1234) * this.group0.xyz) + (this.group0.xyz.yzx * geometric_anti_product.group2.xyz.zxy) - (this.group0.xyz.zxy * geometric_anti_product.group2.xyz.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_anti_product.scalar) * this.group0.xyz) + (float3(geometric_anti_product.e1234) * this.group1.xyz) + (this.group0.xyz.yzx * geometric_anti_product.group3.xyz.zxy) + (this.group1.xyz.yzx * geometric_anti_product.group2.xyz.zxy) - (this.group0.xyz.zxy * geometric_anti_product.group3.xyz.yzx) - (this.group1.xyz.zxy * geometric_anti_product.group2.xyz.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e41 * geometric_anti_product.e4) + (this.e42 * geometric_anti_product.e412), (this.e42 * geometric_anti_product.e4) + (this.e43 * geometric_anti_product.e423), (this.e41 * geometric_anti_product.e431) + (this.e43 * geometric_anti_product.e4), -(this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3) - (this.e23 * geometric_anti_product.e423) - (this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) - float4((this.group0.xyz.zxy * geometric_anti_product.group4.yzx), this.e41 * geometric_anti_product.e1)
        );
    }
}
extension Line: GeometricAntiQuotient<Origin> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Origin) -> Output {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0/other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: GeometricAntiQuotient<Plane> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Plane) -> Output {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * geometric_anti_product.e321) + (this.e31 * geometric_anti_product.e412), (this.e42 * geometric_anti_product.e321) + (this.e12 * geometric_anti_product.e423), (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431), -(this.e42 * geometric_anti_product.e431) - (this.e43 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * geometric_anti_product.e412, this.e43 * geometric_anti_product.e423, this.e41 * geometric_anti_product.e431, -(this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension Line: GeometricAntiQuotient<Point> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Point) -> Output {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_anti_product.e4) * this.group1.xyz), 0.0) + float4((this.group0.xyz.yzx * geometric_anti_product.group0.zxy), 0.0) - float4((this.group0.xyz.zxy * geometric_anti_product.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 1.0) * float4(this.group0.xyz, -(this.e41 * geometric_anti_product.e1) - (this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3))
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<Motor> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension Motor: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(1.0/other.e1234, 0.0, 0.0, 0.0)
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: DualNum) -> Output {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.e1234, -2)) * other.group0.xy, 0.0, 0.0)
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
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group1.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_anti_product.e41 * this.e1234) + (geometric_anti_product.e43 * this.e42), (geometric_anti_product.e41 * this.e43) + (geometric_anti_product.e42 * this.e1234), (geometric_anti_product.e42 * this.e41) + (geometric_anti_product.e43 * this.e1234), -(geometric_anti_product.e42 * this.e42) - (geometric_anti_product.e43 * this.e43)) - (this.group0.zxyx * float4(geometric_anti_product.group0.xyz.yzx, geometric_anti_product.e41)), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product.e41 * this.scalar) + (geometric_anti_product.e43 * this.e31) + (geometric_anti_product.e23 * this.e1234) + (geometric_anti_product.e12 * this.e42), (geometric_anti_product.e41 * this.e12) + (geometric_anti_product.e42 * this.scalar) + (geometric_anti_product.e23 * this.e43) + (geometric_anti_product.e31 * this.e1234), (geometric_anti_product.e42 * this.e23) + (geometric_anti_product.e43 * this.scalar) + (geometric_anti_product.e31 * this.e41) + (geometric_anti_product.e12 * this.e1234), -(geometric_anti_product.e43 * this.e12) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43)) - (this.group1.zxyx * float4(geometric_anti_product.group0.xyz.yzx, geometric_anti_product.e41)) - float4((geometric_anti_product.group1.xyz.yzx * this.group0.zxy), geometric_anti_product.e42 * this.e31)
        );
    }
}
extension Motor: GeometricAntiQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.scalar * geometric_anti_product.e1234) - (this.e41 * geometric_anti_product.e23) - (this.e42 * geometric_anti_product.e31) - (this.e43 * geometric_anti_product.e12), 0.0) + (float2(this.e1234) * geometric_anti_product.group0.xy) - (float2(geometric_anti_product.e41) * float2(this.e23, this.e41)) - (float2(geometric_anti_product.e42) * float2(this.e31, this.e42)) - (float2(geometric_anti_product.e43) * float2(this.e12, this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e42 * geometric_anti_product.e3) + (this.e1234 * geometric_anti_product.e1) + (this.e23 * geometric_anti_product.e4) + (this.e31 * geometric_anti_product.e412) + (this.scalar * geometric_anti_product.e423), (this.e43 * geometric_anti_product.e1) + (this.e1234 * geometric_anti_product.e2) + (this.e31 * geometric_anti_product.e4) + (this.e12 * geometric_anti_product.e423) + (this.scalar * geometric_anti_product.e431), (this.e43 * geometric_anti_product.e321) + (this.e1234 * geometric_anti_product.e3) + (this.e23 * geometric_anti_product.e431) + (this.e12 * geometric_anti_product.e4) + (this.scalar * geometric_anti_product.e412), this.e43 * geometric_anti_product.e412 * -1.0) + (this.group0.xyxw * float4(geometric_anti_product.group4.ww, geometric_anti_product.e2, geometric_anti_product.e4)) - (this.group0.zxyx * float4(geometric_anti_product.group1.yzx, geometric_anti_product.e423)) - (geometric_anti_product.group4.yzxy * float4(this.group1.zxy, this.e42)), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_anti_product.e1234) * this.group0.xyz) + (geometric_anti_product.group2.xyz.xxy * this.group0.wzx) + (geometric_anti_product.group2.xyz.zyz * this.group0.yww) - (geometric_anti_product.group2.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_anti_product.scalar) * this.group0.xyz) + (float3(geometric_anti_product.e1234) * this.group1.xyz) + (geometric_anti_product.group2.xyz.xxy * this.group1.wzx) + (geometric_anti_product.group2.xyz.zyz * this.group1.yww) + (geometric_anti_product.group3.xyz.xxy * this.group0.wzx) + (geometric_anti_product.group3.xyz.zyz * this.group0.yww) - (geometric_anti_product.group2.xyz.yzx * this.group1.zxy) - (geometric_anti_product.group3.xyz.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * geometric_anti_product.e423, this.e1234 * geometric_anti_product.e431, this.e1234 * geometric_anti_product.e412, -(this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3) - (this.e23 * geometric_anti_product.e423) - (this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) + (this.group0.xyxw * float4(geometric_anti_product.group1.ww, geometric_anti_product.e431, geometric_anti_product.e321)) + (float4(geometric_anti_product.group4.zx, geometric_anti_product.e4, geometric_anti_product.e4) * float4(this.group0.yzz, this.scalar)) - (this.group0.zxyx * float4(geometric_anti_product.group4.yzx, geometric_anti_product.e1))
        );
    }
}
extension Motor: GeometricAntiQuotient<Origin> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Origin) -> Output {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0/other.e4 * -1.0, 0.0, 0.0, 0.0)
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
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Plane) -> Output {
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
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Point) -> Output {
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
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<MultiVector> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension MultiVector: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(1.0/other.e1234, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_anti_product.e1234) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group4
        );
    }
}
extension MultiVector: GeometricAntiQuotient<DualNum> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: DualNum) -> Output {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.e1234, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((geometric_anti_product.scalar * this.e1234) + (geometric_anti_product.e1234 * this.scalar), geometric_anti_product.e1234 * this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group1.xyz) - (float3(geometric_anti_product.scalar) * this.group4.xyz)), geometric_anti_product.e1234 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_anti_product.scalar) * this.group2.xyz) + (float3(geometric_anti_product.e1234) * this.group3.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 1.0) * float4(geometric_anti_product.group0.xy.yy, geometric_anti_product.e1234, (geometric_anti_product.e1234 * this.e321) - (geometric_anti_product.scalar * this.e4))
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Flector> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_anti_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((geometric_anti_product.e321 * this.e4) - (geometric_anti_product.e2 * this.e431) - (geometric_anti_product.e3 * this.e412) - (geometric_anti_product.e4 * this.e321), 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * geometric_anti_product.group0.xw), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e1 * this.e1234) + (geometric_anti_product.e3 * this.e42) + (geometric_anti_product.e4 * this.e23) + (geometric_anti_product.e412 * this.e31) + (geometric_anti_product.e321 * this.e41), (geometric_anti_product.e1 * this.e43) + (geometric_anti_product.e2 * this.e1234) + (geometric_anti_product.e4 * this.e31) + (geometric_anti_product.e423 * this.e12) + (geometric_anti_product.e321 * this.e42), (geometric_anti_product.e2 * this.e41) + (geometric_anti_product.e3 * this.e1234) + (geometric_anti_product.e4 * this.e12) + (geometric_anti_product.e431 * this.e23) + (geometric_anti_product.e321 * this.e43), geometric_anti_product.e412 * this.e43 * -1.0) + (float4(this.group0.xy.xx, this.scalar, this.e1234) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e4)) - (geometric_anti_product.group1.yzxy * float4(this.group3.xyz.zxy, this.e42)) - float4((this.group2.xyz.zxy * geometric_anti_product.group0.yzx), geometric_anti_product.e423 * this.e41), 
            /* e41, e42, e43, 0 */
            float4((geometric_anti_product.group1.yzx * this.group4.zxy) - (float3(geometric_anti_product.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group1.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group1.zyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_anti_product.e321) * this.group4.xyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group0.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group0.zyz) + (geometric_anti_product.group1.yzx * this.group1.zxy) - (float3(geometric_anti_product.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * geometric_anti_product.group1.zyz) - (float3(this.e321, this.e3, this.e1) * geometric_anti_product.group1.xxy) - (geometric_anti_product.group0.yzx * this.group4.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e412 * this.e42, geometric_anti_product.e423 * this.e43, geometric_anti_product.e4 * this.e43, -(geometric_anti_product.e2 * this.e42) - (geometric_anti_product.e3 * this.e43) - (geometric_anti_product.e423 * this.e23) - (geometric_anti_product.e431 * this.e31) - (geometric_anti_product.e412 * this.e12)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(geometric_anti_product.group1.xyz, geometric_anti_product.e4)) + (float4(geometric_anti_product.group0.ww, geometric_anti_product.e431, geometric_anti_product.e321) * float4(this.group2.xyz.xyx, this.e1234)) - float4((this.group2.xyz.zxy * geometric_anti_product.group1.yzx), geometric_anti_product.e1 * this.e41)
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Line> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43), 0.0) - (float2(geometric_anti_product.e41) * float2(this.e23, this.e41)) - (float2(geometric_anti_product.e42) * float2(this.e31, this.e42)) - (float2(geometric_anti_product.e43) * float2(this.e12, this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e43 * this.e2) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e41 * this.e3) + (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e42 * this.e1) + (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(geometric_anti_product.group1.xyz.xxy, geometric_anti_product.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(geometric_anti_product.group1.xyz.zyz, geometric_anti_product.e43)) - float4((geometric_anti_product.group0.xyz.yzx * this.group1.zxy), geometric_anti_product.e41 * this.e423), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * geometric_anti_product.group0.xyz) + (geometric_anti_product.group0.xyz.zxy * this.group2.xyz.yzx) - (geometric_anti_product.group0.xyz.yzx * this.group2.xyz.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * geometric_anti_product.group0.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz) + (geometric_anti_product.group0.xyz.zxy * this.group3.xyz.yzx) + (geometric_anti_product.group1.xyz.zxy * this.group2.xyz.yzx) - (geometric_anti_product.group0.xyz.yzx * this.group3.xyz.zxy) - (geometric_anti_product.group1.xyz.yzx * this.group2.xyz.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(geometric_anti_product.group0.xyz.xxy, geometric_anti_product.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(geometric_anti_product.group0.xyz.zyz, geometric_anti_product.e31)) + float4(float3(0.0), (geometric_anti_product.e12 * this.e412) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3)) - float4((geometric_anti_product.group0.xyz.yzx * this.group4.zxy), geometric_anti_product.e41 * this.e1)
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Motor> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_anti_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.e1234) * anti_reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((geometric_anti_product.scalar * this.e1234) - (geometric_anti_product.e41 * this.e23) - (geometric_anti_product.e42 * this.e31) - (geometric_anti_product.e43 * this.e12), 0.0) + (float2(geometric_anti_product.e1234) * this.group0.xy) - (float2(this.e41) * float2(geometric_anti_product.e23, geometric_anti_product.e41)) - (float2(this.e42) * float2(geometric_anti_product.e31, geometric_anti_product.e42)) - (float2(this.e43) * float2(geometric_anti_product.e12, geometric_anti_product.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e43 * this.e2) + (geometric_anti_product.e1234 * this.e1) + (geometric_anti_product.e31 * this.e412) - (geometric_anti_product.scalar * this.e423), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e1234 * this.e2) + (geometric_anti_product.e12 * this.e423) - (geometric_anti_product.scalar * this.e431), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e1234 * this.e3) + (geometric_anti_product.e23 * this.e431) - (geometric_anti_product.scalar * this.e412), 0.0) + (float4(this.e321, this.e3, this.e1, this.e4) * geometric_anti_product.group0.xxyw) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(geometric_anti_product.group1.xxy, geometric_anti_product.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(geometric_anti_product.group1.zyz, geometric_anti_product.e43)) - (geometric_anti_product.group0.yzxx * float4(this.group1.zxy, this.e423)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * geometric_anti_product.group0.xyz) + (this.group2.xyz.xyx * geometric_anti_product.group0.wwy) + (this.group2.xyz.yzz * geometric_anti_product.group0.zxw) - (this.group2.xyz.zxy * geometric_anti_product.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * geometric_anti_product.group0.xyz) + (float3(this.e1234) * geometric_anti_product.group1.xyz) + (this.group2.xyz.xyx * geometric_anti_product.group1.wwy) + (this.group2.xyz.yzz * geometric_anti_product.group1.zxw) + (this.group3.xyz.xyx * geometric_anti_product.group0.wwy) + (this.group3.xyz.yzz * geometric_anti_product.group0.zxw) - (this.group2.xyz.zxy * geometric_anti_product.group1.yzx) - (this.group3.xyz.zxy * geometric_anti_product.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e321) * geometric_anti_product.group0.xxyw) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(geometric_anti_product.group0.zyz, geometric_anti_product.e23)) + (this.group4.xyzy * float4(geometric_anti_product.group0.www, geometric_anti_product.e31)) + float4(float3(0.0), (geometric_anti_product.e12 * this.e412) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3) - (geometric_anti_product.scalar * this.e4)) - (geometric_anti_product.group0.yzxx * float4(this.group4.zxy, this.e1))
        );
    }
}
extension MultiVector: GeometricAntiQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((geometric_anti_product.e1234 * this.scalar) + (geometric_anti_product.e321 * this.e4) - (geometric_anti_product.e2 * this.e431) - (geometric_anti_product.e3 * this.e412) - (geometric_anti_product.e4 * this.e321) - (geometric_anti_product.e23 * this.e41) - (geometric_anti_product.e31 * this.e42) - (geometric_anti_product.e12 * this.e43), 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * geometric_anti_product.group0.xy) - (float2(geometric_anti_product.e41) * float2(this.e23, this.e41)) - (float2(geometric_anti_product.e42) * float2(this.e31, this.e42)) - (float2(geometric_anti_product.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * geometric_anti_product.group1.xw), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e1 * this.e1234) + (geometric_anti_product.e3 * this.e42) + (geometric_anti_product.e4 * this.e23) + (geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e43 * this.e2) + (geometric_anti_product.e31 * this.e412) + (geometric_anti_product.e412 * this.e31) + (geometric_anti_product.e321 * this.e41), (geometric_anti_product.e1 * this.e43) + (geometric_anti_product.e2 * this.e1234) + (geometric_anti_product.e4 * this.e31) + (geometric_anti_product.e41 * this.e3) + (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423) + (geometric_anti_product.e423 * this.e12) + (geometric_anti_product.e321 * this.e42), (geometric_anti_product.e2 * this.e41) + (geometric_anti_product.e3 * this.e1234) + (geometric_anti_product.e4 * this.e12) + (geometric_anti_product.e42 * this.e1) + (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431) + (geometric_anti_product.e431 * this.e23) + (geometric_anti_product.e321 * this.e43), 0.0) + (float4(geometric_anti_product.e1234) * this.group1) + (float4(this.group0.xy.xx, this.scalar, this.e1234) * float4(geometric_anti_product.group4.xyz, geometric_anti_product.e4)) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(geometric_anti_product.group3.xyz.xxy, geometric_anti_product.e43)) - (float4(this.e431, this.e4, this.e4, geometric_anti_product.e423) * float4(geometric_anti_product.group3.xyz.zyz, this.e41)) - (geometric_anti_product.group4.yzxz * float4(this.group3.xyz.zxy, this.e43)) - (this.group4.xyzx * float4(geometric_anti_product.group0.xy.xx, geometric_anti_product.scalar, geometric_anti_product.e41)) - float4((geometric_anti_product.group2.xyz.yzx * this.group1.zxy), geometric_anti_product.e42 * this.e431) - float4((this.group2.xyz.zxy * geometric_anti_product.group1.yzx), geometric_anti_product.e431 * this.e42), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_anti_product.e1234) * this.group2.xyz) + (float3(this.e1234) * geometric_anti_product.group2.xyz) + (geometric_anti_product.group2.xyz.zxy * this.group2.xyz.yzx) + (geometric_anti_product.group4.yzx * this.group4.zxy) - (float3(geometric_anti_product.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group4.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group4.zyz) - (geometric_anti_product.group2.xyz.yzx * this.group2.xyz.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_anti_product.scalar) * this.group2.xyz) + (float3(geometric_anti_product.e1234) * this.group3.xyz) + (float3(geometric_anti_product.e321) * this.group4.xyz) + (float3(this.scalar) * geometric_anti_product.group2.xyz) + (float3(this.e1234) * geometric_anti_product.group3.xyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product.group1.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product.group1.zyz) + (geometric_anti_product.group2.xyz.zxy * this.group3.xyz.yzx) + (geometric_anti_product.group3.xyz.zxy * this.group2.xyz.yzx) + (geometric_anti_product.group4.yzx * this.group1.zxy) - (float3(geometric_anti_product.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * geometric_anti_product.group4.zyz) - (float3(this.e321, this.e3, this.e1) * geometric_anti_product.group4.xxy) - (geometric_anti_product.group2.xyz.yzx * this.group3.xyz.zxy) - (geometric_anti_product.group3.xyz.yzx * this.group2.xyz.zxy) - (geometric_anti_product.group1.yzx * this.group4.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, geometric_anti_product.e321) * float4(geometric_anti_product.group2.xyz.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(geometric_anti_product.group2.xyz.zyz, geometric_anti_product.e23)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(geometric_anti_product.group4.xyz, geometric_anti_product.e4)) + (float4(geometric_anti_product.group1.ww, geometric_anti_product.e431, this.e431) * float4(this.group2.xyz.xyx, geometric_anti_product.e31)) + (float4(geometric_anti_product.group4.zx, geometric_anti_product.e4, this.e412) * float4(this.group2.xyz.yzz, geometric_anti_product.e12)) + float4(float3(0.0), -(geometric_anti_product.e1 * this.e41) - (geometric_anti_product.e2 * this.e42) - (geometric_anti_product.e3 * this.e43) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3) - (geometric_anti_product.e423 * this.e23) - (geometric_anti_product.e431 * this.e31) - (geometric_anti_product.e412 * this.e12)) - float4((geometric_anti_product.group2.xyz.yzx * this.group4.zxy), geometric_anti_product.scalar * this.e4) - float4((this.group2.xyz.zxy * geometric_anti_product.group4.yzx), geometric_anti_product.e41 * this.e1)
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Origin> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: Origin) -> Output {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0/other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_anti_product.e4) * float2(this.e321, this.e4) * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e4) * float4(this.group3.xyz, this.e1234), 
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e4) * this.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_anti_product.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4) * float4(this.group2.xyz, this.scalar)
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Plane> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: Plane) -> Output {
        Plane geometric_anti_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2)) * other.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e4 * geometric_anti_product.e321, 0.0) + (float2(geometric_anti_product.e423) * float2(this.e1, this.e423)) + (float2(geometric_anti_product.e431) * float2(this.e2, this.e431)) + (float2(geometric_anti_product.e412) * float2(this.e3, this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.scalar * geometric_anti_product.e423) + (this.e41 * geometric_anti_product.e321) + (this.e31 * geometric_anti_product.e412), (this.scalar * geometric_anti_product.e431) + (this.e42 * geometric_anti_product.e321) + (this.e12 * geometric_anti_product.e423), (this.scalar * geometric_anti_product.e412) + (this.e43 * geometric_anti_product.e321) + (this.e23 * geometric_anti_product.e431), -(this.e42 * geometric_anti_product.e431) - (this.e43 * geometric_anti_product.e412)) - (geometric_anti_product.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((this.group4.zxy * geometric_anti_product.group0.yzx) - (float3(this.e4) * geometric_anti_product.group0.xyz) - (this.group4.yzx * geometric_anti_product.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_anti_product.e321) * this.group4.xyz) + (this.group1.zxy * geometric_anti_product.group0.yzx) - (float3(this.e321) * geometric_anti_product.group0.xyz) - (this.group1.yzx * geometric_anti_product.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * geometric_anti_product.e412, this.e43 * geometric_anti_product.e423, this.e41 * geometric_anti_product.e431, -(this.e31 * geometric_anti_product.e431) - (this.e12 * geometric_anti_product.e412)) + (float4(this.e1234) * geometric_anti_product.group0) - (geometric_anti_product.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Point> {
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: Point) -> Output {
        Point geometric_anti_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e4, -2)) * float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, other.e4 * -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(this.e423 * geometric_anti_product.e1) - (this.e431 * geometric_anti_product.e2) - (this.e412 * geometric_anti_product.e3) - (this.e321 * geometric_anti_product.e4), this.e4 * geometric_anti_product.e4) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * geometric_anti_product.group0.xyz) + (float3(geometric_anti_product.e4) * this.group3.xyz) + (this.group2.xyz.yzx * geometric_anti_product.group0.zxy) - (this.group2.xyz.zxy * geometric_anti_product.group0.yzx)), this.e1234 * geometric_anti_product.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e4) * this.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e4) * geometric_anti_product.group0.xyz) + (this.group4.yzx * geometric_anti_product.group0.zxy) - (float3(geometric_anti_product.e4) * this.group1.xyz) - (this.group4.zxy * geometric_anti_product.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e4, geometric_anti_product.e4, geometric_anti_product.e4, 1.0) * float4(this.group2.xyz, (this.scalar * geometric_anti_product.e4) - (this.e41 * geometric_anti_product.e1) - (this.e42 * geometric_anti_product.e2) - (this.e43 * geometric_anti_product.e3))
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<Origin> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension Origin: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = Origin;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 / (other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<DualNum> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: DualNum) -> Output {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.e1234, -2)) * other.group0.xy, 0.0, 0.0)
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group1.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(geometric_anti_product.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(geometric_anti_product.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e4) * float2(geometric_anti_product.e321, geometric_anti_product.e4) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(geometric_anti_product.group3.xyz, geometric_anti_product.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * geometric_anti_product.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e4) * geometric_anti_product.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(geometric_anti_product.group2.xyz, geometric_anti_product.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<Origin> {
    associatedtype Output = AntiScalar;
    func geometric_anti_quotient(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e4 / (other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<Plane> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Plane) -> Output {
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Point) -> Output {
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
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<Plane> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension Plane: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = Plane;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/other.e1234) * this.group0
        );
    }
}
extension Plane: GeometricAntiQuotient<DualNum> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: DualNum) -> Output {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.e1234, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.group0.xy.xx, geometric_anti_product.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group0
        );
    }
}
extension Plane: GeometricAntiQuotient<Flector> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group1.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), -(geometric_anti_product.e42 * this.e431) - (geometric_anti_product.e43 * this.e412)) - (this.group0.yzxx * float4(geometric_anti_product.group1.xyz.zxy, geometric_anti_product.e41)), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e42 * this.e412 * -1.0, geometric_anti_product.e43 * this.e423 * -1.0, geometric_anti_product.e41 * this.e431 * -1.0, (geometric_anti_product.e31 * this.e431) + (geometric_anti_product.e12 * this.e412)) + (this.group0.yzxx * float4(geometric_anti_product.group0.xyz.zxy, geometric_anti_product.e23))
        );
    }
}
extension Plane: GeometricAntiQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(geometric_anti_product.e1 * this.e423) - (geometric_anti_product.e2 * this.e431) - (geometric_anti_product.e3 * this.e412) - (geometric_anti_product.e4 * this.e321), (geometric_anti_product.e423 * this.e423) + (geometric_anti_product.e431 * this.e431) + (geometric_anti_product.e412 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.e41 * this.e321) + (geometric_anti_product.e31 * this.e412), (geometric_anti_product.e42 * this.e321) + (geometric_anti_product.e12 * this.e423), (geometric_anti_product.e43 * this.e321) + (geometric_anti_product.e23 * this.e431), geometric_anti_product.e43 * this.e412 * -1.0) - (this.group0.xyzx * float4(geometric_anti_product.group0.xy.xx, geometric_anti_product.scalar, geometric_anti_product.e41)) - (this.group0.yzxy * float4(geometric_anti_product.group3.xyz.zxy, geometric_anti_product.e42)), 
            /* e41, e42, e43, 0 */
            float4((geometric_anti_product.group4.yzx * this.group0.zxy) - (float3(geometric_anti_product.e4) * this.group0.xyz) - (geometric_anti_product.group4.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_anti_product.e321) * this.group0.xyz) + (geometric_anti_product.group1.zxy * this.group0.yzx) - (float3(this.e321) * geometric_anti_product.group4.xyz) - (geometric_anti_product.group1.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e42 * this.e412 * -1.0, geometric_anti_product.e43 * this.e423 * -1.0, geometric_anti_product.e41 * this.e431 * -1.0, (geometric_anti_product.e31 * this.e431) + (geometric_anti_product.e12 * this.e412)) + (float4(geometric_anti_product.e1234) * this.group0) + (this.group0.yzxx * float4(geometric_anti_product.group2.xyz.zxy, geometric_anti_product.e23))
        );
    }
}
extension Plane: GeometricAntiQuotient<Origin> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Origin) -> Output {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0/other.e4 * -1.0, 0.0, 0.0, 0.0)
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Plane) -> Output {
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Point) -> Output {
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
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<Point> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension Point: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = Point;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/other.e1234) * this.group0
        );
    }
}
extension Point: GeometricAntiQuotient<DualNum> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: DualNum) -> Output {
        DualNum geometric_anti_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.e1234, -2)) * other.group0.xy, 0.0, 0.0)
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_anti_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group1.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.group0.xyz.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * geometric_anti_product.group1.xyz), 0.0) - float4((geometric_anti_product.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 1.0) * float4(geometric_anti_product.group0.xyz, -(geometric_anti_product.e41 * this.e1) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3))
        );
    }
}
extension Point: GeometricAntiQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((geometric_anti_product.e423 * this.e1) + (geometric_anti_product.e431 * this.e2) + (geometric_anti_product.e412 * this.e3) + (geometric_anti_product.e321 * this.e4), geometric_anti_product.e4 * this.e4) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product.e1234) * this.group0.xyz) + (geometric_anti_product.group2.xyz.zxy * this.group0.yzx) - (float3(this.e4) * geometric_anti_product.group3.xyz) - (geometric_anti_product.group2.xyz.yzx * this.group0.zxy)), geometric_anti_product.e1234 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * geometric_anti_product.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e4) * geometric_anti_product.group1.xyz) + (geometric_anti_product.group4.yzx * this.group0.zxy) - (float3(geometric_anti_product.e4) * this.group0.xyz) - (geometric_anti_product.group4.zxy * this.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 1.0) * float4(geometric_anti_product.group2.xyz, -(geometric_anti_product.scalar * this.e4) - (geometric_anti_product.e41 * this.e1) - (geometric_anti_product.e42 * this.e2) - (geometric_anti_product.e43 * this.e3))
        );
    }
}
extension Point: GeometricAntiQuotient<Origin> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Origin) -> Output {
        Origin geometric_anti_product = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0/other.e4 * -1.0, 0.0, 0.0, 0.0)
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Plane) -> Output {
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
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Point) -> Output {
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
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiQuotientInfix) -> GeometricAntiQuotientInfixPartial<Scalar> {
        return GeometricAntiQuotientInfixPartial { a: this };
    }
}
extension Scalar: GeometricAntiQuotient<AntiScalar> {
    associatedtype Output = Scalar;
    func geometric_anti_quotient(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar / (other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: GeometricAntiQuotient<DualNum> {
    associatedtype Output = Scalar;
    func geometric_anti_quotient(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar / (other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Flector> {
    associatedtype Output = Flector;
    func geometric_anti_quotient(other: Flector) -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e4, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = Line;
    func geometric_anti_quotient(other: Line) -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2), 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * float3(other_2.e1234 * anti_reverse.e41, other_2.e1234 * anti_reverse.e42, other_2.e1234 * anti_reverse.e43), 0.0)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_quotient(other: Motor) -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e1234, 2), 0.0, 0.0, 0.0)
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
    associatedtype Output = MultiVector;
    func geometric_anti_quotient(other: MultiVector) -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e1234, 2) + powi(other.e4, 2) + powi(other.e41, 2) + powi(other.e42, 2) + powi(other.e43, 2) + powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.e1234) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.e1234) * anti_reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.e1234) * anti_reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.e1234) * anti_reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_anti_product.e1234 * this.scalar, 1.0) * float2(1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar, this.scalar, this.scalar, 0.0) * float4(geometric_anti_product.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * geometric_anti_product.group2.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product.e4 * this.scalar)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Origin> {
    associatedtype Output = Horizon;
    func geometric_anti_quotient(other: Origin) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.scalar / (other.e4) * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Plane> {
    associatedtype Output = Point;
    func geometric_anti_quotient(other: Plane) -> Output {
        AntiScalar other_2 = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powi(other.e423, 2) + powi(other.e431, 2) + powi(other.e412, 2), 0.0, 0.0, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar, this.scalar, this.scalar, 0.0) * float4(float4(other_2.e1234 * other.e423, other_2.e1234 * other.e431, other_2.e1234 * other.e412, other_2.e1234 * other.e321).xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Point> {
    associatedtype Output = Horizon;
    func geometric_anti_quotient(other: Point) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.scalar / (other.e4) * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
