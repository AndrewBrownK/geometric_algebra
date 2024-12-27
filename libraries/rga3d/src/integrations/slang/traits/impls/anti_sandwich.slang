using traits::AntiReverse;
using traits::GeometricAntiProduct;
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<AntiScalar> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension AntiScalar: AntiSandwich<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_sandwich(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<DualNum> {
    associatedtype Output = DualNum;
    func anti_sandwich(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Horizon> {
    associatedtype Output = Horizon;
    func anti_sandwich(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Line> {
    associatedtype Output = Line;
    func anti_sandwich(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0, 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Origin> {
    associatedtype Output = Origin;
    func anti_sandwich(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Plane> {
    associatedtype Output = Plane;
    func anti_sandwich(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Point> {
    associatedtype Output = Point;
    func anti_sandwich(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Scalar> {
    associatedtype Output = Scalar;
    func anti_sandwich(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<DualNum> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension DualNum: AntiSandwich<AntiScalar> {
    associatedtype Output = DualNum;
    func anti_sandwich(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<DualNum> {
    associatedtype Output = DualNum;
    func anti_sandwich(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(this.e1234) * other.group0.xyz)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(other.e423, other.e431, other.e412, 1.0) * float4(this.group0.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Horizon> {
    associatedtype Output = Horizon;
    func anti_sandwich(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Line> {
    associatedtype Output = Line;
    func anti_sandwich(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0) + (float3(this.e1234) * other.group1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group4.xyz) + (float3(this.e1234) * other.group1.xyz)), this.e1234 * other.e4), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group2) + (float3(this.e1234) * other.group3), 
            /* e423, e431, e412, e321 */
            float4(other.e423, other.e431, other.e412, 1.0) * float4(this.group0.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Origin> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Plane> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Point> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Scalar> {
    associatedtype Output = Scalar;
    func anti_sandwich(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<Flector> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension Flector: AntiSandwich<AntiScalar> {
    associatedtype Output = Motor;
    func anti_sandwich(other: AntiScalar) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<DualNum> {
    associatedtype Output = Motor;
    func anti_sandwich(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) - (float3(other.scalar) * this.group1.xyz)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 1.0) * float4(other.group0.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e423 * this.e4) - (other.e412 * this.e431), -(other.e423 * this.e412) - (other.e431 * this.e4), -(other.e431 * this.e423) - (other.e412 * this.e4), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group1.zxyx) - (float4(other.e4) * float4(this.group1.xyz, this.e4)), 
            /* e23, e31, e12, scalar */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (float4(this.e4, this.e412, this.e423, this.e1) * float4(other.group0.xxy, other.e423)) + (float4(this.e431, this.e4, this.e4, this.e2) * float4(other.group0.zyz, other.e431)) + (other.group1.yzxz * this.group0.zxyz) - (float4(this.e2, this.e321, this.e321, this.e321) * float4(other.group1.zyz, other.e4)) - (float4(this.e321, this.e3, this.e1, this.e412) * float4(other.group1.xxy, other.e3)) - (other.group0.yzxx * this.group1.zxyx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<Horizon> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Horizon) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<Line> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.zyz, other.e43)) - float4((other.group0.yzx * this.group0.zxy), this.e423 * other.e41), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group0.yzx * this.group1.zxy), this.e1 * other.e41)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e412 * other.e23), (this.e3 * other.e1234) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e412 * other.scalar), 0.0) + (this.group0.xyxw * other.group0.wwyw) - (this.group1.xyxz * float4(other.group1.wwy, other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e423)) - float4((this.group0.www * other.group1.xyz), this.e431 * other.e42), 
            /* e423, e431, e412, e321 */
            (this.group1.xyxy * float4(other.group0.wwy, other.e31)) + (this.group1.yzzz * float4(other.group0.zxw, other.e12)) + float4(float3(0.0), (this.e321 * other.e1234) - (this.e2 * other.e42) - (this.e3 * other.e43) - (this.e4 * other.scalar)) + float4((this.group0.www * other.group0.xyz), this.e423 * other.e23) - (other.group0.yzxx * float4(this.group1.zxy, this.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e4 * other.e31), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e4 * other.e12), 0.0) + (float4(other.e1234) * this.group0) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(other.group3.xxy, other.e43)) - (this.group1.xyzx * float4(other.group0.xx, other.scalar, other.e41)) - float4((other.group2.yzx * this.group0.zxy), this.e431 * other.e42), 
            /* e41, e42, e43 */
            (this.group1.zxy * other.group4.yzx) - (float3(this.e4) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group1.xyx) - (float3(other.e412, other.e423, other.e4) * this.group1.yzz), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.e3, other.e1, other.e321) * this.group1.yzz) + (float3(other.e321, other.e321, other.e2) * this.group1.xyx) + (this.group0.zxy * other.group4.yzx) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group0.xyx) - (float3(other.e412, other.e423, other.e4) * this.group0.yzz) - (this.group1.zxy * other.group1.yzx), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group1) + (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group2.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group2.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group2.yzx * this.group1.zxy), this.e4 * other.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<Origin> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Origin) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<Plane> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e4 * other.e423) - (this.e431 * other.e412), -(this.e4 * other.e431) - (this.e412 * other.e423), -(this.e4 * other.e412) - (this.e423 * other.e431), (this.e431 * other.e431) + (this.e412 * other.e412)) + (this.group1.zxyx * other.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(this.e2 * other.e412) - (this.e321 * other.e423), -(this.e3 * other.e423) - (this.e321 * other.e431), -(this.e1 * other.e431) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.zxyx * other.group0.yzxx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<Point> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((this.e4 * other.e1) + (this.e431 * other.e3), (this.e4 * other.e2) + (this.e412 * other.e1), (this.e4 * other.e3) + (this.e423 * other.e2), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.zxyy * other.group0.yzxy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Flector: AntiSandwich<Scalar> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Scalar) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar, other.scalar, other.scalar, 0.0) * float4(this.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<Horizon> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension Horizon: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Horizon: AntiSandwich<Line> {
    associatedtype Output = Scalar;
    func anti_sandwich(other: Line) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Horizon: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Horizon: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group2, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Horizon: AntiSandwich<Plane> {
    associatedtype Output = Point;
    func anti_sandwich(other: Plane) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<Line> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension Line: AntiSandwich<AntiScalar> {
    associatedtype Output = Motor;
    func anti_sandwich(other: AntiScalar) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<DualNum> {
    associatedtype Output = Motor;
    func anti_sandwich(other: DualNum) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0) + (float3(other.e1234) * this.group1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group0.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e41) + (other.e412 * this.e42), (other.e4 * this.e42) + (other.e423 * this.e43), (other.e4 * this.e43) + (other.e431 * this.e41), -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group0.zxy * other.group1.yzx), other.e1 * this.e41)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<Horizon> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Horizon) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<Line> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e43 * this.e42, other.e41 * this.e43, other.e42 * this.e41, -(other.e42 * this.e42) - (other.e43 * this.e43)) - float4((other.group0.yzx * this.group0.zxy), other.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((other.e43 * this.e31) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e23 * this.e43), (other.e42 * this.e23) + (other.e31 * this.e41), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e23) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.e1234) + (this.e42 * other.e43), (this.e42 * other.e1234) + (this.e43 * other.e41), (this.e41 * other.e42) + (this.e43 * other.e1234), -(this.e42 * other.e42) - (this.e43 * other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e41)), 
            /* e23, e31, e12, scalar */
            float4((this.e41 * other.scalar) + (this.e42 * other.e12) + (this.e23 * other.e1234) + (this.e31 * other.e43), (this.e42 * other.scalar) + (this.e43 * other.e23) + (this.e31 * other.e1234) + (this.e12 * other.e41), (this.e41 * other.e31) + (this.e43 * other.scalar) + (this.e23 * other.e42) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.yzxx * float4(this.group0.zxy, this.e41)) - float4((this.group1.zxy * other.group0.yzx), this.e42 * other.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e42 * other.e3) + (this.e23 * other.e4) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e43 * other.e1) + (this.e31 * other.e4) + (this.e12 * other.e423), (this.e41 * other.e2) + (this.e43 * other.e321) + (this.e23 * other.e431) + (this.e12 * other.e4), this.e43 * other.e412 * -1.0) - (other.group4.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group0.zxy * other.group1.yzx), this.e41 * other.e423), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group0) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0) + (float3(other.e1234) * this.group1) + (this.group0.yzx * other.group3.zxy) + (this.group1.yzx * other.group2.zxy) - (this.group0.zxy * other.group3.yzx) - (this.group1.zxy * other.group2.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e41 * other.e4) + (this.e42 * other.e412), (this.e42 * other.e4) + (this.e43 * other.e423), (this.e41 * other.e431) + (this.e43 * other.e4), -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group0.zxy * other.group4.yzx), this.e41 * other.e1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<Origin> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<Plane> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<Point> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - float4((this.group0.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 1.0) * float4(this.group0, -(this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Line: AntiSandwich<Scalar> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Scalar) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<Motor> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension Motor: AntiSandwich<AntiScalar> {
    associatedtype Output = Motor;
    func anti_sandwich(other: AntiScalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<DualNum> {
    associatedtype Output = Motor;
    func anti_sandwich(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e431 * this.scalar) + (other.e321 * this.e42), (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e412 * this.scalar) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (other.group0.xxyw * this.group0.wzxw) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - (this.group0.zxyx * float4(other.group0.yzx, other.e423)), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e431 * this.e1234, other.e412 * this.e1234, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(other.e4) * float4(this.group0.xyz, this.scalar)) + (other.group1.xxyw * this.group0.wzxw) - (this.group0.zxyx * float4(other.group1.yzx, other.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<Horizon> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<Line> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.e1234) + (other.e43 * this.e42), (other.e41 * this.e43) + (other.e42 * this.e1234), (other.e42 * this.e41) + (other.e43 * this.e1234), -(other.e42 * this.e42) - (other.e43 * this.e43)) - (this.group0.zxyx * float4(other.group0.yzx, other.e41)), 
            /* e23, e31, e12, scalar */
            float4((other.e41 * this.scalar) + (other.e43 * this.e31) + (other.e23 * this.e1234) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e42 * this.scalar) + (other.e23 * this.e43) + (other.e31 * this.e1234), (other.e42 * this.e23) + (other.e43 * this.scalar) + (other.e31 * this.e41) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.zxyx * float4(other.group0.yzx, other.e41)) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e43 * this.e42) + (other.e1234 * this.e41), (other.e42 * this.e1234) + (other.e1234 * this.e42), (other.e43 * this.e1234) + (other.e1234 * this.e43), -(other.e42 * this.e42) - (other.e43 * this.e43)) + (other.group0.xxyw * this.group0.wzxw) - (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e12 * this.e42) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e23 * this.e43) + (other.e31 * this.e1234) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e31 * this.e41) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xxyw * this.group1.wzxw) + float4((other.group0.zyz * this.group1.yww), other.scalar * this.e1234) - (other.group0.yzxx * this.group1.zxyx) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12), 0.0) + (float2(this.e1234) * other.group0) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e42 * other.e3) + (this.e1234 * other.e1) + (this.e23 * other.e4) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e43 * other.e1) + (this.e1234 * other.e2) + (this.e31 * other.e4) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e1234 * other.e3) + (this.e23 * other.e431) + (this.e12 * other.e4) + (this.scalar * other.e412), this.e43 * other.e412 * -1.0) + (this.group0.xyxw * float4(other.group4.ww, other.e2, other.e4)) - (this.group0.zxyx * float4(other.group1.yzx, other.e423)) - (other.group4.yzxy * float4(this.group1.zxy, this.e42)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group0.xyz) + (other.group2.xxy * this.group0.wzx) + (other.group2.zyz * this.group0.yww) - (other.group2.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xxy * this.group1.wzx) + (other.group2.zyz * this.group1.yww) + (other.group3.xxy * this.group0.wzx) + (other.group3.zyz * this.group0.yww) - (other.group2.yzx * this.group1.zxy) - (other.group3.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.xyxw * float4(other.group1.ww, other.e431, other.e321)) + (float4(other.group4.zx, other.e4, other.e4) * float4(this.group0.yzz, this.scalar)) - (this.group0.zxyx * float4(other.group4.yzx, other.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<Origin> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group1.xyz, this.e1234), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group0.xyz, this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<Plane> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e42 * other.e321) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e23 * other.e431) + (this.scalar * other.e412), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.yzxw * other.group0.zxyw) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<Point> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 1.0) * float4(this.group0.xyz, (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Motor: AntiSandwich<Scalar> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Scalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<MultiVector> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension MultiVector: AntiSandwich<AntiScalar> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group2, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<DualNum> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group1.xyz) - (float3(other.scalar) * this.group4.xyz)), other.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group2, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group2) + (float3(other.e1234) * this.group3), 
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 1.0) * float4(other.group0.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Flector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group0.xw), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1234) + (other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (float4(this.group0.xx, this.scalar, this.e1234) * float4(other.group1.xyz, other.e4)) - (other.group1.yzxy * float4(this.group3.zxy, this.e42)) - float4((this.group2.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e41, e42, e43 */
            (other.group1.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group1.xxy) - (float3(this.e431, this.e4, this.e4) * other.group1.zyz), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e412, this.e423) * other.group0.xxy) + (float3(this.e431, this.e4, this.e4) * other.group0.zyz) + (other.group1.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group1.zyz) - (float3(this.e321, this.e3, this.e1) * other.group1.xxy) - (other.group0.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e423 * this.e43, other.e4 * this.e43, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(this.group0.yy, this.e1234, this.scalar) * float4(other.group1.xyz, other.e4)) + (float4(other.group0.ww, other.e431, other.e321) * float4(this.group2.xyx, this.e1234)) - float4((this.group2.zxy * other.group1.yzx), other.e1 * this.e41)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Horizon> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group2, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group4.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Line> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412), (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423), (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.zyz, other.e43)) - float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e423), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group0) + (other.group0.zxy * this.group2.yzx) - (other.group0.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0) + (float3(this.e1234) * other.group1) + (other.group0.zxy * this.group3.yzx) + (other.group1.zxy * this.group2.yzx) - (other.group0.yzx * this.group3.zxy) - (other.group1.yzx * this.group2.zxy), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.zyz, other.e31)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3)) - float4((other.group0.yzx * this.group4.zxy), other.e41 * this.e1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Motor> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12), 0.0) + (float2(other.e1234) * this.group0) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e43 * this.e2) + (other.e1234 * this.e1) + (other.e31 * this.e412) - (other.scalar * this.e423), (other.e42 * this.e321) + (other.e1234 * this.e2) + (other.e12 * this.e423) - (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) - (other.scalar * this.e412), 0.0) + (float4(this.e321, this.e3, this.e1, this.e4) * other.group0.xxyw) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.zyz, other.e43)) - (other.group0.yzxx * float4(this.group1.zxy, this.e423)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group0.xyz) + (this.group2.xyx * other.group0.wwy) + (this.group2.yzz * other.group0.zxw) - (this.group2.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xyx * other.group1.wwy) + (this.group2.yzz * other.group1.zxw) + (this.group3.xyx * other.group0.wwy) + (this.group3.yzz * other.group0.zxw) - (this.group2.zxy * other.group1.yzx) - (this.group3.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e321) * other.group0.xxyw) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(other.group0.zyz, other.e23)) + (this.group4.xyzy * float4(other.group0.www, other.e31)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4)) - (other.group0.yzxx * float4(this.group4.zxy, this.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * other.group0) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * other.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1234) + (other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), 0.0) + (float4(other.e1234) * this.group1) + (float4(this.group0.xx, this.scalar, this.e1234) * float4(other.group4.xyz, other.e4)) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(other.group3.xxy, other.e43)) - (float4(this.e431, this.e4, this.e4, other.e423) * float4(other.group3.zyz, this.e41)) - (other.group4.yzxz * float4(this.group3.zxy, this.e43)) - (this.group4.xyzx * float4(other.group0.xx, other.scalar, other.e41)) - float4((other.group2.yzx * this.group1.zxy), other.e42 * this.e431) - float4((this.group2.zxy * other.group1.yzx), other.e431 * this.e42), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2) + (float3(this.e1234) * other.group2) + (other.group2.zxy * this.group2.yzx) + (other.group4.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group4.xxy) - (float3(this.e431, this.e4, this.e4) * other.group4.zyz) - (other.group2.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group2) + (float3(other.e1234) * this.group3) + (float3(other.e321) * this.group4.xyz) + (float3(this.scalar) * other.group2) + (float3(this.e1234) * other.group3) + (float3(this.e4, this.e412, this.e423) * other.group1.xxy) + (float3(this.e431, this.e4, this.e4) * other.group1.zyz) + (other.group2.zxy * this.group3.yzx) + (other.group3.zxy * this.group2.yzx) + (other.group4.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group4.zyz) - (float3(this.e321, this.e3, this.e1) * other.group4.xxy) - (other.group2.yzx * this.group3.zxy) - (other.group3.yzx * this.group2.zxy) - (other.group1.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, other.e321) * float4(other.group2.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(other.group2.zyz, other.e23)) + (float4(this.group0.yy, this.e1234, this.scalar) * float4(other.group4.xyz, other.e4)) + (float4(other.group1.ww, other.e431, this.e431) * float4(this.group2.xyx, other.e31)) + (float4(other.group4.zx, other.e4, this.e412) * float4(this.group2.yzz, other.e12)) + float4(float3(0.0), -(other.e1 * this.e41) - (other.e2 * this.e42) - (other.e3 * this.e43) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((other.group2.yzx * this.group4.zxy), other.scalar * this.e4) - float4((this.group2.zxy * other.group4.yzx), other.e41 * this.e1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Origin> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e4) * float2(this.e321, this.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group3, this.e1234), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group2, this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Plane> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e4 * other.e321, 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)), 
            /* e1, e2, e3, e4 */
            float4((this.scalar * other.e423) + (this.e41 * other.e321) + (this.e31 * other.e412), (this.scalar * other.e431) + (this.e42 * other.e321) + (this.e12 * other.e423), (this.scalar * other.e412) + (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * other.group0.yzx) - (float3(this.e4) * other.group0.xyz) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(this.e321) * other.group0.xyz) - (this.group1.yzx * other.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (float4(this.e1234) * other.group0) - (other.group0.yzxx * float4(this.group2.zxy, this.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Point> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), this.e4 * other.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group3) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group0.xyz) + (this.group4.yzx * other.group0.zxy) - (float3(other.e4) * this.group1.xyz) - (this.group4.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 1.0) * float4(this.group2, (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Scalar> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e1234 * other.scalar, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar, other.scalar, other.scalar, 0.0) * float4(this.group4.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group2, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<Origin> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension Origin: AntiSandwich<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_sandwich(other: AntiScalar) -> Output {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<DualNum> {
    associatedtype Output = Motor;
    func anti_sandwich(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e4) * float4(other.group0.xyz, other.e321)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<Horizon> {
    associatedtype Output = Horizon;
    func anti_sandwich(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<Line> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group1, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group1.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group0.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e4) * float2(other.e321, other.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group3, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e4) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group2, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<Origin> {
    associatedtype Output = Origin;
    func anti_sandwich(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4 * -1.0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<Plane> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e4 * other.e321)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<Point> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Origin: AntiSandwich<Scalar> {
    associatedtype Output = Scalar;
    func anti_sandwich(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * other.scalar * -1.0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<Plane> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension Plane: AntiSandwich<AntiScalar> {
    associatedtype Output = Motor;
    func anti_sandwich(other: AntiScalar) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<DualNum> {
    associatedtype Output = Motor;
    func anti_sandwich(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e423) - (other.e412 * this.e431), -(other.e4 * this.e431) - (other.e423 * this.e412), -(other.e4 * this.e412) - (other.e431 * this.e423), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e3 * this.e431) + (other.e321 * this.e423), (other.e1 * this.e412) + (other.e321 * this.e431), (other.e2 * this.e423) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.yzxx * this.group0.zxyx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<Horizon> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Horizon) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<Line> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzy * float4(other.group1.www, other.e42)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (other.group0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(other.group0.www, other.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzx * float4(other.group0.xx, other.scalar, other.e41)) - (this.group0.yzxy * float4(other.group3.zxy, other.e42)), 
            /* e41, e42, e43 */
            (other.group4.yzx * this.group0.zxy) - (float3(other.e4) * this.group0.xyz) - (other.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (float3(this.e321) * other.group4.xyz) - (other.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (float4(other.e1234) * this.group0) + (this.group0.yzxx * float4(other.group2.zxy, other.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<Origin> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Origin) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e4 * this.e321 * -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<Plane> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e412 * this.e431 * -1.0, other.e423 * this.e412 * -1.0, other.e431 * this.e423 * -1.0, (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<Point> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e431 * other.e3, this.e412 * other.e1, this.e423 * other.e2, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.zxyx * other.group0.yzxx)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Plane: AntiSandwich<Scalar> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Scalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar, other.scalar, other.scalar, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<Point> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension Point: AntiSandwich<AntiScalar> {
    associatedtype Output = Motor;
    func anti_sandwich(other: AntiScalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<DualNum> {
    associatedtype Output = Motor;
    func anti_sandwich(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(-(other.e4 * this.e1) - (other.e412 * this.e2), -(other.e4 * this.e2) - (other.e423 * this.e3), -(other.e4 * this.e3) - (other.e431 * this.e1), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<Horizon> {
    associatedtype Output = Horizon;
    func anti_sandwich(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<Line> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.group0.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * other.group1), 0.0) - float4((other.group0.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 1.0) * float4(other.group0, -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group0.zxy * this.group0.yzx) - (float3(this.e4) * other.group1.xyz) - (other.group0.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 1.0) * float4(other.group0.xyz, -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), other.e4 * this.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group2.zxy * this.group0.yzx) - (float3(this.e4) * other.group3) - (other.group2.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group1.xyz) + (other.group4.yzx * this.group0.zxy) - (float3(other.e4) * this.group0.xyz) - (other.group4.zxy * this.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 1.0) * float4(other.group2, -(other.scalar * this.e4) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<Origin> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Origin) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<Plane> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e412 * this.e2 * -1.0, other.e423 * this.e3 * -1.0, other.e431 * this.e1 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.yzxx * this.group0.zxyx)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<Point> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Point: AntiSandwich<Scalar> {
    associatedtype Output = Scalar;
    func anti_sandwich(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * other.scalar * -1.0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiSandwichInfix) -> AntiSandwichInfixPartial<Scalar> {
        return AntiSandwichInfixPartial { a: this };
    }
}
extension Scalar: AntiSandwich<Flector> {
    associatedtype Output = Flector;
    func anti_sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar, this.scalar, this.scalar, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Scalar: AntiSandwich<Line> {
    associatedtype Output = Line;
    func anti_sandwich(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Scalar: AntiSandwich<Motor> {
    associatedtype Output = Motor;
    func anti_sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Scalar: AntiSandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e1234 * this.scalar, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar, this.scalar, this.scalar, 0.0) * float4(other.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group2, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
extension Scalar: AntiSandwich<Plane> {
    associatedtype Output = Horizon;
    func anti_sandwich(other: Plane) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar, this.scalar, this.scalar, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
