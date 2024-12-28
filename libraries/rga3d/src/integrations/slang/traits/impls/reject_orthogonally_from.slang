public extension AntiScalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<AntiScalar> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> AntiScalar.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension AntiScalar: RejectOrthogonallyFrom<DualNum> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension AntiScalar: RejectOrthogonallyFrom<Flector> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> Motor {
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
public extension AntiScalar: RejectOrthogonallyFrom<Horizon> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
public extension AntiScalar: RejectOrthogonallyFrom<Line> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Line) -> AntiScalar {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43)
        );
    }
}
public extension AntiScalar: RejectOrthogonallyFrom<Motor> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> Motor {
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
public extension AntiScalar: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
public extension AntiScalar: RejectOrthogonallyFrom<Plane> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
public extension AntiScalar: RejectOrthogonallyFrom<Point> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Point) -> AntiScalar {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (right_anti_dual.e423 * anti_wedge.e1) + (right_anti_dual.e431 * anti_wedge.e2) + (right_anti_dual.e412 * anti_wedge.e3) + (right_anti_dual.e321 * anti_wedge.e4)
        );
    }
}
public extension AntiScalar: RejectOrthogonallyFrom<Scalar> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<DualNum> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> DualNum.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension DualNum: RejectOrthogonallyFrom<DualNum> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (pow(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
public extension DualNum: RejectOrthogonallyFrom<Flector> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> Motor {
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
public extension DualNum: RejectOrthogonallyFrom<Horizon> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
public extension DualNum: RejectOrthogonallyFrom<Line> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Line) -> AntiScalar {
        Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43)
        );
    }
}
public extension DualNum: RejectOrthogonallyFrom<Motor> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> Motor {
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
public extension DualNum: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
public extension DualNum: RejectOrthogonallyFrom<Plane> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
public extension DualNum: RejectOrthogonallyFrom<Point> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Point) -> AntiScalar {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (right_anti_dual.e423 * anti_wedge.e1) + (right_anti_dual.e431 * anti_wedge.e2) + (right_anti_dual.e412 * anti_wedge.e3) + (right_anti_dual.e321 * anti_wedge.e4)
        );
    }
}
public extension DualNum: RejectOrthogonallyFrom<Scalar> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Flector> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Flector.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension Flector: RejectOrthogonallyFrom<Flector> {
    public typedef Flector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> Flector {
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
public extension Flector: RejectOrthogonallyFrom<Horizon> {
    public typedef Flector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Horizon) -> Flector {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
        Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_anti_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0)
        );
    }
}
public extension Flector: RejectOrthogonallyFrom<Line> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Line) -> Plane {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
public extension Flector: RejectOrthogonallyFrom<Motor> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> Plane {
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
public extension Flector: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
public extension Flector: RejectOrthogonallyFrom<Plane> {
    public typedef Flector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> Flector {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
        Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_anti_dual.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 0.0)
        );
    }
}
public extension Flector: RejectOrthogonallyFrom<Point> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Horizon> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Horizon.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension Horizon: RejectOrthogonallyFrom<Flector> {
    public typedef Flector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> Flector {
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
public extension Horizon: RejectOrthogonallyFrom<Line> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Line) -> Plane {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
public extension Horizon: RejectOrthogonallyFrom<Motor> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> Plane {
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
public extension Horizon: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
public extension Horizon: RejectOrthogonallyFrom<Plane> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0)), 0.0)
        );
    }
}
public extension Horizon: RejectOrthogonallyFrom<Point> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Line> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<Line> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Line.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension Line: RejectOrthogonallyFrom<Flector> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> Motor {
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
public extension Line: RejectOrthogonallyFrom<Horizon> {
    public typedef Line RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Horizon) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * float4(other.e321 * this.e41, other.e321 * this.e42, other.e321 * this.e43, 0.0).xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Line: RejectOrthogonallyFrom<Line> {
    public typedef Line RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Line: RejectOrthogonallyFrom<Motor> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> Motor {
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
public extension Line: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
public extension Line: RejectOrthogonallyFrom<Plane> {
    public typedef Line RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * float4((this.e41 * other.e321) + (this.e31 * other.e412) - (this.e12 * other.e431), (this.e42 * other.e321) + (this.e12 * other.e423) - (this.e23 * other.e412), (this.e43 * other.e321) + (this.e23 * other.e431) - (this.e31 * other.e423), -(this.e41 * other.e423) - (this.e42 * other.e431) - (this.e43 * other.e412)).xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Motor> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Motor.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension Motor: RejectOrthogonallyFrom<DualNum> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (pow(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
public extension Motor: RejectOrthogonallyFrom<Flector> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> Motor {
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
public extension Motor: RejectOrthogonallyFrom<Horizon> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Horizon) -> Motor {
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
public extension Motor: RejectOrthogonallyFrom<Line> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Line) -> Motor {
        Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group0.xyz), -(right_anti_dual.e41 * anti_wedge.e23) - (right_anti_dual.e42 * anti_wedge.e31) - (right_anti_dual.e43 * anti_wedge.e12) - (right_anti_dual.e23 * anti_wedge.e41) - (right_anti_dual.e31 * anti_wedge.e42) - (right_anti_dual.e12 * anti_wedge.e43)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Motor: RejectOrthogonallyFrom<Motor> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> Motor {
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
public extension Motor: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
public extension Motor: RejectOrthogonallyFrom<Plane> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> Motor {
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
public extension Motor: RejectOrthogonallyFrom<Point> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Point) -> AntiScalar {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (right_anti_dual.e423 * anti_wedge.e1) + (right_anti_dual.e431 * anti_wedge.e2) + (right_anti_dual.e412 * anti_wedge.e3) + (right_anti_dual.e321 * anti_wedge.e4)
        );
    }
}
public extension Motor: RejectOrthogonallyFrom<Scalar> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<MultiVector> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> MultiVector.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension MultiVector: RejectOrthogonallyFrom<DualNum> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (pow(other.scalar, 2) * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
public extension MultiVector: RejectOrthogonallyFrom<Flector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual.e423 * anti_wedge.e1) + (right_anti_dual.e431 * anti_wedge.e2) + (right_anti_dual.e412 * anti_wedge.e3) + (right_anti_dual.e321 * anti_wedge.e4) - (right_anti_dual.e1 * anti_wedge.e423) - (right_anti_dual.e2 * anti_wedge.e431) - (right_anti_dual.e3 * anti_wedge.e412) - (right_anti_dual.e4 * anti_wedge.e321)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_anti_dual.group0, 
            /* e41, e42, e43 */
            (float3(anti_wedge.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual.group0.zxy * anti_wedge.group1.yzx) - (right_anti_dual.group0.yzx * anti_wedge.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual.e3 * anti_wedge.e42) + (right_anti_dual.e4 * anti_wedge.e23), (right_anti_dual.e1 * anti_wedge.e43) + (right_anti_dual.e4 * anti_wedge.e31), (right_anti_dual.e2 * anti_wedge.e41) + (right_anti_dual.e4 * anti_wedge.e12), -(right_anti_dual.e2 * anti_wedge.e31) - (right_anti_dual.e3 * anti_wedge.e12)) + (float4(anti_wedge.scalar) * right_anti_dual.group1) - (right_anti_dual.group0.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23))
        );
    }
}
public extension MultiVector: RejectOrthogonallyFrom<Horizon> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Horizon) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, anti_wedge.e321 * right_anti_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * anti_wedge.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * anti_wedge.group3.xyz), 0.0)
        );
    }
}
public extension MultiVector: RejectOrthogonallyFrom<Line> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Line) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(right_anti_dual.e41 * anti_wedge.e23) - (right_anti_dual.e42 * anti_wedge.e31) - (right_anti_dual.e43 * anti_wedge.e12) - (right_anti_dual.e23 * anti_wedge.e41) - (right_anti_dual.e31 * anti_wedge.e42) - (right_anti_dual.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * anti_wedge.group1.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
public extension MultiVector: RejectOrthogonallyFrom<Motor> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual.e1234 * anti_wedge.scalar) + (right_anti_dual.scalar * anti_wedge.e1234) - (right_anti_dual.e41 * anti_wedge.e23) - (right_anti_dual.e42 * anti_wedge.e31) - (right_anti_dual.e43 * anti_wedge.e12) - (right_anti_dual.e23 * anti_wedge.e41) - (right_anti_dual.e31 * anti_wedge.e42) - (right_anti_dual.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) + (float3(anti_wedge.scalar) * right_anti_dual.group0.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.yzx * anti_wedge.group1.zxy), 0.0) - float4((right_anti_dual.group0.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
public extension MultiVector: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
public extension MultiVector: RejectOrthogonallyFrom<Plane> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, anti_wedge.e321 * right_anti_dual.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge.scalar * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * anti_wedge.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual.e4) * anti_wedge.group3.xyz), 0.0)
        );
    }
}
public extension MultiVector: RejectOrthogonallyFrom<Point> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Point) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.group0.xy.xx, anti_wedge.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension MultiVector: RejectOrthogonallyFrom<Scalar> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Origin> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Origin.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension Origin: RejectOrthogonallyFrom<Flector> {
    public typedef Flector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> Flector {
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            other.e321 * this.e4
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
public extension Origin: RejectOrthogonallyFrom<Horizon> {
    public typedef Origin RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
public extension Origin: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_anti_dual.group3.xyz), 0.0) + float4((float3(right_anti_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(right_anti_dual.e4) * anti_wedge.group3.xyz), 0.0) + float4((anti_wedge.group2.xyz.yzx * right_anti_dual.group1.zxy), 0.0) + float4((right_anti_dual.group2.xyz.yzx * anti_wedge.group1.zxy), 0.0) - float4((anti_wedge.group2.xyz.zxy * right_anti_dual.group1.yzx), 0.0) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
public extension Origin: RejectOrthogonallyFrom<Plane> {
    public typedef Origin RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Plane> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Plane.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension Plane: RejectOrthogonallyFrom<Flector> {
    public typedef Flector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> Flector {
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
public extension Plane: RejectOrthogonallyFrom<Horizon> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Horizon) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412)), 0.0)
        );
    }
}
public extension Plane: RejectOrthogonallyFrom<Line> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Line) -> Plane {
        Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e41))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.e4) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.yzx * anti_wedge.group0.zxy), 0.0) - float4((right_anti_dual.group0.xyz.zxy * anti_wedge.group0.yzx), 0.0)
        );
    }
}
public extension Plane: RejectOrthogonallyFrom<Motor> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> Plane {
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
public extension Plane: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * right_anti_dual.e42) + (anti_wedge.e4 * right_anti_dual.e23) + (anti_wedge.e42 * right_anti_dual.e3) + (anti_wedge.e23 * right_anti_dual.e4), (anti_wedge.e1 * right_anti_dual.e43) + (anti_wedge.e4 * right_anti_dual.e31) + (anti_wedge.e43 * right_anti_dual.e1) + (anti_wedge.e31 * right_anti_dual.e4), (anti_wedge.e2 * right_anti_dual.e41) + (anti_wedge.e4 * right_anti_dual.e12) + (anti_wedge.e41 * right_anti_dual.e2) + (anti_wedge.e12 * right_anti_dual.e4), -(anti_wedge.e1 * right_anti_dual.e23) - (anti_wedge.e2 * right_anti_dual.e31) - (anti_wedge.e3 * right_anti_dual.e12) - (anti_wedge.e12 * right_anti_dual.e3)) + (float4(anti_wedge.scalar) * right_anti_dual.group4) + (float4(right_anti_dual.scalar) * anti_wedge.group4) - (right_anti_dual.group1.yzxx * float4(anti_wedge.group2.xyz.zxy, anti_wedge.e23)) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * right_anti_dual.e2)
        );
    }
}
public extension Plane: RejectOrthogonallyFrom<Plane> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3((other.e321 * this.e423) - (other.e423 * this.e321), (other.e321 * this.e431) - (other.e431 * this.e321), (other.e321 * this.e412) - (other.e412 * this.e321))), 0.0)
        );
    }
}
public extension Plane: RejectOrthogonallyFrom<Point> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Point> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<Point> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Point.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension Point: RejectOrthogonallyFrom<Flector> {
    public typedef Flector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Flector) -> Flector {
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
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
public extension Point: RejectOrthogonallyFrom<Horizon> {
    public typedef Origin RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
public extension Point: RejectOrthogonallyFrom<Motor> {
    public typedef Plane RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> Plane {
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
public extension Point: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector anti_wedge = MultiVector.from_groups(
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
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.scalar * right_anti_dual.e1234) + (anti_wedge.e1234 * right_anti_dual.scalar) + (anti_wedge.e1 * right_anti_dual.e423) + (anti_wedge.e2 * right_anti_dual.e431) + (anti_wedge.e3 * right_anti_dual.e412) + (anti_wedge.e4 * right_anti_dual.e321) - (anti_wedge.e41 * right_anti_dual.e23) - (anti_wedge.e42 * right_anti_dual.e31) - (anti_wedge.e43 * right_anti_dual.e12) - (anti_wedge.e23 * right_anti_dual.e41) - (anti_wedge.e31 * right_anti_dual.e42) - (anti_wedge.e12 * right_anti_dual.e43) - (anti_wedge.e423 * right_anti_dual.e1) - (anti_wedge.e431 * right_anti_dual.e2) - (anti_wedge.e412 * right_anti_dual.e3) - (anti_wedge.e321 * right_anti_dual.e4)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * right_anti_dual.group2.xyz) + (float3(anti_wedge.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group2.xyz) - (float3(right_anti_dual.e4) * anti_wedge.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * anti_wedge.group3.xyz) + (anti_wedge.group1.yzx * right_anti_dual.group1.zxy) - (anti_wedge.group1.zxy * right_anti_dual.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group4.xyz), 0.0) + float4((float3(anti_wedge.e4) * right_anti_dual.group3.xyz), 0.0) + float4((float3(right_anti_dual.scalar) * anti_wedge.group4.xyz), 0.0) + float4((float3(right_anti_dual.e4) * anti_wedge.group3.xyz), 0.0) + float4((anti_wedge.group2.xyz.yzx * right_anti_dual.group1.zxy), 0.0) + float4((right_anti_dual.group2.xyz.yzx * anti_wedge.group1.zxy), 0.0) - float4((anti_wedge.group2.xyz.zxy * right_anti_dual.group1.yzx), 0.0) - float4((right_anti_dual.group2.xyz.zxy * anti_wedge.group1.yzx), 0.0)
        );
    }
}
public extension Point: RejectOrthogonallyFrom<Plane> {
    public typedef Origin RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            -(pow(other.e321, 2) * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: RejectOrthogonallyFromInfix) -> RejectOrthogonallyFromInfixPartial<Scalar> {
        return RejectOrthogonallyFromInfixPartial(this);
    }
}
public extension RejectOrthogonallyFromInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Scalar.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
public extension Scalar: RejectOrthogonallyFrom<DualNum> {
    public typedef AntiScalar RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.scalar * other.e1234 * this.scalar
        );
    }
}
public extension Scalar: RejectOrthogonallyFrom<Motor> {
    public typedef Motor RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234 * this.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Scalar: RejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector RejectOrthogonallyFromOutput;
    public func reject_orthogonally_from(other: MultiVector) -> MultiVector {
        Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            other.e1234 * this.scalar
        );
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, right_anti_dual.e1234 * anti_wedge.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * right_anti_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge.scalar) * right_anti_dual.group4.xyz), 0.0)
        );
    }
}
