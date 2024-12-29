implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<AntiScalar> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<AntiScalar>(this);
    }
}
public extension GeometricAntiProductInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension AntiScalar: GeometricAntiProduct<AntiScalar> {
    public typedef AntiScalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<DualNum> {
    public typedef DualNum GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0.xy
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<Flector> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<Horizon> {
    public typedef Horizon GeometricAntiProductOutput;
    public func geometric_anti_product(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<Line> {
    public typedef Line GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<Motor> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<Origin> {
    public typedef Origin GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<Plane> {
    public typedef Plane GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<Point> {
    public typedef Point GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension AntiScalar: GeometricAntiProduct<Scalar> {
    public typedef Scalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<DualNum> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<DualNum>(this);
    }
}
public extension GeometricAntiProductInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public DualNum operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension DualNum: GeometricAntiProduct<AntiScalar> {
    public typedef DualNum GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0.xy
        );
    }
}
public extension DualNum: GeometricAntiProduct<DualNum> {
    public typedef DualNum GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234)
        );
    }
}
public extension DualNum: GeometricAntiProduct<Flector> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(this.e1234) * other.group0.xyz)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321)) * float4(other.group1.xyz, 1.0)
        );
    }
}
public extension DualNum: GeometricAntiProduct<Horizon> {
    public typedef Horizon GeometricAntiProductOutput;
    public func geometric_anti_product(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
public extension DualNum: GeometricAntiProduct<Line> {
    public typedef Line GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz)
        );
    }
}
public extension DualNum: GeometricAntiProduct<Motor> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1)
        );
    }
}
public extension DualNum: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension DualNum: GeometricAntiProduct<Origin> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        );
    }
}
public extension DualNum: GeometricAntiProduct<Plane> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension DualNum: GeometricAntiProduct<Point> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        );
    }
}
public extension DualNum: GeometricAntiProduct<Scalar> {
    public typedef Scalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Flector> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Flector>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Flector operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Flector: GeometricAntiProduct<AntiScalar> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
public extension Flector: GeometricAntiProduct<DualNum> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) - (float3(other.scalar) * this.group1.xyz)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xy.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4)) * float4(this.group1.xyz, 1.0)
        );
    }
}
public extension Flector: GeometricAntiProduct<Flector> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e423 * this.e4) - (other.e412 * this.e431), -(other.e423 * this.e412) - (other.e431 * this.e4), -(other.e431 * this.e423) - (other.e412 * this.e4), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group1.zxyx) - (float4(other.e4) * float4(this.group1.xyz, this.e4)), 
            /* e23, e31, e12, scalar */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (float4(this.e4, this.e412, this.e423, this.e1) * float4(other.group0.xxy, other.e423)) + (float4(this.e431, this.e4, this.e4, this.e2) * float4(other.group0.zyz, other.e431)) + (other.group1.yzxz * this.group0.zxyz) - (float4(this.e2, this.e321, this.e321, this.e321) * float4(other.group1.zyz, other.e4)) - (float4(this.e321, this.e3, this.e1, this.e412) * float4(other.group1.xxy, other.e3)) - (other.group0.yzxx * this.group1.zxyx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431))
        );
    }
}
public extension Flector: GeometricAntiProduct<Horizon> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Horizon) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
    }
}
public extension Flector: GeometricAntiProduct<Line> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xyz.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.xyz.zyz, other.e43)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e423 * other.e41), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xyz.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.xyz.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group0.xyz.yzx * this.group1.zxy), this.e1 * other.e41)
        );
    }
}
public extension Flector: GeometricAntiProduct<Motor> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e412 * other.e23), (this.e3 * other.e1234) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e412 * other.scalar), 0.0) + (this.group0.xyxw * other.group0.wwyw) - (this.group1.xyxz * float4(other.group1.wwy, other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e423)) - float4((this.group0.www * other.group1.xyz), this.e431 * other.e42), 
            /* e423, e431, e412, e321 */
            (this.group1.xyxy * float4(other.group0.wwy, other.e31)) + (this.group1.yzzz * float4(other.group0.zxw, other.e12)) + float4(float3(0.0), (this.e321 * other.e1234) - (this.e2 * other.e42) - (this.e3 * other.e43) - (this.e4 * other.scalar)) + float4((this.group0.www * other.group0.xyz), this.e423 * other.e23) - (other.group0.yzxx * float4(this.group1.zxy, this.e1))
        );
    }
}
public extension Flector: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Flector: GeometricAntiProduct<Origin> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
public extension Flector: GeometricAntiProduct<Plane> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e4 * other.e423) - (this.e431 * other.e412), -(this.e4 * other.e431) - (this.e412 * other.e423), -(this.e4 * other.e412) - (this.e423 * other.e431), (this.e431 * other.e431) + (this.e412 * other.e412)) + (this.group1.zxyx * other.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(this.e2 * other.e412) - (this.e321 * other.e423), -(this.e3 * other.e423) - (this.e321 * other.e431), -(this.e1 * other.e431) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.zxyx * other.group0.yzxx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2))
        );
    }
}
public extension Flector: GeometricAntiProduct<Point> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((this.e4 * other.e1) + (this.e431 * other.e3), (this.e4 * other.e2) + (this.e412 * other.e1), (this.e4 * other.e3) + (this.e423 * other.e2), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.zxyy * other.group0.yzxy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423))
        );
    }
}
public extension Flector: GeometricAntiProduct<Scalar> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Scalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Horizon> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Horizon>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Horizon operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Horizon: GeometricAntiProduct<AntiScalar> {
    public typedef Horizon GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
public extension Horizon: GeometricAntiProduct<DualNum> {
    public typedef Horizon GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
public extension Horizon: GeometricAntiProduct<Flector> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
    }
}
public extension Horizon: GeometricAntiProduct<Line> {
    public typedef Point GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
public extension Horizon: GeometricAntiProduct<Motor> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
public extension Horizon: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Horizon: GeometricAntiProduct<Origin> {
    public typedef Scalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Horizon: GeometricAntiProduct<Plane> {
    public typedef Line GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        );
    }
}
public extension Horizon: GeometricAntiProduct<Point> {
    public typedef Scalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Line> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Line>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Line operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Line: GeometricAntiProduct<AntiScalar> {
    public typedef Line GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz
        );
    }
}
public extension Line: GeometricAntiProduct<DualNum> {
    public typedef Line GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz)
        );
    }
}
public extension Line: GeometricAntiProduct<Flector> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) - (other.group1.yzxy * float4(this.group1.xyz.zxy, this.e42)) - float4((this.group0.xyz.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e41) + (other.e412 * this.e42), (other.e4 * this.e42) + (other.e423 * this.e43), (other.e4 * this.e43) + (other.e431 * this.e41), -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group0.xyz.zxy * other.group1.yzx), other.e1 * this.e41)
        );
    }
}
public extension Line: GeometricAntiProduct<Horizon> {
    public typedef Point GeometricAntiProductOutput;
    public func geometric_anti_product(other: Horizon) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0)
        );
    }
}
public extension Line: GeometricAntiProduct<Line> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e43 * this.e42, other.e41 * this.e43, other.e42 * this.e41, -(other.e42 * this.e42) - (other.e43 * this.e43)) - float4((other.group0.xyz.yzx * this.group0.xyz.zxy), other.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((other.e43 * this.e31) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e23 * this.e43), (other.e42 * this.e23) + (other.e31 * this.e41), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e41 * this.e23) - float4((other.group1.xyz.yzx * this.group0.xyz.zxy), other.e42 * this.e31)
        );
    }
}
public extension Line: GeometricAntiProduct<Motor> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.e1234) + (this.e42 * other.e43), (this.e42 * other.e1234) + (this.e43 * other.e41), (this.e41 * other.e42) + (this.e43 * other.e1234), -(this.e42 * other.e42) - (this.e43 * other.e43)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e41)), 
            /* e23, e31, e12, scalar */
            float4((this.e41 * other.scalar) + (this.e42 * other.e12) + (this.e23 * other.e1234) + (this.e31 * other.e43), (this.e42 * other.scalar) + (this.e43 * other.e23) + (this.e31 * other.e1234) + (this.e12 * other.e41), (this.e41 * other.e31) + (this.e43 * other.scalar) + (this.e23 * other.e42) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e41)) - float4((this.group1.xyz.zxy * other.group0.yzx), this.e42 * other.e31)
        );
    }
}
public extension Line: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Line: GeometricAntiProduct<Origin> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), 0.0)
        );
    }
}
public extension Line: GeometricAntiProduct<Plane> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Line: GeometricAntiProduct<Point> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.xyz.yzx * other.group0.zxy), 0.0) - float4((this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), -(this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        );
    }
}
public extension Line: GeometricAntiProduct<Scalar> {
    public typedef Line GeometricAntiProductOutput;
    public func geometric_anti_product(other: Scalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Motor> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Motor>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Motor: GeometricAntiProduct<AntiScalar> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        );
    }
}
public extension Motor: GeometricAntiProduct<DualNum> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1)
        );
    }
}
public extension Motor: GeometricAntiProduct<Flector> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e431 * this.scalar) + (other.e321 * this.e42), (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e412 * this.scalar) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (other.group0.xxyw * this.group0.wzxw) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - (this.group0.zxyx * float4(other.group0.yzx, other.e423)), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e431 * this.e1234, other.e412 * this.e1234, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(other.e4) * float4(this.group0.xyz, this.scalar)) + (other.group1.xxyw * this.group0.wzxw) - (this.group0.zxyx * float4(other.group1.yzx, other.e1))
        );
    }
}
public extension Motor: GeometricAntiProduct<Horizon> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
public extension Motor: GeometricAntiProduct<Line> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.e1234) + (other.e43 * this.e42), (other.e41 * this.e43) + (other.e42 * this.e1234), (other.e42 * this.e41) + (other.e43 * this.e1234), -(other.e42 * this.e42) - (other.e43 * this.e43)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e41)), 
            /* e23, e31, e12, scalar */
            float4((other.e41 * this.scalar) + (other.e43 * this.e31) + (other.e23 * this.e1234) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e42 * this.scalar) + (other.e23 * this.e43) + (other.e31 * this.e1234), (other.e42 * this.e23) + (other.e43 * this.scalar) + (other.e31 * this.e41) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.zxyx * float4(other.group0.xyz.yzx, other.e41)) - float4((other.group1.xyz.yzx * this.group0.zxy), other.e42 * this.e31)
        );
    }
}
public extension Motor: GeometricAntiProduct<Motor> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e43 * this.e42) + (other.e1234 * this.e41), (other.e42 * this.e1234) + (other.e1234 * this.e42), (other.e43 * this.e1234) + (other.e1234 * this.e43), -(other.e42 * this.e42) - (other.e43 * this.e43)) + (other.group0.xxyw * this.group0.wzxw) - (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e12 * this.e42) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e23 * this.e43) + (other.e31 * this.e1234) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e31 * this.e41) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xxyw * this.group1.wzxw) + float4((other.group0.zyz * this.group1.yww), other.scalar * this.e1234) - (other.group0.yzxx * this.group1.zxyx) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        );
    }
}
public extension Motor: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Motor: GeometricAntiProduct<Origin> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group1.xyz, this.e1234), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group0.xyz, this.scalar)
        );
    }
}
public extension Motor: GeometricAntiProduct<Plane> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e42 * other.e321) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e23 * other.e431) + (this.scalar * other.e412), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.yzxw * other.group0.zxyw) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension Motor: GeometricAntiProduct<Point> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        );
    }
}
public extension Motor: GeometricAntiProduct<Scalar> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<MultiVector> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<MultiVector>(this);
    }
}
public extension GeometricAntiProductInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension MultiVector: GeometricAntiProduct<AntiScalar> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<DualNum> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<Flector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<Horizon> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Horizon) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<Line> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<Motor> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<Origin> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<Plane> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<Point> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> MultiVector {
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
        );
    }
}
public extension MultiVector: GeometricAntiProduct<Scalar> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Scalar) -> MultiVector {
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
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Origin> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Origin>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public Origin operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Origin: GeometricAntiProduct<AntiScalar> {
    public typedef Origin GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
public extension Origin: GeometricAntiProduct<DualNum> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        );
    }
}
public extension Origin: GeometricAntiProduct<Flector> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e4) * float4(other.group0.xyz, other.e321)
        );
    }
}
public extension Origin: GeometricAntiProduct<Horizon> {
    public typedef Scalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
public extension Origin: GeometricAntiProduct<Line> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e4) * other.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        );
    }
}
public extension Origin: GeometricAntiProduct<Motor> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group1.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group0.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension Origin: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Origin: GeometricAntiProduct<Origin> {
    public typedef AntiScalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4 * -1.0
        );
    }
}
public extension Origin: GeometricAntiProduct<Plane> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e4) * other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e4 * other.e321)
        );
    }
}
public extension Origin: GeometricAntiProduct<Point> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        );
    }
}
public extension Origin: GeometricAntiProduct<Scalar> {
    public typedef Horizon GeometricAntiProductOutput;
    public func geometric_anti_product(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * other.scalar * -1.0
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Plane> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Plane>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Plane: GeometricAntiProduct<AntiScalar> {
    public typedef Plane GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Plane: GeometricAntiProduct<DualNum> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Plane: GeometricAntiProduct<Flector> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e423) - (other.e412 * this.e431), -(other.e4 * this.e431) - (other.e423 * this.e412), -(other.e4 * this.e412) - (other.e431 * this.e423), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e3 * this.e431) + (other.e321 * this.e423), (other.e1 * this.e412) + (other.e321 * this.e431), (other.e2 * this.e423) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.yzxx * this.group0.zxyx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2))
        );
    }
}
public extension Plane: GeometricAntiProduct<Horizon> {
    public typedef Line GeometricAntiProductOutput;
    public func geometric_anti_product(other: Horizon) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz
        );
    }
}
public extension Plane: GeometricAntiProduct<Line> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Plane: GeometricAntiProduct<Motor> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzy * float4(other.group1.www, other.e42)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (other.group0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(other.group0.www, other.e23))
        );
    }
}
public extension Plane: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Plane: GeometricAntiProduct<Origin> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e4 * this.e321 * -1.0)
        );
    }
}
public extension Plane: GeometricAntiProduct<Plane> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e412 * this.e431 * -1.0, other.e423 * this.e412 * -1.0, other.e431 * this.e423 * -1.0, (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0)
        );
    }
}
public extension Plane: GeometricAntiProduct<Point> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e431 * other.e3, this.e412 * other.e1, this.e423 * other.e2, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.zxyx * other.group0.yzxx)
        );
    }
}
public extension Plane: GeometricAntiProduct<Scalar> {
    public typedef Point GeometricAntiProductOutput;
    public func geometric_anti_product(other: Scalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Point> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Point>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Point operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Point: GeometricAntiProduct<AntiScalar> {
    public typedef Point GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Point: GeometricAntiProduct<DualNum> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        );
    }
}
public extension Point: GeometricAntiProduct<Flector> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(-(other.e4 * this.e1) - (other.e412 * this.e2), -(other.e4 * this.e2) - (other.e423 * this.e3), -(other.e4 * this.e3) - (other.e431 * this.e1), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423))
        );
    }
}
public extension Point: GeometricAntiProduct<Horizon> {
    public typedef Scalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
public extension Point: GeometricAntiProduct<Line> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * other.group1.xyz), 0.0) - float4((other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        );
    }
}
public extension Point: GeometricAntiProduct<Motor> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group0.zxy * this.group0.yzx) - (float3(this.e4) * other.group1.xyz) - (other.group0.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4))
        );
    }
}
public extension Point: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Point: GeometricAntiProduct<Origin> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Point: GeometricAntiProduct<Plane> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e412 * this.e2 * -1.0, other.e423 * this.e3 * -1.0, other.e431 * this.e1 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.yzxx * this.group0.zxyx)
        );
    }
}
public extension Point: GeometricAntiProduct<Point> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0)
        );
    }
}
public extension Point: GeometricAntiProduct<Scalar> {
    public typedef Horizon GeometricAntiProductOutput;
    public func geometric_anti_product(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * other.scalar * -1.0
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public GeometricAntiProductInfixPartial<Scalar> operator /(GeometricAntiProductInfix rhs) {
        return GeometricAntiProductInfixPartial<Scalar>(this);
    }
}
public extension GeometricAntiProductInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiScalar rhs) {
        return this.a.geometric_anti_product(rhs);
    }
}
public extension Scalar: GeometricAntiProduct<AntiScalar> {
    public typedef Scalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: AntiScalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar: GeometricAntiProduct<DualNum> {
    public typedef Scalar GeometricAntiProductOutput;
    public func geometric_anti_product(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar: GeometricAntiProduct<Flector> {
    public typedef Flector GeometricAntiProductOutput;
    public func geometric_anti_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        );
    }
}
public extension Scalar: GeometricAntiProduct<Line> {
    public typedef Line GeometricAntiProductOutput;
    public func geometric_anti_product(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz
        );
    }
}
public extension Scalar: GeometricAntiProduct<Motor> {
    public typedef Motor GeometricAntiProductOutput;
    public func geometric_anti_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar: GeometricAntiProduct<MultiVector> {
    public typedef MultiVector GeometricAntiProductOutput;
    public func geometric_anti_product(other: MultiVector) -> MultiVector {
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
        );
    }
}
public extension Scalar: GeometricAntiProduct<Origin> {
    public typedef Horizon GeometricAntiProductOutput;
    public func geometric_anti_product(other: Origin) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e4 * this.scalar
        );
    }
}
public extension Scalar: GeometricAntiProduct<Plane> {
    public typedef Point GeometricAntiProductOutput;
    public func geometric_anti_product(other: Plane) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        );
    }
}
public extension Scalar: GeometricAntiProduct<Point> {
    public typedef Horizon GeometricAntiProductOutput;
    public func geometric_anti_product(other: Point) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e4 * this.scalar
        );
    }
}
