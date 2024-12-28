using traits;
using traits;
public extension Line: Add<AntiScalar> {
    public typedef Motor AddOutput;
    public func operator+(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Line: Add<DualNum> {
    public typedef Motor AddOutput;
    public func operator+(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar)
        );
    }
}
public extension Line: Add<Flector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension Line: Add<Horizon> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension Line: Add<Line> {
    public typedef Line AddOutput;
    public func operator+(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz + this.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz + this.group1.xyz
        );
    }
}
public extension Line {
    public func operator+=(const Line& other) {
        *this = Line.from_groups(
            /* e41, e42, e43 */
            other.group0.xyz + this.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz + this.group1.xyz
        );
    }
}
public extension Line: Add<Motor> {
    public typedef Motor AddOutput;
    public func operator+(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.group0.xyz + other.group0.xyz), other.e1234), 
            /* e23, e31, e12, scalar */
            float4((this.group1.xyz + other.group1.xyz), other.scalar)
        );
    }
}
public extension Line: Add<MultiVector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            this.group0.xyz + other.group2.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz + other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Line: Add<Origin> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line: Add<Plane> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension Line: Add<Point> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line: Add<Scalar> {
    public typedef Motor AddOutput;
    public func operator+(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar)
        );
    }
}
public extension Line: Mul<AntiScalar> {
    public typedef Line MulOutput;
    public func operator*(other: AntiScalar) -> Line {
        return this.geometric_product(other);
    }
}
public extension Line {
    public func operator*=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Line: Mul<DualNum> {
    public typedef Line MulOutput;
    public func operator*(other: DualNum) -> Line {
        return this.geometric_product(other);
    }
}
public extension Line {
    public func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
public extension Line: Mul<Flector> {
    public typedef Flector MulOutput;
    public func operator*(other: Flector) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Line: Mul<Horizon> {
    public typedef Flector MulOutput;
    public func operator*(other: Horizon) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Line: Mul<Line> {
    public typedef Motor MulOutput;
    public func operator*(other: Line) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Line: Mul<Motor> {
    public typedef Motor MulOutput;
    public func operator*(other: Motor) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Line: Mul<MultiVector> {
    public typedef MultiVector MulOutput;
    public func operator*(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension Line: Mul<Origin> {
    public typedef Plane MulOutput;
    public func operator*(other: Origin) -> Plane {
        return this.geometric_product(other);
    }
}
public extension Line: Mul<Plane> {
    public typedef Flector MulOutput;
    public func operator*(other: Plane) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Line: Mul<Point> {
    public typedef Flector MulOutput;
    public func operator*(other: Point) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Line: Mul<Scalar> {
    public typedef Line MulOutput;
    public func operator*(other: Scalar) -> Line {
        return this.geometric_product(other);
    }
}
public extension Line {
    public func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Line: Neg {
    public typedef Line NegOutput;
    public func neg() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
public extension Line: Not {
    public typedef Line NotOutput;
    public func not() -> Line {
        return this.right_dual();
    }
}
public extension Line: Sub<AntiScalar> {
    public typedef Motor SubOutput;
    public func operator-(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Line: Sub<DualNum> {
    public typedef Motor SubOutput;
    public func operator-(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar * -1.0)
        );
    }
}
public extension Line: Sub<Flector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Line: Sub<Horizon> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Line: Sub<Line> {
    public typedef Line SubOutput;
    public func operator-(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz - other.group1.xyz
        );
    }
}
public extension Line {
    public func operator-=(const Line& other) {
        *this = Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz - other.group1.xyz
        );
    }
}
public extension Line: Sub<Motor> {
    public typedef Motor SubOutput;
    public func operator-(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.group0.xyz - other.group0.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, scalar */
            float4((this.group1.xyz - other.group1.xyz), other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension Line: Sub<MultiVector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0.xyz - other.group2.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz - other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension Line: Sub<Origin> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line: Sub<Plane> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Line: Sub<Point> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line: Sub<Scalar> {
    public typedef Motor SubOutput;
    public func operator-(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar * -1.0)
        );
    }
}
