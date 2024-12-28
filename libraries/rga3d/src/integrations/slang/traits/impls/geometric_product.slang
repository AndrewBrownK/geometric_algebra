public extension AntiScalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<AntiScalar> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> AntiScalar.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension AntiScalar: GeometricProduct<DualNum> {
    public typedef AntiScalar GeometricProductOutput;
    public func geometric_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension AntiScalar: GeometricProduct<Flector> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiScalar: GeometricProduct<Horizon> {
    public typedef Origin GeometricProductOutput;
    public func geometric_product(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e321 * -1.0
        );
    }
}
public extension AntiScalar: GeometricProduct<Line> {
    public typedef Line GeometricProductOutput;
    public func geometric_product(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension AntiScalar: GeometricProduct<Motor> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension AntiScalar: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension AntiScalar: GeometricProduct<Plane> {
    public typedef Origin GeometricProductOutput;
    public func geometric_product(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e321 * -1.0
        );
    }
}
public extension AntiScalar: GeometricProduct<Point> {
    public typedef Plane GeometricProductOutput;
    public func geometric_product(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiScalar: GeometricProduct<Scalar> {
    public typedef AntiScalar GeometricProductOutput;
    public func geometric_product(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<DualNum> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> DualNum.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension DualNum: GeometricProduct<AntiScalar> {
    public typedef AntiScalar GeometricProductOutput;
    public func geometric_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension DualNum: GeometricProduct<DualNum> {
    public typedef DualNum GeometricProductOutput;
    public func geometric_product(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
public extension DualNum: GeometricProduct<Flector> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)) * float4(other.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group1.xyz) - (float3(this.e1234) * other.group0.xyz)), this.scalar * other.e321)
        );
    }
}
public extension DualNum: GeometricProduct<Horizon> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e321)
        );
    }
}
public extension DualNum: GeometricProduct<Line> {
    public typedef Line GeometricProductOutput;
    public func geometric_product(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension DualNum: GeometricProduct<Motor> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
public extension DualNum: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension DualNum: GeometricProduct<Origin> {
    public typedef Origin GeometricProductOutput;
    public func geometric_product(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.scalar * other.e4
        );
    }
}
public extension DualNum: GeometricProduct<Plane> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension DualNum: GeometricProduct<Point> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension DualNum: GeometricProduct<Scalar> {
    public typedef DualNum GeometricProductOutput;
    public func geometric_product(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<Flector> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Flector.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension Flector: GeometricProduct<AntiScalar> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: AntiScalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        );
    }
}
public extension Flector: GeometricProduct<DualNum> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)) * float4(this.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(other.e1234) * this.group0.xyz)), other.scalar * this.e321)
        );
    }
}
public extension Flector: GeometricProduct<Flector> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyz * this.group0.yzxz) + (float4(this.group0.ww, this.e431, this.e1) * float4(other.group0.xyx, other.e423)) + (float4(this.group1.zx, this.e4, this.e2) * float4(other.group0.yzz, other.e431)) - (other.group0.zxyx * this.group1.yzxx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431)) - (float4(this.group0.zx, this.e321, this.e321) * float4(other.group1.yzz, other.e4)) - (float4(this.group1.ww, this.e2, this.e412) * float4(other.group1.xyx, other.e3)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e3 * this.e321) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx) - (float4(this.group1.ww, this.e2, this.e321) * float4(other.group0.xyx, other.e321))
        );
    }
}
public extension Flector: GeometricProduct<Horizon> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Horizon) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
public extension Flector: GeometricProduct<Line> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e3 * other.e31) + (this.e321 * other.e23), (this.e1 * other.e12) + (this.e321 * other.e31), (this.e2 * other.e23) + (this.e321 * other.e12), -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((other.group1.xyz.zxy * this.group1.yzx), this.e2 * other.e31)
        );
    }
}
public extension Flector: GeometricProduct<Motor> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321 * other.e23, this.e321 * other.e31, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (this.group0.xxyw * other.group1.wzxw) + float4((this.group0.zyz * other.group1.yww), this.e321 * other.e1234) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e423 * other.scalar) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e431 * other.scalar) + (this.e321 * other.e42), (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e412 * other.scalar) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + float4((this.group0.xxy * other.group0.wzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23)) - (other.group1.zxyy * float4(this.group1.yzx, this.e2))
        );
    }
}
public extension Flector: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Flector: GeometricProduct<Origin> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Origin) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Flector: GeometricProduct<Plane> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e3 * other.e431) - (this.e321 * other.e423), -(this.e1 * other.e412) - (this.e321 * other.e431), -(this.e2 * other.e423) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.yzxx * other.group0.zxyx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2)), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
public extension Flector: GeometricProduct<Point> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e4 * other.e1) + (this.e412 * other.e2), (this.e4 * other.e2) + (this.e423 * other.e3), (this.e4 * other.e3) + (this.e431 * other.e1), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.yzxy * other.group0.zxyy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(this.e3 * other.e2) - (this.e321 * other.e1), -(this.e1 * other.e3) - (this.e321 * other.e2), -(this.e2 * other.e1) - (this.e321 * other.e3), (this.e2 * other.e2) + (this.e3 * other.e3)) + (this.group0.yzxx * other.group0.zxyx)
        );
    }
}
public extension Flector: GeometricProduct<Scalar> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Scalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<Horizon> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Horizon.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension Horizon: GeometricProduct<AntiScalar> {
    public typedef Origin GeometricProductOutput;
    public func geometric_product(other: AntiScalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e321
        );
    }
}
public extension Horizon: GeometricProduct<DualNum> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e321)
        );
    }
}
public extension Horizon: GeometricProduct<Flector> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
public extension Horizon: GeometricProduct<Horizon> {
    public typedef Scalar GeometricProductOutput;
    public func geometric_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321 * -1.0
        );
    }
}
public extension Horizon: GeometricProduct<Line> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
public extension Horizon: GeometricProduct<Motor> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group1.xyz, other.e1234), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group0.xyz, other.scalar)
        );
    }
}
public extension Horizon: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Horizon: GeometricProduct<Origin> {
    public typedef AntiScalar GeometricProductOutput;
    public func geometric_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Horizon: GeometricProduct<Plane> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e321) * other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e321 * other.e321 * -1.0)
        );
    }
}
public extension Horizon: GeometricProduct<Point> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e321 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321) * other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Horizon: GeometricProduct<Scalar> {
    public typedef Horizon GeometricProductOutput;
    public func geometric_product(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<Line> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<Line> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Line.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension Line: GeometricProduct<AntiScalar> {
    public typedef Line GeometricProductOutput;
    public func geometric_product(other: AntiScalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Line: GeometricProduct<DualNum> {
    public typedef Line GeometricProductOutput;
    public func geometric_product(other: DualNum) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension Line: GeometricProduct<Flector> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.xyz.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xyz.xxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group1.xyz.yzx * other.group0.zxy), other.e423 * this.e23), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31), 0.0) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.xyz.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xyz.xxy, this.e23)) - float4((this.group1.xyz.yzx * other.group1.zxy), other.e3 * this.e12)
        );
    }
}
public extension Line: GeometricProduct<Horizon> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Line: GeometricProduct<Line> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e42 * this.e12) + (other.e31 * this.e43), (other.e43 * this.e23) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e23 * this.e42), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.xyz.zxy * this.group1.xyz.yzx), other.e41 * this.e23) - float4((other.group1.xyz.zxy * this.group0.xyz.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4(other.e31 * this.e12, other.e12 * this.e23, other.e23 * this.e31, -(other.e31 * this.e31) - (other.e12 * this.e12)) - float4((other.group1.xyz.zxy * this.group1.xyz.yzx), other.e23 * this.e23)
        );
    }
}
public extension Line: GeometricProduct<Motor> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.scalar) + (this.e43 * other.e31) + (this.e23 * other.e1234) + (this.e12 * other.e42), (this.e41 * other.e12) + (this.e42 * other.scalar) + (this.e23 * other.e43) + (this.e31 * other.e1234), (this.e42 * other.e23) + (this.e43 * other.scalar) + (this.e31 * other.e41) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.zxyx * float4(this.group0.xyz.yzx, this.e41)) - float4((this.group1.xyz.yzx * other.group0.zxy), this.e42 * other.e31), 
            /* e23, e31, e12, scalar */
            float4((this.e23 * other.scalar) + (this.e12 * other.e31), (this.e23 * other.e12) + (this.e31 * other.scalar), (this.e31 * other.e23) + (this.e12 * other.scalar), -(this.e31 * other.e31) - (this.e12 * other.e12)) - (other.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
public extension Line: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Line: GeometricProduct<Origin> {
    public typedef Plane GeometricProductOutput;
    public func geometric_product(other: Origin) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
public extension Line: GeometricProduct<Plane> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4((this.group1.xyz.zxy * other.group0.yzx), 0.0) - float4((float3(other.e321) * this.group0.xyz), 0.0) - float4((this.group1.xyz.yzx * other.group0.zxy), 0.0)
        );
    }
}
public extension Line: GeometricProduct<Point> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Line: GeometricProduct<Scalar> {
    public typedef Line GeometricProductOutput;
    public func geometric_product(other: Scalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<Motor> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Motor.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension Motor: GeometricProduct<AntiScalar> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Motor: GeometricProduct<DualNum> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Motor: GeometricProduct<Flector> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (other.group0.xyxx * float4(this.group1.wwy, this.e41)) + (other.group0.yzzy * float4(this.group1.zxw, this.e42)) + float4(float3(0.0), (other.e4 * this.scalar) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e321 * this.e1234)) + float4((other.group1.www * this.group1.xyz), other.e3 * this.e43) - (this.group1.yzxx * float4(other.group0.zxy, other.e423)), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e431 * this.e12) - (other.e321 * this.e41), (other.e4 * this.e31) + (other.e431 * this.scalar) + (other.e412 * this.e23) - (other.e321 * this.e42), (other.e4 * this.e12) + (other.e423 * this.e31) + (other.e412 * this.scalar) - (other.e321 * this.e43), 0.0) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.xyxx * float4(this.group0.wwy, this.e23)) - (other.group0.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(other.group1.zxy, other.e3))
        );
    }
}
public extension Motor: GeometricProduct<Horizon> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4(this.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension Motor: GeometricProduct<Line> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.scalar) + (other.e42 * this.e12) + (other.e23 * this.e1234) + (other.e31 * this.e43), (other.e42 * this.scalar) + (other.e43 * this.e23) + (other.e31 * this.e1234) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e43 * this.scalar) + (other.e23 * this.e42) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e41)) - float4((other.group1.xyz.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e23 * this.scalar) + (other.e31 * this.e12), (other.e31 * this.scalar) + (other.e12 * this.e23), (other.e23 * this.e31) + (other.e12 * this.scalar), -(other.e31 * this.e31) - (other.e12 * this.e12)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e23))
        );
    }
}
public extension Motor: GeometricProduct<Motor> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e31 * this.e43) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e31 * this.e1234) + (other.e12 * this.e41) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e23 * this.e42) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xyxw * this.group1.wwyw) + float4((other.group0.yzz * this.group1.zxw), other.scalar * this.e1234) - (other.group0.zxyx * this.group1.yzxx) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e31 * this.e12) + (other.scalar * this.e23), (other.e12 * this.e23) + (other.scalar * this.e31), (other.e12 * this.scalar) + (other.scalar * this.e12), -(other.e31 * this.e31) - (other.e12 * this.e12)) + (other.group1.xyxw * this.group1.wwyw) - (other.group1.zxyx * this.group1.yzxx)
        );
    }
}
public extension Motor: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Motor: GeometricProduct<Origin> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
public extension Motor: GeometricProduct<Plane> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group0.zxy)), this.scalar * other.e321)
        );
    }
}
public extension Motor: GeometricProduct<Point> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e43 * other.e3) + (this.scalar * other.e4)) + (other.group0.xyzy * float4(this.group1.www, this.e42)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzy * float4(this.group0.www, this.e31)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension Motor: GeometricProduct<Scalar> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<MultiVector> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> MultiVector.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension MultiVector: GeometricProduct<AntiScalar> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: AntiScalar) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<DualNum> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: DualNum) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<Flector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: Flector) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<Horizon> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: Horizon) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<Line> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: Line) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<Motor> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: Motor) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<Origin> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: Origin) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<Plane> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: Plane) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<Point> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: Point) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricProduct<Scalar> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: Scalar) -> MultiVector {
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
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<Origin> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Origin.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension Origin: GeometricProduct<DualNum> {
    public typedef Origin GeometricProductOutput;
    public func geometric_product(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.scalar * this.e4
        );
    }
}
public extension Origin: GeometricProduct<Flector> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Origin: GeometricProduct<Horizon> {
    public typedef AntiScalar GeometricProductOutput;
    public func geometric_product(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
public extension Origin: GeometricProduct<Line> {
    public typedef Plane GeometricProductOutput;
    public func geometric_product(other: Line) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
public extension Origin: GeometricProduct<Motor> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
public extension Origin: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Origin: GeometricProduct<Plane> {
    public typedef AntiScalar GeometricProductOutput;
    public func geometric_product(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
public extension Origin: GeometricProduct<Point> {
    public typedef Line GeometricProductOutput;
    public func geometric_product(other: Point) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Origin: GeometricProduct<Scalar> {
    public typedef Origin GeometricProductOutput;
    public func geometric_product(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * other.scalar
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<Plane> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Plane.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension Plane: GeometricProduct<AntiScalar> {
    public typedef Origin GeometricProductOutput;
    public func geometric_product(other: AntiScalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e321
        );
    }
}
public extension Plane: GeometricProduct<DualNum> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane: GeometricProduct<Flector> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e2 * this.e412) + (other.e321 * this.e423), (other.e3 * this.e423) + (other.e321 * this.e431), (other.e1 * this.e431) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.zxyx * this.group0.yzxx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2)), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
public extension Plane: GeometricProduct<Horizon> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Horizon) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        );
    }
}
public extension Plane: GeometricProduct<Line> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), -(other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((other.group1.xyz.yzx * this.group0.zxy), 0.0) - float4((other.group1.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
public extension Plane: GeometricProduct<Motor> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.scalar * this.e321)
        );
    }
}
public extension Plane: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Plane: GeometricProduct<Origin> {
    public typedef AntiScalar GeometricProductOutput;
    public func geometric_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
public extension Plane: GeometricProduct<Plane> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        );
    }
}
public extension Plane: GeometricProduct<Point> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e412 * other.e2, this.e423 * other.e3, this.e431 * other.e1, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.yzxx * other.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension Plane: GeometricProduct<Scalar> {
    public typedef Plane GeometricProductOutput;
    public func geometric_product(other: Scalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<Point> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<Point> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Point.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension Point: GeometricProduct<AntiScalar> {
    public typedef Plane GeometricProductOutput;
    public func geometric_product(other: AntiScalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        );
    }
}
public extension Point: GeometricProduct<DualNum> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.group0.xy.yy, other.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Point: GeometricProduct<Flector> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e1) - (other.e431 * this.e3), -(other.e4 * this.e2) - (other.e412 * this.e1), -(other.e4 * this.e3) - (other.e423 * this.e2), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.zxyy * this.group0.yzxy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e1 * this.e2) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        );
    }
}
public extension Point: GeometricProduct<Horizon> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Horizon) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e321 * this.e4), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Point: GeometricProduct<Line> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Point: GeometricProduct<Motor> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (other.group1.yzxw * this.group0.zxyw) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e1234 * this.e3) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Point: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Point: GeometricProduct<Origin> {
    public typedef Line GeometricProductOutput;
    public func geometric_product(other: Origin) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Point: GeometricProduct<Plane> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e431 * this.e3 * -1.0, other.e412 * this.e1 * -1.0, other.e423 * this.e2 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension Point: GeometricProduct<Point> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e2 * this.e3 * -1.0, other.e3 * this.e1 * -1.0, other.e1 * this.e2 * -1.0, (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        );
    }
}
public extension Point: GeometricProduct<Scalar> {
    public typedef Point GeometricProductOutput;
    public func geometric_product(other: Scalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricProductInfix) -> GeometricProductInfixPartial<Scalar> {
        return GeometricProductInfixPartial(this);
    }
}
public extension GeometricProductInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Scalar.Output {
        return this.a.geometric_product(rhs);
    }
}
public extension Scalar: GeometricProduct<AntiScalar> {
    public typedef AntiScalar GeometricProductOutput;
    public func geometric_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar: GeometricProduct<DualNum> {
    public typedef DualNum GeometricProductOutput;
    public func geometric_product(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar: GeometricProduct<Flector> {
    public typedef Flector GeometricProductOutput;
    public func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar: GeometricProduct<Horizon> {
    public typedef Horizon GeometricProductOutput;
    public func geometric_product(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
public extension Scalar: GeometricProduct<Line> {
    public typedef Line GeometricProductOutput;
    public func geometric_product(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension Scalar: GeometricProduct<Motor> {
    public typedef Motor GeometricProductOutput;
    public func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar: GeometricProduct<MultiVector> {
    public typedef MultiVector GeometricProductOutput;
    public func geometric_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Scalar: GeometricProduct<Origin> {
    public typedef Origin GeometricProductOutput;
    public func geometric_product(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e4 * this.scalar
        );
    }
}
public extension Scalar: GeometricProduct<Plane> {
    public typedef Plane GeometricProductOutput;
    public func geometric_product(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar: GeometricProduct<Point> {
    public typedef Point GeometricProductOutput;
    public func geometric_product(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar: GeometricProduct<Scalar> {
    public typedef Scalar GeometricProductOutput;
    public func geometric_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
