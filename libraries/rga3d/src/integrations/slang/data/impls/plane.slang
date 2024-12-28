using traits;
using traits;
public extension Plane: Add<AntiScalar> {
    public typedef MultiVector AddOutput;
    public func operator+(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Add<DualNum> {
    public typedef MultiVector AddOutput;
    public func operator+(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Add<Flector> {
    public typedef Flector AddOutput;
    public func operator+(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group0
        );
    }
}
public extension Plane: Add<Horizon> {
    public typedef Plane AddOutput;
    public func operator+(other: Horizon) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 + float4(float3(0.0), other.e321)
        );
    }
}
public extension Plane {
    public func operator+=(const Horizon& other) {
        *this = Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 + float4(float3(0.0), other.e321)
        );
    }
}
public extension Plane: Add<Line> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Add<Motor> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Add<MultiVector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 + this.group0
        );
    }
}
public extension Plane: Add<Origin> {
    public typedef Flector AddOutput;
    public func operator+(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Add<Plane> {
    public typedef Plane AddOutput;
    public func operator+(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            other.group0 + this.group0
        );
    }
}
public extension Plane {
    public func operator+=(const Plane& other) {
        *this = Plane.from_groups(
            /* e423, e431, e412, e321 */
            other.group0 + this.group0
        );
    }
}
public extension Plane: Add<Point> {
    public typedef Flector AddOutput;
    public func operator+(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Add<Scalar> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Mul<AntiScalar> {
    public typedef Origin MulOutput;
    public func operator*(other: AntiScalar) -> Origin {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<DualNum> {
    public typedef Flector MulOutput;
    public func operator*(other: DualNum) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<Flector> {
    public typedef Motor MulOutput;
    public func operator*(other: Flector) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<Horizon> {
    public typedef Motor MulOutput;
    public func operator*(other: Horizon) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<Line> {
    public typedef Flector MulOutput;
    public func operator*(other: Line) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<Motor> {
    public typedef Flector MulOutput;
    public func operator*(other: Motor) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<MultiVector> {
    public typedef MultiVector MulOutput;
    public func operator*(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<Origin> {
    public typedef AntiScalar MulOutput;
    public func operator*(other: Origin) -> AntiScalar {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<Plane> {
    public typedef Motor MulOutput;
    public func operator*(other: Plane) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<Point> {
    public typedef Motor MulOutput;
    public func operator*(other: Point) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Plane: Mul<Scalar> {
    public typedef Plane MulOutput;
    public func operator*(other: Scalar) -> Plane {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Plane: Neg {
    public typedef Plane NegOutput;
    public func neg() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Plane: Not {
    public typedef Origin NotOutput;
    public func not() -> Origin {
        return this.right_dual();
    }
}
public extension Plane: Sub<AntiScalar> {
    public typedef MultiVector SubOutput;
    public func operator-(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Sub<DualNum> {
    public typedef MultiVector SubOutput;
    public func operator-(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Sub<Flector> {
    public typedef Flector SubOutput;
    public func operator-(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0 - other.group1
        );
    }
}
public extension Plane: Sub<Horizon> {
    public typedef Plane SubOutput;
    public func operator-(other: Horizon) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Plane {
    public func operator-=(const Horizon& other) {
        *this = Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Plane: Sub<Line> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Sub<Motor> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Sub<MultiVector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0 - other.group4
        );
    }
}
public extension Plane: Sub<Origin> {
    public typedef Flector SubOutput;
    public func operator-(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Sub<Plane> {
    public typedef Plane SubOutput;
    public func operator-(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 - other.group0
        );
    }
}
public extension Plane {
    public func operator-=(const Plane& other) {
        *this = Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 - other.group0
        );
    }
}
public extension Plane: Sub<Point> {
    public typedef Flector SubOutput;
    public func operator-(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane: Sub<Scalar> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
