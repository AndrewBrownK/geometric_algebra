using traits;
using traits;
public extension Scalar: Add<AntiScalar> {
    public typedef DualNum AddOutput;
    public func operator+(other: AntiScalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, other.e1234)
        );
    }
}
public extension Scalar: Add<DualNum> {
    public typedef DualNum AddOutput;
    public func operator+(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0) + other.group0.xy
        );
    }
}
public extension Scalar: Add<Flector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension Scalar: Add<Horizon> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension Scalar: Add<Line> {
    public typedef Motor AddOutput;
    public func operator+(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar)
        );
    }
}
public extension Scalar: Add<Motor> {
    public typedef Motor AddOutput;
    public func operator+(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0, 
            /* e23, e31, e12, scalar */
            other.group1 + float4(float3(0.0), this.scalar)
        );
    }
}
public extension Scalar: Add<MultiVector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0) + other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Scalar: Add<Origin> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Scalar: Add<Plane> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension Scalar: Add<Point> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Scalar: Add<Scalar> {
    public typedef Scalar AddOutput;
    public func operator+(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar + this.scalar
        );
    }
}
public extension Scalar {
    public func operator+=(const Scalar& other) {
        *this = Scalar.from_groups(
            /* scalar */
            other.scalar + this.scalar
        );
    }
}
public extension Scalar: Mul<AntiScalar> {
    public typedef AntiScalar MulOutput;
    public func operator*(other: AntiScalar) -> AntiScalar {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<DualNum> {
    public typedef DualNum MulOutput;
    public func operator*(other: DualNum) -> DualNum {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<Flector> {
    public typedef Flector MulOutput;
    public func operator*(other: Flector) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<Horizon> {
    public typedef Horizon MulOutput;
    public func operator*(other: Horizon) -> Horizon {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<Line> {
    public typedef Line MulOutput;
    public func operator*(other: Line) -> Line {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<Motor> {
    public typedef Motor MulOutput;
    public func operator*(other: Motor) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<MultiVector> {
    public typedef MultiVector MulOutput;
    public func operator*(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<Origin> {
    public typedef Origin MulOutput;
    public func operator*(other: Origin) -> Origin {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<Plane> {
    public typedef Plane MulOutput;
    public func operator*(other: Plane) -> Plane {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<Point> {
    public typedef Point MulOutput;
    public func operator*(other: Point) -> Point {
        return this.geometric_product(other);
    }
}
public extension Scalar: Mul<Scalar> {
    public typedef Scalar MulOutput;
    public func operator*(other: Scalar) -> Scalar {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Scalar: Neg {
    public typedef Scalar NegOutput;
    public func neg() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * -1.0
        );
    }
}
public extension Scalar: Not {
    public typedef AntiScalar NotOutput;
    public func not() -> AntiScalar {
        return this.right_dual();
    }
}
public extension Scalar: Sub<AntiScalar> {
    public typedef DualNum SubOutput;
    public func operator-(other: AntiScalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, other.e1234) * float2(1.0, -1.0)
        );
    }
}
public extension Scalar: Sub<DualNum> {
    public typedef DualNum SubOutput;
    public func operator-(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar - other.scalar, other.e1234) * float2(1.0, -1.0)
        );
    }
}
public extension Scalar: Sub<Flector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Scalar: Sub<Horizon> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Scalar: Sub<Line> {
    public typedef Motor SubOutput;
    public func operator-(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension Scalar: Sub<Motor> {
    public typedef Motor SubOutput;
    public func operator-(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension Scalar: Sub<MultiVector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar - other.scalar, other.e1234) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension Scalar: Sub<Origin> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Scalar: Sub<Plane> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Scalar: Sub<Point> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Scalar: Sub<Scalar> {
    public typedef Scalar SubOutput;
    public func operator-(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar - other.scalar
        );
    }
}
public extension Scalar {
    public func operator-=(const Scalar& other) {
        *this = Scalar.from_groups(
            /* scalar */
            this.scalar - other.scalar
        );
    }
}
