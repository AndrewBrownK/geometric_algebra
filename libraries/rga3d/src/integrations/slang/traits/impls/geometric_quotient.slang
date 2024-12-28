public extension AntiScalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<AntiScalar> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> AntiScalar.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension AntiScalar: GeometricQuotient<DualNum> {
    public typedef AntiScalar GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 / (other.scalar)
        );
    }
}
public extension AntiScalar: GeometricQuotient<Flector> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Flector {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * geometric_product.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiScalar: GeometricQuotient<Horizon> {
    public typedef Origin GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 / (other.e321)
        );
    }
}
public extension AntiScalar: GeometricQuotient<Line> {
    public typedef Line GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Line {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * float3(reverse.e23 * other_2.scalar, reverse.e31 * other_2.scalar, reverse.e12 * other_2.scalar), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension AntiScalar: GeometricQuotient<Motor> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Motor {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * float4(reverse.e23 * other_2.scalar, reverse.e31 * other_2.scalar, reverse.e12 * other_2.scalar, reverse.scalar * other_2.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension AntiScalar: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e1234 * geometric_product.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(this.e1234) * geometric_product.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * geometric_product.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiScalar: GeometricQuotient<Plane> {
    public typedef Origin GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 / (other.e321)
        );
    }
}
public extension AntiScalar: GeometricQuotient<Point> {
    public typedef Plane GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Plane {
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * float4(other.e1 * other_2.scalar, other.e2 * other_2.scalar, other.e3 * other_2.scalar, other.e4 * other_2.scalar).xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiScalar: GeometricQuotient<Scalar> {
    public typedef AntiScalar GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 / (other.scalar)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<DualNum> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> DualNum.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension DualNum: GeometricQuotient<DualNum> {
    public typedef DualNum GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> DualNum {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.scalar, -2)) * other.group0.xy
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.scalar * this.scalar, (geometric_product.scalar * this.e1234) + (geometric_product.e1234 * this.scalar))
        );
    }
}
public extension DualNum: GeometricQuotient<Flector> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Flector {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * geometric_product.e4) - (this.e1234 * geometric_product.e321)) * float4(geometric_product.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group1.xyz) - (float3(this.e1234) * geometric_product.group0.xyz)), this.scalar * geometric_product.e321)
        );
    }
}
public extension DualNum: GeometricQuotient<Horizon> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> Flector {
        Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * geometric_product.e321)
        );
    }
}
public extension DualNum: GeometricQuotient<Line> {
    public typedef Line GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Line {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1.xyz
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.scalar) * geometric_product.group0.xyz) + (float3(this.e1234) * geometric_product.group1.xyz), 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_product.group1.xyz
        );
    }
}
public extension DualNum: GeometricQuotient<Motor> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Motor {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(this.scalar) * geometric_product.group0) + (float4(this.e1234) * geometric_product.group1), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * geometric_product.group1
        );
    }
}
public extension DualNum: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * geometric_product.scalar, (this.scalar * geometric_product.e1234) + (this.e1234 * geometric_product.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, (this.scalar * geometric_product.e4) - (this.e1234 * geometric_product.e321)) * float4(geometric_product.group1.xyz, 1.0), 
            /* e41, e42, e43 */
            (float3(this.scalar) * geometric_product.group2.xyz) + (float3(this.e1234) * geometric_product.group3.xyz), 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_product.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group4.xyz) - (float3(this.e1234) * geometric_product.group1.xyz)), this.scalar * geometric_product.e321)
        );
    }
}
public extension DualNum: GeometricQuotient<Plane> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> Flector {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * geometric_product.group0
        );
    }
}
public extension DualNum: GeometricQuotient<Point> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Flector {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * geometric_product.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.xy.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(geometric_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension DualNum: GeometricQuotient<Scalar> {
    public typedef DualNum GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0/other.scalar) * this.group0.xy
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Flector> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Flector.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension Flector: GeometricQuotient<DualNum> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> Flector {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.scalar, -2)) * other.group0.xy
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.group0.xy.xx, geometric_product.scalar, (geometric_product.scalar * this.e4) + (geometric_product.e1234 * this.e321)) * float4(this.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group1.xyz) + (float3(geometric_product.e1234) * this.group0.xyz)), geometric_product.scalar * this.e321)
        );
    }
}
public extension Flector: GeometricQuotient<Flector> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Motor {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(geometric_product.e321) * float4(this.group1.xyz, this.e4)) + (geometric_product.group1.zxyz * this.group0.yzxz) + (float4(this.group0.ww, this.e431, this.e1) * float4(geometric_product.group0.xyx, geometric_product.e423)) + (float4(this.group1.zx, this.e4, this.e2) * float4(geometric_product.group0.yzz, geometric_product.e431)) - (geometric_product.group0.zxyx * this.group1.yzxx) - (geometric_product.group0.wwwy * float4(this.group0.xyz, this.e431)) - (float4(this.group0.zx, this.e321, this.e321) * float4(geometric_product.group1.yzz, geometric_product.e4)) - (float4(this.group1.ww, this.e2, this.e412) * float4(geometric_product.group1.xyx, geometric_product.e3)), 
            /* e23, e31, e12, scalar */
            float4(-(geometric_product.e2 * this.e3) - (geometric_product.e321 * this.e1), -(geometric_product.e3 * this.e1) - (geometric_product.e321 * this.e2), -(geometric_product.e3 * this.e321) - (geometric_product.e321 * this.e3), (geometric_product.e2 * this.e2) + (geometric_product.e3 * this.e3)) + (geometric_product.group0.zxyx * this.group0.yzxx) - (float4(this.group1.ww, this.e2, this.e321) * float4(geometric_product.group0.xyx, geometric_product.e321))
        );
    }
}
public extension Flector: GeometricQuotient<Horizon> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> Motor {
        Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product.e321) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
