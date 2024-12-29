implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiScalar> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiScalar>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiScalar: AntiWedge<AntiScalar> {
    public typedef AntiScalar AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension AntiScalar: AntiWedge<DualNum> {
    public typedef DualNum AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0.xy
        );
    }
}
public extension AntiScalar: AntiWedge<Flector> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
public extension AntiScalar: AntiWedge<Horizon> {
    public typedef Horizon AntiWedgeOutput;
    public func anti_wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
public extension AntiScalar: AntiWedge<Line> {
    public typedef Line AntiWedgeOutput;
    public func anti_wedge(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
    }
}
public extension AntiScalar: AntiWedge<Motor> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
    }
}
public extension AntiScalar: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
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
public extension AntiScalar: AntiWedge<Origin> {
    public typedef Origin AntiWedgeOutput;
    public func anti_wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
public extension AntiScalar: AntiWedge<Plane> {
    public typedef Plane AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension AntiScalar: AntiWedge<Point> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension AntiScalar: AntiWedge<Scalar> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<DualNum> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<DualNum>(this);
    }
}
public extension AntiWedgeInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public DualNum operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension DualNum: AntiWedge<AntiScalar> {
    public typedef DualNum AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0.xy
        );
    }
}
public extension DualNum: AntiWedge<DualNum> {
    public typedef DualNum AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234)
        );
    }
}
public extension DualNum: AntiWedge<Flector> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
public extension DualNum: AntiWedge<Horizon> {
    public typedef Horizon AntiWedgeOutput;
    public func anti_wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
public extension DualNum: AntiWedge<Line> {
    public typedef Line AntiWedgeOutput;
    public func anti_wedge(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
    }
}
public extension DualNum: AntiWedge<Motor> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group1.xyz, 1.0)
        );
    }
}
public extension DualNum: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 
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
public extension DualNum: AntiWedge<Origin> {
    public typedef Origin AntiWedgeOutput;
    public func anti_wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
public extension DualNum: AntiWedge<Plane> {
    public typedef Plane AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension DualNum: AntiWedge<Point> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension DualNum: AntiWedge<Scalar> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Flector> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Flector>(this);
    }
}
public extension AntiWedgeInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Flector operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Flector: AntiWedge<AntiScalar> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
public extension Flector: AntiWedge<DualNum> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
public extension Flector: AntiWedge<Flector> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (other.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group1.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
public extension Flector: AntiWedge<Horizon> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Horizon) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
    }
}
public extension Flector: AntiWedge<Line> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Line) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
    }
}
public extension Flector: AntiWedge<Motor> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
public extension Flector: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            (this.group1.zxy * other.group4.yzx) - (this.group1.yzx * other.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group1.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
public extension Flector: AntiWedge<Origin> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Origin) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Flector: AntiWedge<Plane> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
    }
}
public extension Flector: AntiWedge<Point> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Horizon> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Horizon>(this);
    }
}
public extension AntiWedgeInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Horizon operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Horizon: AntiWedge<AntiScalar> {
    public typedef Horizon AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
public extension Horizon: AntiWedge<DualNum> {
    public typedef Horizon AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
public extension Horizon: AntiWedge<Flector> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
    }
}
public extension Horizon: AntiWedge<Line> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Line) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
public extension Horizon: AntiWedge<Motor> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
public extension Horizon: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
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
public extension Horizon: AntiWedge<Origin> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Origin) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Horizon: AntiWedge<Plane> {
    public typedef Line AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        );
    }
}
public extension Horizon: AntiWedge<Point> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Line> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Line>(this);
    }
}
public extension AntiWedgeInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Line operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Line: AntiWedge<AntiScalar> {
    public typedef Line AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz
        );
    }
}
public extension Line: AntiWedge<DualNum> {
    public typedef Line AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz
        );
    }
}
public extension Line: AntiWedge<Flector> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) - (other.group1.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
    }
}
public extension Line: AntiWedge<Horizon> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Horizon) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0)
        );
    }
}
public extension Line: AntiWedge<Line> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Line) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension Line: AntiWedge<Motor> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43))
        );
    }
}
public extension Line: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group4.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line: AntiWedge<Plane> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Motor> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Motor>(this);
    }
}
public extension AntiWedgeInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Motor: AntiWedge<AntiScalar> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        );
    }
}
public extension Motor: AntiWedge<DualNum> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.group0.xy.yy, other.e1234, (other.scalar * this.e1234) + (other.e1234 * this.scalar)) * float4(this.group1.xyz, 1.0)
        );
    }
}
public extension Motor: AntiWedge<Flector> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
public extension Motor: AntiWedge<Horizon> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
public extension Motor: AntiWedge<Line> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
    }
}
public extension Motor: AntiWedge<Motor> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e1234) * other.group0.xyz)), other.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
    }
}
public extension Motor: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * other.e1) + (this.e31 * other.e412), (this.e1234 * other.e2) + (this.e12 * other.e423), (this.e1234 * other.e3) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) + (this.group0 * float4(other.group4.www, other.e4)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group2.xyz) + (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group3.xyz) + (float3(other.e1234) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
    }
}
public extension Motor: AntiWedge<Origin> {
    public typedef Origin AntiWedgeOutput;
    public func anti_wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
public extension Motor: AntiWedge<Plane> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension Motor: AntiWedge<Point> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension Motor: AntiWedge<Scalar> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<MultiVector> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<MultiVector>(this);
    }
}
public extension AntiWedgeInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension MultiVector: AntiWedge<AntiScalar> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> MultiVector {
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
public extension MultiVector: AntiWedge<DualNum> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 
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
public extension MultiVector: AntiWedge<Flector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (other.group1.yzx * this.group4.zxy) - (other.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
public extension MultiVector: AntiWedge<Horizon> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: Horizon) -> MultiVector {
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
public extension MultiVector: AntiWedge<Line> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group4.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiWedge<Motor> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e1234 * this.e1) + (other.e31 * this.e412), (other.e1234 * this.e2) + (other.e12 * this.e423), (other.e1234 * this.e3) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) + (other.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3.xyz) + (float3(this.e1234) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
    }
}
public extension MultiVector: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e43 * this.e412) - (other.e423 * this.e41) - (other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) - (this.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((this.group3.xyz.zxy * other.group4.yzx), other.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group2.xyz) + (other.group4.yzx * this.group4.zxy) - (other.group4.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.e1234) * other.group3.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e1234) * other.group4)
        );
    }
}
public extension MultiVector: AntiWedge<Origin> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiWedge<Plane> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * other.group0.yzx) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
public extension MultiVector: AntiWedge<Point> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiWedge<Scalar> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Origin> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Origin>(this);
    }
}
public extension AntiWedgeInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public Origin operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Origin: AntiWedge<AntiScalar> {
    public typedef Origin AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
public extension Origin: AntiWedge<DualNum> {
    public typedef Origin AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
public extension Origin: AntiWedge<Flector> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
public extension Origin: AntiWedge<Horizon> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
public extension Origin: AntiWedge<Motor> {
    public typedef Origin AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
public extension Origin: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin: AntiWedge<Plane> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e4 * other.e321
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Plane> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Plane>(this);
    }
}
public extension AntiWedgeInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Plane: AntiWedge<AntiScalar> {
    public typedef Plane AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Plane: AntiWedge<DualNum> {
    public typedef Plane AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Plane: AntiWedge<Flector> {
    public typedef Motor AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
public extension Plane: AntiWedge<Horizon> {
    public typedef Line AntiWedgeOutput;
    public func anti_wedge(other: Horizon) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz
        );
    }
}
public extension Plane: AntiWedge<Line> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Line) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
    }
}
public extension Plane: AntiWedge<Motor> {
    public typedef Flector AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Plane: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43 */
            (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Plane: AntiWedge<Origin> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Origin) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e4 * this.e321 * -1.0
        );
    }
}
public extension Plane: AntiWedge<Plane> {
    public typedef Line AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)
        );
    }
}
public extension Plane: AntiWedge<Point> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Point> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Point>(this);
    }
}
public extension AntiWedgeInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Point operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Point: AntiWedge<AntiScalar> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Point: AntiWedge<DualNum> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Point: AntiWedge<Flector> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension Point: AntiWedge<Horizon> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
public extension Point: AntiWedge<Motor> {
    public typedef Point AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
public extension Point: AntiWedge<MultiVector> {
    public typedef MultiVector AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point: AntiWedge<Plane> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Scalar> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Scalar>(this);
    }
}
public extension AntiWedgeInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Scalar: AntiWedge<AntiScalar> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: AntiScalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar: AntiWedge<DualNum> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar: AntiWedge<Motor> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar: AntiWedge<MultiVector> {
    public typedef Scalar AntiWedgeOutput;
    public func anti_wedge(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
