extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<AntiScalar> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension AntiScalar: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(wedge.e1234) * right_anti_dual.group0, 
            /* e23, e31, e12, scalar */
            float4(wedge.e1234) * right_anti_dual.group1
        );
    }
}
extension AntiScalar: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(wedge.e1234) * right_anti_dual.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_anti_dual.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(wedge.e1234) * right_anti_dual.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(wedge.e1234) * right_anti_dual.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * right_anti_dual.group4
        );
    }
}
extension AntiScalar: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = AntiScalar;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<DualNum> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension DualNum: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = DualNum;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar) * float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 0.0, 0.0)
        );
    }
}
extension DualNum: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group1.zxy) - (right_anti_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
extension DualNum: AntiRejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiRejectOrthogonallyFrom<Line> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Line) -> Output {
        Line wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e41, other.e42, other.e43, 1.0) * float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
extension DualNum: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
extension DualNum: AntiRejectOrthogonallyFrom<Plane> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        Point wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = DualNum;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar) * float2(this.scalar * other.scalar, this.e1234 * other.scalar), 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Flector> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Flector: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
extension Flector: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Flector: AntiRejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: Horizon) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 * powi(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiRejectOrthogonallyFrom<Line> {
    associatedtype Output = Point;
    func anti_reject_orthogonally_from(other: Line) -> Output {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41))
        );
    }
}
extension Flector: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e4 * other.e23) + (this.e423 * other.scalar), (this.e4 * other.e31) + (this.e431 * other.scalar), (this.e4 * other.e12) + (this.e412 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group0.zxy * other.group0.yzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * right_anti_dual.e31) + (wedge.e321 * right_anti_dual.e41), (wedge.e423 * right_anti_dual.e12) + (wedge.e321 * right_anti_dual.e42), (wedge.e431 * right_anti_dual.e23) + (wedge.e321 * right_anti_dual.e43), -(wedge.e431 * right_anti_dual.e42) - (wedge.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
extension Flector: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
extension Flector: AntiRejectOrthogonallyFrom<Plane> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: Plane) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(-(powi(other.e321, 2) * this.e4) - (this.e1 * other.e423 * other.e321) - (this.e2 * other.e431 * other.e321) - (this.e3 * other.e412 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * other.e1, this.e4 * other.e2, this.e4 * other.e3, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_anti_dual.e321) + (wedge.e31 * right_anti_dual.e412), (wedge.e42 * right_anti_dual.e321) + (wedge.e12 * right_anti_dual.e423), (wedge.e43 * right_anti_dual.e321) + (wedge.e23 * right_anti_dual.e431), -(wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Flector: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Horizon> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Horizon: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = Horizon;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.e4 * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * right_anti_dual.group1
        );
    }
}
extension Horizon: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        Horizon wedge = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 * other.scalar, 0.0, 0.0, 0.0)
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e321, wedge.e321, wedge.e321, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge.e321 * right_anti_dual.e1234)
        );
    }
}
extension Horizon: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, this.e321 * other.e4) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.scalar)
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
extension Horizon: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = Plane;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e4 * -1.0) * float4(other.e1, other.e2, other.e3, 0.0)
        );
    }
}
extension Horizon: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = Horizon;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 * powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Line> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Line: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = Line;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        Line wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0)
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e1234) * wedge.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e1234) * wedge.group1.xyz, 0.0)
        );
    }
}
extension Line: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group0.zxy) - (right_anti_dual.group1.zxy * wedge.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e321 * wedge.e423, right_anti_dual.e321 * wedge.e431, right_anti_dual.e321 * wedge.e412, -(right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
extension Line: AntiRejectOrthogonallyFrom<Line> {
    associatedtype Output = Line;
    func anti_reject_orthogonally_from(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
    }
}
extension Line: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(this.group0.xyz, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
extension Line: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
extension Line: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = Line;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_anti_dual.group0.yzx * wedge.group0.zxy) - (right_anti_dual.group0.zxy * wedge.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e321) * wedge.group0.xyz) - (float3(wedge.e321) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension Line: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = Line;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        Line wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0)
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e1234) * wedge.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e1234) * wedge.group1.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Motor> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Motor: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e41, this.e42, this.e43, 1.0) * float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
extension Motor: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar), (other.e4 * this.e31) + (other.e431 * this.scalar), (other.e4 * this.e12) + (other.e412 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group1.zxy) - (right_anti_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
extension Motor: AntiRejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiRejectOrthogonallyFrom<Line> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Line) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar, this.scalar, this.scalar, 1.0) * float4(other.group0.xyz, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
extension Motor: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.scalar)
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
extension Motor: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar * other.scalar, (this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * other.group2.xyz) + (float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * other.e4) + (this.scalar * other.e423), (this.e31 * other.e4) + (this.scalar * other.e431), (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
extension Motor: AntiRejectOrthogonallyFrom<Plane> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((wedge.group1.zxy * right_anti_dual.group0.yzx) - (wedge.group1.yzx * right_anti_dual.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge.e321 * right_anti_dual.e423 * -1.0, wedge.e321 * right_anti_dual.e431 * -1.0, wedge.e321 * right_anti_dual.e412 * -1.0, (wedge.e2 * right_anti_dual.e431) + (wedge.e3 * right_anti_dual.e412) + (wedge.e4 * right_anti_dual.e321)) + (right_anti_dual.group0.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
extension Motor: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<MultiVector> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension MultiVector: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(right_anti_dual.e1234) * wedge.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e1234) * wedge.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e1234) * wedge.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group4
        );
    }
}
extension MultiVector: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43, 0 */
            float4((right_anti_dual.group1.yzx * wedge.group4.zxy) - (right_anti_dual.group1.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e321) * wedge.group4.xyz) - (float3(wedge.e321) * right_anti_dual.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0.xy.yy, wedge.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: AntiRejectOrthogonallyFrom<Horizon> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: Horizon) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.scalar)
        );
        Origin right_anti_dual = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(wedge.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge.e1234 * right_anti_dual.e4), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiRejectOrthogonallyFrom<Line> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: Line) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(-(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group4.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43, 0 */
            float4(float3(wedge.e1234) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e23 * this.e4) + (other.scalar * this.e423), (other.e31 * this.e4) + (other.scalar * this.e431), (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.yzx * this.group1.zxy), other.scalar * this.e321) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.scalar * wedge.e1234) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e1234 * wedge.e1) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e1234 * wedge.e2) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e1234 * wedge.e3) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) + (right_anti_dual.group0 * float4(wedge.group4.www, wedge.e4)) - (wedge.group4.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * right_anti_dual.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group4
        );
    }
}
extension MultiVector: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e42 * this.e3) + (other.e23 * this.e4), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e43 * this.e1) + (other.e31 * this.e4), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group4) + (float4(this.scalar) * other.group4) - (this.group1.yzxx * float4(other.group2.xyz.zxy, other.e23)) - float4((this.group2.xyz.zxy * other.group1.yzx), other.e31 * this.e2)
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
extension MultiVector: AntiRejectOrthogonallyFrom<Plane> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: Plane) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
        Origin right_anti_dual = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(wedge.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge.e1234 * right_anti_dual.e4), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((wedge.e1 * right_anti_dual.e423) + (wedge.e2 * right_anti_dual.e431) + (wedge.e3 * right_anti_dual.e412) + (wedge.e4 * right_anti_dual.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_anti_dual.e321) + (wedge.e31 * right_anti_dual.e412), (wedge.e42 * right_anti_dual.e321) + (wedge.e12 * right_anti_dual.e423), (wedge.e43 * right_anti_dual.e321) + (wedge.e23 * right_anti_dual.e431), -(wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43, 0 */
            float4((wedge.group4.zxy * right_anti_dual.group0.yzx) - (wedge.group4.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e321) * wedge.group4.xyz) - (float3(wedge.e321) * right_anti_dual.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0.xy.yy, wedge.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        MultiVector wedge = MultiVector.from_groups(
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
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(right_anti_dual.e1234) * wedge.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e1234) * wedge.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e1234) * wedge.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group4
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Origin> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Origin: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234) * right_anti_dual.group1
        );
    }
}
extension Origin: AntiRejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: Horizon) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiRejectOrthogonallyFrom<Line> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: Line) -> Output {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * wedge.e423) - (right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * right_anti_dual.e31) + (wedge.e321 * right_anti_dual.e41), (wedge.e423 * right_anti_dual.e12) + (wedge.e321 * right_anti_dual.e42), (wedge.e431 * right_anti_dual.e23) + (wedge.e321 * right_anti_dual.e43), -(wedge.e431 * right_anti_dual.e42) - (wedge.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
extension Origin: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group1.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group3.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(right_anti_dual.e1234) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group4.xyz)), 0.0)
        );
    }
}
extension Origin: AntiRejectOrthogonallyFrom<Plane> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: Plane) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 * powi(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        Line wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(-(wedge.e41 * right_anti_dual.e423) - (wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 * powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Plane> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Plane: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = Plane;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(other.scalar * this.e423, other.scalar * this.e431, other.scalar * this.e412, other.scalar * this.e321)
        );
    }
}
extension Plane: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0, 0.0, 0.0)
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(wedge.e1234, wedge.e1234, wedge.e1234, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group0
        );
    }
}
extension Plane: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
extension Plane: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = Plane;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(wedge.e1234, wedge.e1234, wedge.e1234, 0.0) * float4(float4(other.e1, other.e2, other.e3, 0.0).xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = Plane;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(this.e423 * other.scalar, this.e431 * other.scalar, this.e412 * other.scalar, this.e321 * other.scalar)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Point> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Point: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = Point;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, other.scalar * this.e4)
        );
    }
}
extension Point: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * this.e1 * -1.0, other.e4 * this.e2 * -1.0, other.e4 * this.e3 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: AntiRejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: Horizon) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Point: AntiRejectOrthogonallyFrom<Line> {
    associatedtype Output = Point;
    func anti_reject_orthogonally_from(other: Line) -> Output {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41))
        );
    }
}
extension Point: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Flector;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * right_anti_dual.e31) + (wedge.e321 * right_anti_dual.e41), (wedge.e423 * right_anti_dual.e12) + (wedge.e321 * right_anti_dual.e42), (wedge.e431 * right_anti_dual.e23) + (wedge.e321 * right_anti_dual.e43), -(wedge.e431 * right_anti_dual.e42) - (wedge.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * wedge.group0) - (wedge.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
extension Point: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
extension Point: AntiRejectOrthogonallyFrom<Plane> {
    associatedtype Output = Origin;
    func anti_reject_orthogonally_from(other: Plane) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(-(powi(other.e321, 2) * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Point: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = Point;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        Line wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 0.0)
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_anti_dual.e321) + (wedge.e31 * right_anti_dual.e412), (wedge.e42 * right_anti_dual.e321) + (wedge.e12 * right_anti_dual.e423), (wedge.e43 * right_anti_dual.e321) + (wedge.e23 * right_anti_dual.e431), -(wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(wedge.group1.xyz.zxy, wedge.e41))
        );
    }
}
extension Point: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = Point;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.e1 * other.scalar, this.e2 * other.scalar, this.e3 * other.scalar, this.e4 * other.scalar)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Scalar> {
        return AntiRejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Scalar: AntiRejectOrthogonallyFrom<DualNum> {
    associatedtype Output = DualNum;
    func anti_reject_orthogonally_from(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar) * float2(other.scalar * this.scalar, other.e1234 * this.scalar), 0.0, 0.0)
        );
    }
}
extension Scalar: AntiRejectOrthogonallyFrom<Flector> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Flector) -> Output {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group1.zxy) - (right_anti_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
extension Scalar: AntiRejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiRejectOrthogonallyFrom<Line> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Line) -> Output {
        Line wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiRejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func anti_reject_orthogonally_from(other: Motor) -> Output {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * wedge.group0.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * wedge.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
extension Scalar: AntiRejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector wedge = MultiVector.from_groups(
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
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.scalar), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
extension Scalar: AntiRejectOrthogonallyFrom<Plane> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiRejectOrthogonallyFrom<Point> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Point) -> Output {
        Point wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiRejectOrthogonallyFrom<Scalar> {
    associatedtype Output = Scalar;
    func anti_reject_orthogonally_from(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(other.scalar, 2) * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
