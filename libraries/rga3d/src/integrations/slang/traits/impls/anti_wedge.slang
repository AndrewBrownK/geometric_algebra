extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiScalar> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiScalar: AntiWedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<DualNum> {
    associatedtype Output = DualNum;
    func anti_wedge(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e1234) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<Flector> {
    associatedtype Output = Flector;
    func anti_wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
extension AntiScalar: AntiWedge<Horizon> {
    associatedtype Output = Horizon;
    func anti_wedge(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e1234 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<Line> {
    associatedtype Output = Line;
    func anti_wedge(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<Motor> {
    associatedtype Output = Motor;
    func anti_wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
    }
}
extension AntiScalar: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e1234) * other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
    }
}
extension AntiScalar: AntiWedge<Origin> {
    associatedtype Output = Origin;
    func anti_wedge(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e1234 * other.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<Plane> {
    associatedtype Output = Plane;
    func anti_wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension AntiScalar: AntiWedge<Point> {
    associatedtype Output = Point;
    func anti_wedge(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
extension AntiScalar: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<DualNum> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension DualNum: AntiWedge<AntiScalar> {
    associatedtype Output = DualNum;
    func anti_wedge(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e1234) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<DualNum> {
    associatedtype Output = DualNum;
    func anti_wedge(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<Flector> {
    associatedtype Output = Flector;
    func anti_wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
extension DualNum: AntiWedge<Horizon> {
    associatedtype Output = Horizon;
    func anti_wedge(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e1234 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<Line> {
    associatedtype Output = Line;
    func anti_wedge(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0)
        );
    }
}
extension DualNum: AntiWedge<Motor> {
    associatedtype Output = Motor;
    func anti_wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group1.xyz, 1.0)
        );
    }
}
extension DualNum: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
    }
}
extension DualNum: AntiWedge<Origin> {
    associatedtype Output = Origin;
    func anti_wedge(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e1234 * other.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<Plane> {
    associatedtype Output = Plane;
    func anti_wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension DualNum: AntiWedge<Point> {
    associatedtype Output = Point;
    func anti_wedge(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
extension DualNum: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Flector> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Flector: AntiWedge<AntiScalar> {
    associatedtype Output = Flector;
    func anti_wedge(other: AntiScalar) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
extension Flector: AntiWedge<DualNum> {
    associatedtype Output = Flector;
    func anti_wedge(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
extension Flector: AntiWedge<Flector> {
    associatedtype Output = Motor;
    func anti_wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (other.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group1.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
extension Flector: AntiWedge<Horizon> {
    associatedtype Output = Motor;
    func anti_wedge(other: Horizon) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
    }
}
extension Flector: AntiWedge<Line> {
    associatedtype Output = Point;
    func anti_wedge(other: Line) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
    }
}
extension Flector: AntiWedge<Motor> {
    associatedtype Output = Flector;
    func anti_wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
extension Flector: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43, 0 */
            float4((this.group1.zxy * other.group4.yzx) - (this.group1.yzx * other.group4.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e321) * this.group1.xyz) - (float3(this.e321) * other.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
extension Flector: AntiWedge<Origin> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Origin) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiWedge<Plane> {
    associatedtype Output = Motor;
    func anti_wedge(other: Plane) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
    }
}
extension Flector: AntiWedge<Point> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Horizon> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Horizon: AntiWedge<AntiScalar> {
    associatedtype Output = Horizon;
    func anti_wedge(other: AntiScalar) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(other.e1234 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: AntiWedge<DualNum> {
    associatedtype Output = Horizon;
    func anti_wedge(other: DualNum) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(other.e1234 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: AntiWedge<Flector> {
    associatedtype Output = Motor;
    func anti_wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
    }
}
extension Horizon: AntiWedge<Line> {
    associatedtype Output = Point;
    func anti_wedge(other: Line) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
extension Horizon: AntiWedge<Motor> {
    associatedtype Output = Flector;
    func anti_wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
extension Horizon: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group2.xyz), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * other.group4.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
extension Horizon: AntiWedge<Origin> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Origin) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: AntiWedge<Plane> {
    associatedtype Output = Line;
    func anti_wedge(other: Plane) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * other.group0.xyz * float3(-1.0), 0.0)
        );
    }
}
extension Horizon: AntiWedge<Point> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Line> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Line: AntiWedge<AntiScalar> {
    associatedtype Output = Line;
    func anti_wedge(other: AntiScalar) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e1234) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: AntiWedge<DualNum> {
    associatedtype Output = Line;
    func anti_wedge(other: DualNum) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e1234) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: AntiWedge<Flector> {
    associatedtype Output = Point;
    func anti_wedge(other: Flector) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) - (other.group1.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
    }
}
extension Line: AntiWedge<Horizon> {
    associatedtype Output = Point;
    func anti_wedge(other: Horizon) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0)
        );
    }
}
extension Line: AntiWedge<Line> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiWedge<Motor> {
    associatedtype Output = Motor;
    func anti_wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43))
        );
    }
}
extension Line: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group4.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e1234) * this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: AntiWedge<Plane> {
    associatedtype Output = Point;
    func anti_wedge(other: Plane) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Motor> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Motor: AntiWedge<AntiScalar> {
    associatedtype Output = Motor;
    func anti_wedge(other: AntiScalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        );
    }
}
extension Motor: AntiWedge<DualNum> {
    associatedtype Output = Motor;
    func anti_wedge(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.group0.xy.yy, other.e1234, (other.scalar * this.e1234) + (other.e1234 * this.scalar)) * float4(this.group1.xyz, 1.0)
        );
    }
}
extension Motor: AntiWedge<Flector> {
    associatedtype Output = Flector;
    func anti_wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
extension Motor: AntiWedge<Horizon> {
    associatedtype Output = Flector;
    func anti_wedge(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
extension Motor: AntiWedge<Line> {
    associatedtype Output = Motor;
    func anti_wedge(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
    }
}
extension Motor: AntiWedge<Motor> {
    associatedtype Output = Motor;
    func anti_wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e1234) * other.group0.xyz)), other.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
    }
}
extension Motor: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), this.e1234 * other.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * other.e1) + (this.e31 * other.e412), (this.e1234 * other.e2) + (this.e12 * other.e423), (this.e1234 * other.e3) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) + (this.group0 * float4(other.group4.www, other.e4)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group2.xyz) + (float3(other.e1234) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * other.group3.xyz) + (float3(other.e1234) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
    }
}
extension Motor: AntiWedge<Origin> {
    associatedtype Output = Origin;
    func anti_wedge(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e1234 * other.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiWedge<Plane> {
    associatedtype Output = Flector;
    func anti_wedge(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension Motor: AntiWedge<Point> {
    associatedtype Output = Point;
    func anti_wedge(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
extension Motor: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<MultiVector> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension MultiVector: AntiWedge<AntiScalar> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e1234) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e1234) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
    }
}
extension MultiVector: AntiWedge<DualNum> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e1234) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
    }
}
extension MultiVector: AntiWedge<Flector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((other.group1.yzx * this.group4.zxy) - (other.group1.zxy * this.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
extension MultiVector: AntiWedge<Horizon> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group2.xyz), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e321) * this.group4.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
extension MultiVector: AntiWedge<Line> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group4.yzxx * float4(other.group1.xyz.zxy, other.e41)), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<Motor> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e1234 * this.e1) + (other.e31 * this.e412), (other.e1234 * this.e2) + (other.e12 * this.e423), (other.e1234 * this.e3) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) + (other.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e1234) * this.group3.xyz) + (float3(this.e1234) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
    }
}
extension MultiVector: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e43 * this.e412) - (other.e423 * this.e41) - (other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) - (this.group4.yzxx * float4(other.group3.xyz.zxy, other.e41)) - float4((this.group3.xyz.zxy * other.group4.yzx), other.e42 * this.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group2.xyz) + (other.group4.yzx * this.group4.zxy) - (other.group4.zxy * this.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.e1234) * other.group3.xyz) - (float3(this.e321) * other.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e1234) * other.group4)
        );
    }
}
extension MultiVector: AntiWedge<Origin> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<Plane> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43, 0 */
            float4((this.group4.zxy * other.group0.yzx) - (this.group4.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
extension MultiVector: AntiWedge<Point> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Origin> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Origin: AntiWedge<AntiScalar> {
    associatedtype Output = Origin;
    func anti_wedge(other: AntiScalar) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e1234 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiWedge<DualNum> {
    associatedtype Output = Origin;
    func anti_wedge(other: DualNum) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e1234 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiWedge<Flector> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiWedge<Horizon> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiWedge<Motor> {
    associatedtype Output = Origin;
    func anti_wedge(other: Motor) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e1234 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: AntiWedge<Plane> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e4 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Plane> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Plane: AntiWedge<AntiScalar> {
    associatedtype Output = Plane;
    func anti_wedge(other: AntiScalar) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: AntiWedge<DualNum> {
    associatedtype Output = Plane;
    func anti_wedge(other: DualNum) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: AntiWedge<Flector> {
    associatedtype Output = Motor;
    func anti_wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
extension Plane: AntiWedge<Horizon> {
    associatedtype Output = Line;
    func anti_wedge(other: Horizon) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e321) * this.group0.xyz, 0.0)
        );
    }
}
extension Plane: AntiWedge<Line> {
    associatedtype Output = Point;
    func anti_wedge(other: Line) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
    }
}
extension Plane: AntiWedge<Motor> {
    associatedtype Output = Flector;
    func anti_wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group3.xyz.zxy, other.e41)), 
            /* e41, e42, e43, 0 */
            float4((other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Plane: AntiWedge<Origin> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Origin) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e4 * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiWedge<Plane> {
    associatedtype Output = Line;
    func anti_wedge(other: Plane) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
extension Plane: AntiWedge<Point> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Point> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Point: AntiWedge<AntiScalar> {
    associatedtype Output = Point;
    func anti_wedge(other: AntiScalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Point: AntiWedge<DualNum> {
    associatedtype Output = Point;
    func anti_wedge(other: DualNum) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Point: AntiWedge<Flector> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Point: AntiWedge<Horizon> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Point: AntiWedge<Motor> {
    associatedtype Output = Point;
    func anti_wedge(other: Motor) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
extension Point: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Point: AntiWedge<Plane> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Scalar> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Scalar: AntiWedge<AntiScalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiWedge<DualNum> {
    associatedtype Output = Scalar;
    func anti_wedge(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiWedge<Motor> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiWedge<MultiVector> {
    associatedtype Output = Scalar;
    func anti_wedge(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
