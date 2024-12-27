extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiScalar> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiScalar: Wedge<DualNum> {
    associatedtype Output = AntiScalar;
    func wedge(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension AntiScalar: Wedge<Motor> {
    associatedtype Output = AntiScalar;
    func wedge(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension AntiScalar: Wedge<MultiVector> {
    associatedtype Output = AntiScalar;
    func wedge(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension AntiScalar: Wedge<Scalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<DualNum> {
        return WedgeInfixPartial { a: this };
    }
}
extension DualNum: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension DualNum: Wedge<DualNum> {
    associatedtype Output = DualNum;
    func wedge(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
extension DualNum: Wedge<Flector> {
    associatedtype Output = Flector;
    func wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
extension DualNum: Wedge<Horizon> {
    associatedtype Output = Horizon;
    func wedge(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.scalar * other.e321
        );
    }
}
extension DualNum: Wedge<Line> {
    associatedtype Output = Line;
    func wedge(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1
        );
    }
}
extension DualNum: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e41, other.e42, other.e43, 1.0) * float4(this.group0.xx, this.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
extension DualNum: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = Origin;
    func wedge(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4 */
            this.scalar * other.e4
        );
    }
}
extension DualNum: Wedge<Plane> {
    associatedtype Output = Plane;
    func wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension DualNum: Wedge<Point> {
    associatedtype Output = Point;
    func wedge(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
extension DualNum: Wedge<Scalar> {
    associatedtype Output = DualNum;
    func wedge(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Flector> {
        return WedgeInfixPartial { a: this };
    }
}
extension Flector: Wedge<DualNum> {
    associatedtype Output = Flector;
    func wedge(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
extension Flector: Wedge<Flector> {
    associatedtype Output = Motor;
    func wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
extension Flector: Wedge<Horizon> {
    associatedtype Output = AntiScalar;
    func wedge(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
extension Flector: Wedge<Line> {
    associatedtype Output = Plane;
    func wedge(other: Line) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Flector: Wedge<Motor> {
    associatedtype Output = Flector;
    func wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e4 * other.e23) + (this.e423 * other.scalar), (this.e4 * other.e31) + (this.e431 * other.scalar), (this.e4 * other.e12) + (this.e412 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group0.zxy * other.group0.yzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Flector: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = Motor;
    func wedge(other: Origin) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Flector: Wedge<Plane> {
    associatedtype Output = AntiScalar;
    func wedge(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)
        );
    }
}
extension Flector: Wedge<Point> {
    associatedtype Output = Motor;
    func wedge(other: Point) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * other.e1, this.e4 * other.e2, this.e4 * other.e3, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
extension Flector: Wedge<Scalar> {
    associatedtype Output = Flector;
    func wedge(other: Scalar) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Horizon> {
        return WedgeInfixPartial { a: this };
    }
}
extension Horizon: Wedge<DualNum> {
    associatedtype Output = Horizon;
    func wedge(other: DualNum) -> Output {
        return Horizon.from_groups(
            /* e321 */
            other.scalar * this.e321
        );
    }
}
extension Horizon: Wedge<Flector> {
    associatedtype Output = AntiScalar;
    func wedge(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
extension Horizon: Wedge<Motor> {
    associatedtype Output = Horizon;
    func wedge(other: Motor) -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
extension Horizon: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = AntiScalar;
    func wedge(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
extension Horizon: Wedge<Point> {
    associatedtype Output = AntiScalar;
    func wedge(other: Point) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
extension Horizon: Wedge<Scalar> {
    associatedtype Output = Horizon;
    func wedge(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Line> {
        return WedgeInfixPartial { a: this };
    }
}
extension Line: Wedge<DualNum> {
    associatedtype Output = Line;
    func wedge(other: DualNum) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1
        );
    }
}
extension Line: Wedge<Flector> {
    associatedtype Output = Plane;
    func wedge(other: Flector) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Line: Wedge<Line> {
    associatedtype Output = AntiScalar;
    func wedge(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
extension Line: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(this.group0, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group1, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = Plane;
    func wedge(other: Origin) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: Wedge<Point> {
    associatedtype Output = Plane;
    func wedge(other: Point) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Line: Wedge<Scalar> {
    associatedtype Output = Line;
    func wedge(other: Scalar) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Motor> {
        return WedgeInfixPartial { a: this };
    }
}
extension Motor: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension Motor: Wedge<DualNum> {
    associatedtype Output = Motor;
    func wedge(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e41, this.e42, this.e43, 1.0) * float4(other.group0.xx, other.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
extension Motor: Wedge<Flector> {
    associatedtype Output = Flector;
    func wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar), (other.e4 * this.e31) + (other.e431 * this.scalar), (other.e4 * this.e12) + (other.e412 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Motor: Wedge<Horizon> {
    associatedtype Output = Horizon;
    func wedge(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
extension Motor: Wedge<Line> {
    associatedtype Output = Motor;
    func wedge(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar, this.scalar, this.scalar, 1.0) * float4(other.group0, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.scalar)
        );
    }
}
extension Motor: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = Flector;
    func wedge(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: Wedge<Plane> {
    associatedtype Output = Plane;
    func wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Motor: Wedge<Point> {
    associatedtype Output = Flector;
    func wedge(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Motor: Wedge<Scalar> {
    associatedtype Output = Motor;
    func wedge(other: Scalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<MultiVector> {
        return WedgeInfixPartial { a: this };
    }
}
extension MultiVector: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension MultiVector: Wedge<DualNum> {
    associatedtype Output = MultiVector;
    func wedge(other: DualNum) -> Output {
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
    associatedtype Output = MultiVector;
    func wedge(other: Flector) -> Output {
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
    associatedtype Output = MultiVector;
    func wedge(other: Horizon) -> Output {
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
    associatedtype Output = MultiVector;
    func wedge(other: Line) -> Output {
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
    associatedtype Output = MultiVector;
    func wedge(other: Motor) -> Output {
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
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = MultiVector;
    func wedge(other: Origin) -> Output {
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
    associatedtype Output = MultiVector;
    func wedge(other: Plane) -> Output {
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
    associatedtype Output = MultiVector;
    func wedge(other: Point) -> Output {
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
    associatedtype Output = MultiVector;
    func wedge(other: Scalar) -> Output {
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
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Origin> {
        return WedgeInfixPartial { a: this };
    }
}
extension Origin: Wedge<DualNum> {
    associatedtype Output = Origin;
    func wedge(other: DualNum) -> Output {
        return Origin.from_groups(
            /* e4 */
            other.scalar * this.e4
        );
    }
}
extension Origin: Wedge<Flector> {
    associatedtype Output = Motor;
    func wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Origin: Wedge<Horizon> {
    associatedtype Output = AntiScalar;
    func wedge(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
extension Origin: Wedge<Line> {
    associatedtype Output = Plane;
    func wedge(other: Line) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: Wedge<Motor> {
    associatedtype Output = Flector;
    func wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = AntiScalar;
    func wedge(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
extension Origin: Wedge<Point> {
    associatedtype Output = Line;
    func wedge(other: Point) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Origin: Wedge<Scalar> {
    associatedtype Output = Origin;
    func wedge(other: Scalar) -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4 * other.scalar
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Plane> {
        return WedgeInfixPartial { a: this };
    }
}
extension Plane: Wedge<DualNum> {
    associatedtype Output = Plane;
    func wedge(other: DualNum) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: Wedge<Flector> {
    associatedtype Output = AntiScalar;
    func wedge(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
    }
}
extension Plane: Wedge<Motor> {
    associatedtype Output = Plane;
    func wedge(other: Motor) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = AntiScalar;
    func wedge(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
extension Plane: Wedge<Point> {
    associatedtype Output = AntiScalar;
    func wedge(other: Point) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
extension Plane: Wedge<Scalar> {
    associatedtype Output = Plane;
    func wedge(other: Scalar) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Point> {
        return WedgeInfixPartial { a: this };
    }
}
extension Point: Wedge<DualNum> {
    associatedtype Output = Point;
    func wedge(other: DualNum) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Point: Wedge<Flector> {
    associatedtype Output = Motor;
    func wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * this.e1 * -1.0, other.e4 * this.e2 * -1.0, other.e4 * this.e3 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
extension Point: Wedge<Horizon> {
    associatedtype Output = AntiScalar;
    func wedge(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
extension Point: Wedge<Line> {
    associatedtype Output = Plane;
    func wedge(other: Line) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Point: Wedge<Motor> {
    associatedtype Output = Flector;
    func wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Point: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = Line;
    func wedge(other: Origin) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Point: Wedge<Plane> {
    associatedtype Output = AntiScalar;
    func wedge(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
extension Point: Wedge<Point> {
    associatedtype Output = Line;
    func wedge(other: Point) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
    }
}
extension Point: Wedge<Scalar> {
    associatedtype Output = Point;
    func wedge(other: Scalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Scalar> {
        return WedgeInfixPartial { a: this };
    }
}
extension Scalar: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension Scalar: Wedge<DualNum> {
    associatedtype Output = DualNum;
    func wedge(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<Flector> {
    associatedtype Output = Flector;
    func wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<Horizon> {
    associatedtype Output = Horizon;
    func wedge(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
extension Scalar: Wedge<Line> {
    associatedtype Output = Line;
    func wedge(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
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
    associatedtype Output = Origin;
    func wedge(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4 */
            other.e4 * this.scalar
        );
    }
}
extension Scalar: Wedge<Plane> {
    associatedtype Output = Plane;
    func wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<Point> {
    associatedtype Output = Point;
    func wedge(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<Scalar> {
    associatedtype Output = Scalar;
    func wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
