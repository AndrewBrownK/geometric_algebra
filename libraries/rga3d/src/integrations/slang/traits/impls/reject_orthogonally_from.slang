extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<AntiScalar> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension AntiScalar: RejectOrthogonallyFrom<DualNum> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Flector) -> Output {
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (anti_wedge.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e423)) + float4(float3(0.0), (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)) - (right_anti_dual.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.yzx * right_anti_dual.group0.zxy) - (anti_wedge.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiScalar: RejectOrthogonallyFrom<Horizon> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: RejectOrthogonallyFrom<Line> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Line) -> Output {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(-(anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Motor) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_anti_dual.group0) + (float4(right_anti_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension AntiScalar: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
extension AntiScalar: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: RejectOrthogonallyFrom<Point> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Point) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * anti_wedge.e1) + (right_anti_dual.e431 * anti_wedge.e2) + (right_anti_dual.e412 * anti_wedge.e3) + (right_anti_dual.e321 * anti_wedge.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: RejectOrthogonallyFrom<Scalar> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<DualNum> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension DualNum: RejectOrthogonallyFrom<DualNum> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((powi(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Flector) -> Output {
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (anti_wedge.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e423)) + float4(float3(0.0), (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)) - (right_anti_dual.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.yzx * right_anti_dual.group0.zxy) - (anti_wedge.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension DualNum: RejectOrthogonallyFrom<Horizon> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: RejectOrthogonallyFrom<Line> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Line) -> Output {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(-(anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Motor) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.yy, this.e1234, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group1.xyz, 1.0)
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_anti_dual.group0) + (float4(right_anti_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension DualNum: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
extension DualNum: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.e321, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: RejectOrthogonallyFrom<Point> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Point) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * anti_wedge.e1) + (right_anti_dual.e431 * anti_wedge.e2) + (right_anti_dual.e412 * anti_wedge.e3) + (right_anti_dual.e321 * anti_wedge.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: RejectOrthogonallyFrom<Scalar> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Flector> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Flector: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func reject_orthogonally_from(other: Flector) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (other.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group1.wwwx * float4(other.group1.xyz, other.e1))
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual.e4 * anti_wedge.e23) + (right_anti_dual.e423 * anti_wedge.scalar), (right_anti_dual.e4 * anti_wedge.e31) + (right_anti_dual.e431 * anti_wedge.scalar), (right_anti_dual.e4 * anti_wedge.e12) + (right_anti_dual.e412 * anti_wedge.scalar), -(right_anti_dual.e2 * anti_wedge.e31) - (right_anti_dual.e3 * anti_wedge.e12)) + float4((right_anti_dual.group0.zxy * anti_wedge.group0.yzx), right_anti_dual.e321 * anti_wedge.scalar) - (right_anti_dual.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Flector: RejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Flector;
    func reject_orthogonally_from(other: Horizon) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
        Origin right_anti_dual = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_anti_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0)
        );
    }
}
extension Flector: RejectOrthogonallyFrom<Line> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Line) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
extension Flector: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Motor) -> Output {
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((float3(right_anti_dual.scalar) * anti_wedge.group1.xyz), 0.0) + float4((anti_wedge.group0.zxy * right_anti_dual.group0.yzx), 0.0) - float4((anti_wedge.group0.yzx * right_anti_dual.group0.zxy), 0.0)
        );
    }
}
extension Flector: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
extension Flector: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = Flector;
    func reject_orthogonally_from(other: Plane) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
        Origin right_anti_dual = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_anti_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0)
        );
    }
}
extension Flector: RejectOrthogonallyFrom<Point> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Point) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Horizon> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Horizon: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func reject_orthogonally_from(other: Flector) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual.e4 * anti_wedge.e23) + (right_anti_dual.e423 * anti_wedge.scalar), (right_anti_dual.e4 * anti_wedge.e31) + (right_anti_dual.e431 * anti_wedge.scalar), (right_anti_dual.e4 * anti_wedge.e12) + (right_anti_dual.e412 * anti_wedge.scalar), -(right_anti_dual.e2 * anti_wedge.e31) - (right_anti_dual.e3 * anti_wedge.e12)) + float4((right_anti_dual.group0.zxy * anti_wedge.group0.yzx), right_anti_dual.e321 * anti_wedge.scalar) - (right_anti_dual.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Horizon: RejectOrthogonallyFrom<Line> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Line) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
extension Horizon: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Motor) -> Output {
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((float3(right_anti_dual.scalar) * anti_wedge.group1.xyz), 0.0) + float4((anti_wedge.group0.zxy * right_anti_dual.group0.yzx), 0.0) - float4((anti_wedge.group0.yzx * right_anti_dual.group0.zxy), 0.0)
        );
    }
}
extension Horizon: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
extension Horizon: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0)), 0.0)
        );
    }
}
extension Horizon: RejectOrthogonallyFrom<Point> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Point) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Line> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Line: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Flector) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) - (other.group1.yzxx * float4(this.group1.xyz.zxy, this.e41))
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e4 * anti_wedge.e1 * -1.0, right_anti_dual.e4 * anti_wedge.e2 * -1.0, right_anti_dual.e4 * anti_wedge.e3 * -1.0, (right_anti_dual.e431 * anti_wedge.e2) + (right_anti_dual.e412 * anti_wedge.e3) + (right_anti_dual.e321 * anti_wedge.e4)) + (anti_wedge.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e423)), 
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group0.zxy * anti_wedge.group0.yzx) - (right_anti_dual.group0.yzx * anti_wedge.group0.zxy)), 0.0)
        );
    }
}
extension Line: RejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Line;
    func reject_orthogonally_from(other: Horizon) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e321 * -1.0) * float4(other.e321 * this.e41, other.e321 * this.e42, other.e321 * this.e43, 0.0).xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
    }
}
extension Line: RejectOrthogonallyFrom<Line> {
    associatedtype Output = Line;
    func reject_orthogonally_from(other: Line) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
    }
}
extension Line: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Motor) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43))
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_anti_dual.group0) + (float4(right_anti_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Line: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
extension Line: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = Line;
    func reject_orthogonally_from(other: Plane) -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e321 * -1.0) * float4((this.e41 * other.e321) + (this.e31 * other.e412) - (this.e12 * other.e431), (this.e42 * other.e321) + (this.e12 * other.e423) - (this.e23 * other.e412), (this.e43 * other.e321) + (this.e23 * other.e431) - (this.e31 * other.e423), -(this.e41 * other.e423) - (this.e42 * other.e431) - (this.e43 * other.e412)).xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Motor> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Motor: RejectOrthogonallyFrom<DualNum> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((powi(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Flector) -> Output {
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (anti_wedge.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e423)) + float4(float3(0.0), (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)) - (right_anti_dual.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.yzx * right_anti_dual.group0.zxy) - (anti_wedge.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension Motor: RejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Horizon) -> Output {
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(anti_wedge.group0.xyz, anti_wedge.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: RejectOrthogonallyFrom<Line> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Line) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group0.xyz), -(right_anti_dual.e41 * anti_wedge.e23) - (right_anti_dual.e42 * anti_wedge.e31) - (right_anti_dual.e43 * anti_wedge.e12) - (right_anti_dual.e23 * anti_wedge.e41) - (right_anti_dual.e31 * anti_wedge.e42) - (right_anti_dual.e12 * anti_wedge.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Motor) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e1234) * other.group0.xyz)), other.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * right_anti_dual.group0) + (float4(right_anti_dual.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
extension Motor: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Plane) -> Output {
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4(anti_wedge.group0.xyz, anti_wedge.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: RejectOrthogonallyFrom<Point> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Point) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * anti_wedge.e1) + (right_anti_dual.e431 * anti_wedge.e2) + (right_anti_dual.e412 * anti_wedge.e3) + (right_anti_dual.e321 * anti_wedge.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: RejectOrthogonallyFrom<Scalar> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<MultiVector> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension MultiVector: RejectOrthogonallyFrom<DualNum> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((powi(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: Flector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (right_anti_dual.e423 * anti_wedge.e1) + (right_anti_dual.e431 * anti_wedge.e2) + (right_anti_dual.e412 * anti_wedge.e3) + (right_anti_dual.e321 * anti_wedge.e4) - (right_anti_dual.e1 * anti_wedge.e423) - (right_anti_dual.e2 * anti_wedge.e431) - (right_anti_dual.e3 * anti_wedge.e412) - (right_anti_dual.e4 * anti_wedge.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_anti_dual.group0, 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_anti_dual.group0.zxy * anti_wedge.group1.yzx) - (right_anti_dual.group0.yzx * anti_wedge.group1.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual.e3 * anti_wedge.e42) + (right_anti_dual.e4 * anti_wedge.e23), (right_anti_dual.e1 * anti_wedge.e43) + (right_anti_dual.e4 * anti_wedge.e31), (right_anti_dual.e2 * anti_wedge.e41) + (right_anti_dual.e4 * anti_wedge.e12), -(right_anti_dual.e2 * anti_wedge.e31) - (right_anti_dual.e3 * anti_wedge.e12)) + (float4(anti_wedge.scalar) * right_anti_dual.group1) - (right_anti_dual.group0.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23))
        );
    }
}
extension MultiVector: RejectOrthogonallyFrom<Horizon> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: Horizon) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Origin right_anti_dual = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, anti_wedge.e321 * right_anti_dual.e4) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_anti_dual.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e4) * anti_wedge.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * anti_wedge.group3.xyz), 0.0)
        );
    }
}
extension MultiVector: RejectOrthogonallyFrom<Line> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: Line) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, -(right_anti_dual.e41 * anti_wedge.e23) - (right_anti_dual.e42 * anti_wedge.e31) - (right_anti_dual.e43 * anti_wedge.e12) - (right_anti_dual.e23 * anti_wedge.e41) - (right_anti_dual.e31 * anti_wedge.e42) - (right_anti_dual.e12 * anti_wedge.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(float3(anti_wedge.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * anti_wedge.group1.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
extension MultiVector: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: Motor) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (right_anti_dual.e1234 * anti_wedge.scalar) + (right_anti_dual.scalar * anti_wedge.e1234) - (right_anti_dual.e41 * anti_wedge.e23) - (right_anti_dual.e42 * anti_wedge.e31) - (right_anti_dual.e43 * anti_wedge.e12) - (right_anti_dual.e23 * anti_wedge.e41) - (right_anti_dual.e31 * anti_wedge.e42) - (right_anti_dual.e12 * anti_wedge.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) + (float3(anti_wedge.scalar) * right_anti_dual.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.yzx * anti_wedge.group1.zxy), 0.0) - float4((right_anti_dual.group0.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
extension MultiVector: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
extension MultiVector: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: Plane) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Origin right_anti_dual = Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, anti_wedge.e321 * right_anti_dual.e4) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_anti_dual.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e4) * anti_wedge.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * anti_wedge.group3.xyz), 0.0)
        );
    }
}
extension MultiVector: RejectOrthogonallyFrom<Point> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: Point) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.group0.xy.xx, anti_wedge.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: RejectOrthogonallyFrom<Scalar> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * powi(other.scalar, 2), 0.0, 0.0, 0.0)
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Origin> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Origin: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func reject_orthogonally_from(other: Flector) -> Output {
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e4, 0.0, 0.0, 0.0)
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group1.xyz), 0.0)
        );
    }
}
extension Origin: RejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Origin;
    func reject_orthogonally_from(other: Horizon) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Origin: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_anti_dual.group3.xyz), 0.0) + float4((float3(right_anti_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(right_anti_dual.e4) * anti_wedge.group3.xyz), 0.0) + float4((anti_wedge.group2.xyz.yzx * right_anti_dual.group1.zxy), 0.0) + float4((right_anti_dual.group2.xyz.yzx * anti_wedge.group1.zxy), 0.0) - float4((anti_wedge.group2.xyz.zxy * right_anti_dual.group1.yzx), 0.0) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
extension Origin: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = Origin;
    func reject_orthogonally_from(other: Plane) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 * powi(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Plane> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Plane: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func reject_orthogonally_from(other: Flector) -> Output {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual.e4 * anti_wedge.e23) + (right_anti_dual.e423 * anti_wedge.scalar), (right_anti_dual.e4 * anti_wedge.e31) + (right_anti_dual.e431 * anti_wedge.scalar), (right_anti_dual.e4 * anti_wedge.e12) + (right_anti_dual.e412 * anti_wedge.scalar), -(right_anti_dual.e2 * anti_wedge.e31) - (right_anti_dual.e3 * anti_wedge.e12)) + float4((right_anti_dual.group0.zxy * anti_wedge.group0.yzx), right_anti_dual.e321 * anti_wedge.scalar) - (right_anti_dual.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Plane: RejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Horizon) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412)), 0.0)
        );
    }
}
extension Plane: RejectOrthogonallyFrom<Line> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Line) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
extension Plane: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Motor) -> Output {
        Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((float3(right_anti_dual.scalar) * anti_wedge.group1.xyz), 0.0) + float4((anti_wedge.group0.zxy * right_anti_dual.group0.yzx), 0.0) - float4((anti_wedge.group0.yzx * right_anti_dual.group0.zxy), 0.0)
        );
    }
}
extension Plane: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
extension Plane: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3((other.e321 * this.e423) - (other.e423 * this.e321), (other.e321 * this.e431) - (other.e431 * this.e321), (other.e321 * this.e412) - (other.e412 * this.e321))), 0.0)
        );
    }
}
extension Plane: RejectOrthogonallyFrom<Point> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Point) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Point> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Point: RejectOrthogonallyFrom<Flector> {
    associatedtype Output = Flector;
    func reject_orthogonally_from(other: Flector) -> Output {
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0, 0.0, 0.0)
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group1.xyz), 0.0)
        );
    }
}
extension Point: RejectOrthogonallyFrom<Horizon> {
    associatedtype Output = Origin;
    func reject_orthogonally_from(other: Horizon) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(powi(other.e321, 2) * this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Point: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = Plane;
    func reject_orthogonally_from(other: Motor) -> Output {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_anti_dual.group0.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
extension Point: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        MultiVector anti_wedge = MultiVector.from_groups(
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_anti_dual.group3.xyz), 0.0) + float4((float3(right_anti_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(right_anti_dual.e4) * anti_wedge.group3.xyz), 0.0) + float4((anti_wedge.group2.xyz.yzx * right_anti_dual.group1.zxy), 0.0) + float4((right_anti_dual.group2.xyz.yzx * anti_wedge.group1.zxy), 0.0) - float4((anti_wedge.group2.xyz.zxy * right_anti_dual.group1.yzx), 0.0) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
extension Point: RejectOrthogonallyFrom<Plane> {
    associatedtype Output = Origin;
    func reject_orthogonally_from(other: Plane) -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(-(powi(other.e321, 2) * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Scalar> {
        return RejectOrthogonallyFromInfixPartial { a: this };
    }
}
extension Scalar: RejectOrthogonallyFrom<DualNum> {
    associatedtype Output = AntiScalar;
    func reject_orthogonally_from(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.scalar * other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: RejectOrthogonallyFrom<Motor> {
    associatedtype Output = Motor;
    func reject_orthogonally_from(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234 * this.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Scalar: RejectOrthogonallyFrom<MultiVector> {
    associatedtype Output = MultiVector;
    func reject_orthogonally_from(other: MultiVector) -> Output {
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
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
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, right_anti_dual.e1234 * anti_wedge.scalar) * float2(0.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_anti_dual.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(anti_wedge.scalar) * right_anti_dual.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group4.xyz), 0.0)
        );
    }
}
