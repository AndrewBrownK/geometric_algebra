implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<AntiScalar> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<AntiScalar>(this);
    }
}
public extension AntiSandwichInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension AntiScalar: AntiSandwich<AntiScalar> {
    public typedef AntiScalar AntiSandwichOutput;
    public func anti_sandwich(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<DualNum> {
    public typedef DualNum AntiSandwichOutput;
    public func anti_sandwich(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0.xy
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<Horizon> {
    public typedef Horizon AntiSandwichOutput;
    public func anti_sandwich(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<Line> {
    public typedef Line AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<Origin> {
    public typedef Origin AntiSandwichOutput;
    public func anti_sandwich(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<Plane> {
    public typedef Plane AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<Point> {
    public typedef Point AntiSandwichOutput;
    public func anti_sandwich(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension AntiScalar: AntiSandwich<Scalar> {
    public typedef Scalar AntiSandwichOutput;
    public func anti_sandwich(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<DualNum> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<DualNum>(this);
    }
}
public extension AntiSandwichInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public DualNum operator /(AntiScalar rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension DualNum: AntiSandwich<AntiScalar> {
    public typedef DualNum AntiSandwichOutput;
    public func anti_sandwich(other: AntiScalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0.xy
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<DualNum> {
    public typedef DualNum AntiSandwichOutput;
    public func anti_sandwich(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(this.e1234) * other.group0.xyz)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321)) * float4(other.group1.xyz, 1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<Horizon> {
    public typedef Horizon AntiSandwichOutput;
    public func anti_sandwich(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<Line> {
    public typedef Line AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<Origin> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<Plane> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<Point> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension DualNum: AntiSandwich<Scalar> {
    public typedef Scalar AntiSandwichOutput;
    public func anti_sandwich(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<Flector> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<Flector>(this);
    }
}
public extension AntiSandwichInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension Flector: AntiSandwich<AntiScalar> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: AntiScalar) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<DualNum> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: DualNum) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) - (float3(other.scalar) * this.group1.xyz)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xy.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4)) * float4(this.group1.xyz, 1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e423 * this.e4) - (other.e412 * this.e431), -(other.e423 * this.e412) - (other.e431 * this.e4), -(other.e431 * this.e423) - (other.e412 * this.e4), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group1.zxyx) - (float4(other.e4) * float4(this.group1.xyz, this.e4)), 
            /* e23, e31, e12, scalar */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (float4(this.e4, this.e412, this.e423, this.e1) * float4(other.group0.xxy, other.e423)) + (float4(this.e431, this.e4, this.e4, this.e2) * float4(other.group0.zyz, other.e431)) + (other.group1.yzxz * this.group0.zxyz) - (float4(this.e2, this.e321, this.e321, this.e321) * float4(other.group1.zyz, other.e4)) - (float4(this.e321, this.e3, this.e1, this.e412) * float4(other.group1.xxy, other.e3)) - (other.group0.yzxx * this.group1.zxyx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<Horizon> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Horizon) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<Line> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xyz.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.xyz.zyz, other.e43)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e423 * other.e41), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xyz.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.xyz.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group0.xyz.yzx * this.group1.zxy), this.e1 * other.e41)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e412 * other.e23), (this.e3 * other.e1234) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e412 * other.scalar), 0.0) + (this.group0.xyxw * other.group0.wwyw) - (this.group1.xyxz * float4(other.group1.wwy, other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e423)) - float4((this.group0.www * other.group1.xyz), this.e431 * other.e42), 
            /* e423, e431, e412, e321 */
            (this.group1.xyxy * float4(other.group0.wwy, other.e31)) + (this.group1.yzzz * float4(other.group0.zxw, other.e12)) + float4(float3(0.0), (this.e321 * other.e1234) - (this.e2 * other.e42) - (this.e3 * other.e43) - (this.e4 * other.scalar)) + float4((this.group0.www * other.group0.xyz), this.e423 * other.e23) - (other.group0.yzxx * float4(this.group1.zxy, this.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<Origin> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Origin) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<Plane> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e4 * other.e423) - (this.e431 * other.e412), -(this.e4 * other.e431) - (this.e412 * other.e423), -(this.e4 * other.e412) - (this.e423 * other.e431), (this.e431 * other.e431) + (this.e412 * other.e412)) + (this.group1.zxyx * other.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(this.e2 * other.e412) - (this.e321 * other.e423), -(this.e3 * other.e423) - (this.e321 * other.e431), -(this.e1 * other.e431) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.zxyx * other.group0.yzxx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<Point> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Point) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((this.e4 * other.e1) + (this.e431 * other.e3), (this.e4 * other.e2) + (this.e412 * other.e1), (this.e4 * other.e3) + (this.e423 * other.e2), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.zxyy * other.group0.yzxy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Flector: AntiSandwich<Scalar> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Scalar) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<Horizon> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<Horizon>(this);
    }
}
public extension AntiSandwichInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Flector operator /(Flector rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension Horizon: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Horizon: AntiSandwich<Line> {
    public typedef Scalar AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Scalar {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Horizon: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Horizon: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Horizon: AntiSandwich<Plane> {
    public typedef Point AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Point {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<Line> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<Line>(this);
    }
}
public extension AntiSandwichInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension Line: AntiSandwich<AntiScalar> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: AntiScalar) -> Motor {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<DualNum> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: DualNum) -> Motor {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) - (other.group1.yzxy * float4(this.group1.xyz.zxy, this.e42)) - float4((this.group0.xyz.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e41) + (other.e412 * this.e42), (other.e4 * this.e42) + (other.e423 * this.e43), (other.e4 * this.e43) + (other.e431 * this.e41), -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group0.xyz.zxy * other.group1.yzx), other.e1 * this.e41)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<Horizon> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Horizon) -> Flector {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<Line> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e43 * this.e42, other.e41 * this.e43, other.e42 * this.e41, -(other.e42 * this.e42) - (other.e43 * this.e43)) - float4((other.group0.xyz.yzx * this.group0.xyz.zxy), other.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((other.e43 * this.e31) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e23 * this.e43), (other.e42 * this.e23) + (other.e31 * this.e41), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e41 * this.e23) - float4((other.group1.xyz.yzx * this.group0.xyz.zxy), other.e42 * this.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.e1234) + (this.e42 * other.e43), (this.e42 * other.e1234) + (this.e43 * other.e41), (this.e41 * other.e42) + (this.e43 * other.e1234), -(this.e42 * other.e42) - (this.e43 * other.e43)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e41)), 
            /* e23, e31, e12, scalar */
            float4((this.e41 * other.scalar) + (this.e42 * other.e12) + (this.e23 * other.e1234) + (this.e31 * other.e43), (this.e42 * other.scalar) + (this.e43 * other.e23) + (this.e31 * other.e1234) + (this.e12 * other.e41), (this.e41 * other.e31) + (this.e43 * other.scalar) + (this.e23 * other.e42) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e41)) - float4((this.group1.xyz.zxy * other.group0.yzx), this.e42 * other.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<Origin> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<Plane> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<Point> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.xyz.yzx * other.group0.zxy), 0.0) - float4((this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), -(this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Line: AntiSandwich<Scalar> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Scalar) -> Motor {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<Motor> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<Motor>(this);
    }
}
public extension AntiSandwichInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension Motor: AntiSandwich<AntiScalar> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<DualNum> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e431 * this.scalar) + (other.e321 * this.e42), (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e412 * this.scalar) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (other.group0.xxyw * this.group0.wzxw) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - (this.group0.zxyx * float4(other.group0.yzx, other.e423)), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e431 * this.e1234, other.e412 * this.e1234, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(other.e4) * float4(this.group0.xyz, this.scalar)) + (other.group1.xxyw * this.group0.wzxw) - (this.group0.zxyx * float4(other.group1.yzx, other.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<Horizon> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<Line> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.e1234) + (other.e43 * this.e42), (other.e41 * this.e43) + (other.e42 * this.e1234), (other.e42 * this.e41) + (other.e43 * this.e1234), -(other.e42 * this.e42) - (other.e43 * this.e43)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e41)), 
            /* e23, e31, e12, scalar */
            float4((other.e41 * this.scalar) + (other.e43 * this.e31) + (other.e23 * this.e1234) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e42 * this.scalar) + (other.e23 * this.e43) + (other.e31 * this.e1234), (other.e42 * this.e23) + (other.e43 * this.scalar) + (other.e31 * this.e41) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.zxyx * float4(other.group0.xyz.yzx, other.e41)) - float4((other.group1.xyz.yzx * this.group0.zxy), other.e42 * this.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e43 * this.e42) + (other.e1234 * this.e41), (other.e42 * this.e1234) + (other.e1234 * this.e42), (other.e43 * this.e1234) + (other.e1234 * this.e43), -(other.e42 * this.e42) - (other.e43 * this.e43)) + (other.group0.xxyw * this.group0.wzxw) - (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e12 * this.e42) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e23 * this.e43) + (other.e31 * this.e1234) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e31 * this.e41) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xxyw * this.group1.wzxw) + float4((other.group0.zyz * this.group1.yww), other.scalar * this.e1234) - (other.group0.yzxx * this.group1.zxyx) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<Origin> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group1.xyz, this.e1234), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group0.xyz, this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<Plane> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e42 * other.e321) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e23 * other.e431) + (this.scalar * other.e412), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.yzxw * other.group0.zxyw) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<Point> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Motor: AntiSandwich<Scalar> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<MultiVector> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<MultiVector>(this);
    }
}
public extension AntiSandwichInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiScalar rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension MultiVector: AntiSandwich<AntiScalar> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: AntiScalar) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<DualNum> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: DualNum) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<Flector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<Horizon> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: Horizon) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<Line> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<Motor> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<Origin> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: Origin) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<Plane> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<Point> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: Point) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension MultiVector: AntiSandwich<Scalar> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: Scalar) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<Origin> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<Origin>(this);
    }
}
public extension AntiSandwichInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension Origin: AntiSandwich<AntiScalar> {
    public typedef AntiScalar AntiSandwichOutput;
    public func anti_sandwich(other: AntiScalar) -> AntiScalar {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<DualNum> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: DualNum) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e4) * float4(other.group0.xyz, other.e321)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<Horizon> {
    public typedef Horizon AntiSandwichOutput;
    public func anti_sandwich(other: Horizon) -> Horizon {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<Line> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e4) * other.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group1.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group0.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<Origin> {
    public typedef Origin AntiSandwichOutput;
    public func anti_sandwich(other: Origin) -> Origin {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4 * -1.0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<Plane> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e4) * other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e4 * other.e321)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<Point> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Point) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Origin: AntiSandwich<Scalar> {
    public typedef Scalar AntiSandwichOutput;
    public func anti_sandwich(other: Scalar) -> Scalar {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * other.scalar * -1.0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<Plane> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<Plane>(this);
    }
}
public extension AntiSandwichInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension Plane: AntiSandwich<AntiScalar> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: AntiScalar) -> Motor {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<DualNum> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: DualNum) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e423) - (other.e412 * this.e431), -(other.e4 * this.e431) - (other.e423 * this.e412), -(other.e4 * this.e412) - (other.e431 * this.e423), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e3 * this.e431) + (other.e321 * this.e423), (other.e1 * this.e412) + (other.e321 * this.e431), (other.e2 * this.e423) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.yzxx * this.group0.zxyx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<Horizon> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Horizon) -> Flector {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<Line> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzy * float4(other.group1.www, other.e42)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (other.group0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(other.group0.www, other.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<Origin> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Origin) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e4 * this.e321 * -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<Plane> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e412 * this.e431 * -1.0, other.e423 * this.e412 * -1.0, other.e431 * this.e423 * -1.0, (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<Point> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Point) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e431 * other.e3, this.e412 * other.e1, this.e423 * other.e2, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.zxyx * other.group0.yzxx)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Plane: AntiSandwich<Scalar> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Scalar) -> Motor {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<Point> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<Point>(this);
    }
}
public extension AntiSandwichInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension Point: AntiSandwich<AntiScalar> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: AntiScalar) -> Motor {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<DualNum> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: DualNum) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(-(other.e4 * this.e1) - (other.e412 * this.e2), -(other.e4 * this.e2) - (other.e423 * this.e3), -(other.e4 * this.e3) - (other.e431 * this.e1), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<Horizon> {
    public typedef Horizon AntiSandwichOutput;
    public func anti_sandwich(other: Horizon) -> Horizon {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<Line> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * other.group1.xyz), 0.0) - float4((other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group0.zxy * this.group0.yzx) - (float3(this.e4) * other.group1.xyz) - (other.group0.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4))
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<Origin> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Origin) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<Plane> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e412 * this.e2 * -1.0, other.e423 * this.e3 * -1.0, other.e431 * this.e1 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.yzxx * this.group0.zxyx)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<Point> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Point) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Point: AntiSandwich<Scalar> {
    public typedef Scalar AntiSandwichOutput;
    public func anti_sandwich(other: Scalar) -> Scalar {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * other.scalar * -1.0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public AntiSandwichInfixPartial<Scalar> operator /(AntiSandwichInfix rhs) {
        return AntiSandwichInfixPartial<Scalar>(this);
    }
}
public extension AntiSandwichInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public Flector operator /(Flector rhs) {
        return this.a.anti_sandwich(rhs);
    }
}
public extension Scalar: AntiSandwich<Flector> {
    public typedef Flector AntiSandwichOutput;
    public func anti_sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Scalar: AntiSandwich<Line> {
    public typedef Line AntiSandwichOutput;
    public func anti_sandwich(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Scalar: AntiSandwich<Motor> {
    public typedef Motor AntiSandwichOutput;
    public func anti_sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Scalar: AntiSandwich<MultiVector> {
    public typedef MultiVector AntiSandwichOutput;
    public func anti_sandwich(other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(this.anti_reverse());
    }
}
public extension Scalar: AntiSandwich<Plane> {
    public typedef Horizon AntiSandwichOutput;
    public func anti_sandwich(other: Plane) -> Horizon {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
