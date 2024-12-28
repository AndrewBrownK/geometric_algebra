public extension AntiScalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<AntiScalar> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> AntiScalar.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension AntiScalar: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef AntiScalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension AntiScalar: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234 * other.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension AntiScalar: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
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
            float2(1.0, wedge.e1234 * right_anti_dual.e1234) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            float3(wedge.e1234) * right_anti_dual.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(wedge.e1234) * right_anti_dual.group4.xyz), 0.0)
        );
    }
}
public extension AntiScalar: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef AntiScalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<DualNum> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> DualNum.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension DualNum: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef DualNum AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
public extension DualNum: AntiRejectOrthogonallyFrom<Flector> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> Motor {
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
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group1.zxy) - (right_anti_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
public extension DualNum: AntiRejectOrthogonallyFrom<Horizon> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
public extension DualNum: AntiRejectOrthogonallyFrom<Line> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Line) -> Scalar {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43)
        );
    }
}
public extension DualNum: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group0.xyz, 1.0), 
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
public extension DualNum: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
public extension DualNum: AntiRejectOrthogonallyFrom<Plane> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
public extension DualNum: AntiRejectOrthogonallyFrom<Point> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> Scalar {
        Point wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4)
        );
    }
}
public extension DualNum: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef DualNum AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * float2(this.scalar * other.scalar, this.e1234 * other.scalar)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Flector> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Flector.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension Flector: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> Flector {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
public extension Flector: AntiRejectOrthogonallyFrom<Flector> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> Flector {
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
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Flector: AntiRejectOrthogonallyFrom<Horizon> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
public extension Flector: AntiRejectOrthogonallyFrom<Line> {
    public typedef Point AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Line) -> Point {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41))
        );
    }
}
public extension Flector: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Flector {
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
public extension Flector: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
public extension Flector: AntiRejectOrthogonallyFrom<Plane> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            -(pow(other.e321, 2) * this.e4) - (this.e1 * other.e423 * other.e321) - (this.e2 * other.e431 * other.e321) - (this.e3 * other.e412 * other.e321)
        );
    }
}
public extension Flector: AntiRejectOrthogonallyFrom<Point> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> Flector {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * other.e1, this.e4 * other.e2, this.e4 * other.e3, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_anti_dual.e321) + (wedge.e31 * right_anti_dual.e412), (wedge.e42 * right_anti_dual.e321) + (wedge.e12 * right_anti_dual.e423), (wedge.e43 * right_anti_dual.e321) + (wedge.e23 * right_anti_dual.e431), -(wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Flector: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> Flector {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Horizon> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Horizon.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension Horizon: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef Horizon AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            pow(other.scalar, 2) * this.e321
        );
    }
}
public extension Horizon: AntiRejectOrthogonallyFrom<Flector> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> Flector {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(wedge.e1234) * right_anti_dual.group1.xyz), 0.0)
        );
    }
}
public extension Horizon: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Flector {
        Horizon wedge = Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
        Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(wedge.e321) * right_anti_dual.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge.e321 * right_anti_dual.e1234)
        );
    }
}
public extension Horizon: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
public extension Horizon: AntiRejectOrthogonallyFrom<Point> {
    public typedef Plane AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
public extension Horizon: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef Horizon AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * pow(other.scalar, 2)
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Line> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<Line> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Line.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension Line: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef Line AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> Line {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * wedge.group1.xyz
        );
    }
}
public extension Line: AntiRejectOrthogonallyFrom<Flector> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> Motor {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group0.zxy) - (right_anti_dual.group1.zxy * wedge.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e321 * wedge.e423, right_anti_dual.e321 * wedge.e431, right_anti_dual.e321 * wedge.e412, -(right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
public extension Line: AntiRejectOrthogonallyFrom<Line> {
    public typedef Line AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Line: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
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
public extension Line: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e23))
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
public extension Line: AntiRejectOrthogonallyFrom<Point> {
    public typedef Line AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> Line {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (right_anti_dual.group0.yzx * wedge.group0.zxy) - (right_anti_dual.group0.zxy * wedge.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * wedge.group0.xyz) - (float3(wedge.e321) * right_anti_dual.group0.xyz)
        );
    }
}
public extension Line: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef Line AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> Line {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * wedge.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * wedge.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Motor> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Motor.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension Motor: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
public extension Motor: AntiRejectOrthogonallyFrom<Flector> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> Motor {
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
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group1.zxy) - (right_anti_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
public extension Motor: AntiRejectOrthogonallyFrom<Horizon> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension Motor: AntiRejectOrthogonallyFrom<Line> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Line) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43))
        );
    }
}
public extension Motor: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Motor {
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
public extension Motor: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group2.xyz) + (float3(other.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * other.e4) + (this.scalar * other.e423), (this.e31 * other.e4) + (this.scalar * other.e431), (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
public extension Motor: AntiRejectOrthogonallyFrom<Plane> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
public extension Motor: AntiRejectOrthogonallyFrom<Point> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> Motor {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((wedge.group1.zxy * right_anti_dual.group0.yzx) - (wedge.group1.yzx * right_anti_dual.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge.e321 * right_anti_dual.e423 * -1.0, wedge.e321 * right_anti_dual.e431 * -1.0, wedge.e321 * right_anti_dual.e412 * -1.0, (wedge.e2 * right_anti_dual.e431) + (wedge.e3 * right_anti_dual.e412) + (wedge.e4 * right_anti_dual.e321)) + (right_anti_dual.group0.wwwx * float4(wedge.group1.xyz, wedge.e1))
        );
    }
}
public extension Motor: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> Motor {
        Motor wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * wedge.group1
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<MultiVector> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> MultiVector.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension MultiVector: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        );
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual.e1234) * wedge.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group1, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * wedge.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * wedge.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group4
        );
    }
}
public extension MultiVector: AntiRejectOrthogonallyFrom<Flector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (right_anti_dual.group1.yzx * wedge.group4.zxy) - (right_anti_dual.group1.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * wedge.group4.xyz) - (float3(wedge.e321) * right_anti_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0.xy.yy, wedge.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension MultiVector: AntiRejectOrthogonallyFrom<Horizon> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Horizon) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
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
        Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(wedge.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge.e1234 * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiRejectOrthogonallyFrom<Line> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Line) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group4.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            float3(wedge.e1234) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiRejectOrthogonallyFrom<Motor> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz), 
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
            /* scalar, e1234 */
            float2((right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.scalar * wedge.e1234) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e1234 * wedge.e1) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e1234 * wedge.e2) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e1234 * wedge.e3) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) + (right_anti_dual.group0 * float4(wedge.group4.www, wedge.e4)) - (wedge.group4.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(wedge.e1234) * right_anti_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group4
        );
    }
}
public extension MultiVector: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e42 * this.e3) + (other.e23 * this.e4), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e43 * this.e1) + (other.e31 * this.e4), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group4) + (float4(this.scalar) * other.group4) - (this.group1.yzxx * float4(other.group2.xyz.zxy, other.e23)) - float4((this.group2.xyz.zxy * other.group1.yzx), other.e31 * this.e2)
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
public extension MultiVector: AntiRejectOrthogonallyFrom<Plane> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Plane) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
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
        Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(wedge.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge.e1234 * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiRejectOrthogonallyFrom<Point> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((wedge.e1 * right_anti_dual.e423) + (wedge.e2 * right_anti_dual.e431) + (wedge.e3 * right_anti_dual.e412) + (wedge.e4 * right_anti_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_anti_dual.e321) + (wedge.e31 * right_anti_dual.e412), (wedge.e42 * right_anti_dual.e321) + (wedge.e12 * right_anti_dual.e423), (wedge.e43 * right_anti_dual.e321) + (wedge.e23 * right_anti_dual.e431), -(wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(wedge.group3.xyz.zxy, wedge.e41)), 
            /* e41, e42, e43 */
            (wedge.group4.zxy * right_anti_dual.group0.yzx) - (wedge.group4.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * wedge.group4.xyz) - (float3(wedge.e321) * right_anti_dual.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0.xy.yy, wedge.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension MultiVector: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
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
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual.e1234) * wedge.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * wedge.group1, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * wedge.group2.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * wedge.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * wedge.group4
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Origin> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Origin.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension Origin: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.scalar, 2) * this.e4
        );
    }
}
public extension Origin: AntiRejectOrthogonallyFrom<Flector> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> Flector {
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
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Origin: AntiRejectOrthogonallyFrom<Horizon> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
public extension Origin: AntiRejectOrthogonallyFrom<Line> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Line) -> Origin {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Origin.from_groups(
            /* e4 */
            -(right_anti_dual.e41 * wedge.e423) - (right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)
        );
    }
}
public extension Origin: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Flector {
        Flector wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
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
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(wedge.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Origin: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(right_anti_dual.e1234) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group4.xyz)), 0.0)
        );
    }
}
public extension Origin: AntiRejectOrthogonallyFrom<Plane> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
public extension Origin: AntiRejectOrthogonallyFrom<Point> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> Origin {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Origin.from_groups(
            /* e4 */
            -(wedge.e41 * right_anti_dual.e423) - (wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)
        );
    }
}
public extension Origin: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * pow(other.scalar, 2)
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Plane> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Plane.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension Plane: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef Plane AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(other.scalar * this.e423, other.scalar * this.e431, other.scalar * this.e412, other.scalar * this.e321)
        );
    }
}
public extension Plane: AntiRejectOrthogonallyFrom<Flector> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> Flector {
        AntiScalar wedge = AntiScalar.from_groups(
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
        Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(wedge.e1234) * right_anti_dual.group1.xyz), 0.0)
        );
    }
}
public extension Plane: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Flector {
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
public extension Plane: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
public extension Plane: AntiRejectOrthogonallyFrom<Point> {
    public typedef Plane AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
public extension Plane: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef Plane AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(this.e423 * other.scalar, this.e431 * other.scalar, this.e412 * other.scalar, this.e321 * other.scalar)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Point> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<Point> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Point.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension Point: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef Point AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, other.scalar * this.e4)
        );
    }
}
public extension Point: AntiRejectOrthogonallyFrom<Flector> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> Flector {
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
            float4(other.group0.xyz, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(wedge.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Point: AntiRejectOrthogonallyFrom<Horizon> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            pow(other.e321, 2) * this.e4 * -1.0
        );
    }
}
public extension Point: AntiRejectOrthogonallyFrom<Line> {
    public typedef Point AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Line) -> Point {
        Plane wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431), -(right_anti_dual.e42 * wedge.e431) - (right_anti_dual.e43 * wedge.e412)) - (wedge.group0.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e41))
        );
    }
}
public extension Point: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Flector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Flector {
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
public extension Point: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
public extension Point: AntiRejectOrthogonallyFrom<Plane> {
    public typedef Origin AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            -(pow(other.e321, 2) * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
public extension Point: AntiRejectOrthogonallyFrom<Point> {
    public typedef Point AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> Point {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge.e41 * right_anti_dual.e321) + (wedge.e31 * right_anti_dual.e412), (wedge.e42 * right_anti_dual.e321) + (wedge.e12 * right_anti_dual.e423), (wedge.e43 * right_anti_dual.e321) + (wedge.e23 * right_anti_dual.e431), -(wedge.e42 * right_anti_dual.e431) - (wedge.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(wedge.group1.xyz.zxy, wedge.e41))
        );
    }
}
public extension Point: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef Point AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.e1 * other.scalar, this.e2 * other.scalar, this.e3 * other.scalar, this.e4 * other.scalar)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiRejectOrthogonallyFromInfix) -> AntiRejectOrthogonallyFromInfixPartial<Scalar> {
        return AntiRejectOrthogonallyFromInfixPartial(this);
    }
}
public extension AntiRejectOrthogonallyFromInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Scalar.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
public extension Scalar: AntiRejectOrthogonallyFrom<DualNum> {
    public typedef DualNum AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * float2(other.scalar * this.scalar, other.e1234 * this.scalar)
        );
    }
}
public extension Scalar: AntiRejectOrthogonallyFrom<Flector> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Flector) -> Motor {
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
            float4(other.group0.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * wedge.group1.zxy) - (right_anti_dual.group1.zxy * wedge.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(wedge.group1.xyz, wedge.e1)) + float4(float3(0.0), (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321)) - (wedge.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
    }
}
public extension Scalar: AntiRejectOrthogonallyFrom<Horizon> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension Scalar: AntiRejectOrthogonallyFrom<Line> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Line) -> Scalar {
        Line wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
        Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            -(right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43)
        );
    }
}
public extension Scalar: AntiRejectOrthogonallyFrom<Motor> {
    public typedef Motor AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Motor) -> Motor {
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
public extension Scalar: AntiRejectOrthogonallyFrom<MultiVector> {
    public typedef MultiVector AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: MultiVector) -> MultiVector {
        MultiVector wedge = MultiVector.from_groups(
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
            float2((right_anti_dual.scalar * wedge.e1234) + (right_anti_dual.e1234 * wedge.scalar) + (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4) - (right_anti_dual.e1 * wedge.e423) - (right_anti_dual.e2 * wedge.e431) - (right_anti_dual.e3 * wedge.e412) - (right_anti_dual.e4 * wedge.e321) - (right_anti_dual.e41 * wedge.e23) - (right_anti_dual.e42 * wedge.e31) - (right_anti_dual.e43 * wedge.e12) - (right_anti_dual.e23 * wedge.e41) - (right_anti_dual.e31 * wedge.e42) - (right_anti_dual.e12 * wedge.e43), right_anti_dual.e1234 * wedge.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * wedge.e321) + (right_anti_dual.e31 * wedge.e412) + (right_anti_dual.e412 * wedge.e31) + (right_anti_dual.e321 * wedge.e41), (right_anti_dual.e42 * wedge.e321) + (right_anti_dual.e12 * wedge.e423) + (right_anti_dual.e423 * wedge.e12) + (right_anti_dual.e321 * wedge.e42), (right_anti_dual.e43 * wedge.e321) + (right_anti_dual.e23 * wedge.e431) + (right_anti_dual.e431 * wedge.e23) + (right_anti_dual.e321 * wedge.e43), -(right_anti_dual.e43 * wedge.e412) - (right_anti_dual.e423 * wedge.e41) - (right_anti_dual.e431 * wedge.e42) - (right_anti_dual.e412 * wedge.e43)) + (float4(right_anti_dual.e1234) * wedge.group1) + (float4(wedge.e1234) * right_anti_dual.group1) - (wedge.group4.yzxx * float4(right_anti_dual.group3.xyz.zxy, right_anti_dual.e41)) - float4((wedge.group3.xyz.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * wedge.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * wedge.group2.xyz) + (float3(wedge.e1234) * right_anti_dual.group2.xyz) + (right_anti_dual.group4.yzx * wedge.group4.zxy) - (right_anti_dual.group4.zxy * wedge.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * wedge.group3.xyz) + (float3(right_anti_dual.e321) * wedge.group4.xyz) + (float3(wedge.e1234) * right_anti_dual.group3.xyz) - (float3(wedge.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * wedge.group4) + (float4(wedge.e1234) * right_anti_dual.group4)
        );
    }
}
public extension Scalar: AntiRejectOrthogonallyFrom<Plane> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension Scalar: AntiRejectOrthogonallyFrom<Point> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Point) -> Scalar {
        Point wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
        Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (right_anti_dual.e423 * wedge.e1) + (right_anti_dual.e431 * wedge.e2) + (right_anti_dual.e412 * wedge.e3) + (right_anti_dual.e321 * wedge.e4)
        );
    }
}
public extension Scalar: AntiRejectOrthogonallyFrom<Scalar> {
    public typedef Scalar AntiRejectOrthogonallyFromOutput;
    public func anti_reject_orthogonally_from(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
