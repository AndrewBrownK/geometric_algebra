extension AntiScalar {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<AntiScalar> {
        anti_wedge_partial(this)
    }
}
extension AntiScalar: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension AntiScalar: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0
        );
    }
}
extension AntiScalar: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
extension AntiScalar: AntiWedge<Horizon> {
    func anti_wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
extension AntiScalar: AntiWedge<Line> {
    func anti_wedge(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1
        );
    }
}
extension AntiScalar: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
    }
}
extension AntiScalar: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
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
        );
    }
}
extension AntiScalar: AntiWedge<Origin> {
    func anti_wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
extension AntiScalar: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension AntiScalar: AntiWedge<Point> {
    func anti_wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
extension AntiScalar: AntiWedge<Scalar> {
    func anti_wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
extension DualNum {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<DualNum> {
        anti_wedge_partial(this)
    }
}
extension DualNum: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0
        );
    }
}
extension DualNum: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234)
        );
    }
}
extension DualNum: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
extension DualNum: AntiWedge<Horizon> {
    func anti_wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
extension DualNum: AntiWedge<Line> {
    func anti_wedge(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1
        );
    }
}
extension DualNum: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e23, other.e31, other.e12, 1.0) * float4(this.group0.yy, this.e1234, (this.scalar * other.e1234) + (this.e1234 * other.scalar))
        );
    }
}
extension DualNum: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
    }
}
extension DualNum: AntiWedge<Origin> {
    func anti_wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
extension DualNum: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension DualNum: AntiWedge<Point> {
    func anti_wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
extension DualNum: AntiWedge<Scalar> {
    func anti_wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
extension Flector {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Flector> {
        anti_wedge_partial(this)
    }
}
extension Flector: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
extension Flector: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
extension Flector: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (other.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group1.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
extension Flector: AntiWedge<Horizon> {
    func anti_wedge(other: Horizon) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
    }
}
extension Flector: AntiWedge<Line> {
    func anti_wedge(other: Line) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41))
        );
    }
}
extension Flector: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
extension Flector: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (this.group1.zxy * other.group4.yzx) - (this.group1.yzx * other.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group1.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
extension Flector: AntiWedge<Origin> {
    func anti_wedge(other: Origin) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
extension Flector: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
    }
}
extension Flector: AntiWedge<Point> {
    func anti_wedge(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
extension Horizon {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Horizon> {
        anti_wedge_partial(this)
    }
}
extension Horizon: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
extension Horizon: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
extension Horizon: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
    }
}
extension Horizon: AntiWedge<Line> {
    func anti_wedge(other: Line) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
extension Horizon: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
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
        );
    }
}
extension Horizon: AntiWedge<Origin> {
    func anti_wedge(other: Origin) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
extension Horizon: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        );
    }
}
extension Horizon: AntiWedge<Point> {
    func anti_wedge(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
extension Line {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Line> {
        anti_wedge_partial(this)
    }
}
extension Line: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1
        );
    }
}
extension Line: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1
        );
    }
}
extension Line: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41))
        );
    }
}
extension Line: AntiWedge<Horizon> {
    func anti_wedge(other: Horizon) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: AntiWedge<Line> {
    func anti_wedge(other: Line) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
extension Line: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e1234, other.e1234, other.e1234, 1.0) * float4(this.group1, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43))
        );
    }
}
extension Line: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41))
        );
    }
}
extension Motor {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Motor> {
        anti_wedge_partial(this)
    }
}
extension Motor: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        );
    }
}
extension Motor: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, 1.0) * float4(other.group0.yy, other.e1234, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
extension Motor: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
extension Motor: AntiWedge<Horizon> {
    func anti_wedge(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
extension Motor: AntiWedge<Line> {
    func anti_wedge(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(other.group1, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
    }
}
extension Motor: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e1234) * other.group0.xyz)), other.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
    }
}
extension Motor: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * other.e1) + (this.e31 * other.e412), (this.e1234 * other.e2) + (this.e12 * other.e423), (this.e1234 * other.e3) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) + (this.group0 * float4(other.group4.www, other.e4)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group2) + (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group3) + (float3(other.e1234) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
    }
}
extension Motor: AntiWedge<Origin> {
    func anti_wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
extension Motor: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension Motor: AntiWedge<Point> {
    func anti_wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
extension Motor: AntiWedge<Scalar> {
    func anti_wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
extension MultiVector {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<MultiVector> {
        anti_wedge_partial(this)
    }
}
extension MultiVector: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> MultiVector {
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
        );
    }
}
extension MultiVector: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group2, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
    }
}
extension MultiVector: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (other.group1.yzx * this.group4.zxy) - (other.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
extension MultiVector: AntiWedge<Horizon> {
    func anti_wedge(other: Horizon) -> MultiVector {
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
        );
    }
}
extension MultiVector: AntiWedge<Line> {
    func anti_wedge(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e1234 * this.e1) + (other.e31 * this.e412), (other.e1234 * this.e2) + (other.e12 * this.e423), (other.e1234 * this.e3) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) + (other.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2) + (float3(this.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3) + (float3(this.e1234) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
    }
}
extension MultiVector: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e43 * this.e412) - (other.e423 * this.e41) - (other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) - (this.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((this.group3.zxy * other.group4.yzx), other.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2) + (float3(this.e1234) * other.group2) + (other.group4.yzx * this.group4.zxy) - (other.group4.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3) + (float3(other.e321) * this.group4.xyz) + (float3(this.e1234) * other.group3) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e1234) * other.group4)
        );
    }
}
extension MultiVector: AntiWedge<Origin> {
    func anti_wedge(other: Origin) -> MultiVector {
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
extension MultiVector: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * other.group0.yzx) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension MultiVector: AntiWedge<Point> {
    func anti_wedge(other: Point) -> MultiVector {
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
extension MultiVector: AntiWedge<Scalar> {
    func anti_wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
extension Origin {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Origin> {
        anti_wedge_partial(this)
    }
}
extension Origin: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
extension Origin: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
extension Origin: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
extension Origin: AntiWedge<Horizon> {
    func anti_wedge(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
extension Origin: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
extension Origin: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
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
extension Origin: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e4 * other.e321
        );
    }
}
extension Plane {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Plane> {
        anti_wedge_partial(this)
    }
}
extension Plane: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
extension Plane: AntiWedge<Horizon> {
    func anti_wedge(other: Horizon) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz
        );
    }
}
extension Plane: AntiWedge<Line> {
    func anti_wedge(other: Line) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41))
        );
    }
}
extension Plane: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: AntiWedge<Origin> {
    func anti_wedge(other: Origin) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e4 * this.e321 * -1.0
        );
    }
}
extension Plane: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)
        );
    }
}
extension Plane: AntiWedge<Point> {
    func anti_wedge(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
extension Point {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Point> {
        anti_wedge_partial(this)
    }
}
extension Point: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Point: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Point: AntiWedge<Flector> {
    func anti_wedge(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
extension Point: AntiWedge<Horizon> {
    func anti_wedge(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
extension Point: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Point: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> MultiVector {
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
extension Point: AntiWedge<Plane> {
    func anti_wedge(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
extension Scalar {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Scalar> {
        anti_wedge_partial(this)
    }
}
extension Scalar: AntiWedge<AntiScalar> {
    func anti_wedge(other: AntiScalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
extension Scalar: AntiWedge<DualNum> {
    func anti_wedge(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
extension Scalar: AntiWedge<Motor> {
    func anti_wedge(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
extension Scalar: AntiWedge<MultiVector> {
    func anti_wedge(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