public extension Flector: GeometricQuotient<Line> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Flector {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1.xyz
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e3 * geometric_product.e31) + (this.e321 * geometric_product.e23), (this.e1 * geometric_product.e12) + (this.e321 * geometric_product.e31), (this.e2 * geometric_product.e23) + (this.e321 * geometric_product.e12), -(this.e2 * geometric_product.e42) - (this.e3 * geometric_product.e43) - (this.e423 * geometric_product.e23) - (this.e431 * geometric_product.e31) - (this.e412 * geometric_product.e12)) - (this.group0.yzxx * float4(geometric_product.group1.xyz.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * geometric_product.e42) + (this.e4 * geometric_product.e23) + (this.e412 * geometric_product.e31) + (this.e321 * geometric_product.e41), (this.e1 * geometric_product.e43) + (this.e4 * geometric_product.e31) + (this.e423 * geometric_product.e12) + (this.e321 * geometric_product.e42), (this.e2 * geometric_product.e41) + (this.e4 * geometric_product.e12) + (this.e431 * geometric_product.e23) + (this.e321 * geometric_product.e43), this.e3 * geometric_product.e12 * -1.0) - (this.group0.yzxx * float4(geometric_product.group0.xyz.zxy, geometric_product.e23)) - float4((geometric_product.group1.xyz.zxy * this.group1.yzx), this.e2 * geometric_product.e31)
        );
    }
}
public extension Flector: GeometricQuotient<Motor> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Flector {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321 * geometric_product.e23, this.e321 * geometric_product.e31, this.e321 * geometric_product.e12, -(this.e2 * geometric_product.e42) - (this.e3 * geometric_product.e43) - (this.e423 * geometric_product.e23) - (this.e431 * geometric_product.e31) - (this.e412 * geometric_product.e12)) + (this.group0.xxyw * geometric_product.group1.wzxw) + float4((this.group0.zyz * geometric_product.group1.yww), this.e321 * geometric_product.e1234) - (this.group0.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * geometric_product.e42) + (this.e4 * geometric_product.e23) + (this.e423 * geometric_product.scalar) + (this.e412 * geometric_product.e31) + (this.e321 * geometric_product.e41), (this.e2 * geometric_product.e1234) + (this.e4 * geometric_product.e31) + (this.e423 * geometric_product.e12) + (this.e431 * geometric_product.scalar) + (this.e321 * geometric_product.e42), (this.e3 * geometric_product.e1234) + (this.e4 * geometric_product.e12) + (this.e431 * geometric_product.e23) + (this.e412 * geometric_product.scalar) + (this.e321 * geometric_product.e43), this.e3 * geometric_product.e12 * -1.0) + float4((this.group0.xxy * geometric_product.group0.wzx), this.e321 * geometric_product.scalar) - (this.group0.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23)) - (geometric_product.group1.zxyy * float4(this.group1.yzx, this.e2))
        );
    }
}
public extension Flector: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e4 * geometric_product.e321) - (this.e431 * geometric_product.e2) - (this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)) - (float2(geometric_product.e321, geometric_product.e1) * this.group1.wx), 
            /* e1, e2, e3, e4 */
            float4(this.e3 * geometric_product.e31, this.e1 * geometric_product.e12, this.e321 * geometric_product.e12, -(this.e2 * geometric_product.e42) - (this.e3 * geometric_product.e43) - (this.e423 * geometric_product.e23) - (this.e431 * geometric_product.e31) - (this.e412 * geometric_product.e12)) + (float4(geometric_product.scalar) * this.group0) + (float4(this.group1.ww, this.e2, this.e321) * float4(geometric_product.group3.xyz.xyx, geometric_product.e1234)) - (this.group0.yzxx * float4(geometric_product.group3.xyz.zxy, geometric_product.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * geometric_product.group1.xyz) + (float3(geometric_product.e2, geometric_product.e321, geometric_product.e321) * this.group1.zyz) + (float3(geometric_product.e321, geometric_product.e3, geometric_product.e1) * this.group1.xxy) + (this.group0.yzx * geometric_product.group4.zxy) - (float3(this.e321) * geometric_product.group4.xyz) - (float3(geometric_product.e4, geometric_product.e412, geometric_product.e423) * this.group0.xxy) - (float3(geometric_product.e431, geometric_product.e4, geometric_product.e4) * this.group0.zyz) - (this.group1.yzx * geometric_product.group1.zxy), 
            /* e23, e31, e12 */
            (this.group0.yzx * geometric_product.group1.zxy) - (float3(this.e321) * geometric_product.group1.xyz) - (float3(geometric_product.e2, geometric_product.e321, geometric_product.e321) * this.group0.zyz) - (float3(geometric_product.e321, geometric_product.e3, geometric_product.e1) * this.group0.xxy), 
            /* e423, e431, e412, e321 */
            float4((this.e1 * geometric_product.e1234) + (this.e3 * geometric_product.e42) + (this.e4 * geometric_product.e23) + (this.e412 * geometric_product.e31) + (this.e321 * geometric_product.e41), (this.e1 * geometric_product.e43) + (this.e2 * geometric_product.e1234) + (this.e4 * geometric_product.e31) + (this.e423 * geometric_product.e12) + (this.e321 * geometric_product.e42), (this.e2 * geometric_product.e41) + (this.e3 * geometric_product.e1234) + (this.e4 * geometric_product.e12) + (this.e431 * geometric_product.e23) + (this.e321 * geometric_product.e43), this.e3 * geometric_product.e12 * -1.0) + (float4(geometric_product.scalar) * this.group1) - (this.group0.yzxx * float4(geometric_product.group2.xyz.zxy, geometric_product.e23)) - float4((geometric_product.group3.xyz.zxy * this.group1.yzx), this.e2 * geometric_product.e31)
        );
    }
}
public extension Flector: GeometricQuotient<Plane> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> Motor {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e3 * geometric_product.e431) - (this.e321 * geometric_product.e423), -(this.e1 * geometric_product.e412) - (this.e321 * geometric_product.e431), -(this.e2 * geometric_product.e423) - (this.e321 * geometric_product.e412), (this.e3 * geometric_product.e412) + (this.e4 * geometric_product.e321)) + (this.group0.yzxx * geometric_product.group0.zxyx) + (geometric_product.group0.wwwy * float4(this.group1.xyz, this.e2)), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
public extension Flector: GeometricQuotient<Point> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Motor {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e4 * geometric_product.e1) + (this.e412 * geometric_product.e2), (this.e4 * geometric_product.e2) + (this.e423 * geometric_product.e3), (this.e4 * geometric_product.e3) + (this.e431 * geometric_product.e1), -(this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)) - (this.group1.yzxy * geometric_product.group0.zxyy) - (geometric_product.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(this.e3 * geometric_product.e2) - (this.e321 * geometric_product.e1), -(this.e1 * geometric_product.e3) - (this.e321 * geometric_product.e2), -(this.e2 * geometric_product.e1) - (this.e321 * geometric_product.e3), (this.e2 * geometric_product.e2) + (this.e3 * geometric_product.e3)) + (this.group0.yzxx * geometric_product.group0.zxyx)
        );
    }
}
public extension Flector: GeometricQuotient<Scalar> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> Flector {
        Scalar geometric_product = Scalar.from_groups(
            /* scalar */
            1.0/other.scalar
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_product.scalar) * this.group1
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Horizon> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Horizon.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension Horizon: GeometricQuotient<DualNum> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> Flector {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.scalar, -2)) * other.group0.xy
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_product.scalar * this.e321)
        );
    }
}
public extension Horizon: GeometricQuotient<Flector> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Motor {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e321) * float4(geometric_product.group1.xyz, geometric_product.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(geometric_product.group0.xyz, geometric_product.e321) * float4(-1.0)
        );
    }
}
public extension Horizon: GeometricQuotient<Horizon> {
    public typedef Scalar GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 / (other.e321)
        );
    }
}
public extension Horizon: GeometricQuotient<Line> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Flector {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1.xyz
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_product.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * geometric_product.group0.xyz), 0.0)
        );
    }
}
public extension Horizon: GeometricQuotient<Motor> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Flector {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(geometric_product.group1.xyz, geometric_product.e1234), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(geometric_product.group0.xyz, geometric_product.scalar)
        );
    }
}
public extension Horizon: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321) * float2(geometric_product.e321, geometric_product.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(geometric_product.group3.xyz, geometric_product.e1234), 
            /* e41, e42, e43 */
            float3(this.e321) * geometric_product.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e321) * geometric_product.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(geometric_product.group2.xyz, geometric_product.scalar)
        );
    }
}
public extension Horizon: GeometricQuotient<Plane> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> Motor {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e321) * geometric_product.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e321 * geometric_product.e321 * -1.0)
        );
    }
}
public extension Horizon: GeometricQuotient<Point> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Motor {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e321 * geometric_product.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321) * geometric_product.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Horizon: GeometricQuotient<Scalar> {
    public typedef Horizon GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 / (other.scalar)
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Line> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<Line> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Line.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension Line: GeometricQuotient<DualNum> {
    public typedef Line GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> Line {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.scalar, -2)) * other.group0.xy
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group0.xyz) + (float3(geometric_product.e1234) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3(geometric_product.scalar) * this.group1.xyz
        );
    }
}
public extension Line: GeometricQuotient<Flector> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Flector {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group1.xyz.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group1.xyz.xxy, this.e41)) + float4(float3(0.0), (geometric_product.e3 * this.e43) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12)) - float4((this.group1.xyz.yzx * geometric_product.group0.zxy), geometric_product.e423 * this.e23), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e3 * this.e42) + (geometric_product.e4 * this.e23) + (geometric_product.e431 * this.e12), (geometric_product.e1 * this.e43) + (geometric_product.e4 * this.e31) + (geometric_product.e412 * this.e23), (geometric_product.e2 * this.e41) + (geometric_product.e4 * this.e12) + (geometric_product.e423 * this.e31), 0.0) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group0.xyz.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group0.xyz.xxy, this.e23)) - float4((this.group1.xyz.yzx * geometric_product.group1.zxy), geometric_product.e3 * this.e12)
        );
    }
}
public extension Line: GeometricQuotient<Horizon> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> Flector {
        Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product.e321) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_product.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Line: GeometricQuotient<Line> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Motor {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1.xyz
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e42 * this.e12) + (geometric_product.e31 * this.e43), (geometric_product.e43 * this.e23) + (geometric_product.e12 * this.e41), (geometric_product.e41 * this.e31) + (geometric_product.e23 * this.e42), -(geometric_product.e43 * this.e12) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) - float4((geometric_product.group0.xyz.zxy * this.group1.xyz.yzx), geometric_product.e41 * this.e23) - float4((geometric_product.group1.xyz.zxy * this.group0.xyz.yzx), geometric_product.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e31 * this.e12, geometric_product.e12 * this.e23, geometric_product.e23 * this.e31, -(geometric_product.e31 * this.e31) - (geometric_product.e12 * this.e12)) - float4((geometric_product.group1.xyz.zxy * this.group1.xyz.yzx), geometric_product.e23 * this.e23)
        );
    }
}
public extension Line: GeometricQuotient<Motor> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Motor {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * geometric_product.scalar) + (this.e43 * geometric_product.e31) + (this.e23 * geometric_product.e1234) + (this.e12 * geometric_product.e42), (this.e41 * geometric_product.e12) + (this.e42 * geometric_product.scalar) + (this.e23 * geometric_product.e43) + (this.e31 * geometric_product.e1234), (this.e42 * geometric_product.e23) + (this.e43 * geometric_product.scalar) + (this.e31 * geometric_product.e41) + (this.e12 * geometric_product.e1234), -(this.e43 * geometric_product.e12) - (this.e23 * geometric_product.e41) - (this.e31 * geometric_product.e42) - (this.e12 * geometric_product.e43)) - (geometric_product.group1.zxyx * float4(this.group0.xyz.yzx, this.e41)) - float4((this.group1.xyz.yzx * geometric_product.group0.zxy), this.e42 * geometric_product.e31), 
            /* e23, e31, e12, scalar */
            float4((this.e23 * geometric_product.scalar) + (this.e12 * geometric_product.e31), (this.e23 * geometric_product.e12) + (this.e31 * geometric_product.scalar), (this.e31 * geometric_product.e23) + (this.e12 * geometric_product.scalar), -(this.e31 * geometric_product.e31) - (this.e12 * geometric_product.e12)) - (geometric_product.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
public extension Line: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e23 * geometric_product.e41) - (this.e31 * geometric_product.e42) - (this.e12 * geometric_product.e43)) - (float2(geometric_product.e23) * float2(this.e23, this.e41)) - (float2(geometric_product.e31) * float2(this.e31, this.e42)) - (float2(geometric_product.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group1.xyz.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group1.xyz.xxy, this.e41)) + float4(float3(0.0), (this.e43 * geometric_product.e3) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)) - float4((this.group1.xyz.yzx * geometric_product.group1.zxy), this.e23 * geometric_product.e423), 
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group0.xyz) + (float3(geometric_product.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * geometric_product.group3.xyz.yzx) + (this.group1.xyz.zxy * geometric_product.group2.xyz.yzx) - (this.group0.xyz.yzx * geometric_product.group3.xyz.zxy) - (this.group1.xyz.yzx * geometric_product.group2.xyz.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_product.scalar) * this.group1.xyz) + (this.group1.xyz.zxy * geometric_product.group3.xyz.yzx) - (this.group1.xyz.yzx * geometric_product.group3.xyz.zxy), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4) + (this.e12 * geometric_product.e431), (this.e43 * geometric_product.e1) + (this.e23 * geometric_product.e412) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e31 * geometric_product.e423) + (this.e12 * geometric_product.e4), 0.0) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group0.xyz.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group0.xyz.xxy, this.e23)) - float4((this.group1.xyz.yzx * geometric_product.group4.zxy), this.e12 * geometric_product.e3)
        );
    }
}
public extension Line: GeometricQuotient<Plane> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> Flector {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product.e321) * this.group1.xyz), -(this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)), 
            /* e423, e431, e412, e321 */
            float4((this.group1.xyz.zxy * geometric_product.group0.yzx), 0.0) - float4((float3(geometric_product.e321) * this.group0.xyz), 0.0) - float4((this.group1.xyz.yzx * geometric_product.group0.zxy), 0.0)
        );
    }
}
public extension Line: GeometricQuotient<Point> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Flector {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * geometric_product.e3 * -1.0, this.e12 * geometric_product.e1 * -1.0, this.e23 * geometric_product.e2 * -1.0, (this.e42 * geometric_product.e2) + (this.e43 * geometric_product.e3)) + (geometric_product.group0.yzxx * float4(this.group1.xyz.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4), (this.e43 * geometric_product.e1) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e12 * geometric_product.e4), -(this.e31 * geometric_product.e2) - (this.e12 * geometric_product.e3)) - (geometric_product.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Line: GeometricQuotient<Scalar> {
    public typedef Line GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> Line {
        Scalar geometric_product = Scalar.from_groups(
            /* scalar */
            1.0/other.scalar
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(geometric_product.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(geometric_product.scalar) * this.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Motor> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Motor.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension Motor: GeometricQuotient<DualNum> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> Motor {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.scalar, -2)) * other.group0.xy
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(geometric_product.scalar) * this.group0) + (float4(geometric_product.e1234) * this.group1), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.scalar) * this.group1
        );
    }
}
public extension Motor: GeometricQuotient<Flector> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Flector {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (geometric_product.group0.xyxx * float4(this.group1.wwy, this.e41)) + (geometric_product.group0.yzzy * float4(this.group1.zxw, this.e42)) + float4(float3(0.0), (geometric_product.e4 * this.scalar) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12) - (geometric_product.e321 * this.e1234)) + float4((geometric_product.group1.www * this.group1.xyz), geometric_product.e3 * this.e43) - (this.group1.yzxx * float4(geometric_product.group0.zxy, geometric_product.e423)), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e4 * this.e23) + (geometric_product.e423 * this.scalar) + (geometric_product.e431 * this.e12) - (geometric_product.e321 * this.e41), (geometric_product.e4 * this.e31) + (geometric_product.e431 * this.scalar) + (geometric_product.e412 * this.e23) - (geometric_product.e321 * this.e42), (geometric_product.e4 * this.e12) + (geometric_product.e423 * this.e31) + (geometric_product.e412 * this.scalar) - (geometric_product.e321 * this.e43), 0.0) + float4((geometric_product.group0.zxy * this.group0.yzx), geometric_product.e321 * this.scalar) - (geometric_product.group0.xyxx * float4(this.group0.wwy, this.e23)) - (geometric_product.group0.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(geometric_product.group1.zxy, geometric_product.e3))
        );
    }
}
public extension Motor: GeometricQuotient<Horizon> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> Flector {
        Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.e321) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.e321) * float4(this.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension Motor: GeometricQuotient<Line> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Motor {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1.xyz
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e41 * this.scalar) + (geometric_product.e42 * this.e12) + (geometric_product.e23 * this.e1234) + (geometric_product.e31 * this.e43), (geometric_product.e42 * this.scalar) + (geometric_product.e43 * this.e23) + (geometric_product.e31 * this.e1234) + (geometric_product.e12 * this.e41), (geometric_product.e41 * this.e31) + (geometric_product.e43 * this.scalar) + (geometric_product.e23 * this.e42) + (geometric_product.e12 * this.e1234), -(geometric_product.e43 * this.e12) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) - (this.group1.yzxx * float4(geometric_product.group0.xyz.zxy, geometric_product.e41)) - float4((geometric_product.group1.xyz.zxy * this.group0.yzx), geometric_product.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((geometric_product.e23 * this.scalar) + (geometric_product.e31 * this.e12), (geometric_product.e31 * this.scalar) + (geometric_product.e12 * this.e23), (geometric_product.e23 * this.e31) + (geometric_product.e12 * this.scalar), -(geometric_product.e31 * this.e31) - (geometric_product.e12 * this.e12)) - (this.group1.yzxx * float4(geometric_product.group1.xyz.zxy, geometric_product.e23))
        );
    }
}
public extension Motor: GeometricQuotient<Motor> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Motor {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e1234 * this.e23) + (geometric_product.e23 * this.e1234) + (geometric_product.e31 * this.e43) + (geometric_product.scalar * this.e41), (geometric_product.e1234 * this.e31) + (geometric_product.e31 * this.e1234) + (geometric_product.e12 * this.e41) + (geometric_product.scalar * this.e42), (geometric_product.e1234 * this.e12) + (geometric_product.e23 * this.e42) + (geometric_product.e12 * this.e1234) + (geometric_product.scalar * this.e43), -(geometric_product.e43 * this.e12) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) + (geometric_product.group0.xyxw * this.group1.wwyw) + float4((geometric_product.group0.yzz * this.group1.zxw), geometric_product.scalar * this.e1234) - (geometric_product.group0.zxyx * this.group1.yzxx) - float4((geometric_product.group1.zxy * this.group0.yzx), geometric_product.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((geometric_product.e31 * this.e12) + (geometric_product.scalar * this.e23), (geometric_product.e12 * this.e23) + (geometric_product.scalar * this.e31), (geometric_product.e12 * this.scalar) + (geometric_product.scalar * this.e12), -(geometric_product.e31 * this.e31) - (geometric_product.e12 * this.e12)) + (geometric_product.group1.xyxw * this.group1.wwyw) - (geometric_product.group1.zxyx * this.group1.yzxx)
        );
    }
}
public extension Motor: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.scalar * geometric_product.e1234) - (this.e41 * geometric_product.e23) - (this.e42 * geometric_product.e31) - (this.e43 * geometric_product.e12)) + (float2(geometric_product.scalar) * float2(this.scalar, this.e1234)) - (float2(this.e23) * float2(geometric_product.e23, geometric_product.e41)) - (float2(this.e31) * float2(geometric_product.e31, geometric_product.e42)) - (float2(this.e12) * float2(geometric_product.e12, geometric_product.e43)), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group1.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group1.xxy, this.e41)) + (geometric_product.group1.xyzz * float4(this.group1.www, this.e43)) + float4(float3(0.0), (this.scalar * geometric_product.e4) - (this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)) - float4((this.group1.yzx * geometric_product.group1.zxy), this.e1234 * geometric_product.e321), 
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group0.xyz) + (float3(geometric_product.e1234) * this.group1.xyz) + (geometric_product.group2.xyz.xyx * this.group1.wwy) + (geometric_product.group2.xyz.yzz * this.group1.zxw) + (geometric_product.group3.xyz.xyx * this.group0.wwy) + (geometric_product.group3.xyz.yzz * this.group0.zxw) - (geometric_product.group2.xyz.zxy * this.group1.yzx) - (geometric_product.group3.xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(geometric_product.scalar) * this.group1.xyz) + (geometric_product.group3.xyz.xyx * this.group1.wwy) + (geometric_product.group3.xyz.yzz * this.group1.zxw) - (geometric_product.group3.xyz.zxy * this.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * geometric_product.e4) + (this.e12 * geometric_product.e431) + (this.scalar * geometric_product.e423) - (this.e31 * geometric_product.e412), (this.e23 * geometric_product.e412) + (this.e31 * geometric_product.e4) + (this.scalar * geometric_product.e431) - (this.e12 * geometric_product.e423), (this.e31 * geometric_product.e423) + (this.e12 * geometric_product.e4) + (this.scalar * geometric_product.e412) - (this.e23 * geometric_product.e431), 0.0) + float4((this.group0.yzx * geometric_product.group1.zxy), this.scalar * geometric_product.e321) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group0.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group0.xxy, this.e23)) - (geometric_product.group1.xyzz * float4(this.group0.www, this.e12))
        );
    }
}
public extension Motor: GeometricQuotient<Plane> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> Flector {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product.e321) * this.group1.xyz), -(this.e1234 * geometric_product.e321) - (this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group0.xyz) + (this.group1.zxy * geometric_product.group0.yzx) - (float3(geometric_product.e321) * this.group0.xyz) - (this.group1.yzx * geometric_product.group0.zxy)), this.scalar * geometric_product.e321)
        );
    }
}
public extension Motor: GeometricQuotient<Point> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Flector {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)) * other.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * geometric_product.e3 * -1.0, this.e12 * geometric_product.e1 * -1.0, this.e23 * geometric_product.e2 * -1.0, (this.e43 * geometric_product.e3) + (this.scalar * geometric_product.e4)) + (geometric_product.group0.xyzy * float4(this.group1.www, this.e42)) + (geometric_product.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4), (this.e43 * geometric_product.e1) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e12 * geometric_product.e4), this.e12 * geometric_product.e3 * -1.0) - (geometric_product.group0.xyzy * float4(this.group0.www, this.e31)) - (geometric_product.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension Motor: GeometricQuotient<Scalar> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> Motor {
        Scalar geometric_product = Scalar.from_groups(
            /* scalar */
            1.0/other.scalar
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(geometric_product.scalar) * this.group1
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<MultiVector> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> MultiVector.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension MultiVector: GeometricQuotient<DualNum> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> MultiVector {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.scalar, -2)) * other.group0.xy
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.scalar * this.scalar, (geometric_product.scalar * this.e1234) + (geometric_product.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.group0.xy.xx, geometric_product.scalar, (geometric_product.scalar * this.e4) + (geometric_product.e1234 * this.e321)) * float4(this.group1.xyz, 1.0), 
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group2.xyz) + (float3(geometric_product.e1234) * this.group3.xyz), 
            /* e23, e31, e12 */
            float3(geometric_product.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group4.xyz) + (float3(geometric_product.e1234) * this.group1.xyz)), geometric_product.scalar * this.e321)
        );
    }
}
public extension MultiVector: GeometricQuotient<Flector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> MultiVector {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (geometric_product.e321 * this.e4) - (geometric_product.e2 * this.e431) - (geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321)) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)) - (float2(geometric_product.e321, geometric_product.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(this.scalar) * geometric_product.group0) + (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group3.xyz.zyz, this.e42)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group3.xyz.xxy, this.e41)) + float4(float3(0.0), (geometric_product.e3 * this.e43) - (geometric_product.e423 * this.e23) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12)) - float4((this.group3.xyz.yzx * geometric_product.group0.zxy), geometric_product.e321 * this.e1234), 
            /* e41, e42, e43 */
            (float3(geometric_product.e321) * this.group4.xyz) + (float3(this.e4, this.e4, this.e431) * geometric_product.group0.xyx) + (float3(this.e412, this.e423, this.e4) * geometric_product.group0.yzz) + (geometric_product.group1.zxy * this.group1.yzx) - (float3(geometric_product.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group1.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group1.xyx) - (geometric_product.group0.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (geometric_product.group0.zxy * this.group1.yzx) - (float3(geometric_product.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group0.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group0.xyx), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e3 * this.e42) + (geometric_product.e4 * this.e23) + (geometric_product.e431 * this.e12) - (geometric_product.e412 * this.e31), (geometric_product.e1 * this.e43) + (geometric_product.e4 * this.e31) + (geometric_product.e412 * this.e23) - (geometric_product.e423 * this.e12), (geometric_product.e2 * this.e41) + (geometric_product.e4 * this.e12) + (geometric_product.e423 * this.e31) - (geometric_product.e431 * this.e23), 0.0) + (float4(this.scalar) * geometric_product.group1) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e3) * float4(this.group2.xyz.zyz, this.e12)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e2) * float4(this.group2.xyz.xxy, this.e31)) - (geometric_product.group0.xyzx * float4(this.group0.xy.yy, this.e1234, this.e23))
        );
    }
}
public extension MultiVector: GeometricQuotient<Horizon> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> MultiVector {
        Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.e321) * float2(this.e321, this.e4) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.e321) * float4(this.group3.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(geometric_product.e321) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(geometric_product.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.e321) * float4(this.group2.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension MultiVector: GeometricQuotient<Line> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> MultiVector {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1.xyz
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) - (float2(this.e23) * float2(geometric_product.e23, geometric_product.e41)) - (float2(this.e31) * float2(geometric_product.e31, geometric_product.e42)) - (float2(this.e12) * float2(geometric_product.e12, geometric_product.e43)), 
            /* e1, e2, e3, e4 */
            float4((geometric_product.e23 * this.e321) + (geometric_product.e31 * this.e3), (geometric_product.e31 * this.e321) + (geometric_product.e12 * this.e1), (geometric_product.e23 * this.e2) + (geometric_product.e12 * this.e321), -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)) - (this.group1.yzxx * float4(geometric_product.group1.xyz.zxy, geometric_product.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * geometric_product.group0.xyz) + (float3(this.e1234) * geometric_product.group1.xyz) + (geometric_product.group0.xyz.yzx * this.group3.xyz.zxy) + (geometric_product.group1.xyz.yzx * this.group2.xyz.zxy) - (geometric_product.group0.xyz.zxy * this.group3.xyz.yzx) - (geometric_product.group1.xyz.zxy * this.group2.xyz.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_product.group1.xyz) + (geometric_product.group1.xyz.yzx * this.group3.xyz.zxy) - (geometric_product.group1.xyz.zxy * this.group3.xyz.yzx), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e41 * this.e321) + (geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4) + (geometric_product.e31 * this.e412), (geometric_product.e42 * this.e321) + (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4) + (geometric_product.e12 * this.e423), (geometric_product.e41 * this.e2) + (geometric_product.e43 * this.e321) + (geometric_product.e23 * this.e431) + (geometric_product.e12 * this.e4), geometric_product.e12 * this.e3 * -1.0) - (this.group1.yzxx * float4(geometric_product.group0.xyz.zxy, geometric_product.e23)) - float4((geometric_product.group1.xyz.zxy * this.group4.yzx), geometric_product.e31 * this.e2)
        );
    }
}
public extension MultiVector: GeometricQuotient<Motor> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> MultiVector {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (geometric_product.scalar * this.e1234) - (geometric_product.e41 * this.e23) - (geometric_product.e42 * this.e31) - (geometric_product.e43 * this.e12)) + (float2(this.scalar) * float2(geometric_product.scalar, geometric_product.e1234)) - (float2(geometric_product.e23) * float2(this.e23, this.e41)) - (float2(geometric_product.e31) * float2(this.e31, this.e42)) - (float2(geometric_product.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar * this.e1, geometric_product.scalar * this.e2, geometric_product.scalar * this.e3, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)) + (geometric_product.group1.yzzw * float4(this.group1.zx, this.e321, this.e4)) + (float4(this.group4.ww, this.e2, this.e321) * float4(geometric_product.group1.xyx, geometric_product.e1234)) - (this.group1.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * geometric_product.group0.xyz) + (float3(this.e1234) * geometric_product.group1.xyz) + (this.group2.xyz.xxy * geometric_product.group1.wzx) + (this.group2.xyz.zyz * geometric_product.group1.yww) + (this.group3.xyz.xxy * geometric_product.group0.wzx) + (this.group3.xyz.zyz * geometric_product.group0.yww) - (this.group2.xyz.yzx * geometric_product.group1.zxy) - (this.group3.xyz.yzx * geometric_product.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_product.group1.xyz) + (this.group3.xyz.xxy * geometric_product.group1.wzx) + (this.group3.xyz.zyz * geometric_product.group1.yww) - (this.group3.xyz.yzx * geometric_product.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e42 * this.e3) + (geometric_product.e1234 * this.e1) + (geometric_product.e23 * this.e4) + (geometric_product.e31 * this.e412) + (geometric_product.scalar * this.e423), (geometric_product.e43 * this.e1) + (geometric_product.e1234 * this.e2) + (geometric_product.e31 * this.e4) + (geometric_product.e12 * this.e423) + (geometric_product.scalar * this.e431), (geometric_product.e43 * this.e321) + (geometric_product.e1234 * this.e3) + (geometric_product.e23 * this.e431) + (geometric_product.e12 * this.e4) + (geometric_product.scalar * this.e412), geometric_product.e12 * this.e3 * -1.0) + (float4(this.group4.ww, this.e2, this.e321) * float4(geometric_product.group0.xyx, geometric_product.scalar)) - (geometric_product.group1.zxyy * float4(this.group4.yzx, this.e2)) - (this.group1.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23))
        );
    }
}
public extension MultiVector: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (geometric_product.e1234 * this.scalar) + (geometric_product.e321 * this.e4) - (geometric_product.e2 * this.e431) - (geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321) - (geometric_product.e23 * this.e41) - (geometric_product.e31 * this.e42) - (geometric_product.e12 * this.e43)) + (float2(geometric_product.scalar) * this.group0.xy) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)) - (float2(this.e23) * float2(geometric_product.e23, geometric_product.e41)) - (float2(this.e31) * float2(geometric_product.e31, geometric_product.e42)) - (float2(this.e12) * float2(geometric_product.e12, geometric_product.e43)) - (float2(geometric_product.e321, geometric_product.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product.scalar) * this.group1) + (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e3) * float4(this.group3.xyz.zyz, this.e43)) + (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e2) * float4(this.group3.xyz.xxy, this.e42)) + (float4(this.group0.xy.xx, this.scalar, geometric_product.e1234) * float4(geometric_product.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, geometric_product.e1) * float4(geometric_product.group3.xyz.yzz, this.e41)) + (float4(this.group4.ww, this.e2, geometric_product.e4) * float4(geometric_product.group3.xyz.xyx, this.scalar)) + float4(float3(0.0), -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412) - (geometric_product.e423 * this.e23) - (geometric_product.e431 * this.e31) - (geometric_product.e412 * this.e12)) - float4((geometric_product.group3.xyz.zxy * this.group1.yzx), geometric_product.e321 * this.e1234) - float4((this.group3.xyz.yzx * geometric_product.group1.zxy), geometric_product.e41 * this.e1), 
            /* e41, e42, e43 */
            (float3(geometric_product.scalar) * this.group2.xyz) + (float3(geometric_product.e1234) * this.group3.xyz) + (float3(geometric_product.e321) * this.group4.xyz) + (float3(this.scalar) * geometric_product.group2.xyz) + (float3(this.e1234) * geometric_product.group3.xyz) + (float3(this.e4, this.e4, this.e431) * geometric_product.group1.xyx) + (float3(this.e412, this.e423, this.e4) * geometric_product.group1.yzz) + (geometric_product.group2.xyz.yzx * this.group3.xyz.zxy) + (geometric_product.group3.xyz.yzx * this.group2.xyz.zxy) + (geometric_product.group4.zxy * this.group1.yzx) - (float3(geometric_product.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group4.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group4.xyx) - (geometric_product.group2.xyz.zxy * this.group3.xyz.yzx) - (geometric_product.group3.xyz.zxy * this.group2.xyz.yzx) - (geometric_product.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(geometric_product.scalar) * this.group3.xyz) + (float3(this.scalar) * geometric_product.group3.xyz) + (geometric_product.group3.xyz.yzx * this.group3.xyz.zxy) + (geometric_product.group1.zxy * this.group1.yzx) - (float3(geometric_product.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * geometric_product.group1.yzz) - (float3(this.e321, this.e321, this.e2) * geometric_product.group1.xyx) - (geometric_product.group3.xyz.zxy * this.group3.xyz.yzx), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e3 * this.e42) + (geometric_product.e4 * this.e23) + (geometric_product.e41 * this.e321) + (geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4) + (geometric_product.e31 * this.e412) + (geometric_product.e423 * this.scalar) + (geometric_product.e431 * this.e12), (geometric_product.e1 * this.e43) + (geometric_product.e4 * this.e31) + (geometric_product.e42 * this.e321) + (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4) + (geometric_product.e12 * this.e423) + (geometric_product.e431 * this.scalar) + (geometric_product.e412 * this.e23), (geometric_product.e2 * this.e41) + (geometric_product.e4 * this.e12) + (geometric_product.e41 * this.e2) + (geometric_product.e43 * this.e321) + (geometric_product.e23 * this.e431) + (geometric_product.e12 * this.e4) + (geometric_product.e423 * this.e31) + (geometric_product.e412 * this.scalar), 0.0) + (float4(geometric_product.scalar) * this.group4) + (float4(geometric_product.group0.xy.yy, geometric_product.e1234, this.scalar) * float4(this.group1.xyz, geometric_product.e321)) - (float4(geometric_product.e2, geometric_product.e321, geometric_product.e321, geometric_product.e2) * float4(this.group2.xyz.zyz, this.e31)) - (float4(geometric_product.e321, geometric_product.e3, geometric_product.e1, geometric_product.e1) * float4(this.group2.xyz.xxy, this.e23)) - (this.group1.yzxy * float4(geometric_product.group2.xyz.zxy, geometric_product.e31)) - (float4(this.group0.xy.yy, this.e1234, geometric_product.e23) * float4(geometric_product.group1.xyz, this.e1)) - float4((geometric_product.group3.xyz.zxy * this.group4.yzx), geometric_product.e12 * this.e3) - float4((this.group3.xyz.yzx * geometric_product.group4.zxy), geometric_product.e3 * this.e12)
        );
    }
}
public extension MultiVector: GeometricQuotient<Plane> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> MultiVector {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * geometric_product.e321, (this.e1 * geometric_product.e423) + (this.e2 * geometric_product.e431) + (this.e3 * geometric_product.e412) + (this.e4 * geometric_product.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product.e321) * this.group3.xyz), -(this.e1234 * geometric_product.e321) - (this.e23 * geometric_product.e423) - (this.e31 * geometric_product.e431) - (this.e12 * geometric_product.e412)), 
            /* e41, e42, e43 */
            (float3(geometric_product.e321) * this.group4.xyz) + (this.group1.yzx * geometric_product.group0.zxy) - (float3(this.e321) * geometric_product.group0.xyz) - (this.group1.zxy * geometric_product.group0.yzx), 
            /* e23, e31, e12 */
            float3(geometric_product.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product.group0.xyz) + (this.group3.xyz.zxy * geometric_product.group0.yzx) - (float3(geometric_product.e321) * this.group2.xyz) - (this.group3.xyz.yzx * geometric_product.group0.zxy)), this.scalar * geometric_product.e321)
        );
    }
}
public extension MultiVector: GeometricQuotient<Point> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> MultiVector {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)) * other.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * geometric_product.e1) + (this.e2 * geometric_product.e2) + (this.e3 * geometric_product.e3), -(this.e423 * geometric_product.e1) - (this.e431 * geometric_product.e2) - (this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)), 
            /* e1, e2, e3, e4 */
            float4(this.e31 * geometric_product.e3 * -1.0, this.e12 * geometric_product.e1 * -1.0, this.e23 * geometric_product.e2 * -1.0, (this.e42 * geometric_product.e2) + (this.e43 * geometric_product.e3)) + (float4(this.scalar) * geometric_product.group0) + (geometric_product.group0.yzxx * float4(this.group3.xyz.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * geometric_product.group0.xyz) + (this.group4.zxy * geometric_product.group0.yzx) - (float3(geometric_product.e4) * this.group1.xyz) - (this.group4.yzx * geometric_product.group0.zxy), 
            /* e23, e31, e12 */
            (this.group1.yzx * geometric_product.group0.zxy) - (float3(this.e321) * geometric_product.group0.xyz) - (this.group1.zxy * geometric_product.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * geometric_product.e3) + (this.e23 * geometric_product.e4), (this.e43 * geometric_product.e1) + (this.e31 * geometric_product.e4), (this.e41 * geometric_product.e2) + (this.e12 * geometric_product.e4), this.e12 * geometric_product.e3 * -1.0) - (geometric_product.group0.xyzx * float4(this.group0.xy.yy, this.e1234, this.e23)) - (geometric_product.group0.yzxy * float4(this.group2.xyz.zxy, this.e31))
        );
    }
}
public extension MultiVector: GeometricQuotient<Scalar> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> MultiVector {
        Scalar geometric_product = Scalar.from_groups(
            /* scalar */
            1.0/other.scalar
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(geometric_product.scalar) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(geometric_product.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(geometric_product.scalar) * this.group4
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Origin> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Origin.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension Origin: GeometricQuotient<DualNum> {
    public typedef Origin GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 / (other.scalar)
        );
    }
}
public extension Origin: GeometricQuotient<Flector> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Motor {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(geometric_product.group0.xyz, geometric_product.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension Origin: GeometricQuotient<Horizon> {
    public typedef AntiScalar GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 / (other.e321) * -1.0
        );
    }
}
public extension Origin: GeometricQuotient<Line> {
    public typedef Plane GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Plane {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * float3(reverse.e23 * other_2.scalar, reverse.e31 * other_2.scalar, reverse.e12 * other_2.scalar)), 0.0)
        );
    }
}
public extension Origin: GeometricQuotient<Motor> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Flector {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * geometric_product.group1.xyz), 0.0)
        );
    }
}
public extension Origin: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, geometric_product.e321 * this.e4) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product.scalar * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * geometric_product.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * geometric_product.group3.xyz), 0.0)
        );
    }
}
public extension Origin: GeometricQuotient<Plane> {
    public typedef AntiScalar GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 / (other.e321) * -1.0
        );
    }
}
public extension Origin: GeometricQuotient<Point> {
    public typedef Line GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Line {
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * float4(other.e1 * other_2.scalar, other.e2 * other_2.scalar, other.e3 * other_2.scalar, other.e4 * other_2.scalar).xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Origin: GeometricQuotient<Scalar> {
    public typedef Origin GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 / (other.scalar)
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Plane> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Plane.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension Plane: GeometricQuotient<DualNum> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> Flector {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.scalar, -2)) * other.group0.xy
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.scalar) * this.group0
        );
    }
}
public extension Plane: GeometricQuotient<Flector> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Motor {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product.e2 * this.e412) + (geometric_product.e321 * this.e423), (geometric_product.e3 * this.e423) + (geometric_product.e321 * this.e431), (geometric_product.e1 * this.e431) + (geometric_product.e321 * this.e412), -(geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321)) - (geometric_product.group0.zxyx * this.group0.yzxx) - (this.group0.wwwy * float4(geometric_product.group1.xyz, geometric_product.e2)), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(geometric_product.group0.xyz, geometric_product.e321) * float4(-1.0)
        );
    }
}
public extension Plane: GeometricQuotient<Horizon> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> Motor {
        Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((float3(geometric_product.e321) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product.e321 * this.e321 * -1.0)
        );
    }
}
public extension Plane: GeometricQuotient<Line> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Flector {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1.xyz
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_product.group1.xyz), -(geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * geometric_product.group0.xyz), 0.0) + float4((geometric_product.group1.xyz.yzx * this.group0.zxy), 0.0) - float4((geometric_product.group1.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
public extension Plane: GeometricQuotient<Motor> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Flector {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_product.group1.xyz), (geometric_product.e1234 * this.e321) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group0.xyz) + (float3(this.e321) * geometric_product.group0.xyz) + (geometric_product.group1.yzx * this.group0.zxy) - (geometric_product.group1.zxy * this.group0.yzx)), geometric_product.scalar * this.e321)
        );
    }
}
public extension Plane: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_product.e321 * this.e321, -(geometric_product.e1 * this.e423) - (geometric_product.e2 * this.e431) - (geometric_product.e3 * this.e412) - (geometric_product.e4 * this.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_product.group3.xyz), (geometric_product.e1234 * this.e321) - (geometric_product.e23 * this.e423) - (geometric_product.e31 * this.e431) - (geometric_product.e12 * this.e412)), 
            /* e41, e42, e43 */
            (float3(geometric_product.e321) * this.group0.xyz) + (geometric_product.group1.yzx * this.group0.zxy) - (float3(this.e321) * geometric_product.group4.xyz) - (geometric_product.group1.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            float3(this.e321) * geometric_product.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product.scalar) * this.group0.xyz) + (float3(this.e321) * geometric_product.group2.xyz) + (geometric_product.group3.xyz.yzx * this.group0.zxy) - (geometric_product.group3.xyz.zxy * this.group0.yzx)), geometric_product.scalar * this.e321)
        );
    }
}
public extension Plane: GeometricQuotient<Plane> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> Motor {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(geometric_product.e321) * this.group0.xyz) - (float3(this.e321) * geometric_product.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product.e321 * this.e321 * -1.0)
        );
    }
}
public extension Plane: GeometricQuotient<Point> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Motor {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e412 * geometric_product.e2, this.e423 * geometric_product.e3, this.e431 * geometric_product.e1, -(this.e431 * geometric_product.e2) - (this.e412 * geometric_product.e3) - (this.e321 * geometric_product.e4)) - (this.group0.yzxx * geometric_product.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(geometric_product.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension Plane: GeometricQuotient<Scalar> {
    public typedef Plane GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/other.scalar) * this.group0
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Point> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<Point> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Point.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension Point: GeometricQuotient<DualNum> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> Flector {
        DualNum geometric_product = DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(other.scalar, -2)) * other.group0.xy
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_product.group0.xy.yy, geometric_product.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Point: GeometricQuotient<Flector> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Motor {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(geometric_product.e4 * this.e1) - (geometric_product.e431 * this.e3), -(geometric_product.e4 * this.e2) - (geometric_product.e412 * this.e1), -(geometric_product.e4 * this.e3) - (geometric_product.e423 * this.e2), (geometric_product.e412 * this.e3) + (geometric_product.e321 * this.e4)) + (geometric_product.group1.zxyy * this.group0.yzxy) + (this.group0.wwwx * float4(geometric_product.group0.xyz, geometric_product.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(geometric_product.e2 * this.e3) - (geometric_product.e321 * this.e1), -(geometric_product.e3 * this.e1) - (geometric_product.e321 * this.e2), -(geometric_product.e1 * this.e2) - (geometric_product.e321 * this.e3), (geometric_product.e2 * this.e2) + (geometric_product.e3 * this.e3)) + (geometric_product.group0.zxyx * this.group0.yzxx)
        );
    }
}
public extension Point: GeometricQuotient<Horizon> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> Motor {
        Horizon geometric_product = Horizon.from_groups(
            /* e321 */
            1.0/other.e321 * -1.0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_product.e321 * this.e4), 
            /* e23, e31, e12, scalar */
            float4((float3(geometric_product.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Point: GeometricQuotient<Line> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Flector {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1.xyz
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.e31 * this.e3, geometric_product.e12 * this.e1, geometric_product.e23 * this.e2, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group1.xyz.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4), (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4), (geometric_product.e41 * this.e2) + (geometric_product.e12 * this.e4), -(geometric_product.e31 * this.e2) - (geometric_product.e12 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group0.xyz.zxy, geometric_product.e23))
        );
    }
}
public extension Point: GeometricQuotient<Motor> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Flector {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product.scalar * this.e1, geometric_product.scalar * this.e2, geometric_product.scalar * this.e3, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3)) + (geometric_product.group1.yzxw * this.group0.zxyw) - (this.group0.yzxx * float4(geometric_product.group1.zxy, geometric_product.e41)), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e42 * this.e3) + (geometric_product.e1234 * this.e1) + (geometric_product.e23 * this.e4), (geometric_product.e43 * this.e1) + (geometric_product.e1234 * this.e2) + (geometric_product.e31 * this.e4), (geometric_product.e41 * this.e2) + (geometric_product.e1234 * this.e3) + (geometric_product.e12 * this.e4), -(geometric_product.e31 * this.e2) - (geometric_product.e12 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group0.zxy, geometric_product.e23))
        );
    }
}
public extension Point: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, geometric_product.e321 * this.e4) + (float2(this.e1) * float2(geometric_product.e1, geometric_product.e423)) + (float2(this.e2) * float2(geometric_product.e2, geometric_product.e431)) + (float2(this.e3) * float2(geometric_product.e3, geometric_product.e412)), 
            /* e1, e2, e3, e4 */
            float4(geometric_product.e31 * this.e3, geometric_product.e12 * this.e1, geometric_product.e23 * this.e2, -(geometric_product.e42 * this.e2) - (geometric_product.e43 * this.e3)) + (float4(geometric_product.scalar) * this.group0) - (this.group0.yzxx * float4(geometric_product.group3.xyz.zxy, geometric_product.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * geometric_product.group1.xyz) + (geometric_product.group4.zxy * this.group0.yzx) - (float3(geometric_product.e4) * this.group0.xyz) - (geometric_product.group4.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (geometric_product.group1.zxy * this.group0.yzx) - (float3(geometric_product.e321) * this.group0.xyz) - (geometric_product.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((geometric_product.e1234 * this.e1) + (geometric_product.e42 * this.e3) + (geometric_product.e23 * this.e4), (geometric_product.e1234 * this.e2) + (geometric_product.e43 * this.e1) + (geometric_product.e31 * this.e4), (geometric_product.e1234 * this.e3) + (geometric_product.e41 * this.e2) + (geometric_product.e12 * this.e4), -(geometric_product.e31 * this.e2) - (geometric_product.e12 * this.e3)) - (this.group0.yzxx * float4(geometric_product.group2.xyz.zxy, geometric_product.e23))
        );
    }
}
public extension Point: GeometricQuotient<Plane> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> Motor {
        Plane geometric_product = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(other.e321, -2)) * float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, other.e321 * -1.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product.e431 * this.e3 * -1.0, geometric_product.e412 * this.e1 * -1.0, geometric_product.e423 * this.e2 * -1.0, (geometric_product.e431 * this.e2) + (geometric_product.e412 * this.e3) + (geometric_product.e321 * this.e4)) + (geometric_product.group0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(geometric_product.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension Point: GeometricQuotient<Point> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Motor {
        Point geometric_product = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)) * other.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.e4) * geometric_product.group0.xyz) - (float3(geometric_product.e4) * this.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_product.e2 * this.e3 * -1.0, geometric_product.e3 * this.e1 * -1.0, geometric_product.e1 * this.e2 * -1.0, (geometric_product.e2 * this.e2) + (geometric_product.e3 * this.e3)) + (geometric_product.group0.zxyx * this.group0.yzxx)
        );
    }
}
public extension Point: GeometricQuotient<Scalar> {
    public typedef Point GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/other.scalar) * this.group0
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: GeometricQuotientInfix) -> GeometricQuotientInfixPartial<Scalar> {
        return GeometricQuotientInfixPartial(this);
    }
}
public extension GeometricQuotientInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Scalar.Output {
        return this.a.geometric_quotient(rhs);
    }
}
public extension Scalar: GeometricQuotient<DualNum> {
    public typedef DualNum GeometricQuotientOutput;
    public func geometric_quotient(other: DualNum) -> DualNum {
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, -2)
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * float2(other.scalar * other_2.scalar, other.e1234 * other_2.scalar)
        );
    }
}
public extension Scalar: GeometricQuotient<Flector> {
    public typedef Flector GeometricQuotientOutput;
    public func geometric_quotient(other: Flector) -> Flector {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e321, 2)
        );
        Flector geometric_product = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * geometric_product.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * geometric_product.group1
        );
    }
}
public extension Scalar: GeometricQuotient<Horizon> {
    public typedef Horizon GeometricQuotientOutput;
    public func geometric_quotient(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.scalar / (other.e321) * -1.0
        );
    }
}
public extension Scalar: GeometricQuotient<Line> {
    public typedef Line GeometricQuotientOutput;
    public func geometric_quotient(other: Line) -> Line {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2)
        );
        Line geometric_product = Line.from_groups(
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group1.xyz
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * geometric_product.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_product.group1.xyz
        );
    }
}
public extension Scalar: GeometricQuotient<Motor> {
    public typedef Motor GeometricQuotientOutput;
    public func geometric_quotient(other: Motor) -> Motor {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.scalar, 2)
        );
        Motor geometric_product = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other_2.scalar) * reverse.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * geometric_product.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * geometric_product.group1
        );
    }
}
public extension Scalar: GeometricQuotient<MultiVector> {
    public typedef MultiVector GeometricQuotientOutput;
    public func geometric_quotient(other: MultiVector) -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.scalar, 2) + pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2) + pow(other.e23, 2) + pow(other.e31, 2) + pow(other.e12, 2) + pow(other.e321, 2)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other_2.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_2.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other_2.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_2.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_2.scalar) * reverse.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * geometric_product.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * geometric_product.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * geometric_product.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * geometric_product.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * geometric_product.group4
        );
    }
}
public extension Scalar: GeometricQuotient<Plane> {
    public typedef Plane GeometricQuotientOutput;
    public func geometric_quotient(other: Plane) -> Plane {
        Plane reverse = Plane.from_groups(
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e321, -2)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * float4(reverse.e423 * other_2.scalar, reverse.e431 * other_2.scalar, reverse.e412 * other_2.scalar, reverse.e321 * other_2.scalar)
        );
    }
}
public extension Scalar: GeometricQuotient<Point> {
    public typedef Point GeometricQuotientOutput;
    public func geometric_quotient(other: Point) -> Point {
        Scalar other_2 = Scalar.from_groups(
            /* scalar */
            pow(other.e1, 2) + pow(other.e2, 2) + pow(other.e3, 2)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.e1 * other_2.scalar, other.e2 * other_2.scalar, other.e3 * other_2.scalar, other.e4 * other_2.scalar)
        );
    }
}
public extension Scalar: GeometricQuotient<Scalar> {
    public typedef Scalar GeometricQuotientOutput;
    public func geometric_quotient(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar / (other.scalar)
        );
    }
}
