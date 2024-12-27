extension AntiScalar {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<AntiScalar> {
        geometric_quotient_partial(this)
    }
}
extension AntiScalar: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 / (other.scalar)
        );
    }
}
extension AntiScalar: GeometricQuotient<Flector> {
    func geometric_quotient(other: Flector) -> Flector {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
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
            float4(this.e1234, this.e1234, this.e1234, 0.0) * float4(geometric_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Horizon> {
    func geometric_quotient(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 / (other.e321)
        );
    }
}
extension AntiScalar: GeometricQuotient<Line> {
    func geometric_quotient(other: Line) -> Line {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * float3(reverse.e23 * other_2.scalar, reverse.e31 * other_2.scalar, reverse.e12 * other_2.scalar), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Motor {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
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
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e1234 * geometric_product.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_product.group3, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234, this.e1234, this.e1234, 0.0) * float4(geometric_product.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 / (other.e321)
        );
    }
}
extension AntiScalar: GeometricQuotient<Point> {
    func geometric_quotient(other: Point) -> Plane {
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234, this.e1234, this.e1234, 0.0) * float4(float4(other.e1 * other_2.scalar, other.e2 * other_2.scalar, other.e3 * other_2.scalar, other.e4 * other_2.scalar).xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Scalar> {
    func geometric_quotient(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 / (other.scalar)
        );
    }
}
extension DualNum {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<DualNum> {
        geometric_quotient_partial(this)
    }
}
extension DualNum: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> DualNum {
DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.scalar, -2)) * other.group0
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.scalar * this.scalar, (geometric_product.scalar * this.e1234) + (geometric_product.e1234 * this.scalar))
        );
    }
}
extension DualNum: GeometricQuotient<Flector> {
    func geometric_quotient(other: Flector) -> Flector {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
        );
Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.e1, geometric_product.e2, geometric_product.e3, 1.0) * float4(this.group0.xx, this.scalar, (this.scalar * geometric_product.e4) - (this.e1234 * geometric_product.e321)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group1.xyz) - (float3(this.e1234) * geometric_product.group0.xyz)), this.scalar * geometric_product.e321)
        );
    }
}
extension DualNum: GeometricQuotient<Horizon> {
    func geometric_quotient(other: Horizon) -> Flector {
Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
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
    func geometric_quotient(other: Line) -> Line {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.scalar) * geometric_product.group0) + (float3(this.e1234) * geometric_product.group1), 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_product.group1
        );
    }
}
extension DualNum: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Motor {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
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
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * geometric_product.scalar, (this.scalar * geometric_product.e1234) + (this.e1234 * geometric_product.scalar)), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.e1, geometric_product.e2, geometric_product.e3, 1.0) * float4(this.group0.xx, this.scalar, (this.scalar * geometric_product.e4) - (this.e1234 * geometric_product.e321)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * geometric_product.group2) + (float3(this.e1234) * geometric_product.group3), 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_product.group3, 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group4.xyz) - (float3(this.e1234) * geometric_product.group1.xyz)), this.scalar * geometric_product.e321)
        );
    }
}
extension DualNum: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> Flector {
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
    func geometric_quotient(other: Point) -> Flector {
Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * geometric_product.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(geometric_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension DualNum: GeometricQuotient<Scalar> {
    func geometric_quotient(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0/other.scalar) * this.group0
        );
    }
}
extension Flector {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Flector> {
        geometric_quotient_partial(this)
    }
}
extension Flector: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> Flector {
DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.scalar, -2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 1.0) * float4(geometric_product.group0.xx, geometric_product.scalar, (geometric_product.scalar * this.e4) + (geometric_product.e1234 * this.e321)), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group1.xyz) + (float3(geometric_product.e1234) * this.group0.xyz)), geometric_product.scalar * this.e321)
        );
    }
}
extension Flector: GeometricQuotient<Flector> {
    func geometric_quotient(other: Flector) -> Motor {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
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
    func geometric_quotient(other: Horizon) -> Motor {
Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
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
    func geometric_quotient(other: Line) -> Flector {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e3 * geometric_product.e31) + (this.e321 * geometric_product.e23), (this.e1 * geometric_product.e12) + (this.e321 * geometric_product.e31), (this.e2 * geometric_product.e23) + (this.e321 * geometric_product.e12), -(this.e2 * geometric_product.e42) - (this.e3 * geometric_product.e43) - (this.e423 * geometric_product.e23) - (this.e431 * geometric_product.e31) - (this.e412 * geometric_product.e12)) - (this.group0.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * geometric_product.e42) + (this.e4 * geometric_product.e23) + (this.e412 * geometric_product.e31) + (this.e321 * geometric_product.e41), (this.e1 * geometric_product.e43) + (this.e4 * geometric_product.e31) + (this.e423 * geometric_product.e12) + (this.e321 * geometric_product.e42), (this.e2 * geometric_product.e41) + (this.e4 * geometric_product.e12) + (this.e431 * geometric_product.e23) + (this.e321 * geometric_product.e43), this.e3 * geometric_product.e12 * -1.0) - (this.group0.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23)) - float4((geometric_product.group1.zxy * this.group1.yzx), this.e2 * geometric_product.e31)
        );
    }
}
extension Flector: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Flector {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
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
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e4 * geometric_product.e321) - (this.e431 * geometric_product.e2) - (this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)) - (float2(geometric_product.e321, geometric_product.e1) * this.group1.wx), 
            /* e1, e2, e3, e4 */
            float4(this.e3 * geometric_product.e31, this.e1 * geometric_product.e12, this.e321 * geometric_product.e12, -(this.e2 * geometric_product.e42) - (this.e3 * geometric_product.e43) - (this.e423 * geometric_product.e23) - (this.e431 * geometric_product.e31) - (this.e412 * geometric_product.e12)) + (float4(geometric_product.scalar) * this.group0) + (float4(this.group1.ww, this.e2, this.e321) * float4(geometric_product.group3.xyx, geometric_product.e1234)) - (this.group0.yzxx * float4(geometric_product.group3.zxy, geometric_product.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * geometric_product.group1.xyz) + (float3(geometric_product.e2, geometric_product.e321, geometric_product.e321) * this.group1.zyz) + (float3(geometric_product.e321, geometric_product.e3, geometric_product.e1) * this.group1.xxy) + (this.group0.yzx * geometric_product.group4.zxy) - (float3(this.e321) * geometric_product.group4.xyz) - (float3(geometric_product.e4, geometric_product.e412, geometric_product.e423) * this.group0.xxy) - (float3(geometric_product.e431, geometric_product.e4, geometric_product.e4) * this.group0.zyz) - (this.group1.yzx * geometric_product.group1.zxy), 
            /* e23, e31, e12 */
            (this.group0.yzx * geometric_product.group1.zxy) - (float3(this.e321) * geometric_product.group1.xyz) - (float3(geometric_product.e2, geometric_product.e321, geometric_product.e321) * this.group0.zyz) - (float3(geometric_product.e321, geometric_product.e3, geometric_product.e1) * this.group0.xxy), 
            /* e423, e431, e412, e321 */
            float4((this.e1 * geometric_product.e1234) + (this.e3 * geometric_product.e42) + (this.e4 * geometric_product.e23) + (this.e412 * geometric_product.e31) + (this.e321 * geometric_product.e41), (this.e1 * geometric_product.e43) + (this.e2 * geometric_product.e1234) + (this.e4 * geometric_product.e31) + (this.e423 * geometric_product.e12) + (this.e321 * geometric_product.e42), (this.e2 * geometric_product.e41) + (this.e3 * geometric_product.e1234) + (this.e4 * geometric_product.e12) + (this.e431 * geometric_product.e23) + (this.e321 * geometric_product.e43), this.e3 * geometric_product.e12 * -1.0) + (float4(geometric_product.scalar) * this.group1) - (this.group0.yzxx * float4(geometric_product.group2.zxy, geometric_product.e23)) - float4((geometric_product.group3.zxy * this.group1.yzx), this.e2 * geometric_product.e31)
        );
    }
}
extension Flector: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> Motor {
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
    func geometric_quotient(other: Point) -> Motor {
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
    func geometric_quotient(other: Scalar) -> Flector {
Scalar geometric_product = Scalar.from_groups(
            /* scalar */
            1.0/other.scalar
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
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Horizon> {
        geometric_quotient_partial(this)
    }
}
extension Horizon: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> Flector {
DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.scalar, -2)) * other.group0
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
    func geometric_quotient(other: Flector) -> Motor {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
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
    func geometric_quotient(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 / (other.e321)
        );
    }
}
extension Horizon: GeometricQuotient<Line> {
    func geometric_quotient(other: Line) -> Flector {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(geometric_product.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(geometric_product.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Flector {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
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
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321) * float2(geometric_product.e321, geometric_product.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(geometric_product.group3, geometric_product.e1234), 
            /* e41, e42, e43 */
            float3(this.e321) * geometric_product.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e321) * geometric_product.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(geometric_product.group2, geometric_product.scalar)
        );
    }
}
extension Horizon: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> Motor {
Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(geometric_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e321 * geometric_product.e321 * -1.0)
        );
    }
}
extension Horizon: GeometricQuotient<Point> {
    func geometric_quotient(other: Point) -> Motor {
Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e321 * geometric_product.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(geometric_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Horizon: GeometricQuotient<Scalar> {
    func geometric_quotient(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 / (other.scalar)
        );
    }
}
extension Line {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Line> {
        geometric_quotient_partial(this)
    }
}
extension Line: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> Line {
DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.scalar, -2)) * other.group0
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group0) + (float3(geometric_product.e1234) * this.group1), 
            /* e23, e31, e12 */
            float3(geometric_product.scalar) * this.group1
        );
    }
}
extension Line: GeometricQuotient<Flector> {
    func geometric_quotient(other: Flector) -> Flector {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
        );
Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group1.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group1.xxy, this.e41)) + float4(float3(0.0), (geometric_product.e3 * this.e43) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12)) - float4((this.group1.yzx * geometric_product.group0.zxy), geometric_product.e423 * this.e23), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e3 * this.e42) + (geometric_product.e4 * this.e23) + (geometric_product.e431 * this.e12), (geometric_product.e1 * this.e43) + (geometric_product.e4 * this.e31) + (geometric_product.e412 * this.e23), (geometric_product.e2 * this.e41) + (geometric_product.e4 * this.e12) + (geometric_product.e423 * this.e31), 0.0) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group0.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group0.xxy, this.e23)) - float4((this.group1.yzx * geometric_product.group1.zxy), geometric_product.e3 * this.e12)
        );
    }
}
extension Line: GeometricQuotient<Horizon> {
    func geometric_quotient(other: Horizon) -> Flector {
Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.e321, geometric_product.e321, geometric_product.e321, 0.0) * float4(this.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.e321, geometric_product.e321, geometric_product.e321, 0.0) * float4(this.group0, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Line: GeometricQuotient<Line> {
    func geometric_quotient(other: Line) -> Motor {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e42 * this.e12) + (geometric_product.e31 * this.e43), (geometric_product.e43 * this.e23) + (geometric_product.e12 * this.e41), (geometric_product.e41 * this.e31) + (geometric_product.e23 * this.e42), -(geometric_product.e43 * this.e12) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) - float4((geometric_product.group0.zxy * this.group1.yzx), geometric_product.e41 * this.e23) - float4((geometric_product.group1.zxy * this.group0.yzx), geometric_product.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e31 * this.e12, geometric_product.e12 * this.e23, geometric_product.e23 * this.e31, -(geometric_product.e31 * this.e31) - (geometric_product.e12 * this.e12)) - float4((geometric_product.group1.zxy * this.group1.yzx), geometric_product.e23 * this.e23)
        );
    }
}
extension Line: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Motor {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
        );
Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * geometric_product.scalar) + (this.e43 * geometric_product.e31) + (this.e23 * geometric_product.e1234) + (this.e12 * geometric_product.e42), (this.e41 * geometric_product.e12) + (this.e42 * geometric_product.scalar) + (this.e23 * geometric_product.e43) + (this.e31 * geometric_product.e1234), (this.e42 * geometric_product.e23) + (this.e43 * geometric_product.scalar) + (this.e31 * geometric_product.e41) + (this.e12 * geometric_product.e1234), -(this.e43 * geometric_product.e12) - (this.e23 * geometric_product.e41) - (this.e31 * geometric_product.e42) - (this.e12 * geometric_product.e43)) - (geometric_product.group1.zxyx * float4(this.group0.yzx, this.e41)) - float4((this.group1.yzx * geometric_product.group0.zxy), this.e42 * geometric_product.e31), 
            /* e23, e31, e12, scalar */
            float4((this.e23 * geometric_product.scalar) + (this.e12 * geometric_product.e31), (this.e23 * geometric_product.e12) + (this.e31 * geometric_product.scalar), (this.e31 * geometric_product.e23) + (this.e12 * geometric_product.scalar), -(this.e31 * geometric_product.e31) - (this.e12 * geometric_product.e12)) - (geometric_product.group1.zxyx * float4(this.group1.yzx, this.e23))
        );
    }
}
extension Line: GeometricQuotient<MultiVector> {
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e23 * geometric_product.e41) - (this.e31 * geometric_product.e42) - (this.e12 * geometric_product.e43)) - (float2(geometric_product.e23) * float2(this.e23, this.e41)) - (float2(geometric_product.e31) * float2(this.e31, this.e42)) - (float2(geometric_product.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group1.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group1.xxy, this.e41)) + float4(float3(0.0), (this.e43 * geometric_product.e3) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)) - float4((this.group1.yzx * geometric_product.group1.zxy), this.e23 * geometric_product.e423), 
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group0) + (float3(geometric_product.e1234) * this.group1) + (this.group0.zxy * geometric_product.group3.yzx) + (this.group1.zxy * geometric_product.group2.yzx) - (this.group0.yzx * geometric_product.group3.zxy) - (this.group1.yzx * geometric_product.group2.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_product.scalar) * this.group1) + (this.group1.zxy * geometric_product.group3.yzx) - (this.group1.yzx * geometric_product.group3.zxy), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4) + (this.e12 * geometric_product.e431), (this.e43 * geometric_product.e1) + (this.e23 * geometric_product.e412) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e31 * geometric_product.e423) + (this.e12 * geometric_product.e4), 0.0) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group0.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group0.xxy, this.e23)) - float4((this.group1.yzx * geometric_product.group4.zxy), this.e12 * geometric_product.e3)
        );
    }
}
extension Line: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> Flector {
Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.e321, geometric_product.e321, geometric_product.e321, 1.0) * float4(this.group1, -(this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)), 
            /* e423, e431, e412, e321 */
            float4((this.group1.zxy * geometric_product.group0.yzx), 0.0) - float4((float3(geometric_product.e321) * this.group0), 0.0) - float4((this.group1.yzx * geometric_product.group0.zxy), 0.0)
        );
    }
}
extension Line: GeometricQuotient<Point> {
    func geometric_quotient(other: Point) -> Flector {
Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * geometric_product.e3 * -1.0, this.e12 * geometric_product.e1 * -1.0, this.e23 * geometric_product.e2 * -1.0, (this.e42 * geometric_product.e2) + (this.e43 * geometric_product.e3)) + (geometric_product.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4), (this.e43 * geometric_product.e1) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e12 * geometric_product.e4), -(this.e31 * geometric_product.e2) - (this.e12 * geometric_product.e3)) - (geometric_product.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Line: GeometricQuotient<Scalar> {
    func geometric_quotient(other: Scalar) -> Line {
Scalar geometric_product = Scalar.from_groups(
            /* scalar */
            1.0/other.scalar
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(geometric_product.scalar) * this.group0, 
            /* e23, e31, e12 */
            float3(geometric_product.scalar) * this.group1
        );
    }
}
extension Motor {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Motor> {
        geometric_quotient_partial(this)
    }
}
extension Motor: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> Motor {
DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.scalar, -2)) * other.group0
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
    func geometric_quotient(other: Flector) -> Flector {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
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
    func geometric_quotient(other: Horizon) -> Flector {
Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
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
    func geometric_quotient(other: Line) -> Motor {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e41 * this.scalar) + (geometric_product.e42 * this.e12) + (geometric_product.e23 * this.e1234) + (geometric_product.e31 * this.e43), (geometric_product.e42 * this.scalar) + (geometric_product.e43 * this.e23) + (geometric_product.e31 * this.e1234) + (geometric_product.e12 * this.e41), (geometric_product.e41 * this.e31) + (geometric_product.e43 * this.scalar) + (geometric_product.e23 * this.e42) + (geometric_product.e12 * this.e1234), -(geometric_product.e43 * this.e12) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) - (this.group1.yzxx * float4(geometric_product.group0.zxy, geometric_product.e41)) - float4((geometric_product.group1.zxy * this.group0.yzx), geometric_product.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((geometric_product.e23 * this.scalar) + (geometric_product.e31 * this.e12), (geometric_product.e31 * this.scalar) + (geometric_product.e12 * this.e23), (geometric_product.e23 * this.e31) + (geometric_product.e12 * this.scalar), -(geometric_product.e31 * this.e31) - (geometric_product.e12 * this.e12)) - (this.group1.yzxx * float4(geometric_product.group1.zxy, geometric_product.e23))
        );
    }
}
extension Motor: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Motor {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
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
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.scalar * geometric_product.e1234) - (this.e41 * geometric_product.e23) - (this.e42 * geometric_product.e31) - (this.e43 * geometric_product.e12)) + (float2(geometric_product.scalar) * float2(this.scalar, this.e1234)) - (float2(this.e23) * float2(geometric_product.e23, geometric_product.e41)) - (float2(this.e31) * float2(geometric_product.e31, geometric_product.e42)) - (float2(this.e12) * float2(geometric_product.e12, geometric_product.e43)), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group1.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group1.xxy, this.e41)) + (geometric_product.group1.xyzz * float4(this.group1.www, this.e43)) + float4(float3(0.0), (this.scalar * geometric_product.e4) - (this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)) - float4((this.group1.yzx * geometric_product.group1.zxy), this.e1234 * geometric_product.e321), 
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group0.xyz) + (float3(geometric_product.e1234) * this.group1.xyz) + (geometric_product.group2.xyx * this.group1.wwy) + (geometric_product.group2.yzz * this.group1.zxw) + (geometric_product.group3.xyx * this.group0.wwy) + (geometric_product.group3.yzz * this.group0.zxw) - (geometric_product.group2.zxy * this.group1.yzx) - (geometric_product.group3.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(geometric_product.scalar) * this.group1.xyz) + (geometric_product.group3.xyx * this.group1.wwy) + (geometric_product.group3.yzz * this.group1.zxw) - (geometric_product.group3.zxy * this.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * geometric_product.e4) + (this.e12 * geometric_product.e431) + (this.scalar * geometric_product.e423) - (this.e31 * geometric_product.e412), (this.e23 * geometric_product.e412) + (this.e31 * geometric_product.e4) + (this.scalar * geometric_product.e431) - (this.e12 * geometric_product.e423), (this.e31 * geometric_product.e423) + (this.e12 * geometric_product.e4) + (this.scalar * geometric_product.e412) - (this.e23 * geometric_product.e431), 0.0) + float4((this.group0.yzx * geometric_product.group1.zxy), this.scalar * geometric_product.e321) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group0.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group0.xxy, this.e23)) - (geometric_product.group1.xyzz * float4(this.group0.www, this.e12))
        );
    }
}
extension Motor: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> Flector {
Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.e321, geometric_product.e321, geometric_product.e321, 1.0) * float4(this.group1.xyz, -(this.e1234 * geometric_product.e321) - (this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group0.xyz) + (this.group1.zxy * geometric_product.group0.yzx) - (float3(geometric_product.e321) * this.group0.xyz) - (this.group1.yzx * geometric_product.group0.zxy)), this.scalar * geometric_product.e321)
        );
    }
}
extension Motor: GeometricQuotient<Point> {
    func geometric_quotient(other: Point) -> Flector {
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
    func geometric_quotient(other: Scalar) -> Motor {
Scalar geometric_product = Scalar.from_groups(
            /* scalar */
            1.0/other.scalar
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
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<MultiVector> {
        geometric_quotient_partial(this)
    }
}
extension MultiVector: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> MultiVector {
DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.scalar, -2)) * other.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.scalar * this.scalar, (geometric_product.scalar * this.e1234) + (geometric_product.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 1.0) * float4(geometric_product.group0.xx, geometric_product.scalar, (geometric_product.scalar * this.e4) + (geometric_product.e1234 * this.e321)), 
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group2) + (float3(geometric_product.e1234) * this.group3), 
            /* e23, e31, e12 */
            float3(geometric_product.scalar) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group4.xyz) + (float3(geometric_product.e1234) * this.group1.xyz)), geometric_product.scalar * this.e321)
        );
    }
}
extension MultiVector: GeometricQuotient<Flector> {
    func geometric_quotient(other: Flector) -> MultiVector {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
        );
Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (geometric_product.e321 * this.e4) - (geometric_product.e2 * this.e431) - (geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321)) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)) - (float2(geometric_product.e321, geometric_product.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(this.scalar) * geometric_product.group0) + (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group3.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group3.xxy, this.e41)) + float4(float3(0.0), (geometric_product.e3 * this.e43) - (geometric_product.e423 * this.e23) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12)) - float4((this.group3.yzx * geometric_product.group0.zxy), geometric_product.e321 * this.e1234), 
            /* e41, e42, e43 */
            (float3(geometric_product.e321) * this.group4.xyz) + (float3(this.e4, this.e4, this.e431) * geometric_product.group0.xyx) + (float3(this.e412, this.e423, this.e4) * geometric_product.group0.yzz) + (geometric_product.group1.zxy * this.group1.yzx) - (float3(geometric_product.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group1.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group1.xyx) - (geometric_product.group0.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (geometric_product.group0.zxy * this.group1.yzx) - (float3(geometric_product.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group0.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group0.xyx), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e3 * this.e42) + (geometric_product.e4 * this.e23) + (geometric_product.e431 * this.e12) - (geometric_product.e412 * this.e31), (geometric_product.e1 * this.e43) + (geometric_product.e4 * this.e31) + (geometric_product.e412 * this.e23) - (geometric_product.e423 * this.e12), (geometric_product.e2 * this.e41) + (geometric_product.e4 * this.e12) + (geometric_product.e423 * this.e31) - (geometric_product.e431 * this.e23), 0.0) + (float4(this.scalar) * geometric_product.group1) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e3) * float4(this.group2.zyz, this.e12)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e2) * float4(this.group2.xxy, this.e31)) - (geometric_product.group0.xyzx * float4(this.group0.yy, this.e1234, this.e23))
        );
    }
}
extension MultiVector: GeometricQuotient<Horizon> {
    func geometric_quotient(other: Horizon) -> MultiVector {
Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.e321) * float2(this.e321, this.e4) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.e321) * float4(this.group3, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(geometric_product.e321) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(geometric_product.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.e321) * float4(this.group2, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector: GeometricQuotient<Line> {
    func geometric_quotient(other: Line) -> MultiVector {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) - (float2(this.e23) * float2(geometric_product.e23, geometric_product.e41)) - (float2(this.e31) * float2(geometric_product.e31, geometric_product.e42)) - (float2(this.e12) * float2(geometric_product.e12, geometric_product.e43)), 
            /* e1, e2, e3, e4 */
            float4((geometric_product.e23 * this.e321) + (geometric_product.e31 * this.e3), (geometric_product.e31 * this.e321) + (geometric_product.e12 * this.e1), (geometric_product.e23 * this.e2) + (geometric_product.e12 * this.e321), -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)) - (this.group1.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * geometric_product.group0) + (float3(this.e1234) * geometric_product.group1) + (geometric_product.group0.yzx * this.group3.zxy) + (geometric_product.group1.yzx * this.group2.zxy) - (geometric_product.group0.zxy * this.group3.yzx) - (geometric_product.group1.zxy * this.group2.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_product.group1) + (geometric_product.group1.yzx * this.group3.zxy) - (geometric_product.group1.zxy * this.group3.yzx), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e41 * this.e321) + (geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4) + (geometric_product.e31 * this.e412), (geometric_product.e42 * this.e321) + (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4) + (geometric_product.e12 * this.e423), (geometric_product.e41 * this.e2) + (geometric_product.e43 * this.e321) + (geometric_product.e23 * this.e431) + (geometric_product.e12 * this.e4), geometric_product.e12 * this.e3 * -1.0) - (this.group1.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23)) - float4((geometric_product.group1.zxy * this.group4.yzx), geometric_product.e31 * this.e2)
        );
    }
}
extension MultiVector: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> MultiVector {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
        );
Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (geometric_product.scalar * this.e1234) - (geometric_product.e41 * this.e23) - (geometric_product.e42 * this.e31) - (geometric_product.e43 * this.e12)) + (float2(this.scalar) * float2(geometric_product.scalar, geometric_product.e1234)) - (float2(geometric_product.e23) * float2(this.e23, this.e41)) - (float2(geometric_product.e31) * float2(this.e31, this.e42)) - (float2(geometric_product.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar * this.e1, geometric_product.scalar * this.e2, geometric_product.scalar * this.e3, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)) + (geometric_product.group1.yzzw * float4(this.group1.zx, this.e321, this.e4)) + (float4(this.group4.ww, this.e2, this.e321) * float4(geometric_product.group1.xyx, geometric_product.e1234)) - (this.group1.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * geometric_product.group0.xyz) + (float3(this.e1234) * geometric_product.group1.xyz) + (this.group2.xxy * geometric_product.group1.wzx) + (this.group2.zyz * geometric_product.group1.yww) + (this.group3.xxy * geometric_product.group0.wzx) + (this.group3.zyz * geometric_product.group0.yww) - (this.group2.yzx * geometric_product.group1.zxy) - (this.group3.yzx * geometric_product.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_product.group1.xyz) + (this.group3.xxy * geometric_product.group1.wzx) + (this.group3.zyz * geometric_product.group1.yww) - (this.group3.yzx * geometric_product.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e42 * this.e3) + (geometric_product.e1234 * this.e1) + (geometric_product.e23 * this.e4) + (geometric_product.e31 * this.e412) + (geometric_product.scalar * this.e423), (geometric_product.e43 * this.e1) + (geometric_product.e1234 * this.e2) + (geometric_product.e31 * this.e4) + (geometric_product.e12 * this.e423) + (geometric_product.scalar * this.e431), (geometric_product.e43 * this.e321) + (geometric_product.e1234 * this.e3) + (geometric_product.e23 * this.e431) + (geometric_product.e12 * this.e4) + (geometric_product.scalar * this.e412), geometric_product.e12 * this.e3 * -1.0) + (float4(this.group4.ww, this.e2, this.e321) * float4(geometric_product.group0.xyx, geometric_product.scalar)) - (geometric_product.group1.zxyy * float4(this.group4.yzx, this.e2)) - (this.group1.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23))
        );
    }
}
extension MultiVector: GeometricQuotient<MultiVector> {
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (geometric_product.e1234 * this.scalar) + (geometric_product.e321 * this.e4) - (geometric_product.e2 * this.e431) - (geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) + (float2(geometric_product.scalar) * this.group0) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)) - (float2(this.e23) * float2(geometric_product.e23, geometric_product.e41)) - (float2(this.e31) * float2(geometric_product.e31, geometric_product.e42)) - (float2(this.e12) * float2(geometric_product.e12, geometric_product.e43)) - (float2(geometric_product.e321, geometric_product.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product.scalar) * this.group1) + (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e3) * float4(this.group3.zyz, this.e43)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e2) * float4(this.group3.xxy, this.e42)) + (float4(this.group0.xx, this.scalar, geometric_product.e1234) * float4(geometric_product.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, geometric_product.e1) * float4(geometric_product.group3.yzz, this.e41)) + (float4(this.group4.ww, this.e2, geometric_product.e4) * float4(geometric_product.group3.xyx, this.scalar)) + float4(float3(0.0), -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412) - (geometric_product.e423 * this.e23) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12)) - float4((geometric_product.group3.zxy * this.group1.yzx), geometric_product.e321 * this.e1234) - float4((this.group3.yzx * geometric_product.group1.zxy), geometric_product.e41 * this.e1), 
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group2) + (float3(geometric_product.e1234) * this.group3) + (float3(geometric_product.e321) * this.group4.xyz) + (float3(this.scalar) * geometric_product.group2) + (float3(this.e1234) * geometric_product.group3) + (float3(this.e4, this.e4, this.e431) * geometric_product.group1.xyx) + (float3(this.e412, this.e423, this.e4) * geometric_product.group1.yzz) + (geometric_product.group2.yzx * this.group3.zxy) + (geometric_product.group3.yzx * this.group2.zxy) + (geometric_product.group4.zxy * this.group1.yzx) - (float3(geometric_product.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group4.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group4.xyx) - (geometric_product.group2.zxy * this.group3.yzx) - (geometric_product.group3.zxy * this.group2.yzx) - (geometric_product.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(geometric_product.scalar) * this.group3) + (float3(this.scalar) * geometric_product.group3) + (geometric_product.group3.yzx * this.group3.zxy) + (geometric_product.group1.zxy * this.group1.yzx) - (float3(geometric_product.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group1.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group1.xyx) - (geometric_product.group3.zxy * this.group3.yzx), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e3 * this.e42) + (geometric_product.e4 * this.e23) + (geometric_product.e41 * this.e321) + (geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4) + (geometric_product.e31 * this.e412) + (geometric_product.e423 * this.scalar) + (geometric_product.e431 * this.e12), (geometric_product.e1 * this.e43) + (geometric_product.e4 * this.e31) + (geometric_product.e42 * this.e321) + (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4) + (geometric_product.e12 * this.e423) + (geometric_product.e431 * this.scalar) + (geometric_product.e412 * this.e23), (geometric_product.e2 * this.e41) + (geometric_product.e4 * this.e12) + (geometric_product.e41 * this.e2) + (geometric_product.e43 * this.e321) + (geometric_product.e23 * this.e431) + (geometric_product.e12 * this.e4) + (geometric_product.e423 * this.e31) + (geometric_product.e412 * this.scalar), 0.0) + (float4(geometric_product.scalar) * this.group4) + (float4(geometric_product.group0.yy, geometric_product.e1234, this.scalar) * float4(this.group1.xyz, geometric_product.e321)) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group2.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group2.xxy, this.e23)) - (this.group1.yzxy * float4(geometric_product.group2.zxy, geometric_product.e31)) - (float4(this.group0.yy, this.e1234, geometric_product.e23) * float4(geometric_product.group1.xyz, this.e1)) - float4((geometric_product.group3.zxy * this.group4.yzx), geometric_product.e12 * this.e3) - float4((this.group3.yzx * geometric_product.group4.zxy), geometric_product.e3 * this.e12)
        );
    }
}
extension MultiVector: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> MultiVector {
Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * geometric_product.e321, (this.e1 * geometric_product.e423) + (this.e2 * geometric_product.e431) + (this.e3 * geometric_product.e412) + (this.e4 * geometric_product.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.e321, geometric_product.e321, geometric_product.e321, 1.0) * float4(this.group3, -(this.e1234 * geometric_product.e321) - (this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)), 
            /* e41, e42, e43 */
            (float3(geometric_product.e321) * this.group4.xyz) + (this.group1.yzx * geometric_product.group0.zxy) - (float3(this.e321) * geometric_product.group0.xyz) - (this.group1.zxy * geometric_product.group0.yzx), 
            /* e23, e31, e12 */
            float3(geometric_product.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group0.xyz) + (this.group3.zxy * geometric_product.group0.yzx) - (float3(geometric_product.e321) * this.group2) - (this.group3.yzx * geometric_product.group0.zxy)), this.scalar * geometric_product.e321)
        );
    }
}
extension MultiVector: GeometricQuotient<Point> {
    func geometric_quotient(other: Point) -> MultiVector {
Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)) * other.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * geometric_product.e1) + (this.e2 * geometric_product.e2) + (this.e3 * geometric_product.e3), -(this.e423 * geometric_product.e1) - (this.e431 * geometric_product.e2) - (this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)), 
            /* e1, e2, e3, e4 */
            float4(this.e31 * geometric_product.e3 * -1.0, this.e12 * geometric_product.e1 * -1.0, this.e23 * geometric_product.e2 * -1.0, (this.e42 * geometric_product.e2) + (this.e43 * geometric_product.e3)) + (float4(this.scalar) * geometric_product.group0) + (geometric_product.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * geometric_product.group0.xyz) + (this.group4.zxy * geometric_product.group0.yzx) - (float3(geometric_product.e4) * this.group1.xyz) - (this.group4.yzx * geometric_product.group0.zxy), 
            /* e23, e31, e12 */
            (this.group1.yzx * geometric_product.group0.zxy) - (float3(this.e321) * geometric_product.group0.xyz) - (this.group1.zxy * geometric_product.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4), (this.e43 * geometric_product.e1) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e12 * geometric_product.e4), this.e12 * geometric_product.e3 * -1.0) - (geometric_product.group0.xyzx * float4(this.group0.yy, this.e1234, this.e23)) - (geometric_product.group0.yzxy * float4(this.group2.zxy, this.e31))
        );
    }
}
extension MultiVector: GeometricQuotient<Scalar> {
    func geometric_quotient(other: Scalar) -> MultiVector {
Scalar geometric_product = Scalar.from_groups(
            /* scalar */
            1.0/other.scalar
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.scalar) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(geometric_product.scalar) * this.group2, 
            /* e23, e31, e12 */
            float3(geometric_product.scalar) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(geometric_product.scalar) * this.group4
        );
    }
}
extension Origin {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Origin> {
        geometric_quotient_partial(this)
    }
}
extension Origin: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 / (other.scalar)
        );
    }
}
extension Origin: GeometricQuotient<Flector> {
    func geometric_quotient(other: Flector) -> Motor {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
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
    func geometric_quotient(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 / (other.e321) * -1.0
        );
    }
}
extension Origin: GeometricQuotient<Line> {
    func geometric_quotient(other: Line) -> Plane {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(float3(reverse.e23 * other_2.scalar, reverse.e31 * other_2.scalar, reverse.e12 * other_2.scalar), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Flector {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
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
            float4(this.e4, this.e4, this.e4, 0.0) * float4(geometric_product.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricQuotient<MultiVector> {
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, geometric_product.e321 * this.e4) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product.scalar * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * geometric_product.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(geometric_product.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 / (other.e321) * -1.0
        );
    }
}
extension Origin: GeometricQuotient<Point> {
    func geometric_quotient(other: Point) -> Line {
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * float4(other.e1 * other_2.scalar, other.e2 * other_2.scalar, other.e3 * other_2.scalar, other.e4 * other_2.scalar).xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Origin: GeometricQuotient<Scalar> {
    func geometric_quotient(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 / (other.scalar)
        );
    }
}
extension Plane {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Plane> {
        geometric_quotient_partial(this)
    }
}
extension Plane: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> Flector {
DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.scalar, -2)) * other.group0
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
    func geometric_quotient(other: Flector) -> Motor {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
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
    func geometric_quotient(other: Horizon) -> Motor {
Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product.e321, geometric_product.e321, geometric_product.e321, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product.e321 * this.e321 * -1.0)
        );
    }
}
extension Plane: GeometricQuotient<Line> {
    func geometric_quotient(other: Line) -> Flector {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 1.0) * float4(geometric_product.group1, -(geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * geometric_product.group0), 0.0) + float4((geometric_product.group1.yzx * this.group0.zxy), 0.0) - float4((geometric_product.group1.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Plane: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Flector {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
        );
Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 1.0) * float4(geometric_product.group1.xyz, (geometric_product.e1234 * this.e321) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group0.xyz) + (float3(this.e321) * geometric_product.group0.xyz) + (geometric_product.group1.yzx * this.group0.zxy) - (geometric_product.group1.zxy * this.group0.yzx)), geometric_product.scalar * this.e321)
        );
    }
}
extension Plane: GeometricQuotient<MultiVector> {
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.e321 * this.e321, -(geometric_product.e1 * this.e423) - (geometric_product.e2 * this.e431) - (geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 1.0) * float4(geometric_product.group3, (geometric_product.e1234 * this.e321) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)), 
            /* e41, e42, e43 */
            (float3(geometric_product.e321) * this.group0.xyz) + (geometric_product.group1.yzx * this.group0.zxy) - (float3(this.e321) * geometric_product.group4.xyz) - (geometric_product.group1.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            float3(this.e321) * geometric_product.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group0.xyz) + (float3(this.e321) * geometric_product.group2) + (geometric_product.group3.yzx * this.group0.zxy) - (geometric_product.group3.zxy * this.group0.yzx)), geometric_product.scalar * this.e321)
        );
    }
}
extension Plane: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> Motor {
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
    func geometric_quotient(other: Point) -> Motor {
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
    func geometric_quotient(other: Scalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/other.scalar) * this.group0
        );
    }
}
extension Point {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Point> {
        geometric_quotient_partial(this)
    }
}
extension Point: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> Flector {
DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(other.scalar, -2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_product.group0.yy, geometric_product.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: GeometricQuotient<Flector> {
    func geometric_quotient(other: Flector) -> Motor {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
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
    func geometric_quotient(other: Horizon) -> Motor {
Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_product.e321 * this.e4), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e321, geometric_product.e321, geometric_product.e321, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricQuotient<Line> {
    func geometric_quotient(other: Line) -> Flector {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.e31 * this.e3, geometric_product.e12 * this.e1, geometric_product.e23 * this.e2, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4), (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4), (geometric_product.e41 * this.e2) + (geometric_product.e12 * this.e4), -(geometric_product.e31 * this.e2) - (geometric_product.e12 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23))
        );
    }
}
extension Point: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Flector {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
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
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, geometric_product.e321 * this.e4) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.e31 * this.e3, geometric_product.e12 * this.e1, geometric_product.e23 * this.e2, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3)) + (float4(geometric_product.scalar) * this.group0) - (this.group0.yzxx * float4(geometric_product.group3.zxy, geometric_product.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * geometric_product.group1.xyz) + (geometric_product.group4.zxy * this.group0.yzx) - (float3(geometric_product.e4) * this.group0.xyz) - (geometric_product.group4.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (geometric_product.group1.zxy * this.group0.yzx) - (float3(geometric_product.e321) * this.group0.xyz) - (geometric_product.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e1234 * this.e1) + (geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4), (geometric_product.e1234 * this.e2) + (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4), (geometric_product.e1234 * this.e3) + (geometric_product.e41 * this.e2) + (geometric_product.e12 * this.e4), -(geometric_product.e31 * this.e2) - (geometric_product.e12 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group2.zxy, geometric_product.e23))
        );
    }
}
extension Point: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> Motor {
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
    func geometric_quotient(other: Point) -> Motor {
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
    func geometric_quotient(other: Scalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/other.scalar) * this.group0
        );
    }
}
extension Scalar {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Scalar> {
        geometric_quotient_partial(this)
    }
}
extension Scalar: GeometricQuotient<DualNum> {
    func geometric_quotient(other: DualNum) -> DualNum {
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, -2)
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * float2(other.scalar * other_2.scalar, other.e1234 * other_2.scalar)
        );
    }
}
extension Scalar: GeometricQuotient<Flector> {
    func geometric_quotient(other: Flector) -> Flector {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e321, 2)
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
    func geometric_quotient(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.scalar / (other.e321) * -1.0
        );
    }
}
extension Scalar: GeometricQuotient<Line> {
    func geometric_quotient(other: Line) -> Line {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2)
        );
Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * geometric_product.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_product.group1
        );
    }
}
extension Scalar: GeometricQuotient<Motor> {
    func geometric_quotient(other: Motor) -> Motor {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.scalar, 2)
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
    func geometric_quotient(other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) + powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2) + powi(other.e23, 2) + powi(other.e31, 2) + powi(other.e12, 2) + powi(other.e321, 2)
        );
MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * geometric_product.group0, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * geometric_product.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * geometric_product.group2, 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_product.group3, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * geometric_product.group4
        );
    }
}
extension Scalar: GeometricQuotient<Plane> {
    func geometric_quotient(other: Plane) -> Plane {
Plane reverse = Plane.from_groups(
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e321, -2)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * float4(reverse.e423 * other_2.scalar, reverse.e431 * other_2.scalar, reverse.e412 * other_2.scalar, reverse.e321 * other_2.scalar)
        );
    }
}
extension Scalar: GeometricQuotient<Point> {
    func geometric_quotient(other: Point) -> Point {
Scalar other_2 = Scalar.from_groups(
            /* scalar */
            powi(other.e1, 2) + powi(other.e2, 2) + powi(other.e3, 2)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.e1 * other_2.scalar, other.e2 * other_2.scalar, other.e3 * other_2.scalar, other.e4 * other_2.scalar)
        );
    }
}
extension Scalar: GeometricQuotient<Scalar> {
    func geometric_quotient(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar / (other.scalar)
        );
    }
}
