using traits::GeometricProduct;
using traits::Reverse;
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<AntiScalar> {
        return SandwichInfixPartial { a: this };
    }
}
extension AntiScalar: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension AntiScalar: Sandwich<Line> {
    associatedtype Output = Line;
    func sandwich(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
extension AntiScalar: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
extension AntiScalar: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, this.e1234 * other.scalar) * float2(0.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group3.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group1.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension AntiScalar: Sandwich<Point> {
    associatedtype Output = Origin;
    func sandwich(other: Point) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<DualNum> {
        return SandwichInfixPartial { a: this };
    }
}
extension DualNum: Sandwich<AntiScalar> {
    associatedtype Output = AntiScalar;
    func sandwich(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<DualNum> {
    associatedtype Output = DualNum;
    func sandwich(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)) * float4(other.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group1.xyz) - (float3(this.e1234) * other.group0.xyz)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<Horizon> {
    associatedtype Output = Flector;
    func sandwich(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<Line> {
    associatedtype Output = Line;
    func sandwich(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)) * float4(other.group1.xyz, 1.0), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group4.xyz) - (float3(this.e1234) * other.group1.xyz)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<Origin> {
    associatedtype Output = Origin;
    func sandwich(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.scalar * other.e4, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<Plane> {
    associatedtype Output = Flector;
    func sandwich(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<Point> {
    associatedtype Output = Flector;
    func sandwich(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension DualNum: Sandwich<Scalar> {
    associatedtype Output = DualNum;
    func sandwich(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<Flector> {
        return SandwichInfixPartial { a: this };
    }
}
extension Flector: Sandwich<AntiScalar> {
    associatedtype Output = Motor;
    func sandwich(other: AntiScalar) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<DualNum> {
    associatedtype Output = Motor;
    func sandwich(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)) * float4(this.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(other.e1234) * this.group0.xyz)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyz * this.group0.yzxz) + (float4(this.group0.ww, this.e431, this.e1) * float4(other.group0.xyx, other.e423)) + (float4(this.group1.zx, this.e4, this.e2) * float4(other.group0.yzz, other.e431)) - (other.group0.zxyx * this.group1.yzxx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431)) - (float4(this.group0.zx, this.e321, this.e321) * float4(other.group1.yzz, other.e4)) - (float4(this.group1.ww, this.e2, this.e412) * float4(other.group1.xyx, other.e3)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e3 * this.e321) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx) - (float4(this.group1.ww, this.e2, this.e321) * float4(other.group0.xyx, other.e321))
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<Horizon> {
    associatedtype Output = Flector;
    func sandwich(other: Horizon) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<Line> {
    associatedtype Output = Motor;
    func sandwich(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e3 * other.e31) + (this.e321 * other.e23), (this.e1 * other.e12) + (this.e321 * other.e31), (this.e2 * other.e23) + (this.e321 * other.e12), -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((other.group1.xyz.zxy * this.group1.yzx), this.e2 * other.e31)
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321 * other.e23, this.e321 * other.e31, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (this.group0.xxyw * other.group1.wzxw) + float4((this.group0.zyz * other.group1.yww), this.e321 * other.e1234) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e423 * other.scalar) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e431 * other.scalar) + (this.e321 * other.e42), (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e412 * other.scalar) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + float4((this.group0.xxy * other.group0.wzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23)) - (other.group1.zxyy * float4(this.group1.yzx, this.e2))
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group1.wx), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (float4(other.scalar) * this.group0) + (float4(this.group1.ww, this.e2, this.e321) * float4(other.group3.xyz.xyx, other.e1234)) - (this.group0.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (float3(other.e2, other.e321, other.e321) * this.group1.zyz) + (float3(other.e321, other.e3, other.e1) * this.group1.xxy) + (this.group0.yzx * other.group4.zxy) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e412, other.e423) * this.group0.xxy) - (float3(other.e431, other.e4, other.e4) * this.group0.zyz) - (this.group1.yzx * other.group1.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group0.yzx * other.group1.zxy) - (float3(this.e321) * other.group1.xyz) - (float3(other.e2, other.e321, other.e321) * this.group0.zyz) - (float3(other.e321, other.e3, other.e1) * this.group0.xxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e1 * other.e1234) + (this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23)) - float4((other.group3.xyz.zxy * this.group1.yzx), this.e2 * other.e31)
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<Origin> {
    associatedtype Output = Flector;
    func sandwich(other: Origin) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<Plane> {
    associatedtype Output = Flector;
    func sandwich(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e3 * other.e431) - (this.e321 * other.e423), -(this.e1 * other.e412) - (this.e321 * other.e431), -(this.e2 * other.e423) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.yzxx * other.group0.zxyx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2)), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<Point> {
    associatedtype Output = Flector;
    func sandwich(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e4 * other.e1) + (this.e412 * other.e2), (this.e4 * other.e2) + (this.e423 * other.e3), (this.e4 * other.e3) + (this.e431 * other.e1), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.yzxy * other.group0.zxyy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(this.e3 * other.e2) - (this.e321 * other.e1), -(this.e1 * other.e3) - (this.e321 * other.e2), -(this.e2 * other.e1) - (this.e321 * other.e3), (this.e2 * other.e2) + (this.e3 * other.e3)) + (this.group0.yzxx * other.group0.zxyx)
        ).geometric_product(this.reverse());
    }
}
extension Flector: Sandwich<Scalar> {
    associatedtype Output = Motor;
    func sandwich(other: Scalar) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        ).geometric_product(this.reverse());
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<Horizon> {
        return SandwichInfixPartial { a: this };
    }
}
extension Horizon: Sandwich<AntiScalar> {
    associatedtype Output = AntiScalar;
    func sandwich(other: AntiScalar) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e1234 * this.e321, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<DualNum> {
    associatedtype Output = Motor;
    func sandwich(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<Horizon> {
    associatedtype Output = Horizon;
    func sandwich(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321 * -1.0, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<Line> {
    associatedtype Output = Motor;
    func sandwich(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group1.xyz, other.e1234), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group0.xyz, other.scalar)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321) * float2(other.e321, other.e4) * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group3.xyz, other.e1234), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e321) * other.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * other.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group2.xyz, other.scalar)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<Origin> {
    associatedtype Output = Origin;
    func sandwich(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<Plane> {
    associatedtype Output = Flector;
    func sandwich(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e321) * other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e321 * other.e321 * -1.0)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<Point> {
    associatedtype Output = Flector;
    func sandwich(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e321 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321) * other.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Horizon: Sandwich<Scalar> {
    associatedtype Output = Scalar;
    func sandwich(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 * other.scalar, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<Line> {
        return SandwichInfixPartial { a: this };
    }
}
extension Line: Sandwich<AntiScalar> {
    associatedtype Output = Motor;
    func sandwich(other: AntiScalar) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group1.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<DualNum> {
    associatedtype Output = Motor;
    func sandwich(other: DualNum) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.xyz.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xyz.xxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group1.xyz.yzx * other.group0.zxy), other.e423 * this.e23), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31), 0.0) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.xyz.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xyz.xxy, this.e23)) - float4((this.group1.xyz.yzx * other.group1.zxy), other.e3 * this.e12)
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<Horizon> {
    associatedtype Output = Flector;
    func sandwich(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<Line> {
    associatedtype Output = Motor;
    func sandwich(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e42 * this.e12) + (other.e31 * this.e43), (other.e43 * this.e23) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e23 * this.e42), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.xyz.zxy * this.group1.xyz.yzx), other.e41 * this.e23) - float4((other.group1.xyz.zxy * this.group0.xyz.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4(other.e31 * this.e12, other.e12 * this.e23, other.e23 * this.e31, -(other.e31 * this.e31) - (other.e12 * this.e12)) - float4((other.group1.xyz.zxy * this.group1.xyz.yzx), other.e23 * this.e23)
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.scalar) + (this.e43 * other.e31) + (this.e23 * other.e1234) + (this.e12 * other.e42), (this.e41 * other.e12) + (this.e42 * other.scalar) + (this.e23 * other.e43) + (this.e31 * other.e1234), (this.e42 * other.e23) + (this.e43 * other.scalar) + (this.e31 * other.e41) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.zxyx * float4(this.group0.xyz.yzx, this.e41)) - float4((this.group1.xyz.yzx * other.group0.zxy), this.e42 * other.e31), 
            /* e23, e31, e12, scalar */
            float4((this.e23 * other.scalar) + (this.e12 * other.e31), (this.e23 * other.e12) + (this.e31 * other.scalar), (this.e31 * other.e23) + (this.e12 * other.scalar), -(this.e31 * other.e31) - (this.e12 * other.e12)) - (other.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.xyz.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xyz.xxy, this.e41)) + float4(float3(0.0), (this.e43 * other.e3) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.xyz.yzx * other.group1.zxy), this.e23 * other.e423), 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.xyz.yzx) + (this.group1.xyz.zxy * other.group2.xyz.yzx) - (this.group0.xyz.yzx * other.group3.xyz.zxy) - (this.group1.xyz.yzx * other.group2.xyz.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group1.xyz) + (this.group1.xyz.zxy * other.group3.xyz.yzx) - (this.group1.xyz.yzx * other.group3.xyz.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4) + (this.e12 * other.e431), (this.e43 * other.e1) + (this.e23 * other.e412) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e31 * other.e423) + (this.e12 * other.e4), 0.0) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.xyz.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xyz.xxy, this.e23)) - float4((this.group1.xyz.yzx * other.group4.zxy), this.e12 * other.e3)
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<Origin> {
    associatedtype Output = Flector;
    func sandwich(other: Origin) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<Plane> {
    associatedtype Output = Flector;
    func sandwich(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4((this.group1.xyz.zxy * other.group0.yzx), 0.0) - float4((float3(other.e321) * this.group0.xyz), 0.0) - float4((this.group1.xyz.yzx * other.group0.zxy), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<Point> {
    associatedtype Output = Flector;
    func sandwich(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        ).geometric_product(this.reverse());
    }
}
extension Line: Sandwich<Scalar> {
    associatedtype Output = Motor;
    func sandwich(other: Scalar) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<Motor> {
        return SandwichInfixPartial { a: this };
    }
}
extension Motor: Sandwich<AntiScalar> {
    associatedtype Output = Motor;
    func sandwich(other: AntiScalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<DualNum> {
    associatedtype Output = Motor;
    func sandwich(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (other.group0.xyxx * float4(this.group1.wwy, this.e41)) + (other.group0.yzzy * float4(this.group1.zxw, this.e42)) + float4(float3(0.0), (other.e4 * this.scalar) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e321 * this.e1234)) + float4((other.group1.www * this.group1.xyz), other.e3 * this.e43) - (this.group1.yzxx * float4(other.group0.zxy, other.e423)), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e431 * this.e12) - (other.e321 * this.e41), (other.e4 * this.e31) + (other.e431 * this.scalar) + (other.e412 * this.e23) - (other.e321 * this.e42), (other.e4 * this.e12) + (other.e423 * this.e31) + (other.e412 * this.scalar) - (other.e321 * this.e43), 0.0) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.xyxx * float4(this.group0.wwy, this.e23)) - (other.group0.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(other.group1.zxy, other.e3))
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<Horizon> {
    associatedtype Output = Flector;
    func sandwich(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4(this.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<Line> {
    associatedtype Output = Motor;
    func sandwich(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.scalar) + (other.e42 * this.e12) + (other.e23 * this.e1234) + (other.e31 * this.e43), (other.e42 * this.scalar) + (other.e43 * this.e23) + (other.e31 * this.e1234) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e43 * this.scalar) + (other.e23 * this.e42) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e41)) - float4((other.group1.xyz.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e23 * this.scalar) + (other.e31 * this.e12), (other.e31 * this.scalar) + (other.e12 * this.e23), (other.e23 * this.e31) + (other.e12 * this.scalar), -(other.e31 * this.e31) - (other.e12 * this.e12)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e31 * this.e43) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e31 * this.e1234) + (other.e12 * this.e41) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e23 * this.e42) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xyxw * this.group1.wwyw) + float4((other.group0.yzz * this.group1.zxw), other.scalar * this.e1234) - (other.group0.zxyx * this.group1.yzxx) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e31 * this.e12) + (other.scalar * this.e23), (other.e12 * this.e23) + (other.scalar * this.e31), (other.e12 * this.scalar) + (other.scalar * this.e12), -(other.e31 * this.e31) - (other.e12 * this.e12)) + (other.group1.xyxw * this.group1.wwyw) - (other.group1.zxyx * this.group1.yzxx)
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12)) + (float2(other.scalar) * float2(this.scalar, this.e1234)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xxy, this.e41)) + (other.group1.xyzz * float4(this.group1.www, this.e43)) + float4(float3(0.0), (this.scalar * other.e4) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.yzx * other.group1.zxy), this.e1234 * other.e321), 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xyz.xyx * this.group1.wwy) + (other.group2.xyz.yzz * this.group1.zxw) + (other.group3.xyz.xyx * this.group0.wwy) + (other.group3.xyz.yzz * this.group0.zxw) - (other.group2.xyz.zxy * this.group1.yzx) - (other.group3.xyz.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group1.xyz) + (other.group3.xyz.xyx * this.group1.wwy) + (other.group3.xyz.yzz * this.group1.zxw) - (other.group3.xyz.zxy * this.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * other.e4) + (this.e12 * other.e431) + (this.scalar * other.e423) - (this.e31 * other.e412), (this.e23 * other.e412) + (this.e31 * other.e4) + (this.scalar * other.e431) - (this.e12 * other.e423), (this.e31 * other.e423) + (this.e12 * other.e4) + (this.scalar * other.e412) - (this.e23 * other.e431), 0.0) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xxy, this.e23)) - (other.group1.xyzz * float4(this.group0.www, this.e12))
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<Origin> {
    associatedtype Output = Flector;
    func sandwich(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<Plane> {
    associatedtype Output = Flector;
    func sandwich(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group0.zxy)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<Point> {
    associatedtype Output = Flector;
    func sandwich(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e43 * other.e3) + (this.scalar * other.e4)) + (other.group0.xyzy * float4(this.group1.www, this.e42)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzy * float4(this.group0.www, this.e31)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        ).geometric_product(this.reverse());
    }
}
extension Motor: Sandwich<Scalar> {
    associatedtype Output = Motor;
    func sandwich(other: Scalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        ).geometric_product(this.reverse());
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<MultiVector> {
        return SandwichInfixPartial { a: this };
    }
}
extension MultiVector: Sandwich<AntiScalar> {
    associatedtype Output = MultiVector;
    func sandwich(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, other.e1234 * this.scalar) * float2(0.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group3.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<DualNum> {
    associatedtype Output = MultiVector;
    func sandwich(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)) * float4(this.group1.xyz, 1.0), 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group4.xyz) + (float3(other.e1234) * this.group1.xyz)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<Flector> {
    associatedtype Output = MultiVector;
    func sandwich(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group4.wx), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.scalar) * other.group0) + (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group3.xyz.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group3.xyz.xxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group3.xyz.yzx * other.group0.zxy), other.e321 * this.e1234), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e4, this.e431) * other.group0.xyx) + (float3(this.e412, this.e423, this.e4) * other.group0.yzz) + (other.group1.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group0.zxy * this.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group0.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group0.yzz) - (float3(this.e321, this.e321, this.e2) * other.group0.xyx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12) - (other.e412 * this.e31), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23) - (other.e423 * this.e12), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31) - (other.e431 * this.e23), 0.0) + (float4(this.scalar) * other.group1) - (float4(other.e2, other.e321, other.e321, other.e3) * float4(this.group2.xyz.zyz, this.e12)) - (float4(other.e321, other.e3, other.e1, other.e2) * float4(this.group2.xyz.xxy, this.e31)) - (other.group0.xyzx * float4(this.group0.xy.yy, this.e1234, this.e23))
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<Horizon> {
    associatedtype Output = MultiVector;
    func sandwich(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e321) * float2(this.e321, this.e4) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group3.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e321) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e321) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4(this.group2.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<Line> {
    associatedtype Output = MultiVector;
    func sandwich(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e23 * this.e321) + (other.e31 * this.e3), (other.e31 * this.e321) + (other.e12 * this.e1), (other.e23 * this.e2) + (other.e12 * this.e321), -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.xyz.zxy) + (other.group1.xyz.yzx * this.group2.xyz.zxy) - (other.group0.xyz.zxy * this.group3.xyz.yzx) - (other.group1.xyz.zxy * this.group2.xyz.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group1.xyz) + (other.group1.xyz.yzx * this.group3.xyz.zxy) - (other.group1.xyz.zxy * this.group3.xyz.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423), (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4), other.e12 * this.e3 * -1.0) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((other.group1.xyz.zxy * this.group4.yzx), other.e31 * this.e2)
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<Motor> {
    associatedtype Output = MultiVector;
    func sandwich(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12)) + (float2(this.scalar) * float2(other.scalar, other.e1234)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) + (other.group1.yzzw * float4(this.group1.zx, this.e321, this.e4)) + (float4(this.group4.ww, this.e2, this.e321) * float4(other.group1.xyx, other.e1234)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xyz.xxy * other.group1.wzx) + (this.group2.xyz.zyz * other.group1.yww) + (this.group3.xyz.xxy * other.group0.wzx) + (this.group3.xyz.zyz * other.group0.yww) - (this.group2.xyz.yzx * other.group1.zxy) - (this.group3.xyz.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group1.xyz) + (this.group3.xyz.xxy * other.group1.wzx) + (this.group3.xyz.zyz * other.group1.yww) - (this.group3.xyz.yzx * other.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.scalar * this.e423), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.scalar * this.e412), other.e12 * this.e3 * -1.0) + (float4(this.group4.ww, this.e2, this.e321) * float4(other.group0.xyx, other.scalar)) - (other.group1.zxyy * float4(this.group4.yzx, this.e2)) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (float2(other.scalar) * this.group0.xy) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)) - (float2(other.e321, other.e1) * this.group4.wx), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(other.e2, other.e321, other.e321, other.e3) * float4(this.group3.xyz.zyz, this.e43)) + (float4(other.e321, other.e3, other.e1, other.e2) * float4(this.group3.xyz.xxy, this.e42)) + (float4(this.group0.xy.xx, this.scalar, other.e1234) * float4(other.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, other.e1) * float4(other.group3.xyz.yzz, this.e41)) + (float4(this.group4.ww, this.e2, other.e4) * float4(other.group3.xyz.xyx, this.scalar)) + float4(float3(0.0), -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((other.group3.xyz.zxy * this.group1.yzx), other.e321 * this.e1234) - float4((this.group3.xyz.yzx * other.group1.zxy), other.e41 * this.e1), 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz) + (float3(this.e4, this.e4, this.e431) * other.group1.xyx) + (float3(this.e412, this.e423, this.e4) * other.group1.yzz) + (other.group2.xyz.yzx * this.group3.xyz.zxy) + (other.group3.xyz.yzx * this.group2.xyz.zxy) + (other.group4.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group4.yzz) - (float3(this.e321, this.e321, this.e2) * other.group4.xyx) - (other.group2.xyz.zxy * this.group3.xyz.yzx) - (other.group3.xyz.zxy * this.group2.xyz.yzx) - (other.group1.zxy * this.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + (other.group3.xyz.yzx * this.group3.xyz.zxy) + (other.group1.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group3.xyz.zxy * this.group3.xyz.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.e423 * this.scalar) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.e431 * this.scalar) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.e423 * this.e31) + (other.e412 * this.scalar), 0.0) + (float4(other.scalar) * this.group4) + (float4(other.group0.xy.yy, other.e1234, this.scalar) * float4(this.group1.xyz, other.e321)) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group2.xyz.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group2.xyz.xxy, this.e23)) - (this.group1.yzxy * float4(other.group2.xyz.zxy, other.e31)) - (float4(this.group0.xy.yy, this.e1234, other.e23) * float4(other.group1.xyz, this.e1)) - float4((other.group3.xyz.zxy * this.group4.yzx), other.e12 * this.e3) - float4((this.group3.xyz.yzx * other.group4.zxy), other.e3 * this.e12)
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<Origin> {
    associatedtype Output = MultiVector;
    func sandwich(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, this.e321 * other.e4) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group3.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<Plane> {
    associatedtype Output = MultiVector;
    func sandwich(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321 * other.e321, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group3.xyz), -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e321) * this.group4.xyz) + (this.group1.yzx * other.group0.zxy) - (float3(this.e321) * other.group0.xyz) - (this.group1.zxy * other.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e321) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group3.xyz.zxy * other.group0.yzx) - (float3(other.e321) * this.group2.xyz) - (this.group3.xyz.yzx * other.group0.zxy)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<Point> {
    associatedtype Output = MultiVector;
    func sandwich(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3), -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (float4(this.scalar) * other.group0) + (other.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group0.xyz) + (this.group4.zxy * other.group0.yzx) - (float3(other.e4) * this.group1.xyz) - (this.group4.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * other.group0.zxy) - (float3(this.e321) * other.group0.xyz) - (this.group1.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzx * float4(this.group0.xy.yy, this.e1234, this.e23)) - (other.group0.yzxy * float4(this.group2.xyz.zxy, this.e31))
        ).geometric_product(this.reverse());
    }
}
extension MultiVector: Sandwich<Scalar> {
    associatedtype Output = MultiVector;
    func sandwich(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        ).geometric_product(this.reverse());
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<Origin> {
        return SandwichInfixPartial { a: this };
    }
}
extension Origin: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
extension Origin: Sandwich<Line> {
    associatedtype Output = AntiScalar;
    func sandwich(other: Line) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Origin: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Origin: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group1.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group3.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Origin: Sandwich<Point> {
    associatedtype Output = Plane;
    func sandwich(other: Point) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<Plane> {
        return SandwichInfixPartial { a: this };
    }
}
extension Plane: Sandwich<AntiScalar> {
    associatedtype Output = AntiScalar;
    func sandwich(other: AntiScalar) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e1234 * this.e321, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<DualNum> {
    associatedtype Output = Motor;
    func sandwich(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e2 * this.e412) + (other.e321 * this.e423), (other.e3 * this.e423) + (other.e321 * this.e431), (other.e1 * this.e431) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.zxyx * this.group0.yzxx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2)), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<Horizon> {
    associatedtype Output = Flector;
    func sandwich(other: Horizon) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<Line> {
    associatedtype Output = Motor;
    func sandwich(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), -(other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((other.group1.xyz.yzx * this.group0.zxy), 0.0) - float4((other.group1.xyz.zxy * this.group0.yzx), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e321 * this.e321, -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group3.xyz), (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e321) * this.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (float3(this.e321) * other.group4.xyz) - (other.group1.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * other.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group2.xyz) + (other.group3.xyz.yzx * this.group0.zxy) - (other.group3.xyz.zxy * this.group0.yzx)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<Origin> {
    associatedtype Output = Origin;
    func sandwich(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.e4 * this.e321 * -1.0, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<Plane> {
    associatedtype Output = Flector;
    func sandwich(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<Point> {
    associatedtype Output = Flector;
    func sandwich(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e412 * other.e2, this.e423 * other.e3, this.e431 * other.e1, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.yzxx * other.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Plane: Sandwich<Scalar> {
    associatedtype Output = Motor;
    func sandwich(other: Scalar) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        ).geometric_product(this.reverse());
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<Point> {
        return SandwichInfixPartial { a: this };
    }
}
extension Point: Sandwich<AntiScalar> {
    associatedtype Output = Motor;
    func sandwich(other: AntiScalar) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<DualNum> {
    associatedtype Output = Motor;
    func sandwich(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.group0.xy.yy, other.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e1) - (other.e431 * this.e3), -(other.e4 * this.e2) - (other.e412 * this.e1), -(other.e4 * this.e3) - (other.e423 * this.e2), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.zxyy * this.group0.yzxy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e1 * this.e2) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<Horizon> {
    associatedtype Output = Flector;
    func sandwich(other: Horizon) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e321 * this.e4), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<Line> {
    associatedtype Output = Motor;
    func sandwich(other: Line) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (other.group1.yzxw * this.group0.zxyw) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e1234 * this.e3) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e321 * this.e4) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (float4(other.scalar) * this.group0) - (this.group0.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group4.zxy * this.group0.yzx) - (float3(other.e4) * this.group0.xyz) - (other.group4.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group1.zxy * this.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (other.group1.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e1234 * this.e1) + (other.e42 * this.e3) + (other.e23 * this.e4), (other.e1234 * this.e2) + (other.e43 * this.e1) + (other.e31 * this.e4), (other.e1234 * this.e3) + (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<Origin> {
    associatedtype Output = Flector;
    func sandwich(other: Origin) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<Plane> {
    associatedtype Output = Flector;
    func sandwich(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e431 * this.e3 * -1.0, other.e412 * this.e1 * -1.0, other.e423 * this.e2 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<Point> {
    associatedtype Output = Flector;
    func sandwich(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e2 * this.e3 * -1.0, other.e3 * this.e1 * -1.0, other.e1 * this.e2 * -1.0, (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        ).geometric_product(this.reverse());
    }
}
extension Point: Sandwich<Scalar> {
    associatedtype Output = Motor;
    func sandwich(other: Scalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        ).geometric_product(this.reverse());
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: SandwichInfix) -> SandwichInfixPartial<Scalar> {
        return SandwichInfixPartial { a: this };
    }
}
extension Scalar: Sandwich<AntiScalar> {
    associatedtype Output = AntiScalar;
    func sandwich(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<DualNum> {
    associatedtype Output = DualNum;
    func sandwich(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<Flector> {
    associatedtype Output = Flector;
    func sandwich(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<Horizon> {
    associatedtype Output = Horizon;
    func sandwich(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(other.e321 * this.scalar, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<Line> {
    associatedtype Output = Line;
    func sandwich(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<Motor> {
    associatedtype Output = Motor;
    func sandwich(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<MultiVector> {
    associatedtype Output = MultiVector;
    func sandwich(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<Origin> {
    associatedtype Output = Origin;
    func sandwich(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e4 * this.scalar, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<Plane> {
    associatedtype Output = Plane;
    func sandwich(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<Point> {
    associatedtype Output = Point;
    func sandwich(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        ).geometric_product(this.reverse());
    }
}
extension Scalar: Sandwich<Scalar> {
    associatedtype Output = Scalar;
    func sandwich(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
