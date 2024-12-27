extension AntiScalar {
    func div(_rhs: wedge) -> wedge_partial<AntiScalar> {
        wedge_partial(this)
    }
}
extension AntiScalar: Wedge<DualNum> {
    func wedge(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension AntiScalar: Wedge<Motor> {
    func wedge(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension AntiScalar: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension AntiScalar: Wedge<Scalar> {
    func wedge(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension DualNum {
    func div(_rhs: wedge) -> wedge_partial<DualNum> {
        wedge_partial(this)
    }
}
extension DualNum: Wedge<AntiScalar> {
    func wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension DualNum: Wedge<DualNum> {
    func wedge(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
extension DualNum: Wedge<Flector> {
    func wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
extension DualNum: Wedge<Horizon> {
    func wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.scalar * other.e321
        );
    }
}
extension DualNum: Wedge<Line> {
    func wedge(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1
        );
    }
}
extension DualNum: Wedge<Motor> {
    func wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e41, other.e42, other.e43, 1.0) * float4(this.group0.xx, this.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
extension DualNum: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group2, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
        );
    }
}
extension DualNum: Wedge<Origin> {
    func wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.scalar * other.e4
        );
    }
}
extension DualNum: Wedge<Plane> {
    func wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension DualNum: Wedge<Point> {
    func wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
extension DualNum: Wedge<Scalar> {
    func wedge(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0
        );
    }
}
extension Flector {
    func div(_rhs: wedge) -> wedge_partial<Flector> {
        wedge_partial(this)
    }
}
extension Flector: Wedge<DualNum> {
    func wedge(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
extension Flector: Wedge<Flector> {
    func wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
extension Flector: Wedge<Horizon> {
    func wedge(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
extension Flector: Wedge<Line> {
    func wedge(other: Line) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Flector: Wedge<Motor> {
    func wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e4 * other.e23) + (this.e423 * other.scalar), (this.e4 * other.e31) + (this.e431 * other.scalar), (this.e4 * other.e12) + (this.e412 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group0.zxy * other.group0.yzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Flector: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.zxy, other.e23))
        );
    }
}
extension Flector: Wedge<Origin> {
    func wedge(other: Origin) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Flector: Wedge<Plane> {
    func wedge(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)
        );
    }
}
extension Flector: Wedge<Point> {
    func wedge(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * other.e1, this.e4 * other.e2, this.e4 * other.e3, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
extension Flector: Wedge<Scalar> {
    func wedge(other: Scalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
extension Horizon {
    func div(_rhs: wedge) -> wedge_partial<Horizon> {
        wedge_partial(this)
    }
}
extension Horizon: Wedge<DualNum> {
    func wedge(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.scalar * this.e321
        );
    }
}
extension Horizon: Wedge<Flector> {
    func wedge(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
extension Horizon: Wedge<Motor> {
    func wedge(other: Motor) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
extension Horizon: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e321 * other.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.scalar)
        );
    }
}
extension Horizon: Wedge<Origin> {
    func wedge(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
extension Horizon: Wedge<Point> {
    func wedge(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
extension Horizon: Wedge<Scalar> {
    func wedge(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
extension Line {
    func div(_rhs: wedge) -> wedge_partial<Line> {
        wedge_partial(this)
    }
}
extension Line: Wedge<DualNum> {
    func wedge(other: DualNum) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1
        );
    }
}
extension Line: Wedge<Flector> {
    func wedge(other: Flector) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Line: Wedge<Line> {
    func wedge(other: Line) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
extension Line: Wedge<Motor> {
    func wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(this.group0, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group1, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Line: Wedge<Origin> {
    func wedge(other: Origin) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: Wedge<Point> {
    func wedge(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Line: Wedge<Scalar> {
    func wedge(other: Scalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1
        );
    }
}
extension Motor {
    func div(_rhs: wedge) -> wedge_partial<Motor> {
        wedge_partial(this)
    }
}
extension Motor: Wedge<AntiScalar> {
    func wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension Motor: Wedge<DualNum> {
    func wedge(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e41, this.e42, this.e43, 1.0) * float4(other.group0.xx, other.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
extension Motor: Wedge<Flector> {
    func wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar), (other.e4 * this.e31) + (other.e431 * this.scalar), (other.e4 * this.e12) + (other.e412 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Motor: Wedge<Horizon> {
    func wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
extension Motor: Wedge<Line> {
    func wedge(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar, this.scalar, this.scalar, 1.0) * float4(other.group0, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: Wedge<Motor> {
    func wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.scalar)
        );
    }
}
extension Motor: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group2) + (float3(other.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group3) + (float3(other.scalar) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * other.e4) + (this.scalar * other.e423), (this.e31 * other.e4) + (this.scalar * other.e431), (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Motor: Wedge<Origin> {
    func wedge(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: Wedge<Plane> {
    func wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Motor: Wedge<Point> {
    func wedge(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Motor: Wedge<Scalar> {
    func wedge(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
extension MultiVector {
    func div(_rhs: wedge) -> wedge_partial<MultiVector> {
        wedge_partial(this)
    }
}
extension MultiVector: Wedge<AntiScalar> {
    func wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension MultiVector: Wedge<DualNum> {
    func wedge(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group2, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        );
    }
}
extension MultiVector: Wedge<Flector> {
    func wedge(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group0.yzxx * float4(this.group2.zxy, this.e23))
        );
    }
}
extension MultiVector: Wedge<Horizon> {
    func wedge(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.scalar)
        );
    }
}
extension MultiVector: Wedge<Line> {
    func wedge(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension MultiVector: Wedge<Motor> {
    func wedge(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2) + (float3(this.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3) + (float3(this.scalar) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4((other.e23 * this.e4) + (other.scalar * this.e423), (other.e31 * this.e4) + (other.scalar * this.e431), (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.yzx * this.group1.zxy), other.scalar * this.e321) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension MultiVector: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2) + (float3(this.scalar) * other.group2) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3) + (float3(this.scalar) * other.group3) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e42 * this.e3) + (other.e23 * this.e4), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e43 * this.e1) + (other.e31 * this.e4), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group4) + (float4(this.scalar) * other.group4) - (this.group1.yzxx * float4(other.group2.zxy, other.e23)) - float4((this.group2.zxy * other.group1.yzx), other.e31 * this.e2)
        );
    }
}
extension MultiVector: Wedge<Origin> {
    func wedge(other: Origin) -> MultiVector {
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
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<Plane> {
    func wedge(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension MultiVector: Wedge<Point> {
    func wedge(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group2.zxy, this.e23))
        );
    }
}
extension MultiVector: Wedge<Scalar> {
    func wedge(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group2, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        );
    }
}
extension Origin {
    func div(_rhs: wedge) -> wedge_partial<Origin> {
        wedge_partial(this)
    }
}
extension Origin: Wedge<DualNum> {
    func wedge(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.scalar * this.e4
        );
    }
}
extension Origin: Wedge<Flector> {
    func wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Origin: Wedge<Horizon> {
    func wedge(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
extension Origin: Wedge<Line> {
    func wedge(other: Line) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: Wedge<Motor> {
    func wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
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
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: Wedge<Plane> {
    func wedge(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
extension Origin: Wedge<Point> {
    func wedge(other: Point) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Origin: Wedge<Scalar> {
    func wedge(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * other.scalar
        );
    }
}
extension Plane {
    func div(_rhs: wedge) -> wedge_partial<Plane> {
        wedge_partial(this)
    }
}
extension Plane: Wedge<DualNum> {
    func wedge(other: DualNum) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: Wedge<Flector> {
    func wedge(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
    }
}
extension Plane: Wedge<Motor> {
    func wedge(other: Motor) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: Wedge<Origin> {
    func wedge(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
extension Plane: Wedge<Point> {
    func wedge(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
extension Plane: Wedge<Scalar> {
    func wedge(other: Scalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Point {
    func div(_rhs: wedge) -> wedge_partial<Point> {
        wedge_partial(this)
    }
}
extension Point: Wedge<DualNum> {
    func wedge(other: DualNum) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Point: Wedge<Flector> {
    func wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * this.e1 * -1.0, other.e4 * this.e2 * -1.0, other.e4 * this.e3 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
extension Point: Wedge<Horizon> {
    func wedge(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
extension Point: Wedge<Line> {
    func wedge(other: Line) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Point: Wedge<Motor> {
    func wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Point: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.zxy, other.e23))
        );
    }
}
extension Point: Wedge<Origin> {
    func wedge(other: Origin) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Point: Wedge<Plane> {
    func wedge(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
extension Point: Wedge<Point> {
    func wedge(other: Point) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
    }
}
extension Point: Wedge<Scalar> {
    func wedge(other: Scalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Scalar {
    func div(_rhs: wedge) -> wedge_partial<Scalar> {
        wedge_partial(this)
    }
}
extension Scalar: Wedge<AntiScalar> {
    func wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension Scalar: Wedge<DualNum> {
    func wedge(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<Flector> {
    func wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<Horizon> {
    func wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
extension Scalar: Wedge<Line> {
    func wedge(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<Motor> {
    func wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<MultiVector> {
    func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group2, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
        );
    }
}
extension Scalar: Wedge<Origin> {
    func wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e4 * this.scalar
        );
    }
}
extension Scalar: Wedge<Plane> {
    func wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<Point> {
    func wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<Scalar> {
    func wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
