public extension AntiScalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiScalar> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> AntiScalar.Output {
        return this.a.wedge(rhs);
    }
}
public extension AntiScalar: Wedge<DualNum> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension AntiScalar: Wedge<Motor> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension AntiScalar: Wedge<MultiVector> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension AntiScalar: Wedge<Scalar> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<DualNum> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> DualNum.Output {
        return this.a.wedge(rhs);
    }
}
public extension DualNum: Wedge<AntiScalar> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension DualNum: Wedge<DualNum> {
    public typedef DualNum WedgeOutput;
    public func wedge(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
public extension DualNum: Wedge<Flector> {
    public typedef Flector WedgeOutput;
    public func wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension DualNum: Wedge<Horizon> {
    public typedef Horizon WedgeOutput;
    public func wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.scalar * other.e321
        );
    }
}
public extension DualNum: Wedge<Line> {
    public typedef Line WedgeOutput;
    public func wedge(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension DualNum: Wedge<Motor> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
public extension DualNum: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
public extension DualNum: Wedge<Origin> {
    public typedef Origin WedgeOutput;
    public func wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.scalar * other.e4
        );
    }
}
public extension DualNum: Wedge<Plane> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension DualNum: Wedge<Point> {
    public typedef Point WedgeOutput;
    public func wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension DualNum: Wedge<Scalar> {
    public typedef DualNum WedgeOutput;
    public func wedge(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Flector> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Flector.Output {
        return this.a.wedge(rhs);
    }
}
public extension Flector: Wedge<DualNum> {
    public typedef Flector WedgeOutput;
    public func wedge(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Flector: Wedge<Flector> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
public extension Flector: Wedge<Horizon> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
public extension Flector: Wedge<Line> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Line) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Flector: Wedge<Motor> {
    public typedef Flector WedgeOutput;
    public func wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e4 * other.e23) + (this.e423 * other.scalar), (this.e4 * other.e31) + (this.e431 * other.scalar), (this.e4 * other.e12) + (this.e412 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group0.zxy * other.group0.yzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Flector: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
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
            float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        );
    }
}
public extension Flector: Wedge<Origin> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Origin) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Flector: Wedge<Plane> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)
        );
    }
}
public extension Flector: Wedge<Point> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * other.e1, this.e4 * other.e2, this.e4 * other.e3, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension Flector: Wedge<Scalar> {
    public typedef Flector WedgeOutput;
    public func wedge(other: Scalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Horizon> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Horizon.Output {
        return this.a.wedge(rhs);
    }
}
public extension Horizon: Wedge<DualNum> {
    public typedef Horizon WedgeOutput;
    public func wedge(other: DualNum) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.scalar * this.e321
        );
    }
}
public extension Horizon: Wedge<Flector> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
public extension Horizon: Wedge<Motor> {
    public typedef Horizon WedgeOutput;
    public func wedge(other: Motor) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
public extension Horizon: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
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
public extension Horizon: Wedge<Origin> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Horizon: Wedge<Point> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension Horizon: Wedge<Scalar> {
    public typedef Horizon WedgeOutput;
    public func wedge(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Line> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<Line> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Line.Output {
        return this.a.wedge(rhs);
    }
}
public extension Line: Wedge<DualNum> {
    public typedef Line WedgeOutput;
    public func wedge(other: DualNum) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension Line: Wedge<Flector> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Flector) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Line: Wedge<Line> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Line) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension Line: Wedge<Motor> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
public extension Line: Wedge<Origin> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Origin) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
public extension Line: Wedge<Point> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Line: Wedge<Scalar> {
    public typedef Line WedgeOutput;
    public func wedge(other: Scalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Motor> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Motor.Output {
        return this.a.wedge(rhs);
    }
}
public extension Motor: Wedge<AntiScalar> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension Motor: Wedge<DualNum> {
    public typedef Motor WedgeOutput;
    public func wedge(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xy.xx, other.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Motor: Wedge<Flector> {
    public typedef Flector WedgeOutput;
    public func wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar), (other.e4 * this.e31) + (other.e431 * this.scalar), (other.e4 * this.e12) + (other.e412 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension Motor: Wedge<Horizon> {
    public typedef Horizon WedgeOutput;
    public func wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
public extension Motor: Wedge<Line> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor: Wedge<Motor> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.scalar)
        );
    }
}
public extension Motor: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
public extension Motor: Wedge<Origin> {
    public typedef Flector WedgeOutput;
    public func wedge(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
public extension Motor: Wedge<Plane> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Motor: Wedge<Point> {
    public typedef Flector WedgeOutput;
    public func wedge(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension Motor: Wedge<Scalar> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<MultiVector> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> MultiVector.Output {
        return this.a.wedge(rhs);
    }
}
public extension MultiVector: Wedge<AntiScalar> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension MultiVector: Wedge<DualNum> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
public extension MultiVector: Wedge<Flector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: Flector) -> MultiVector {
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
            float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
    }
}
public extension MultiVector: Wedge<Horizon> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: Horizon) -> MultiVector {
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
public extension MultiVector: Wedge<Line> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: Line) -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
public extension MultiVector: Wedge<Motor> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
public extension MultiVector: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
public extension MultiVector: Wedge<Origin> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: Origin) -> MultiVector {
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
            float4((float3(other.e4) * this.group3.xyz), 0.0)
        );
    }
}
public extension MultiVector: Wedge<Plane> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: Plane) -> MultiVector {
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
public extension MultiVector: Wedge<Point> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: Point) -> MultiVector {
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
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group2.xyz.zxy, this.e23))
        );
    }
}
public extension MultiVector: Wedge<Scalar> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Origin> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Origin.Output {
        return this.a.wedge(rhs);
    }
}
public extension Origin: Wedge<DualNum> {
    public typedef Origin WedgeOutput;
    public func wedge(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.scalar * this.e4
        );
    }
}
public extension Origin: Wedge<Flector> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Origin: Wedge<Horizon> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
public extension Origin: Wedge<Line> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Line) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
public extension Origin: Wedge<Motor> {
    public typedef Flector WedgeOutput;
    public func wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
public extension Origin: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
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
            float4((float3(this.e4) * other.group3.xyz), 0.0)
        );
    }
}
public extension Origin: Wedge<Plane> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
public extension Origin: Wedge<Point> {
    public typedef Line WedgeOutput;
    public func wedge(other: Point) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Origin: Wedge<Scalar> {
    public typedef Origin WedgeOutput;
    public func wedge(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * other.scalar
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Plane> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Plane.Output {
        return this.a.wedge(rhs);
    }
}
public extension Plane: Wedge<DualNum> {
    public typedef Plane WedgeOutput;
    public func wedge(other: DualNum) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane: Wedge<Flector> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
    }
}
public extension Plane: Wedge<Motor> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Motor) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
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
public extension Plane: Wedge<Origin> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
public extension Plane: Wedge<Point> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension Plane: Wedge<Scalar> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Scalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Point> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<Point> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Point.Output {
        return this.a.wedge(rhs);
    }
}
public extension Point: Wedge<DualNum> {
    public typedef Point WedgeOutput;
    public func wedge(other: DualNum) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Point: Wedge<Flector> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * this.e1 * -1.0, other.e4 * this.e2 * -1.0, other.e4 * this.e3 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
public extension Point: Wedge<Horizon> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
public extension Point: Wedge<Line> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Line) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Point: Wedge<Motor> {
    public typedef Flector WedgeOutput;
    public func wedge(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Point: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
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
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.xyz.zxy, other.e23))
        );
    }
}
public extension Point: Wedge<Origin> {
    public typedef Line WedgeOutput;
    public func wedge(other: Origin) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Point: Wedge<Plane> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension Point: Wedge<Point> {
    public typedef Line WedgeOutput;
    public func wedge(other: Point) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
    }
}
public extension Point: Wedge<Scalar> {
    public typedef Point WedgeOutput;
    public func wedge(other: Scalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Scalar> {
        return WedgeInfixPartial(this);
    }
}
public extension WedgeInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Scalar.Output {
        return this.a.wedge(rhs);
    }
}
public extension Scalar: Wedge<AntiScalar> {
    public typedef AntiScalar WedgeOutput;
    public func wedge(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar: Wedge<DualNum> {
    public typedef DualNum WedgeOutput;
    public func wedge(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar: Wedge<Flector> {
    public typedef Flector WedgeOutput;
    public func wedge(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar: Wedge<Horizon> {
    public typedef Horizon WedgeOutput;
    public func wedge(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
public extension Scalar: Wedge<Line> {
    public typedef Line WedgeOutput;
    public func wedge(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension Scalar: Wedge<Motor> {
    public typedef Motor WedgeOutput;
    public func wedge(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar: Wedge<MultiVector> {
    public typedef MultiVector WedgeOutput;
    public func wedge(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
public extension Scalar: Wedge<Origin> {
    public typedef Origin WedgeOutput;
    public func wedge(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e4 * this.scalar
        );
    }
}
public extension Scalar: Wedge<Plane> {
    public typedef Plane WedgeOutput;
    public func wedge(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar: Wedge<Point> {
    public typedef Point WedgeOutput;
    public func wedge(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar: Wedge<Scalar> {
    public typedef Scalar WedgeOutput;
    public func wedge(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
