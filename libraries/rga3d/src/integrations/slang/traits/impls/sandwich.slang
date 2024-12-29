implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<AntiScalar> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<AntiScalar>(this);
    }
}
public extension SandwichInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public Flector operator /(Flector rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension AntiScalar: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension AntiScalar: Sandwich<Line> {
    public typedef Line SandwichOutput;
    public func sandwich(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(this.reverse());
    }
}
public extension AntiScalar: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
public extension AntiScalar: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e1234 * other.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group1.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension AntiScalar: Sandwich<Point> {
    public typedef Origin SandwichOutput;
    public func sandwich(other: Point) -> Origin {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<DualNum> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<DualNum>(this);
    }
}
public extension SandwichInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension DualNum: Sandwich<AntiScalar> {
    public typedef AntiScalar SandwichOutput;
    public func sandwich(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<DualNum> {
    public typedef DualNum SandwichOutput;
    public func sandwich(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)) * float4(other.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group1.xyz) - (float3(this.e1234) * other.group0.xyz)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<Horizon> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<Line> {
    public typedef Line SandwichOutput;
    public func sandwich(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)) * float4(other.group1.xyz, 1.0), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group4.xyz) - (float3(this.e1234) * other.group1.xyz)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<Origin> {
    public typedef Origin SandwichOutput;
    public func sandwich(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.scalar * other.e4
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<Plane> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<Point> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension DualNum: Sandwich<Scalar> {
    public typedef DualNum SandwichOutput;
    public func sandwich(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy
        ).geometric_product(this.reverse());
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<Flector> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<Flector>(this);
    }
}
public extension SandwichInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension Flector: Sandwich<AntiScalar> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: AntiScalar) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<DualNum> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: DualNum) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)) * float4(this.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(other.e1234) * this.group0.xyz)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyz * this.group0.yzxz) + (float4(this.group0.ww, this.e431, this.e1) * float4(other.group0.xyx, other.e423)) + (float4(this.group1.zx, this.e4, this.e2) * float4(other.group0.yzz, other.e431)) - (other.group0.zxyx * this.group1.yzxx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431)) - (float4(this.group0.zx, this.e321, this.e321) * float4(other.group1.yzz, other.e4)) - (float4(this.group1.ww, this.e2, this.e412) * float4(other.group1.xyx, other.e3)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e3 * this.e321) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx) - (float4(this.group1.ww, this.e2, this.e321) * float4(other.group0.xyx, other.e321))
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<Horizon> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Horizon) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<Line> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Line) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e3 * other.e31) + (this.e321 * other.e23), (this.e1 * other.e12) + (this.e321 * other.e31), (this.e2 * other.e23) + (this.e321 * other.e12), -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((other.group1.xyz.zxy * this.group1.yzx), this.e2 * other.e31)
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321 * other.e23, this.e321 * other.e31, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (this.group0.xxyw * other.group1.wzxw) + float4((this.group0.zyz * other.group1.yww), this.e321 * other.e1234) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e423 * other.scalar) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e431 * other.scalar) + (this.e321 * other.e42), (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e412 * other.scalar) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + float4((this.group0.xxy * other.group0.wzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23)) - (other.group1.zxyy * float4(this.group1.yzx, this.e2))
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group1.wx), 
            /* e1, e2, e3, e4 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (float4(other.scalar) * this.group0) + (float4(this.group1.ww, this.e2, this.e321) * float4(other.group3.xyz.xyx, other.e1234)) - (this.group0.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.e2, other.e321, other.e321) * this.group1.zyz) + (float3(other.e321, other.e3, other.e1) * this.group1.xxy) + (this.group0.yzx * other.group4.zxy) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e412, other.e423) * this.group0.xxy) - (float3(other.e431, other.e4, other.e4) * this.group0.zyz) - (this.group1.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) - (float3(this.e321) * other.group1.xyz) - (float3(other.e2, other.e321, other.e321) * this.group0.zyz) - (float3(other.e321, other.e3, other.e1) * this.group0.xxy), 
            /* e423, e431, e412, e321 */
            float4((this.e1 * other.e1234) + (this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23)) - float4((other.group3.xyz.zxy * this.group1.yzx), this.e2 * other.e31)
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<Origin> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Origin) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<Plane> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Plane) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e3 * other.e431) - (this.e321 * other.e423), -(this.e1 * other.e412) - (this.e321 * other.e431), -(this.e2 * other.e423) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.yzxx * other.group0.zxyx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2)), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<Point> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Point) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e4 * other.e1) + (this.e412 * other.e2), (this.e4 * other.e2) + (this.e423 * other.e3), (this.e4 * other.e3) + (this.e431 * other.e1), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.yzxy * other.group0.zxyy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(this.e3 * other.e2) - (this.e321 * other.e1), -(this.e1 * other.e3) - (this.e321 * other.e2), -(this.e2 * other.e1) - (this.e321 * other.e3), (this.e2 * other.e2) + (this.e3 * other.e3)) + (this.group0.yzxx * other.group0.zxyx)
        ).geometric_product(this.reverse());
    }
}
public extension Flector: Sandwich<Scalar> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Scalar) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        ).geometric_product(this.reverse());
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<Horizon> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<Horizon>(this);
    }
}
public extension SandwichInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension Horizon: Sandwich<AntiScalar> {
    public typedef AntiScalar SandwichOutput;
    public func sandwich(other: AntiScalar) -> AntiScalar {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e321
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<DualNum> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: DualNum) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<Horizon> {
    public typedef Horizon SandwichOutput;
    public func sandwich(other: Horizon) -> Horizon {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321 * -1.0
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<Line> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Line) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group1.xyz, other.e1234), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group0.xyz, other.scalar)
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321) * float2(other.e321, other.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group3.xyz, other.e1234), 
            /* e41, e42, e43 */
            float3(this.e321) * other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group2.xyz, other.scalar)
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<Origin> {
    public typedef Origin SandwichOutput;
    public func sandwich(other: Origin) -> Origin {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<Plane> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Plane) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e321) * other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e321 * other.e321 * -1.0)
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<Point> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Point) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e321 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321) * other.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Horizon: Sandwich<Scalar> {
    public typedef Scalar SandwichOutput;
    public func sandwich(other: Scalar) -> Scalar {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        ).geometric_product(this.reverse());
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<Line> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<Line>(this);
    }
}
public extension SandwichInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension Line: Sandwich<AntiScalar> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: AntiScalar) -> Motor {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<DualNum> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: DualNum) -> Motor {
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.xyz.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xyz.xxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group1.xyz.yzx * other.group0.zxy), other.e423 * this.e23), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31), 0.0) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.xyz.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xyz.xxy, this.e23)) - float4((this.group1.xyz.yzx * other.group1.zxy), other.e3 * this.e12)
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<Horizon> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<Line> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e42 * this.e12) + (other.e31 * this.e43), (other.e43 * this.e23) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e23 * this.e42), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.xyz.zxy * this.group1.xyz.yzx), other.e41 * this.e23) - float4((other.group1.xyz.zxy * this.group0.xyz.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4(other.e31 * this.e12, other.e12 * this.e23, other.e23 * this.e31, -(other.e31 * this.e31) - (other.e12 * this.e12)) - float4((other.group1.xyz.zxy * this.group1.xyz.yzx), other.e23 * this.e23)
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.scalar) + (this.e43 * other.e31) + (this.e23 * other.e1234) + (this.e12 * other.e42), (this.e41 * other.e12) + (this.e42 * other.scalar) + (this.e23 * other.e43) + (this.e31 * other.e1234), (this.e42 * other.e23) + (this.e43 * other.scalar) + (this.e31 * other.e41) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.zxyx * float4(this.group0.xyz.yzx, this.e41)) - float4((this.group1.xyz.yzx * other.group0.zxy), this.e42 * other.e31), 
            /* e23, e31, e12, scalar */
            float4((this.e23 * other.scalar) + (this.e12 * other.e31), (this.e23 * other.e12) + (this.e31 * other.scalar), (this.e31 * other.e23) + (this.e12 * other.scalar), -(this.e31 * other.e31) - (this.e12 * other.e12)) - (other.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.xyz.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xyz.xxy, this.e41)) + float4(float3(0.0), (this.e43 * other.e3) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.xyz.yzx * other.group1.zxy), this.e23 * other.e423), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.xyz.yzx) + (this.group1.xyz.zxy * other.group2.xyz.yzx) - (this.group0.xyz.yzx * other.group3.xyz.zxy) - (this.group1.xyz.yzx * other.group2.xyz.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group1.xyz) + (this.group1.xyz.zxy * other.group3.xyz.yzx) - (this.group1.xyz.yzx * other.group3.xyz.zxy), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4) + (this.e12 * other.e431), (this.e43 * other.e1) + (this.e23 * other.e412) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e31 * other.e423) + (this.e12 * other.e4), 0.0) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.xyz.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xyz.xxy, this.e23)) - float4((this.group1.xyz.yzx * other.group4.zxy), this.e12 * other.e3)
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<Origin> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Origin) -> Flector {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<Plane> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4((this.group1.xyz.zxy * other.group0.yzx), 0.0) - float4((float3(other.e321) * this.group0.xyz), 0.0) - float4((this.group1.xyz.yzx * other.group0.zxy), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<Point> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        ).geometric_product(this.reverse());
    }
}
public extension Line: Sandwich<Scalar> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Scalar) -> Motor {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        ).geometric_product(this.reverse());
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<Motor> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<Motor>(this);
    }
}
public extension SandwichInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension Motor: Sandwich<AntiScalar> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<DualNum> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (other.group0.xyxx * float4(this.group1.wwy, this.e41)) + (other.group0.yzzy * float4(this.group1.zxw, this.e42)) + float4(float3(0.0), (other.e4 * this.scalar) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e321 * this.e1234)) + float4((other.group1.www * this.group1.xyz), other.e3 * this.e43) - (this.group1.yzxx * float4(other.group0.zxy, other.e423)), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e431 * this.e12) - (other.e321 * this.e41), (other.e4 * this.e31) + (other.e431 * this.scalar) + (other.e412 * this.e23) - (other.e321 * this.e42), (other.e4 * this.e12) + (other.e423 * this.e31) + (other.e412 * this.scalar) - (other.e321 * this.e43), 0.0) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.xyxx * float4(this.group0.wwy, this.e23)) - (other.group0.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(other.group1.zxy, other.e3))
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<Horizon> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4(this.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<Line> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.scalar) + (other.e42 * this.e12) + (other.e23 * this.e1234) + (other.e31 * this.e43), (other.e42 * this.scalar) + (other.e43 * this.e23) + (other.e31 * this.e1234) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e43 * this.scalar) + (other.e23 * this.e42) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e41)) - float4((other.group1.xyz.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e23 * this.scalar) + (other.e31 * this.e12), (other.e31 * this.scalar) + (other.e12 * this.e23), (other.e23 * this.e31) + (other.e12 * this.scalar), -(other.e31 * this.e31) - (other.e12 * this.e12)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e31 * this.e43) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e31 * this.e1234) + (other.e12 * this.e41) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e23 * this.e42) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xyxw * this.group1.wwyw) + float4((other.group0.yzz * this.group1.zxw), other.scalar * this.e1234) - (other.group0.zxyx * this.group1.yzxx) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e31 * this.e12) + (other.scalar * this.e23), (other.e12 * this.e23) + (other.scalar * this.e31), (other.e12 * this.scalar) + (other.scalar * this.e12), -(other.e31 * this.e31) - (other.e12 * this.e12)) + (other.group1.xyxw * this.group1.wwyw) - (other.group1.zxyx * this.group1.yzxx)
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12)) + (float2(other.scalar) * float2(this.scalar, this.e1234)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xxy, this.e41)) + (other.group1.xyzz * float4(this.group1.www, this.e43)) + float4(float3(0.0), (this.scalar * other.e4) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.yzx * other.group1.zxy), this.e1234 * other.e321), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xyz.xyx * this.group1.wwy) + (other.group2.xyz.yzz * this.group1.zxw) + (other.group3.xyz.xyx * this.group0.wwy) + (other.group3.xyz.yzz * this.group0.zxw) - (other.group2.xyz.zxy * this.group1.yzx) - (other.group3.xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group1.xyz) + (other.group3.xyz.xyx * this.group1.wwy) + (other.group3.xyz.yzz * this.group1.zxw) - (other.group3.xyz.zxy * this.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * other.e4) + (this.e12 * other.e431) + (this.scalar * other.e423) - (this.e31 * other.e412), (this.e23 * other.e412) + (this.e31 * other.e4) + (this.scalar * other.e431) - (this.e12 * other.e423), (this.e31 * other.e423) + (this.e12 * other.e4) + (this.scalar * other.e412) - (this.e23 * other.e431), 0.0) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xxy, this.e23)) - (other.group1.xyzz * float4(this.group0.www, this.e12))
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<Origin> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<Plane> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group0.zxy)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<Point> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e43 * other.e3) + (this.scalar * other.e4)) + (other.group0.xyzy * float4(this.group1.www, this.e42)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzy * float4(this.group0.www, this.e31)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        ).geometric_product(this.reverse());
    }
}
public extension Motor: Sandwich<Scalar> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<MultiVector> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<MultiVector>(this);
    }
}
public extension SandwichInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiScalar rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension MultiVector: Sandwich<AntiScalar> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e1234 * this.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<DualNum> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)) * float4(this.group1.xyz, 1.0), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group4.xyz) + (float3(other.e1234) * this.group1.xyz)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<Flector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(this.scalar) * other.group0) + (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group3.xyz.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group3.xyz.xxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group3.xyz.yzx * other.group0.zxy), other.e321 * this.e1234), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e4, this.e431) * other.group0.xyx) + (float3(this.e412, this.e423, this.e4) * other.group0.yzz) + (other.group1.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group0.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group0.yzz) - (float3(this.e321, this.e321, this.e2) * other.group0.xyx), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12) - (other.e412 * this.e31), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23) - (other.e423 * this.e12), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31) - (other.e431 * this.e23), 0.0) + (float4(this.scalar) * other.group1) - (float4(other.e2, other.e321, other.e321, other.e3) * float4(this.group2.xyz.zyz, this.e12)) - (float4(other.e321, other.e3, other.e1, other.e2) * float4(this.group2.xyz.xxy, this.e31)) - (other.group0.xyzx * float4(this.group0.xy.yy, this.e1234, this.e23))
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<Horizon> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321) * float2(this.e321, this.e4) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group3.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(other.e321) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4(this.group2.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<Line> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e23 * this.e321) + (other.e31 * this.e3), (other.e31 * this.e321) + (other.e12 * this.e1), (other.e23 * this.e2) + (other.e12 * this.e321), -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.xyz.zxy) + (other.group1.xyz.yzx * this.group2.xyz.zxy) - (other.group0.xyz.zxy * this.group3.xyz.yzx) - (other.group1.xyz.zxy * this.group2.xyz.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group1.xyz) + (other.group1.xyz.yzx * this.group3.xyz.zxy) - (other.group1.xyz.zxy * this.group3.xyz.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423), (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4), other.e12 * this.e3 * -1.0) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((other.group1.xyz.zxy * this.group4.yzx), other.e31 * this.e2)
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<Motor> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12)) + (float2(this.scalar) * float2(other.scalar, other.e1234)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) + (other.group1.yzzw * float4(this.group1.zx, this.e321, this.e4)) + (float4(this.group4.ww, this.e2, this.e321) * float4(other.group1.xyx, other.e1234)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xyz.xxy * other.group1.wzx) + (this.group2.xyz.zyz * other.group1.yww) + (this.group3.xyz.xxy * other.group0.wzx) + (this.group3.xyz.zyz * other.group0.yww) - (this.group2.xyz.yzx * other.group1.zxy) - (this.group3.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group1.xyz) + (this.group3.xyz.xxy * other.group1.wzx) + (this.group3.xyz.zyz * other.group1.yww) - (this.group3.xyz.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.scalar * this.e423), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.scalar * this.e412), other.e12 * this.e3 * -1.0) + (float4(this.group4.ww, this.e2, this.e321) * float4(other.group0.xyx, other.scalar)) - (other.group1.zxyy * float4(this.group4.yzx, this.e2)) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (float2(other.scalar) * this.group0.xy) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)) - (float2(other.e321, other.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(other.e2, other.e321, other.e321, other.e3) * float4(this.group3.xyz.zyz, this.e43)) + (float4(other.e321, other.e3, other.e1, other.e2) * float4(this.group3.xyz.xxy, this.e42)) + (float4(this.group0.xy.xx, this.scalar, other.e1234) * float4(other.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, other.e1) * float4(other.group3.xyz.yzz, this.e41)) + (float4(this.group4.ww, this.e2, other.e4) * float4(other.group3.xyz.xyx, this.scalar)) + float4(float3(0.0), -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((other.group3.xyz.zxy * this.group1.yzx), other.e321 * this.e1234) - float4((this.group3.xyz.yzx * other.group1.zxy), other.e41 * this.e1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz) + (float3(this.e4, this.e4, this.e431) * other.group1.xyx) + (float3(this.e412, this.e423, this.e4) * other.group1.yzz) + (other.group2.xyz.yzx * this.group3.xyz.zxy) + (other.group3.xyz.yzx * this.group2.xyz.zxy) + (other.group4.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group4.yzz) - (float3(this.e321, this.e321, this.e2) * other.group4.xyx) - (other.group2.xyz.zxy * this.group3.xyz.yzx) - (other.group3.xyz.zxy * this.group2.xyz.yzx) - (other.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + (other.group3.xyz.yzx * this.group3.xyz.zxy) + (other.group1.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group3.xyz.zxy * this.group3.xyz.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.e423 * this.scalar) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.e431 * this.scalar) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.e423 * this.e31) + (other.e412 * this.scalar), 0.0) + (float4(other.scalar) * this.group4) + (float4(other.group0.xy.yy, other.e1234, this.scalar) * float4(this.group1.xyz, other.e321)) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group2.xyz.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group2.xyz.xxy, this.e23)) - (this.group1.yzxy * float4(other.group2.xyz.zxy, other.e31)) - (float4(this.group0.xy.yy, this.e1234, other.e23) * float4(other.group1.xyz, this.e1)) - float4((other.group3.xyz.zxy * this.group4.yzx), other.e12 * this.e3) - float4((this.group3.xyz.yzx * other.group4.zxy), other.e3 * this.e12)
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<Origin> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e321 * other.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group3.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<Plane> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * other.e321, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group3.xyz), -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group4.xyz) + (this.group1.yzx * other.group0.zxy) - (float3(this.e321) * other.group0.xyz) - (this.group1.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group3.xyz.zxy * other.group0.yzx) - (float3(other.e321) * this.group2.xyz) - (this.group3.xyz.yzx * other.group0.zxy)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<Point> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3), -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (float4(this.scalar) * other.group0) + (other.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) + (this.group4.zxy * other.group0.yzx) - (float3(other.e4) * this.group1.xyz) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) - (float3(this.e321) * other.group0.xyz) - (this.group1.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzx * float4(this.group0.xy.yy, this.e1234, this.e23)) - (other.group0.yzxy * float4(this.group2.xyz.zxy, this.e31))
        ).geometric_product(this.reverse());
    }
}
public extension MultiVector: Sandwich<Scalar> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        ).geometric_product(this.reverse());
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<Origin> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<Origin>(this);
    }
}
public extension SandwichInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public Flector operator /(Flector rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension Origin: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Origin: Sandwich<Line> {
    public typedef AntiScalar SandwichOutput;
    public func sandwich(other: Line) -> AntiScalar {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Origin: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Origin: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group3.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Origin: Sandwich<Point> {
    public typedef Plane SandwichOutput;
    public func sandwich(other: Point) -> Plane {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<Plane> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<Plane>(this);
    }
}
public extension SandwichInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension Plane: Sandwich<AntiScalar> {
    public typedef AntiScalar SandwichOutput;
    public func sandwich(other: AntiScalar) -> AntiScalar {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e321
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<DualNum> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: DualNum) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e2 * this.e412) + (other.e321 * this.e423), (other.e3 * this.e423) + (other.e321 * this.e431), (other.e1 * this.e431) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.zxyx * this.group0.yzxx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2)), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<Horizon> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Horizon) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<Line> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Line) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), -(other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((other.group1.xyz.yzx * this.group0.zxy), 0.0) - float4((other.group1.xyz.zxy * this.group0.yzx), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321 * this.e321, -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group3.xyz), (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (float3(this.e321) * other.group4.xyz) - (other.group1.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group2.xyz) + (other.group3.xyz.yzx * this.group0.zxy) - (other.group3.xyz.zxy * this.group0.yzx)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<Origin> {
    public typedef Origin SandwichOutput;
    public func sandwich(other: Origin) -> Origin {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<Plane> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Plane) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<Point> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Point) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e412 * other.e2, this.e423 * other.e3, this.e431 * other.e1, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.yzxx * other.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Plane: Sandwich<Scalar> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Scalar) -> Motor {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        ).geometric_product(this.reverse());
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<Point> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<Point>(this);
    }
}
public extension SandwichInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension Point: Sandwich<AntiScalar> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: AntiScalar) -> Motor {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<DualNum> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: DualNum) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.group0.xy.yy, other.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e1) - (other.e431 * this.e3), -(other.e4 * this.e2) - (other.e412 * this.e1), -(other.e4 * this.e3) - (other.e423 * this.e2), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.zxyy * this.group0.yzxy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e1 * this.e2) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<Horizon> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Horizon) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e321 * this.e4), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<Line> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Line) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (other.group1.yzxw * this.group0.zxyw) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e1234 * this.e3) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e321 * this.e4) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)), 
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (float4(other.scalar) * this.group0) - (this.group0.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) + (other.group4.zxy * this.group0.yzx) - (float3(other.e4) * this.group0.xyz) - (other.group4.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (other.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e1234 * this.e1) + (other.e42 * this.e3) + (other.e23 * this.e4), (other.e1234 * this.e2) + (other.e43 * this.e1) + (other.e31 * this.e4), (other.e1234 * this.e3) + (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<Origin> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Origin) -> Flector {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<Plane> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Plane) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e431 * this.e3 * -1.0, other.e412 * this.e1 * -1.0, other.e423 * this.e2 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<Point> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Point) -> Flector {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e2 * this.e3 * -1.0, other.e3 * this.e1 * -1.0, other.e1 * this.e2 * -1.0, (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        ).geometric_product(this.reverse());
    }
}
public extension Point: Sandwich<Scalar> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Scalar) -> Motor {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        ).geometric_product(this.reverse());
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public SandwichInfixPartial<Scalar> operator /(SandwichInfix rhs) {
        return SandwichInfixPartial<Scalar>(this);
    }
}
public extension SandwichInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.sandwich(rhs);
    }
}
public extension Scalar: Sandwich<AntiScalar> {
    public typedef AntiScalar SandwichOutput;
    public func sandwich(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<DualNum> {
    public typedef DualNum SandwichOutput;
    public func sandwich(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0.xy
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<Flector> {
    public typedef Flector SandwichOutput;
    public func sandwich(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<Horizon> {
    public typedef Horizon SandwichOutput;
    public func sandwich(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e321 * this.scalar
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<Line> {
    public typedef Line SandwichOutput;
    public func sandwich(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<Motor> {
    public typedef Motor SandwichOutput;
    public func sandwich(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<MultiVector> {
    public typedef MultiVector SandwichOutput;
    public func sandwich(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<Origin> {
    public typedef Origin SandwichOutput;
    public func sandwich(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e4 * this.scalar
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<Plane> {
    public typedef Plane SandwichOutput;
    public func sandwich(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<Point> {
    public typedef Point SandwichOutput;
    public func sandwich(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        ).geometric_product(this.reverse());
    }
}
public extension Scalar: Sandwich<Scalar> {
    public typedef Scalar SandwichOutput;
    public func sandwich(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        ).geometric_product(this.reverse());
    }
}
