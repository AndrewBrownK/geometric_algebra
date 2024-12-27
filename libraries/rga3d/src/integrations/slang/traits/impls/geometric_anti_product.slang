extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<AntiScalar> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension AntiScalar: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricAntiProduct<DualNum> {
    associatedtype Output = DualNum;
    func geometric_anti_product(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e1234) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricAntiProduct<Flector> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
extension AntiScalar: GeometricAntiProduct<Horizon> {
    associatedtype Output = Horizon;
    func geometric_anti_product(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e1234 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricAntiProduct<Line> {
    associatedtype Output = Line;
    func geometric_anti_product(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: GeometricAntiProduct<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
    }
}
extension AntiScalar: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e1234) * other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
    }
}
extension AntiScalar: GeometricAntiProduct<Origin> {
    associatedtype Output = Origin;
    func geometric_anti_product(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e1234 * other.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricAntiProduct<Plane> {
    associatedtype Output = Plane;
    func geometric_anti_product(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension AntiScalar: GeometricAntiProduct<Point> {
    associatedtype Output = Point;
    func geometric_anti_product(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
extension AntiScalar: GeometricAntiProduct<Scalar> {
    associatedtype Output = Scalar;
    func geometric_anti_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<DualNum> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension DualNum: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = DualNum;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e1234) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: GeometricAntiProduct<DualNum> {
    associatedtype Output = DualNum;
    func geometric_anti_product(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 0.0, 0.0)
        );
    }
}
extension DualNum: GeometricAntiProduct<Flector> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(this.e1234) * other.group0.xyz)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321)) * float4(other.group1.xyz, 1.0)
        );
    }
}
extension DualNum: GeometricAntiProduct<Horizon> {
    associatedtype Output = Horizon;
    func geometric_anti_product(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e1234 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: GeometricAntiProduct<Line> {
    associatedtype Output = Line;
    func geometric_anti_product(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz), 0.0)
        );
    }
}
extension DualNum: GeometricAntiProduct<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1)
        );
    }
}
extension DualNum: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group4.xyz) + (float3(this.e1234) * other.group1.xyz)), this.e1234 * other.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321)) * float4(other.group4.xyz, 1.0)
        );
    }
}
extension DualNum: GeometricAntiProduct<Origin> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        );
    }
}
extension DualNum: GeometricAntiProduct<Plane> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension DualNum: GeometricAntiProduct<Point> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        );
    }
}
extension DualNum: GeometricAntiProduct<Scalar> {
    associatedtype Output = Scalar;
    func geometric_anti_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<Flector> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension Flector: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
extension Flector: GeometricAntiProduct<DualNum> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) - (float3(other.scalar) * this.group1.xyz)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xy.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4)) * float4(this.group1.xyz, 1.0)
        );
    }
}
extension Flector: GeometricAntiProduct<Flector> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e423 * this.e4) - (other.e412 * this.e431), -(other.e423 * this.e412) - (other.e431 * this.e4), -(other.e431 * this.e423) - (other.e412 * this.e4), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group1.zxyx) - (float4(other.e4) * float4(this.group1.xyz, this.e4)), 
            /* e23, e31, e12, scalar */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (float4(this.e4, this.e412, this.e423, this.e1) * float4(other.group0.xxy, other.e423)) + (float4(this.e431, this.e4, this.e4, this.e2) * float4(other.group0.zyz, other.e431)) + (other.group1.yzxz * this.group0.zxyz) - (float4(this.e2, this.e321, this.e321, this.e321) * float4(other.group1.zyz, other.e4)) - (float4(this.e321, this.e3, this.e1, this.e412) * float4(other.group1.xxy, other.e3)) - (other.group0.yzxx * this.group1.zxyx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431))
        );
    }
}
extension Flector: GeometricAntiProduct<Horizon> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Horizon) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
    }
}
extension Flector: GeometricAntiProduct<Line> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xyz.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.xyz.zyz, other.e43)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e423 * other.e41), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xyz.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.xyz.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group0.xyz.yzx * this.group1.zxy), this.e1 * other.e41)
        );
    }
}
extension Flector: GeometricAntiProduct<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e412 * other.e23), (this.e3 * other.e1234) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e412 * other.scalar), 0.0) + (this.group0.xyxw * other.group0.wwyw) - (this.group1.xyxz * float4(other.group1.wwy, other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e423)) - float4((this.group0.www * other.group1.xyz), this.e431 * other.e42), 
            /* e423, e431, e412, e321 */
            (this.group1.xyxy * float4(other.group0.wwy, other.e31)) + (this.group1.yzzz * float4(other.group0.zxw, other.e12)) + float4(float3(0.0), (this.e321 * other.e1234) - (this.e2 * other.e42) - (this.e3 * other.e43) - (this.e4 * other.scalar)) + float4((this.group0.www * other.group0.xyz), this.e423 * other.e23) - (other.group0.yzxx * float4(this.group1.zxy, this.e1))
        );
    }
}
extension Flector: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group1.xw), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e4 * other.e31), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e4 * other.e12), 0.0) + (float4(other.e1234) * this.group0) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(other.group3.xyz.xxy, other.e43)) - (this.group1.xyzx * float4(other.group0.xy.xx, other.scalar, other.e41)) - float4((other.group2.xyz.yzx * this.group0.zxy), this.e431 * other.e42), 
            /* e41, e42, e43, 0 */
            float4((this.group1.zxy * other.group4.yzx) - (float3(this.e4) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group1.xyx) - (float3(other.e412, other.e423, other.e4) * this.group1.yzz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (float3(other.e3, other.e1, other.e321) * this.group1.yzz) + (float3(other.e321, other.e321, other.e2) * this.group1.xyx) + (this.group0.zxy * other.group4.yzx) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group0.xyx) - (float3(other.e412, other.e423, other.e4) * this.group0.yzz) - (this.group1.zxy * other.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group1) + (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group2.xyz.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group2.xyz.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group2.xyz.yzx * this.group1.zxy), this.e4 * other.scalar)
        );
    }
}
extension Flector: GeometricAntiProduct<Origin> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Origin) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
extension Flector: GeometricAntiProduct<Plane> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e4 * other.e423) - (this.e431 * other.e412), -(this.e4 * other.e431) - (this.e412 * other.e423), -(this.e4 * other.e412) - (this.e423 * other.e431), (this.e431 * other.e431) + (this.e412 * other.e412)) + (this.group1.zxyx * other.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(this.e2 * other.e412) - (this.e321 * other.e423), -(this.e3 * other.e423) - (this.e321 * other.e431), -(this.e1 * other.e431) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.zxyx * other.group0.yzxx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2))
        );
    }
}
extension Flector: GeometricAntiProduct<Point> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((this.e4 * other.e1) + (this.e431 * other.e3), (this.e4 * other.e2) + (this.e412 * other.e1), (this.e4 * other.e3) + (this.e423 * other.e2), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.zxyy * other.group0.yzxy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423))
        );
    }
}
extension Flector: GeometricAntiProduct<Scalar> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Scalar) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<Horizon> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension Horizon: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = Horizon;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(other.e1234 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: GeometricAntiProduct<DualNum> {
    associatedtype Output = Horizon;
    func geometric_anti_product(other: DualNum) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(other.e1234 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: GeometricAntiProduct<Flector> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
    }
}
extension Horizon: GeometricAntiProduct<Line> {
    associatedtype Output = Point;
    func geometric_anti_product(other: Line) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
extension Horizon: GeometricAntiProduct<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
extension Horizon: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group2.xyz), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * other.group4.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
extension Horizon: GeometricAntiProduct<Origin> {
    associatedtype Output = Scalar;
    func geometric_anti_product(other: Origin) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: GeometricAntiProduct<Plane> {
    associatedtype Output = Line;
    func geometric_anti_product(other: Plane) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * other.group0.xyz * float3(-1.0), 0.0)
        );
    }
}
extension Horizon: GeometricAntiProduct<Point> {
    associatedtype Output = Scalar;
    func geometric_anti_product(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<Line> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension Line: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = Line;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e1234) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: GeometricAntiProduct<DualNum> {
    associatedtype Output = Line;
    func geometric_anti_product(other: DualNum) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz), 0.0)
        );
    }
}
extension Line: GeometricAntiProduct<Flector> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) - (other.group1.yzxy * float4(this.group1.xyz.zxy, this.e42)) - float4((this.group0.xyz.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e41) + (other.e412 * this.e42), (other.e4 * this.e42) + (other.e423 * this.e43), (other.e4 * this.e43) + (other.e431 * this.e41), -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group0.xyz.zxy * other.group1.yzx), other.e1 * this.e41)
        );
    }
}
extension Line: GeometricAntiProduct<Horizon> {
    associatedtype Output = Point;
    func geometric_anti_product(other: Horizon) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0)
        );
    }
}
extension Line: GeometricAntiProduct<Line> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e43 * this.e42, other.e41 * this.e43, other.e42 * this.e41, -(other.e42 * this.e42) - (other.e43 * this.e43)) - float4((other.group0.xyz.yzx * this.group0.xyz.zxy), other.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((other.e43 * this.e31) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e23 * this.e43), (other.e42 * this.e23) + (other.e31 * this.e41), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e41 * this.e23) - float4((other.group1.xyz.yzx * this.group0.xyz.zxy), other.e42 * this.e31)
        );
    }
}
extension Line: GeometricAntiProduct<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.e1234) + (this.e42 * other.e43), (this.e42 * other.e1234) + (this.e43 * other.e41), (this.e41 * other.e42) + (this.e43 * other.e1234), -(this.e42 * other.e42) - (this.e43 * other.e43)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e41)), 
            /* e23, e31, e12, scalar */
            float4((this.e41 * other.scalar) + (this.e42 * other.e12) + (this.e23 * other.e1234) + (this.e31 * other.e43), (this.e42 * other.scalar) + (this.e43 * other.e23) + (this.e31 * other.e1234) + (this.e12 * other.e41), (this.e41 * other.e31) + (this.e43 * other.scalar) + (this.e23 * other.e42) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e41)) - float4((this.group1.xyz.zxy * other.group0.yzx), this.e42 * other.e31)
        );
    }
}
extension Line: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e42 * other.e3) + (this.e23 * other.e4) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e43 * other.e1) + (this.e31 * other.e4) + (this.e12 * other.e423), (this.e41 * other.e2) + (this.e43 * other.e321) + (this.e23 * other.e431) + (this.e12 * other.e4), this.e43 * other.e412 * -1.0) - (other.group4.yzxy * float4(this.group1.xyz.zxy, this.e42)) - float4((this.group0.xyz.zxy * other.group1.yzx), this.e41 * other.e423), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group0.xyz) + (this.group0.xyz.yzx * other.group2.xyz.zxy) - (this.group0.xyz.zxy * other.group2.xyz.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.xyz.zxy) + (this.group1.xyz.yzx * other.group2.xyz.zxy) - (this.group0.xyz.zxy * other.group3.xyz.yzx) - (this.group1.xyz.zxy * other.group2.xyz.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e41 * other.e4) + (this.e42 * other.e412), (this.e42 * other.e4) + (this.e43 * other.e423), (this.e41 * other.e431) + (this.e43 * other.e4), -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group0.xyz.zxy * other.group4.yzx), this.e41 * other.e1)
        );
    }
}
extension Line: GeometricAntiProduct<Origin> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), 0.0)
        );
    }
}
extension Line: GeometricAntiProduct<Plane> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension Line: GeometricAntiProduct<Point> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.xyz.yzx * other.group0.zxy), 0.0) - float4((this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), -(this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        );
    }
}
extension Line: GeometricAntiProduct<Scalar> {
    associatedtype Output = Line;
    func geometric_anti_product(other: Scalar) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<Motor> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension Motor: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        );
    }
}
extension Motor: GeometricAntiProduct<DualNum> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1)
        );
    }
}
extension Motor: GeometricAntiProduct<Flector> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e431 * this.scalar) + (other.e321 * this.e42), (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e412 * this.scalar) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (other.group0.xxyw * this.group0.wzxw) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - (this.group0.zxyx * float4(other.group0.yzx, other.e423)), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e431 * this.e1234, other.e412 * this.e1234, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(other.e4) * float4(this.group0.xyz, this.scalar)) + (other.group1.xxyw * this.group0.wzxw) - (this.group0.zxyx * float4(other.group1.yzx, other.e1))
        );
    }
}
extension Motor: GeometricAntiProduct<Horizon> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
extension Motor: GeometricAntiProduct<Line> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.e1234) + (other.e43 * this.e42), (other.e41 * this.e43) + (other.e42 * this.e1234), (other.e42 * this.e41) + (other.e43 * this.e1234), -(other.e42 * this.e42) - (other.e43 * this.e43)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e41)), 
            /* e23, e31, e12, scalar */
            float4((other.e41 * this.scalar) + (other.e43 * this.e31) + (other.e23 * this.e1234) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e42 * this.scalar) + (other.e23 * this.e43) + (other.e31 * this.e1234), (other.e42 * this.e23) + (other.e43 * this.scalar) + (other.e31 * this.e41) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.zxyx * float4(other.group0.xyz.yzx, other.e41)) - float4((other.group1.xyz.yzx * this.group0.zxy), other.e42 * this.e31)
        );
    }
}
extension Motor: GeometricAntiProduct<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e43 * this.e42) + (other.e1234 * this.e41), (other.e42 * this.e1234) + (other.e1234 * this.e42), (other.e43 * this.e1234) + (other.e1234 * this.e43), -(other.e42 * this.e42) - (other.e43 * this.e43)) + (other.group0.xxyw * this.group0.wzxw) - (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e12 * this.e42) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e23 * this.e43) + (other.e31 * this.e1234) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e31 * this.e41) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xxyw * this.group1.wzxw) + float4((other.group0.zyz * this.group1.yww), other.scalar * this.e1234) - (other.group0.yzxx * this.group1.zxyx) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        );
    }
}
extension Motor: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12), 0.0) + (float2(this.e1234) * other.group0.xy) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e42 * other.e3) + (this.e1234 * other.e1) + (this.e23 * other.e4) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e43 * other.e1) + (this.e1234 * other.e2) + (this.e31 * other.e4) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e1234 * other.e3) + (this.e23 * other.e431) + (this.e12 * other.e4) + (this.scalar * other.e412), this.e43 * other.e412 * -1.0) + (this.group0.xyxw * float4(other.group4.ww, other.e2, other.e4)) - (this.group0.zxyx * float4(other.group1.yzx, other.e423)) - (other.group4.yzxy * float4(this.group1.zxy, this.e42)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group0.xyz) + (other.group2.xyz.xxy * this.group0.wzx) + (other.group2.xyz.zyz * this.group0.yww) - (other.group2.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xyz.xxy * this.group1.wzx) + (other.group2.xyz.zyz * this.group1.yww) + (other.group3.xyz.xxy * this.group0.wzx) + (other.group3.xyz.zyz * this.group0.yww) - (other.group2.xyz.yzx * this.group1.zxy) - (other.group3.xyz.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.xyxw * float4(other.group1.ww, other.e431, other.e321)) + (float4(other.group4.zx, other.e4, other.e4) * float4(this.group0.yzz, this.scalar)) - (this.group0.zxyx * float4(other.group4.yzx, other.e1))
        );
    }
}
extension Motor: GeometricAntiProduct<Origin> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group1.xyz, this.e1234), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group0.xyz, this.scalar)
        );
    }
}
extension Motor: GeometricAntiProduct<Plane> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e42 * other.e321) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e23 * other.e431) + (this.scalar * other.e412), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.yzxw * other.group0.zxyw) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Motor: GeometricAntiProduct<Point> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        );
    }
}
extension Motor: GeometricAntiProduct<Scalar> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Scalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<MultiVector> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension MultiVector: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e1234) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e1234) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
    }
}
extension MultiVector: GeometricAntiProduct<DualNum> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group1.xyz) - (float3(other.scalar) * this.group4.xyz)), other.e1234 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xy.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4)) * float4(this.group4.xyz, 1.0)
        );
    }
}
extension MultiVector: GeometricAntiProduct<Flector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group0.xw), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1234) + (other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (float4(this.group0.xy.xx, this.scalar, this.e1234) * float4(other.group1.xyz, other.e4)) - (other.group1.yzxy * float4(this.group3.xyz.zxy, this.e42)) - float4((this.group2.xyz.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e41, e42, e43, 0 */
            float4((other.group1.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group1.xxy) - (float3(this.e431, this.e4, this.e4) * other.group1.zyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e412, this.e423) * other.group0.xxy) + (float3(this.e431, this.e4, this.e4) * other.group0.zyz) + (other.group1.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group1.zyz) - (float3(this.e321, this.e3, this.e1) * other.group1.xxy) - (other.group0.yzx * this.group4.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e423 * this.e43, other.e4 * this.e43, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(other.group1.xyz, other.e4)) + (float4(other.group0.ww, other.e431, other.e321) * float4(this.group2.xyz.xyx, this.e1234)) - float4((this.group2.xyz.zxy * other.group1.yzx), other.e1 * this.e41)
        );
    }
}
extension MultiVector: GeometricAntiProduct<Horizon> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group2.xyz), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e321) * this.group4.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
extension MultiVector: GeometricAntiProduct<Line> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412), (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423), (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xyz.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.xyz.zyz, other.e43)) - float4((other.group0.xyz.yzx * this.group1.zxy), other.e41 * this.e423), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group0.xyz) + (other.group0.xyz.zxy * this.group2.xyz.yzx) - (other.group0.xyz.yzx * this.group2.xyz.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.xyz.yzx) + (other.group1.xyz.zxy * this.group2.xyz.yzx) - (other.group0.xyz.yzx * this.group3.xyz.zxy) - (other.group1.xyz.yzx * this.group2.xyz.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xyz.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.xyz.zyz, other.e31)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3)) - float4((other.group0.xyz.yzx * this.group4.zxy), other.e41 * this.e1)
        );
    }
}
extension MultiVector: GeometricAntiProduct<Motor> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12), 0.0) + (float2(other.e1234) * this.group0.xy) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e43 * this.e2) + (other.e1234 * this.e1) + (other.e31 * this.e412) - (other.scalar * this.e423), (other.e42 * this.e321) + (other.e1234 * this.e2) + (other.e12 * this.e423) - (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) - (other.scalar * this.e412), 0.0) + (float4(this.e321, this.e3, this.e1, this.e4) * other.group0.xxyw) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.zyz, other.e43)) - (other.group0.yzxx * float4(this.group1.zxy, this.e423)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group0.xyz) + (this.group2.xyz.xyx * other.group0.wwy) + (this.group2.xyz.yzz * other.group0.zxw) - (this.group2.xyz.zxy * other.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xyz.xyx * other.group1.wwy) + (this.group2.xyz.yzz * other.group1.zxw) + (this.group3.xyz.xyx * other.group0.wwy) + (this.group3.xyz.yzz * other.group0.zxw) - (this.group2.xyz.zxy * other.group1.yzx) - (this.group3.xyz.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e321) * other.group0.xxyw) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(other.group0.zyz, other.e23)) + (this.group4.xyzy * float4(other.group0.www, other.e31)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4)) - (other.group0.yzxx * float4(this.group4.zxy, this.e1))
        );
    }
}
extension MultiVector: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * other.group0.xy) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * other.group1.xw), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1234) + (other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), 0.0) + (float4(other.e1234) * this.group1) + (float4(this.group0.xy.xx, this.scalar, this.e1234) * float4(other.group4.xyz, other.e4)) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(other.group3.xyz.xxy, other.e43)) - (float4(this.e431, this.e4, this.e4, other.e423) * float4(other.group3.xyz.zyz, this.e41)) - (other.group4.yzxz * float4(this.group3.xyz.zxy, this.e43)) - (this.group4.xyzx * float4(other.group0.xy.xx, other.scalar, other.e41)) - float4((other.group2.xyz.yzx * this.group1.zxy), other.e42 * this.e431) - float4((this.group2.xyz.zxy * other.group1.yzx), other.e431 * this.e42), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group2.xyz) + (other.group2.xyz.zxy * this.group2.xyz.yzx) + (other.group4.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group4.xxy) - (float3(this.e431, this.e4, this.e4) * other.group4.zyz) - (other.group2.xyz.yzx * this.group2.xyz.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz) + (float3(this.e4, this.e412, this.e423) * other.group1.xxy) + (float3(this.e431, this.e4, this.e4) * other.group1.zyz) + (other.group2.xyz.zxy * this.group3.xyz.yzx) + (other.group3.xyz.zxy * this.group2.xyz.yzx) + (other.group4.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group4.zyz) - (float3(this.e321, this.e3, this.e1) * other.group4.xxy) - (other.group2.xyz.yzx * this.group3.xyz.zxy) - (other.group3.xyz.yzx * this.group2.xyz.zxy) - (other.group1.yzx * this.group4.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, other.e321) * float4(other.group2.xyz.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(other.group2.xyz.zyz, other.e23)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(other.group4.xyz, other.e4)) + (float4(other.group1.ww, other.e431, this.e431) * float4(this.group2.xyz.xyx, other.e31)) + (float4(other.group4.zx, other.e4, this.e412) * float4(this.group2.xyz.yzz, other.e12)) + float4(float3(0.0), -(other.e1 * this.e41) - (other.e2 * this.e42) - (other.e3 * this.e43) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((other.group2.xyz.yzx * this.group4.zxy), other.scalar * this.e4) - float4((this.group2.xyz.zxy * other.group4.yzx), other.e41 * this.e1)
        );
    }
}
extension MultiVector: GeometricAntiProduct<Origin> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e4) * float2(this.e321, this.e4) * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group3.xyz, this.e1234), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group2.xyz, this.scalar)
        );
    }
}
extension MultiVector: GeometricAntiProduct<Plane> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e4 * other.e321, 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.scalar * other.e423) + (this.e41 * other.e321) + (this.e31 * other.e412), (this.scalar * other.e431) + (this.e42 * other.e321) + (this.e12 * other.e423), (this.scalar * other.e412) + (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((this.group4.zxy * other.group0.yzx) - (float3(this.e4) * other.group0.xyz) - (this.group4.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e321) * this.group4.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(this.e321) * other.group0.xyz) - (this.group1.yzx * other.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (float4(this.e1234) * other.group0) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
    }
}
extension MultiVector: GeometricAntiProduct<Point> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), this.e4 * other.e4) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group3.xyz) + (this.group2.xyz.yzx * other.group0.zxy) - (this.group2.xyz.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e4) * other.group0.xyz) + (this.group4.yzx * other.group0.zxy) - (float3(other.e4) * this.group1.xyz) - (this.group4.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group2.xyz), (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        );
    }
}
extension MultiVector: GeometricAntiProduct<Scalar> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e1234 * other.scalar, 1.0) * float2(1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group4.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group2.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<Origin> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension Origin: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = Origin;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e1234 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: GeometricAntiProduct<DualNum> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        );
    }
}
extension Origin: GeometricAntiProduct<Flector> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e4) * float4(other.group0.xyz, other.e321)
        );
    }
}
extension Origin: GeometricAntiProduct<Horizon> {
    associatedtype Output = Scalar;
    func geometric_anti_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: GeometricAntiProduct<Line> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e4) * other.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        );
    }
}
extension Origin: GeometricAntiProduct<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group1.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group0.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Origin: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e4) * float2(other.e321, other.e4) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e4) * other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group2.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Origin: GeometricAntiProduct<Origin> {
    associatedtype Output = AntiScalar;
    func geometric_anti_product(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.e4 * this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: GeometricAntiProduct<Plane> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e4) * other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e4 * other.e321)
        );
    }
}
extension Origin: GeometricAntiProduct<Point> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        );
    }
}
extension Origin: GeometricAntiProduct<Scalar> {
    associatedtype Output = Horizon;
    func geometric_anti_product(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e4 * other.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<Plane> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension Plane: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = Plane;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: GeometricAntiProduct<DualNum> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: GeometricAntiProduct<Flector> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e423) - (other.e412 * this.e431), -(other.e4 * this.e431) - (other.e423 * this.e412), -(other.e4 * this.e412) - (other.e431 * this.e423), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e3 * this.e431) + (other.e321 * this.e423), (other.e1 * this.e412) + (other.e321 * this.e431), (other.e2 * this.e423) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.yzxx * this.group0.zxyx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2))
        );
    }
}
extension Plane: GeometricAntiProduct<Horizon> {
    associatedtype Output = Line;
    func geometric_anti_product(other: Horizon) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e321) * this.group0.xyz, 0.0)
        );
    }
}
extension Plane: GeometricAntiProduct<Line> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension Plane: GeometricAntiProduct<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzy * float4(other.group1.www, other.e42)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (other.group0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(other.group0.www, other.e23))
        );
    }
}
extension Plane: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzx * float4(other.group0.xy.xx, other.scalar, other.e41)) - (this.group0.yzxy * float4(other.group3.xyz.zxy, other.e42)), 
            /* e41, e42, e43, 0 */
            float4((other.group4.yzx * this.group0.zxy) - (float3(other.e4) * this.group0.xyz) - (other.group4.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e321) * this.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (float3(this.e321) * other.group4.xyz) - (other.group1.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (float4(other.e1234) * this.group0) + (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        );
    }
}
extension Plane: GeometricAntiProduct<Origin> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Origin) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e4 * this.e321 * -1.0)
        );
    }
}
extension Plane: GeometricAntiProduct<Plane> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e412 * this.e431 * -1.0, other.e423 * this.e412 * -1.0, other.e431 * this.e423 * -1.0, (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0)
        );
    }
}
extension Plane: GeometricAntiProduct<Point> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e431 * other.e3, this.e412 * other.e1, this.e423 * other.e2, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.zxyx * other.group0.yzxx)
        );
    }
}
extension Plane: GeometricAntiProduct<Scalar> {
    associatedtype Output = Point;
    func geometric_anti_product(other: Scalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<Point> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension Point: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = Point;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Point: GeometricAntiProduct<DualNum> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        );
    }
}
extension Point: GeometricAntiProduct<Flector> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(-(other.e4 * this.e1) - (other.e412 * this.e2), -(other.e4 * this.e2) - (other.e423 * this.e3), -(other.e4 * this.e3) - (other.e431 * this.e1), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423))
        );
    }
}
extension Point: GeometricAntiProduct<Horizon> {
    associatedtype Output = Scalar;
    func geometric_anti_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Point: GeometricAntiProduct<Line> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * other.group1.xyz), 0.0) - float4((other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        );
    }
}
extension Point: GeometricAntiProduct<Motor> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group0.zxy * this.group0.yzx) - (float3(this.e4) * other.group1.xyz) - (other.group0.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4))
        );
    }
}
extension Point: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), other.e4 * this.e4) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group2.xyz.zxy * this.group0.yzx) - (float3(this.e4) * other.group3.xyz) - (other.group2.xyz.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group4.yzx * this.group0.zxy) - (float3(other.e4) * this.group0.xyz) - (other.group4.zxy * this.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group2.xyz), -(other.scalar * this.e4) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        );
    }
}
extension Point: GeometricAntiProduct<Origin> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Origin) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension Point: GeometricAntiProduct<Plane> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e412 * this.e2 * -1.0, other.e423 * this.e3 * -1.0, other.e431 * this.e1 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.yzxx * this.group0.zxyx)
        );
    }
}
extension Point: GeometricAntiProduct<Point> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0)
        );
    }
}
extension Point: GeometricAntiProduct<Scalar> {
    associatedtype Output = Horizon;
    func geometric_anti_product(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e4 * other.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: GeometricAntiProductInfix) -> GeometricAntiProductInfixPartial<Scalar> {
        return GeometricAntiProductInfixPartial { a: this };
    }
}
extension Scalar: GeometricAntiProduct<AntiScalar> {
    associatedtype Output = Scalar;
    func geometric_anti_product(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: GeometricAntiProduct<DualNum> {
    associatedtype Output = Scalar;
    func geometric_anti_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: GeometricAntiProduct<Flector> {
    associatedtype Output = Flector;
    func geometric_anti_product(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        );
    }
}
extension Scalar: GeometricAntiProduct<Line> {
    associatedtype Output = Line;
    func geometric_anti_product(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0)
        );
    }
}
extension Scalar: GeometricAntiProduct<Motor> {
    associatedtype Output = Motor;
    func geometric_anti_product(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: GeometricAntiProduct<MultiVector> {
    associatedtype Output = MultiVector;
    func geometric_anti_product(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e1234 * this.scalar, 1.0) * float2(1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group4.xyz), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group2.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        );
    }
}
extension Scalar: GeometricAntiProduct<Origin> {
    associatedtype Output = Horizon;
    func geometric_anti_product(other: Origin) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(other.e4 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: GeometricAntiProduct<Plane> {
    associatedtype Output = Point;
    func geometric_anti_product(other: Plane) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        );
    }
}
extension Scalar: GeometricAntiProduct<Point> {
    associatedtype Output = Horizon;
    func geometric_anti_product(other: Point) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(other.e4 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
