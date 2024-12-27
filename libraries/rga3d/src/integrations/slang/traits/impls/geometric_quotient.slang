extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<AntiScalar> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension AntiScalar: GeometricQuotient<DualNum> {
    associatedtype Output = AntiScalar;
    func geometric_quotient(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 / (other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Flector> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * geometric_product.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Horizon> {
    associatedtype Output = Origin;
    func geometric_quotient(other: Horizon) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e1234 / (other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Line> {
    associatedtype Output = Line;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * float3(reverse.e23 * other_2.scalar, reverse.e31 * other_2.scalar, reverse.e12 * other_2.scalar), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * float4(reverse.e23 * other_2.scalar, reverse.e31 * other_2.scalar, reverse.e12 * other_2.scalar, reverse.scalar * other_2.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, this.e1234 * geometric_product.scalar) * float2(0.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * geometric_product.group3.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * geometric_product.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Plane> {
    associatedtype Output = Origin;
    func geometric_quotient(other: Plane) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e1234 / (other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Point> {
    associatedtype Output = Plane;
    func geometric_quotient(other: Point) -> Output {
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2), 0.0, 0.0, 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * float4(other.e1 * other_2.scalar, other.e2 * other_2.scalar, other.e3 * other_2.scalar, other.e4 * other_2.scalar).xyz * float3(-1.0)), 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Scalar> {
    associatedtype Output = AntiScalar;
    func geometric_quotient(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 / (other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<DualNum> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension DualNum: GeometricQuotient<DualNum> {
    associatedtype Output = DualNum;
    func geometric_quotient(other: DualNum) -> Output {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.scalar, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_product.scalar * this.scalar, (geometric_product.scalar * this.e1234) + (geometric_product.e1234 * this.scalar)), 0.0, 0.0)
        );
    }
}
extension DualNum: GeometricQuotient<Flector> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * geometric_product.e4) - (this.e1234 * geometric_product.e321)) * float4(geometric_product.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group1.xyz) - (float3(this.e1234) * geometric_product.group0.xyz)), this.scalar * geometric_product.e321)
        );
    }
}
extension DualNum: GeometricQuotient<Horizon> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Horizon) -> Output {
        Horizon geometric_product = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(1.0/other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * geometric_product.e321)
        );
    }
}
extension DualNum: GeometricQuotient<Line> {
    associatedtype Output = Line;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group1.xyz, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * geometric_product.group0.xyz) + (float3(this.e1234) * geometric_product.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * geometric_product.group1.xyz, 0.0)
        );
    }
}
extension DualNum: GeometricQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(this.scalar) * geometric_product.group0) + (float4(this.e1234) * geometric_product.group1), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * geometric_product.group1
        );
    }
}
extension DualNum: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar * geometric_product.scalar, (this.scalar * geometric_product.e1234) + (this.e1234 * geometric_product.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * geometric_product.e4) - (this.e1234 * geometric_product.e321)) * float4(geometric_product.group1.xyz, 1.0), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * geometric_product.group2.xyz) + (float3(this.e1234) * geometric_product.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * geometric_product.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group4.xyz) - (float3(this.e1234) * geometric_product.group1.xyz)), this.scalar * geometric_product.e321)
        );
    }
}
extension DualNum: GeometricQuotient<Plane> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Plane) -> Output {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * geometric_product.group0
        );
    }
}
extension DualNum: GeometricQuotient<Point> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Point) -> Output {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * geometric_product.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(geometric_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension DualNum: GeometricQuotient<Scalar> {
    associatedtype Output = DualNum;
    func geometric_quotient(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0/other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Flector> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension Flector: GeometricQuotient<DualNum> {
    associatedtype Output = Flector;
    func geometric_quotient(other: DualNum) -> Output {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.scalar, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.group0.xy.xx, geometric_product.scalar, (geometric_product.scalar * this.e4) + (geometric_product.e1234 * this.e321)) * float4(this.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group1.xyz) + (float3(geometric_product.e1234) * this.group0.xyz)), geometric_product.scalar * this.e321)
        );
    }
}
extension Flector: GeometricQuotient<Flector> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(geometric_product.e321) * float4(this.group1.xyz, this.e4)) + (geometric_product.group1.zxyz * this.group0.yzxz) + (float4(this.group0.ww, this.e431, this.e1) * float4(geometric_product.group0.xyx, geometric_product.e423)) + (float4(this.group1.zx, this.e4, this.e2) * float4(geometric_product.group0.yzz, geometric_product.e431)) - (geometric_product.group0.zxyx * this.group1.yzxx) - (geometric_product.group0.wwwy * float4(this.group0.xyz, this.e431)) - (float4(this.group0.zx, this.e321, this.e321) * float4(geometric_product.group1.yzz, geometric_product.e4)) - (float4(this.group1.ww, this.e2, this.e412) * float4(geometric_product.group1.xyx, geometric_product.e3)), 
            /* e23, e31, e12, scalar */
            float4(-(geometric_product.e2 * this.e3) - (geometric_product.e321 * this.e1), -(geometric_product.e3 * this.e1) - (geometric_product.e321 * this.e2), -(geometric_product.e3 * this.e321) - (geometric_product.e321 * this.e3), (geometric_product.e2 * this.e2) + (geometric_product.e3 * this.e3)) + (geometric_product.group0.zxyx * this.group0.yzxx) - (float4(this.group1.ww, this.e2, this.e321) * float4(geometric_product.group0.xyx, geometric_product.e321))
        );
    }
}
extension Flector: GeometricQuotient<Horizon> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Horizon) -> Output {
        Horizon geometric_product = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(1.0/other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product.e321) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
extension Flector: GeometricQuotient<Line> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group1.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e3 * geometric_product.e31) + (this.e321 * geometric_product.e23), (this.e1 * geometric_product.e12) + (this.e321 * geometric_product.e31), (this.e2 * geometric_product.e23) + (this.e321 * geometric_product.e12), -(this.e2 * geometric_product.e42) - (this.e3 * geometric_product.e43) - (this.e423 * geometric_product.e23) - (this.e431 * geometric_product.e31) - (this.e412 * geometric_product.e12)) - (this.group0.yzxx * float4(geometric_product.group1.xyz.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * geometric_product.e42) + (this.e4 * geometric_product.e23) + (this.e412 * geometric_product.e31) + (this.e321 * geometric_product.e41), (this.e1 * geometric_product.e43) + (this.e4 * geometric_product.e31) + (this.e423 * geometric_product.e12) + (this.e321 * geometric_product.e42), (this.e2 * geometric_product.e41) + (this.e4 * geometric_product.e12) + (this.e431 * geometric_product.e23) + (this.e321 * geometric_product.e43), this.e3 * geometric_product.e12 * -1.0) - (this.group0.yzxx * float4(geometric_product.group0.xyz.zxy, geometric_product.e23)) - float4((geometric_product.group1.xyz.zxy * this.group1.yzx), this.e2 * geometric_product.e31)
        );
    }
}
extension Flector: GeometricQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321 * geometric_product.e23, this.e321 * geometric_product.e31, this.e321 * geometric_product.e12, -(this.e2 * geometric_product.e42) - (this.e3 * geometric_product.e43) - (this.e423 * geometric_product.e23) - (this.e431 * geometric_product.e31) - (this.e412 * geometric_product.e12)) + (this.group0.xxyw * geometric_product.group1.wzxw) + float4((this.group0.zyz * geometric_product.group1.yww), this.e321 * geometric_product.e1234) - (this.group0.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * geometric_product.e42) + (this.e4 * geometric_product.e23) + (this.e423 * geometric_product.scalar) + (this.e412 * geometric_product.e31) + (this.e321 * geometric_product.e41), (this.e2 * geometric_product.e1234) + (this.e4 * geometric_product.e31) + (this.e423 * geometric_product.e12) + (this.e431 * geometric_product.scalar) + (this.e321 * geometric_product.e42), (this.e3 * geometric_product.e1234) + (this.e4 * geometric_product.e12) + (this.e431 * geometric_product.e23) + (this.e412 * geometric_product.scalar) + (this.e321 * geometric_product.e43), this.e3 * geometric_product.e12 * -1.0) + float4((this.group0.xxy * geometric_product.group0.wzx), this.e321 * geometric_product.scalar) - (this.group0.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23)) - (geometric_product.group1.zxyy * float4(this.group1.yzx, this.e2))
        );
    }
}
extension Flector: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (this.e4 * geometric_product.e321) - (this.e431 * geometric_product.e2) - (this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)) - (float2(geometric_product.e321, geometric_product.e1) * this.group1.wx), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e3 * geometric_product.e31, this.e1 * geometric_product.e12, this.e321 * geometric_product.e12, -(this.e2 * geometric_product.e42) - (this.e3 * geometric_product.e43) - (this.e423 * geometric_product.e23) - (this.e431 * geometric_product.e31) - (this.e412 * geometric_product.e12)) + (float4(geometric_product.scalar) * this.group0) + (float4(this.group1.ww, this.e2, this.e321) * float4(geometric_product.group3.xyz.xyx, geometric_product.e1234)) - (this.group0.yzxx * float4(geometric_product.group3.xyz.zxy, geometric_product.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * geometric_product.group1.xyz) + (float3(geometric_product.e2, geometric_product.e321, geometric_product.e321) * this.group1.zyz) + (float3(geometric_product.e321, geometric_product.e3, geometric_product.e1) * this.group1.xxy) + (this.group0.yzx * geometric_product.group4.zxy) - (float3(this.e321) * geometric_product.group4.xyz) - (float3(geometric_product.e4, geometric_product.e412, geometric_product.e423) * this.group0.xxy) - (float3(geometric_product.e431, geometric_product.e4, geometric_product.e4) * this.group0.zyz) - (this.group1.yzx * geometric_product.group1.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group0.yzx * geometric_product.group1.zxy) - (float3(this.e321) * geometric_product.group1.xyz) - (float3(geometric_product.e2, geometric_product.e321, geometric_product.e321) * this.group0.zyz) - (float3(geometric_product.e321, geometric_product.e3, geometric_product.e1) * this.group0.xxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e1 * geometric_product.e1234) + (this.e3 * geometric_product.e42) + (this.e4 * geometric_product.e23) + (this.e412 * geometric_product.e31) + (this.e321 * geometric_product.e41), (this.e1 * geometric_product.e43) + (this.e2 * geometric_product.e1234) + (this.e4 * geometric_product.e31) + (this.e423 * geometric_product.e12) + (this.e321 * geometric_product.e42), (this.e2 * geometric_product.e41) + (this.e3 * geometric_product.e1234) + (this.e4 * geometric_product.e12) + (this.e431 * geometric_product.e23) + (this.e321 * geometric_product.e43), this.e3 * geometric_product.e12 * -1.0) + (float4(geometric_product.scalar) * this.group1) - (this.group0.yzxx * float4(geometric_product.group2.xyz.zxy, geometric_product.e23)) - float4((geometric_product.group3.xyz.zxy * this.group1.yzx), this.e2 * geometric_product.e31)
        );
    }
}
extension Flector: GeometricQuotient<Plane> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Plane) -> Output {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e3 * geometric_product.e431) - (this.e321 * geometric_product.e423), -(this.e1 * geometric_product.e412) - (this.e321 * geometric_product.e431), -(this.e2 * geometric_product.e423) - (this.e321 * geometric_product.e412), (this.e3 * geometric_product.e412) + (this.e4 * geometric_product.e321)) + (this.group0.yzxx * geometric_product.group0.zxyx) + (geometric_product.group0.wwwy * float4(this.group1.xyz, this.e2)), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
extension Flector: GeometricQuotient<Point> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Point) -> Output {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e4 * geometric_product.e1) + (this.e412 * geometric_product.e2), (this.e4 * geometric_product.e2) + (this.e423 * geometric_product.e3), (this.e4 * geometric_product.e3) + (this.e431 * geometric_product.e1), -(this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)) - (this.group1.yzxy * geometric_product.group0.zxyy) - (geometric_product.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(this.e3 * geometric_product.e2) - (this.e321 * geometric_product.e1), -(this.e1 * geometric_product.e3) - (this.e321 * geometric_product.e2), -(this.e2 * geometric_product.e1) - (this.e321 * geometric_product.e3), (this.e2 * geometric_product.e2) + (this.e3 * geometric_product.e3)) + (this.group0.yzxx * geometric_product.group0.zxyx)
        );
    }
}
extension Flector: GeometricQuotient<Scalar> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Scalar) -> Output {
        Scalar geometric_product = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(1.0/other.scalar, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_product.scalar) * this.group1
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Horizon> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension Horizon: GeometricQuotient<DualNum> {
    associatedtype Output = Flector;
    func geometric_quotient(other: DualNum) -> Output {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.scalar, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_product.scalar * this.e321)
        );
    }
}
extension Horizon: GeometricQuotient<Flector> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e321) * float4(geometric_product.group1.xyz, geometric_product.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(geometric_product.group0.xyz, geometric_product.e321) * float4(-1.0)
        );
    }
}
extension Horizon: GeometricQuotient<Horizon> {
    associatedtype Output = Scalar;
    func geometric_quotient(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 / (other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: GeometricQuotient<Line> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group1.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_product.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * geometric_product.group0.xyz), 0.0)
        );
    }
}
extension Horizon: GeometricQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(geometric_product.group1.xyz, geometric_product.e1234), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(geometric_product.group0.xyz, geometric_product.scalar)
        );
    }
}
extension Horizon: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321) * float2(geometric_product.e321, geometric_product.e4) * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(geometric_product.group3.xyz, geometric_product.e1234), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e321) * geometric_product.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * geometric_product.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(geometric_product.group2.xyz, geometric_product.scalar)
        );
    }
}
extension Horizon: GeometricQuotient<Plane> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Plane) -> Output {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e321) * geometric_product.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e321 * geometric_product.e321 * -1.0)
        );
    }
}
extension Horizon: GeometricQuotient<Point> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Point) -> Output {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e321 * geometric_product.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321) * geometric_product.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension Horizon: GeometricQuotient<Scalar> {
    associatedtype Output = Horizon;
    func geometric_quotient(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 / (other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Line> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension Line: GeometricQuotient<DualNum> {
    associatedtype Output = Line;
    func geometric_quotient(other: DualNum) -> Output {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.scalar, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(geometric_product.scalar) * this.group0.xyz) + (float3(geometric_product.e1234) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_product.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: GeometricQuotient<Flector> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group1.xyz.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group1.xyz.xxy, this.e41)) + float4(float3(0.0), (geometric_product.e3 * this.e43) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12)) - float4((this.group1.xyz.yzx * geometric_product.group0.zxy), geometric_product.e423 * this.e23), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e3 * this.e42) + (geometric_product.e4 * this.e23) + (geometric_product.e431 * this.e12), (geometric_product.e1 * this.e43) + (geometric_product.e4 * this.e31) + (geometric_product.e412 * this.e23), (geometric_product.e2 * this.e41) + (geometric_product.e4 * this.e12) + (geometric_product.e423 * this.e31), 0.0) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group0.xyz.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group0.xyz.xxy, this.e23)) - float4((this.group1.xyz.yzx * geometric_product.group1.zxy), geometric_product.e3 * this.e12)
        );
    }
}
extension Line: GeometricQuotient<Horizon> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Horizon) -> Output {
        Horizon geometric_product = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(1.0/other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product.e321) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_product.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension Line: GeometricQuotient<Line> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group1.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e42 * this.e12) + (geometric_product.e31 * this.e43), (geometric_product.e43 * this.e23) + (geometric_product.e12 * this.e41), (geometric_product.e41 * this.e31) + (geometric_product.e23 * this.e42), -(geometric_product.e43 * this.e12) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) - float4((geometric_product.group0.xyz.zxy * this.group1.xyz.yzx), geometric_product.e41 * this.e23) - float4((geometric_product.group1.xyz.zxy * this.group0.xyz.yzx), geometric_product.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e31 * this.e12, geometric_product.e12 * this.e23, geometric_product.e23 * this.e31, -(geometric_product.e31 * this.e31) - (geometric_product.e12 * this.e12)) - float4((geometric_product.group1.xyz.zxy * this.group1.xyz.yzx), geometric_product.e23 * this.e23)
        );
    }
}
extension Line: GeometricQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * geometric_product.scalar) + (this.e43 * geometric_product.e31) + (this.e23 * geometric_product.e1234) + (this.e12 * geometric_product.e42), (this.e41 * geometric_product.e12) + (this.e42 * geometric_product.scalar) + (this.e23 * geometric_product.e43) + (this.e31 * geometric_product.e1234), (this.e42 * geometric_product.e23) + (this.e43 * geometric_product.scalar) + (this.e31 * geometric_product.e41) + (this.e12 * geometric_product.e1234), -(this.e43 * geometric_product.e12) - (this.e23 * geometric_product.e41) - (this.e31 * geometric_product.e42) - (this.e12 * geometric_product.e43)) - (geometric_product.group1.zxyx * float4(this.group0.xyz.yzx, this.e41)) - float4((this.group1.xyz.yzx * geometric_product.group0.zxy), this.e42 * geometric_product.e31), 
            /* e23, e31, e12, scalar */
            float4((this.e23 * geometric_product.scalar) + (this.e12 * geometric_product.e31), (this.e23 * geometric_product.e12) + (this.e31 * geometric_product.scalar), (this.e31 * geometric_product.e23) + (this.e12 * geometric_product.scalar), -(this.e31 * geometric_product.e31) - (this.e12 * geometric_product.e12)) - (geometric_product.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
extension Line: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(this.e23 * geometric_product.e41) - (this.e31 * geometric_product.e42) - (this.e12 * geometric_product.e43)) - (float2(geometric_product.e23) * float2(this.e23, this.e41)) - (float2(geometric_product.e31) * float2(this.e31, this.e42)) - (float2(geometric_product.e12) * float2(this.e12, this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group1.xyz.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group1.xyz.xxy, this.e41)) + float4(float3(0.0), (this.e43 * geometric_product.e3) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)) - float4((this.group1.xyz.yzx * geometric_product.group1.zxy), this.e23 * geometric_product.e423), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_product.scalar) * this.group0.xyz) + (float3(geometric_product.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * geometric_product.group3.xyz.yzx) + (this.group1.xyz.zxy * geometric_product.group2.xyz.yzx) - (this.group0.xyz.yzx * geometric_product.group3.xyz.zxy) - (this.group1.xyz.yzx * geometric_product.group2.xyz.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_product.scalar) * this.group1.xyz) + (this.group1.xyz.zxy * geometric_product.group3.xyz.yzx) - (this.group1.xyz.yzx * geometric_product.group3.xyz.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4) + (this.e12 * geometric_product.e431), (this.e43 * geometric_product.e1) + (this.e23 * geometric_product.e412) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e31 * geometric_product.e423) + (this.e12 * geometric_product.e4), 0.0) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group0.xyz.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group0.xyz.xxy, this.e23)) - float4((this.group1.xyz.yzx * geometric_product.group4.zxy), this.e12 * geometric_product.e3)
        );
    }
}
extension Line: GeometricQuotient<Plane> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Plane) -> Output {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product.e321) * this.group1.xyz), -(this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)), 
            /* e423, e431, e412, e321 */
            float4((this.group1.xyz.zxy * geometric_product.group0.yzx), 0.0) - float4((float3(geometric_product.e321) * this.group0.xyz), 0.0) - float4((this.group1.xyz.yzx * geometric_product.group0.zxy), 0.0)
        );
    }
}
extension Line: GeometricQuotient<Point> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Point) -> Output {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * geometric_product.e3 * -1.0, this.e12 * geometric_product.e1 * -1.0, this.e23 * geometric_product.e2 * -1.0, (this.e42 * geometric_product.e2) + (this.e43 * geometric_product.e3)) + (geometric_product.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4), (this.e43 * geometric_product.e1) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e12 * geometric_product.e4), -(this.e31 * geometric_product.e2) - (this.e12 * geometric_product.e3)) - (geometric_product.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension Line: GeometricQuotient<Scalar> {
    associatedtype Output = Line;
    func geometric_quotient(other: Scalar) -> Output {
        Scalar geometric_product = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(1.0/other.scalar, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(geometric_product.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_product.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Motor> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension Motor: GeometricQuotient<DualNum> {
    associatedtype Output = Motor;
    func geometric_quotient(other: DualNum) -> Output {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.scalar, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(geometric_product.scalar) * this.group0) + (float4(geometric_product.e1234) * this.group1), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.scalar) * this.group1
        );
    }
}
extension Motor: GeometricQuotient<Flector> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (geometric_product.group0.xyxx * float4(this.group1.wwy, this.e41)) + (geometric_product.group0.yzzy * float4(this.group1.zxw, this.e42)) + float4(float3(0.0), (geometric_product.e4 * this.scalar) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12) - (geometric_product.e321 * this.e1234)) + float4((geometric_product.group1.www * this.group1.xyz), geometric_product.e3 * this.e43) - (this.group1.yzxx * float4(geometric_product.group0.zxy, geometric_product.e423)), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e4 * this.e23) + (geometric_product.e423 * this.scalar) + (geometric_product.e431 * this.e12) - (geometric_product.e321 * this.e41), (geometric_product.e4 * this.e31) + (geometric_product.e431 * this.scalar) + (geometric_product.e412 * this.e23) - (geometric_product.e321 * this.e42), (geometric_product.e4 * this.e12) + (geometric_product.e423 * this.e31) + (geometric_product.e412 * this.scalar) - (geometric_product.e321 * this.e43), 0.0) + float4((geometric_product.group0.zxy * this.group0.yzx), geometric_product.e321 * this.scalar) - (geometric_product.group0.xyxx * float4(this.group0.wwy, this.e23)) - (geometric_product.group0.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(geometric_product.group1.zxy, geometric_product.e3))
        );
    }
}
extension Motor: GeometricQuotient<Horizon> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Horizon) -> Output {
        Horizon geometric_product = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(1.0/other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.e321) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.e321) * float4(this.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Motor: GeometricQuotient<Line> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group1.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e41 * this.scalar) + (geometric_product.e42 * this.e12) + (geometric_product.e23 * this.e1234) + (geometric_product.e31 * this.e43), (geometric_product.e42 * this.scalar) + (geometric_product.e43 * this.e23) + (geometric_product.e31 * this.e1234) + (geometric_product.e12 * this.e41), (geometric_product.e41 * this.e31) + (geometric_product.e43 * this.scalar) + (geometric_product.e23 * this.e42) + (geometric_product.e12 * this.e1234), -(geometric_product.e43 * this.e12) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) - (this.group1.yzxx * float4(geometric_product.group0.xyz.zxy, geometric_product.e41)) - float4((geometric_product.group1.xyz.zxy * this.group0.yzx), geometric_product.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((geometric_product.e23 * this.scalar) + (geometric_product.e31 * this.e12), (geometric_product.e31 * this.scalar) + (geometric_product.e12 * this.e23), (geometric_product.e23 * this.e31) + (geometric_product.e12 * this.scalar), -(geometric_product.e31 * this.e31) - (geometric_product.e12 * this.e12)) - (this.group1.yzxx * float4(geometric_product.group1.xyz.zxy, geometric_product.e23))
        );
    }
}
extension Motor: GeometricQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e1234 * this.e23) + (geometric_product.e23 * this.e1234) + (geometric_product.e31 * this.e43) + (geometric_product.scalar * this.e41), (geometric_product.e1234 * this.e31) + (geometric_product.e31 * this.e1234) + (geometric_product.e12 * this.e41) + (geometric_product.scalar * this.e42), (geometric_product.e1234 * this.e12) + (geometric_product.e23 * this.e42) + (geometric_product.e12 * this.e1234) + (geometric_product.scalar * this.e43), -(geometric_product.e43 * this.e12) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) + (geometric_product.group0.xyxw * this.group1.wwyw) + float4((geometric_product.group0.yzz * this.group1.zxw), geometric_product.scalar * this.e1234) - (geometric_product.group0.zxyx * this.group1.yzxx) - float4((geometric_product.group1.zxy * this.group0.yzx), geometric_product.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((geometric_product.e31 * this.e12) + (geometric_product.scalar * this.e23), (geometric_product.e12 * this.e23) + (geometric_product.scalar * this.e31), (geometric_product.e12 * this.scalar) + (geometric_product.scalar * this.e12), -(geometric_product.e31 * this.e31) - (geometric_product.e12 * this.e12)) + (geometric_product.group1.xyxw * this.group1.wwyw) - (geometric_product.group1.zxyx * this.group1.yzxx)
        );
    }
}
extension Motor: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (this.scalar * geometric_product.e1234) - (this.e41 * geometric_product.e23) - (this.e42 * geometric_product.e31) - (this.e43 * geometric_product.e12)) + (float2(geometric_product.scalar) * float2(this.scalar, this.e1234)) - (float2(this.e23) * float2(geometric_product.e23, geometric_product.e41)) - (float2(this.e31) * float2(geometric_product.e31, geometric_product.e42)) - (float2(this.e12) * float2(geometric_product.e12, geometric_product.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group1.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group1.xxy, this.e41)) + (geometric_product.group1.xyzz * float4(this.group1.www, this.e43)) + float4(float3(0.0), (this.scalar * geometric_product.e4) - (this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)) - float4((this.group1.yzx * geometric_product.group1.zxy), this.e1234 * geometric_product.e321), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_product.scalar) * this.group0.xyz) + (float3(geometric_product.e1234) * this.group1.xyz) + (geometric_product.group2.xyz.xyx * this.group1.wwy) + (geometric_product.group2.xyz.yzz * this.group1.zxw) + (geometric_product.group3.xyz.xyx * this.group0.wwy) + (geometric_product.group3.xyz.yzz * this.group0.zxw) - (geometric_product.group2.xyz.zxy * this.group1.yzx) - (geometric_product.group3.xyz.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_product.scalar) * this.group1.xyz) + (geometric_product.group3.xyz.xyx * this.group1.wwy) + (geometric_product.group3.xyz.yzz * this.group1.zxw) - (geometric_product.group3.xyz.zxy * this.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * geometric_product.e4) + (this.e12 * geometric_product.e431) + (this.scalar * geometric_product.e423) - (this.e31 * geometric_product.e412), (this.e23 * geometric_product.e412) + (this.e31 * geometric_product.e4) + (this.scalar * geometric_product.e431) - (this.e12 * geometric_product.e423), (this.e31 * geometric_product.e423) + (this.e12 * geometric_product.e4) + (this.scalar * geometric_product.e412) - (this.e23 * geometric_product.e431), 0.0) + float4((this.group0.yzx * geometric_product.group1.zxy), this.scalar * geometric_product.e321) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group0.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group0.xxy, this.e23)) - (geometric_product.group1.xyzz * float4(this.group0.www, this.e12))
        );
    }
}
extension Motor: GeometricQuotient<Plane> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Plane) -> Output {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product.e321) * this.group1.xyz), -(this.e1234 * geometric_product.e321) - (this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group0.xyz) + (this.group1.zxy * geometric_product.group0.yzx) - (float3(geometric_product.e321) * this.group0.xyz) - (this.group1.yzx * geometric_product.group0.zxy)), this.scalar * geometric_product.e321)
        );
    }
}
extension Motor: GeometricQuotient<Point> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Point) -> Output {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * geometric_product.e3 * -1.0, this.e12 * geometric_product.e1 * -1.0, this.e23 * geometric_product.e2 * -1.0, (this.e43 * geometric_product.e3) + (this.scalar * geometric_product.e4)) + (geometric_product.group0.xyzy * float4(this.group1.www, this.e42)) + (geometric_product.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4), (this.e43 * geometric_product.e1) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e12 * geometric_product.e4), this.e12 * geometric_product.e3 * -1.0) - (geometric_product.group0.xyzy * float4(this.group0.www, this.e31)) - (geometric_product.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Motor: GeometricQuotient<Scalar> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Scalar) -> Output {
        Scalar geometric_product = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(1.0/other.scalar, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(geometric_product.scalar) * this.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<MultiVector> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension MultiVector: GeometricQuotient<DualNum> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: DualNum) -> Output {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.scalar, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_product.scalar * this.scalar, (geometric_product.scalar * this.e1234) + (geometric_product.e1234 * this.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.group0.xy.xx, geometric_product.scalar, (geometric_product.scalar * this.e4) + (geometric_product.e1234 * this.e321)) * float4(this.group1.xyz, 1.0), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_product.scalar) * this.group2.xyz) + (float3(geometric_product.e1234) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_product.scalar) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group4.xyz) + (float3(geometric_product.e1234) * this.group1.xyz)), geometric_product.scalar * this.e321)
        );
    }
}
extension MultiVector: GeometricQuotient<Flector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (geometric_product.e321 * this.e4) - (geometric_product.e2 * this.e431) - (geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321)) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)) - (float2(geometric_product.e321, geometric_product.e1) * this.group4.wx), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.scalar) * geometric_product.group0) + (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group3.xyz.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group3.xyz.xxy, this.e41)) + float4(float3(0.0), (geometric_product.e3 * this.e43) - (geometric_product.e423 * this.e23) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12)) - float4((this.group3.xyz.yzx * geometric_product.group0.zxy), geometric_product.e321 * this.e1234), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_product.e321) * this.group4.xyz) + (float3(this.e4, this.e4, this.e431) * geometric_product.group0.xyx) + (float3(this.e412, this.e423, this.e4) * geometric_product.group0.yzz) + (geometric_product.group1.zxy * this.group1.yzx) - (float3(geometric_product.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group1.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group1.xyx) - (geometric_product.group0.zxy * this.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((geometric_product.group0.zxy * this.group1.yzx) - (float3(geometric_product.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group0.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group0.xyx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e3 * this.e42) + (geometric_product.e4 * this.e23) + (geometric_product.e431 * this.e12) - (geometric_product.e412 * this.e31), (geometric_product.e1 * this.e43) + (geometric_product.e4 * this.e31) + (geometric_product.e412 * this.e23) - (geometric_product.e423 * this.e12), (geometric_product.e2 * this.e41) + (geometric_product.e4 * this.e12) + (geometric_product.e423 * this.e31) - (geometric_product.e431 * this.e23), 0.0) + (float4(this.scalar) * geometric_product.group1) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e3) * float4(this.group2.xyz.zyz, this.e12)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e2) * float4(this.group2.xyz.xxy, this.e31)) - (geometric_product.group0.xyzx * float4(this.group0.xy.yy, this.e1234, this.e23))
        );
    }
}
extension MultiVector: GeometricQuotient<Horizon> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: Horizon) -> Output {
        Horizon geometric_product = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(1.0/other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_product.e321) * float2(this.e321, this.e4) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.e321) * float4(this.group3.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(geometric_product.e321) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_product.e321) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.e321) * float4(this.group2.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector: GeometricQuotient<Line> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) - (float2(this.e23) * float2(geometric_product.e23, geometric_product.e41)) - (float2(this.e31) * float2(geometric_product.e31, geometric_product.e42)) - (float2(this.e12) * float2(geometric_product.e12, geometric_product.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((geometric_product.e23 * this.e321) + (geometric_product.e31 * this.e3), (geometric_product.e31 * this.e321) + (geometric_product.e12 * this.e1), (geometric_product.e23 * this.e2) + (geometric_product.e12 * this.e321), -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)) - (this.group1.yzxx * float4(geometric_product.group1.xyz.zxy, geometric_product.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * geometric_product.group0.xyz) + (float3(this.e1234) * geometric_product.group1.xyz) + (geometric_product.group0.xyz.yzx * this.group3.xyz.zxy) + (geometric_product.group1.xyz.yzx * this.group2.xyz.zxy) - (geometric_product.group0.xyz.zxy * this.group3.xyz.yzx) - (geometric_product.group1.xyz.zxy * this.group2.xyz.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * geometric_product.group1.xyz) + (geometric_product.group1.xyz.yzx * this.group3.xyz.zxy) - (geometric_product.group1.xyz.zxy * this.group3.xyz.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e41 * this.e321) + (geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4) + (geometric_product.e31 * this.e412), (geometric_product.e42 * this.e321) + (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4) + (geometric_product.e12 * this.e423), (geometric_product.e41 * this.e2) + (geometric_product.e43 * this.e321) + (geometric_product.e23 * this.e431) + (geometric_product.e12 * this.e4), geometric_product.e12 * this.e3 * -1.0) - (this.group1.yzxx * float4(geometric_product.group0.xyz.zxy, geometric_product.e23)) - float4((geometric_product.group1.xyz.zxy * this.group4.yzx), geometric_product.e31 * this.e2)
        );
    }
}
extension MultiVector: GeometricQuotient<Motor> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (geometric_product.scalar * this.e1234) - (geometric_product.e41 * this.e23) - (geometric_product.e42 * this.e31) - (geometric_product.e43 * this.e12)) + (float2(this.scalar) * float2(geometric_product.scalar, geometric_product.e1234)) - (float2(geometric_product.e23) * float2(this.e23, this.e41)) - (float2(geometric_product.e31) * float2(this.e31, this.e42)) - (float2(geometric_product.e12) * float2(this.e12, this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar * this.e1, geometric_product.scalar * this.e2, geometric_product.scalar * this.e3, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)) + (geometric_product.group1.yzzw * float4(this.group1.zx, this.e321, this.e4)) + (float4(this.group4.ww, this.e2, this.e321) * float4(geometric_product.group1.xyx, geometric_product.e1234)) - (this.group1.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * geometric_product.group0.xyz) + (float3(this.e1234) * geometric_product.group1.xyz) + (this.group2.xyz.xxy * geometric_product.group1.wzx) + (this.group2.xyz.zyz * geometric_product.group1.yww) + (this.group3.xyz.xxy * geometric_product.group0.wzx) + (this.group3.xyz.zyz * geometric_product.group0.yww) - (this.group2.xyz.yzx * geometric_product.group1.zxy) - (this.group3.xyz.yzx * geometric_product.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * geometric_product.group1.xyz) + (this.group3.xyz.xxy * geometric_product.group1.wzx) + (this.group3.xyz.zyz * geometric_product.group1.yww) - (this.group3.xyz.yzx * geometric_product.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e42 * this.e3) + (geometric_product.e1234 * this.e1) + (geometric_product.e23 * this.e4) + (geometric_product.e31 * this.e412) + (geometric_product.scalar * this.e423), (geometric_product.e43 * this.e1) + (geometric_product.e1234 * this.e2) + (geometric_product.e31 * this.e4) + (geometric_product.e12 * this.e423) + (geometric_product.scalar * this.e431), (geometric_product.e43 * this.e321) + (geometric_product.e1234 * this.e3) + (geometric_product.e23 * this.e431) + (geometric_product.e12 * this.e4) + (geometric_product.scalar * this.e412), geometric_product.e12 * this.e3 * -1.0) + (float4(this.group4.ww, this.e2, this.e321) * float4(geometric_product.group0.xyx, geometric_product.scalar)) - (geometric_product.group1.zxyy * float4(this.group4.yzx, this.e2)) - (this.group1.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23))
        );
    }
}
extension MultiVector: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (geometric_product.e1234 * this.scalar) + (geometric_product.e321 * this.e4) - (geometric_product.e2 * this.e431) - (geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) + (float2(geometric_product.scalar) * this.group0.xy) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)) - (float2(this.e23) * float2(geometric_product.e23, geometric_product.e41)) - (float2(this.e31) * float2(geometric_product.e31, geometric_product.e42)) - (float2(this.e12) * float2(geometric_product.e12, geometric_product.e43)) - (float2(geometric_product.e321, geometric_product.e1) * this.group4.wx), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product.scalar) * this.group1) + (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e3) * float4(this.group3.xyz.zyz, this.e43)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e2) * float4(this.group3.xyz.xxy, this.e42)) + (float4(this.group0.xy.xx, this.scalar, geometric_product.e1234) * float4(geometric_product.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, geometric_product.e1) * float4(geometric_product.group3.xyz.yzz, this.e41)) + (float4(this.group4.ww, this.e2, geometric_product.e4) * float4(geometric_product.group3.xyz.xyx, this.scalar)) + float4(float3(0.0), -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412) - (geometric_product.e423 * this.e23) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12)) - float4((geometric_product.group3.xyz.zxy * this.group1.yzx), geometric_product.e321 * this.e1234) - float4((this.group3.xyz.yzx * geometric_product.group1.zxy), geometric_product.e41 * this.e1), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_product.scalar) * this.group2.xyz) + (float3(geometric_product.e1234) * this.group3.xyz) + (float3(geometric_product.e321) * this.group4.xyz) + (float3(this.scalar) * geometric_product.group2.xyz) + (float3(this.e1234) * geometric_product.group3.xyz) + (float3(this.e4, this.e4, this.e431) * geometric_product.group1.xyx) + (float3(this.e412, this.e423, this.e4) * geometric_product.group1.yzz) + (geometric_product.group2.xyz.yzx * this.group3.xyz.zxy) + (geometric_product.group3.xyz.yzx * this.group2.xyz.zxy) + (geometric_product.group4.zxy * this.group1.yzx) - (float3(geometric_product.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group4.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group4.xyx) - (geometric_product.group2.xyz.zxy * this.group3.xyz.yzx) - (geometric_product.group3.xyz.zxy * this.group2.xyz.yzx) - (geometric_product.group1.zxy * this.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(geometric_product.scalar) * this.group3.xyz) + (float3(this.scalar) * geometric_product.group3.xyz) + (geometric_product.group3.xyz.yzx * this.group3.xyz.zxy) + (geometric_product.group1.zxy * this.group1.yzx) - (float3(geometric_product.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group1.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group1.xyx) - (geometric_product.group3.xyz.zxy * this.group3.xyz.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e3 * this.e42) + (geometric_product.e4 * this.e23) + (geometric_product.e41 * this.e321) + (geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4) + (geometric_product.e31 * this.e412) + (geometric_product.e423 * this.scalar) + (geometric_product.e431 * this.e12), (geometric_product.e1 * this.e43) + (geometric_product.e4 * this.e31) + (geometric_product.e42 * this.e321) + (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4) + (geometric_product.e12 * this.e423) + (geometric_product.e431 * this.scalar) + (geometric_product.e412 * this.e23), (geometric_product.e2 * this.e41) + (geometric_product.e4 * this.e12) + (geometric_product.e41 * this.e2) + (geometric_product.e43 * this.e321) + (geometric_product.e23 * this.e431) + (geometric_product.e12 * this.e4) + (geometric_product.e423 * this.e31) + (geometric_product.e412 * this.scalar), 0.0) + (float4(geometric_product.scalar) * this.group4) + (float4(geometric_product.group0.xy.yy, geometric_product.e1234, this.scalar) * float4(this.group1.xyz, geometric_product.e321)) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group2.xyz.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group2.xyz.xxy, this.e23)) - (this.group1.yzxy * float4(geometric_product.group2.xyz.zxy, geometric_product.e31)) - (float4(this.group0.xy.yy, this.e1234, geometric_product.e23) * float4(geometric_product.group1.xyz, this.e1)) - float4((geometric_product.group3.xyz.zxy * this.group4.yzx), geometric_product.e12 * this.e3) - float4((this.group3.xyz.yzx * geometric_product.group4.zxy), geometric_product.e3 * this.e12)
        );
    }
}
extension MultiVector: GeometricQuotient<Plane> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: Plane) -> Output {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321 * geometric_product.e321, (this.e1 * geometric_product.e423) + (this.e2 * geometric_product.e431) + (this.e3 * geometric_product.e412) + (this.e4 * geometric_product.e321)) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product.e321) * this.group3.xyz), -(this.e1234 * geometric_product.e321) - (this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_product.e321) * this.group4.xyz) + (this.group1.yzx * geometric_product.group0.zxy) - (float3(this.e321) * geometric_product.group0.xyz) - (this.group1.zxy * geometric_product.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_product.e321) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group0.xyz) + (this.group3.xyz.zxy * geometric_product.group0.yzx) - (float3(geometric_product.e321) * this.group2.xyz) - (this.group3.xyz.yzx * geometric_product.group0.zxy)), this.scalar * geometric_product.e321)
        );
    }
}
extension MultiVector: GeometricQuotient<Point> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: Point) -> Output {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e1 * geometric_product.e1) + (this.e2 * geometric_product.e2) + (this.e3 * geometric_product.e3), -(this.e423 * geometric_product.e1) - (this.e431 * geometric_product.e2) - (this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e31 * geometric_product.e3 * -1.0, this.e12 * geometric_product.e1 * -1.0, this.e23 * geometric_product.e2 * -1.0, (this.e42 * geometric_product.e2) + (this.e43 * geometric_product.e3)) + (float4(this.scalar) * geometric_product.group0) + (geometric_product.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * geometric_product.group0.xyz) + (this.group4.zxy * geometric_product.group0.yzx) - (float3(geometric_product.e4) * this.group1.xyz) - (this.group4.yzx * geometric_product.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * geometric_product.group0.zxy) - (float3(this.e321) * geometric_product.group0.xyz) - (this.group1.zxy * geometric_product.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4), (this.e43 * geometric_product.e1) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e12 * geometric_product.e4), this.e12 * geometric_product.e3 * -1.0) - (geometric_product.group0.xyzx * float4(this.group0.xy.yy, this.e1234, this.e23)) - (geometric_product.group0.yzxy * float4(this.group2.xyz.zxy, this.e31))
        );
    }
}
extension MultiVector: GeometricQuotient<Scalar> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: Scalar) -> Output {
        Scalar geometric_product = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(1.0/other.scalar, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_product.scalar) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(geometric_product.scalar) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_product.scalar) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.scalar) * this.group4
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Origin> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension Origin: GeometricQuotient<DualNum> {
    associatedtype Output = Origin;
    func geometric_quotient(other: DualNum) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 / (other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: GeometricQuotient<Flector> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(geometric_product.group0.xyz, geometric_product.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Origin: GeometricQuotient<Horizon> {
    associatedtype Output = AntiScalar;
    func geometric_quotient(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e4 / (other.e321) * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: GeometricQuotient<Line> {
    associatedtype Output = Plane;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * float3(reverse.e23 * other_2.scalar, reverse.e31 * other_2.scalar, reverse.e12 * other_2.scalar)), 0.0)
        );
    }
}
extension Origin: GeometricQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * geometric_product.group1.xyz), 0.0)
        );
    }
}
extension Origin: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, geometric_product.e321 * this.e4) * float2(0.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product.scalar * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * geometric_product.group1.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * geometric_product.group3.xyz), 0.0)
        );
    }
}
extension Origin: GeometricQuotient<Plane> {
    associatedtype Output = AntiScalar;
    func geometric_quotient(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e4 / (other.e321) * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: GeometricQuotient<Point> {
    associatedtype Output = Line;
    func geometric_quotient(other: Point) -> Output {
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2), 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * float4(other.e1 * other_2.scalar, other.e2 * other_2.scalar, other.e3 * other_2.scalar, other.e4 * other_2.scalar).xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
    }
}
extension Origin: GeometricQuotient<Scalar> {
    associatedtype Output = Origin;
    func geometric_quotient(other: Scalar) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 / (other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Plane> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension Plane: GeometricQuotient<DualNum> {
    associatedtype Output = Flector;
    func geometric_quotient(other: DualNum) -> Output {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.scalar, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.scalar) * this.group0
        );
    }
}
extension Plane: GeometricQuotient<Flector> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e2 * this.e412) + (geometric_product.e321 * this.e423), (geometric_product.e3 * this.e423) + (geometric_product.e321 * this.e431), (geometric_product.e1 * this.e431) + (geometric_product.e321 * this.e412), -(geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321)) - (geometric_product.group0.zxyx * this.group0.yzxx) - (this.group0.wwwy * float4(geometric_product.group1.xyz, geometric_product.e2)), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(geometric_product.group0.xyz, geometric_product.e321) * float4(-1.0)
        );
    }
}
extension Plane: GeometricQuotient<Horizon> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Horizon) -> Output {
        Horizon geometric_product = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(1.0/other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(geometric_product.e321) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product.e321 * this.e321 * -1.0)
        );
    }
}
extension Plane: GeometricQuotient<Line> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group1.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_product.group1.xyz), -(geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * geometric_product.group0.xyz), 0.0) + float4((geometric_product.group1.xyz.yzx * this.group0.zxy), 0.0) - float4((geometric_product.group1.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Plane: GeometricQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_product.group1.xyz), (geometric_product.e1234 * this.e321) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group0.xyz) + (float3(this.e321) * geometric_product.group0.xyz) + (geometric_product.group1.yzx * this.group0.zxy) - (geometric_product.group1.zxy * this.group0.yzx)), geometric_product.scalar * this.e321)
        );
    }
}
extension Plane: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_product.e321 * this.e321, -(geometric_product.e1 * this.e423) - (geometric_product.e2 * this.e431) - (geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321)) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_product.group3.xyz), (geometric_product.e1234 * this.e321) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)), 
            /* e41, e42, e43, 0 */
            float4((float3(geometric_product.e321) * this.group0.xyz) + (geometric_product.group1.yzx * this.group0.zxy) - (float3(this.e321) * geometric_product.group4.xyz) - (geometric_product.group1.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * geometric_product.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group0.xyz) + (float3(this.e321) * geometric_product.group2.xyz) + (geometric_product.group3.xyz.yzx * this.group0.zxy) - (geometric_product.group3.xyz.zxy * this.group0.yzx)), geometric_product.scalar * this.e321)
        );
    }
}
extension Plane: GeometricQuotient<Plane> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Plane) -> Output {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(geometric_product.e321) * this.group0.xyz) - (float3(this.e321) * geometric_product.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product.e321 * this.e321 * -1.0)
        );
    }
}
extension Plane: GeometricQuotient<Point> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Point) -> Output {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e412 * geometric_product.e2, this.e423 * geometric_product.e3, this.e431 * geometric_product.e1, -(this.e431 * geometric_product.e2) - (this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)) - (this.group0.yzxx * geometric_product.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(geometric_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Plane: GeometricQuotient<Scalar> {
    associatedtype Output = Plane;
    func geometric_quotient(other: Scalar) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/other.scalar) * this.group0
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Point> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension Point: GeometricQuotient<DualNum> {
    associatedtype Output = Flector;
    func geometric_quotient(other: DualNum) -> Output {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(powi(other.scalar, -2)) * other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_product.group0.xy.yy, geometric_product.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: GeometricQuotient<Flector> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(geometric_product.e4 * this.e1) - (geometric_product.e431 * this.e3), -(geometric_product.e4 * this.e2) - (geometric_product.e412 * this.e1), -(geometric_product.e4 * this.e3) - (geometric_product.e423 * this.e2), (geometric_product.e412 * this.e3) + (geometric_product.e321 * this.e4)) + (geometric_product.group1.zxyy * this.group0.yzxy) + (this.group0.wwwx * float4(geometric_product.group0.xyz, geometric_product.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(geometric_product.e2 * this.e3) - (geometric_product.e321 * this.e1), -(geometric_product.e3 * this.e1) - (geometric_product.e321 * this.e2), -(geometric_product.e1 * this.e2) - (geometric_product.e321 * this.e3), (geometric_product.e2 * this.e2) + (geometric_product.e3 * this.e3)) + (geometric_product.group0.zxyx * this.group0.yzxx)
        );
    }
}
extension Point: GeometricQuotient<Horizon> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Horizon) -> Output {
        Horizon geometric_product = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(1.0/other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_product.e321 * this.e4), 
            /* e23, e31, e12, scalar */
            float4((float3(geometric_product.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension Point: GeometricQuotient<Line> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group1.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.e31 * this.e3, geometric_product.e12 * this.e1, geometric_product.e23 * this.e2, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group1.xyz.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4), (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4), (geometric_product.e41 * this.e2) + (geometric_product.e12 * this.e4), -(geometric_product.e31 * this.e2) - (geometric_product.e12 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group0.xyz.zxy, geometric_product.e23))
        );
    }
}
extension Point: GeometricQuotient<Motor> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar * this.e1, geometric_product.scalar * this.e2, geometric_product.scalar * this.e3, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3)) + (geometric_product.group1.yzxw * this.group0.zxyw) - (this.group0.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e42 * this.e3) + (geometric_product.e1234 * this.e1) + (geometric_product.e23 * this.e4), (geometric_product.e43 * this.e1) + (geometric_product.e1234 * this.e2) + (geometric_product.e31 * this.e4), (geometric_product.e41 * this.e2) + (geometric_product.e1234 * this.e3) + (geometric_product.e12 * this.e4), -(geometric_product.e31 * this.e2) - (geometric_product.e12 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23))
        );
    }
}
extension Point: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, geometric_product.e321 * this.e4) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.e31 * this.e3, geometric_product.e12 * this.e1, geometric_product.e23 * this.e2, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3)) + (float4(geometric_product.scalar) * this.group0) - (this.group0.yzxx * float4(geometric_product.group3.xyz.zxy, geometric_product.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * geometric_product.group1.xyz) + (geometric_product.group4.zxy * this.group0.yzx) - (float3(geometric_product.e4) * this.group0.xyz) - (geometric_product.group4.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((geometric_product.group1.zxy * this.group0.yzx) - (float3(geometric_product.e321) * this.group0.xyz) - (geometric_product.group1.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e1234 * this.e1) + (geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4), (geometric_product.e1234 * this.e2) + (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4), (geometric_product.e1234 * this.e3) + (geometric_product.e41 * this.e2) + (geometric_product.e12 * this.e4), -(geometric_product.e31 * this.e2) - (geometric_product.e12 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group2.xyz.zxy, geometric_product.e23))
        );
    }
}
extension Point: GeometricQuotient<Plane> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Plane) -> Output {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product.e431 * this.e3 * -1.0, geometric_product.e412 * this.e1 * -1.0, geometric_product.e423 * this.e2 * -1.0, (geometric_product.e431 * this.e2) + (geometric_product.e412 * this.e3) + (geometric_product.e321 * this.e4)) + (geometric_product.group0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(geometric_product.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricQuotient<Point> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Point) -> Output {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.e4) * geometric_product.group0.xyz) - (float3(geometric_product.e4) * this.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e2 * this.e3 * -1.0, geometric_product.e3 * this.e1 * -1.0, geometric_product.e1 * this.e2 * -1.0, (geometric_product.e2 * this.e2) + (geometric_product.e3 * this.e3)) + (geometric_product.group0.zxyx * this.group0.yzxx)
        );
    }
}
extension Point: GeometricQuotient<Scalar> {
    associatedtype Output = Point;
    func geometric_quotient(other: Scalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/other.scalar) * this.group0
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Scalar> {
        return GeometricQuotientInfixPartial { a: this };
    }
}
extension Scalar: GeometricQuotient<DualNum> {
    associatedtype Output = DualNum;
    func geometric_quotient(other: DualNum) -> Output {
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, -2), 0.0, 0.0, 0.0)
        );
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar) * float2(other.scalar * other_2.scalar, other.e1234 * other_2.scalar), 0.0, 0.0)
        );
    }
}
extension Scalar: GeometricQuotient<Flector> {
    associatedtype Output = Flector;
    func geometric_quotient(other: Flector) -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * geometric_product.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * geometric_product.group1
        );
    }
}
extension Scalar: GeometricQuotient<Horizon> {
    associatedtype Output = Horizon;
    func geometric_quotient(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.scalar / (other.e321) * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: GeometricQuotient<Line> {
    associatedtype Output = Line;
    func geometric_quotient(other: Line) -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2), 0.0, 0.0, 0.0)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group1.xyz, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * geometric_product.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * geometric_product.group1.xyz, 0.0)
        );
    }
}
extension Scalar: GeometricQuotient<Motor> {
    associatedtype Output = Motor;
    func geometric_quotient(other: Motor) -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * geometric_product.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * geometric_product.group1
        );
    }
}
extension Scalar: GeometricQuotient<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_quotient(other: MultiVector) -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other_2.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other_2.scalar) * reverse.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other_2.scalar) * reverse.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar) * geometric_product.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * geometric_product.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * geometric_product.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * geometric_product.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * geometric_product.group4
        );
    }
}
extension Scalar: GeometricQuotient<Plane> {
    associatedtype Output = Plane;
    func geometric_quotient(other: Plane) -> Output {
        Plane reverse = Plane.from_groups(
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e321, -2), 0.0, 0.0, 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * float4(reverse.e423 * other_2.scalar, reverse.e431 * other_2.scalar, reverse.e412 * other_2.scalar, reverse.e321 * other_2.scalar)
        );
    }
}
extension Scalar: GeometricQuotient<Point> {
    associatedtype Output = Point;
    func geometric_quotient(other: Point) -> Output {
        Scalar other_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2), 0.0, 0.0, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.e1 * other_2.scalar, other.e2 * other_2.scalar, other.e3 * other_2.scalar, other.e4 * other_2.scalar)
        );
    }
}
extension Scalar: GeometricQuotient<Scalar> {
    associatedtype Output = Scalar;
    func geometric_quotient(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar / (other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
