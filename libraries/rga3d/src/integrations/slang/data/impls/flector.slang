using traits;
using traits;
public extension Flector: Add<AntiScalar> {
    public typedef MultiVector AddOutput;
    public func operator+(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Add<DualNum> {
    public typedef MultiVector AddOutput;
    public func operator+(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Add<Flector> {
    public typedef Flector AddOutput;
    public func operator+(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group1
        );
    }
}
public extension Flector {
    public func operator+=(const Flector& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group1
        );
    }
}
public extension Flector: Add<Horizon> {
    public typedef Flector AddOutput;
    public func operator+(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + float4(float3(0.0), other.e321)
        );
    }
}
public extension Flector {
    public func operator+=(const Horizon& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + float4(float3(0.0), other.e321)
        );
    }
}
public extension Flector: Add<Line> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Add<Motor> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Add<MultiVector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group0 + other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group1 + other.group4
        );
    }
}
public extension Flector: Add<Origin> {
    public typedef Flector AddOutput;
    public func operator+(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public func operator+=(const Origin& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Add<Plane> {
    public typedef Flector AddOutput;
    public func operator+(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + other.group0
        );
    }
}
public extension Flector {
    public func operator+=(const Plane& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + other.group0
        );
    }
}
public extension Flector: Add<Point> {
    public typedef Flector AddOutput;
    public func operator+(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public func operator+=(const Point& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Add<Scalar> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Mul<AntiScalar> {
    public typedef Flector MulOutput;
    public func operator*(other: AntiScalar) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public func operator*=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Flector: Mul<DualNum> {
    public typedef Flector MulOutput;
    public func operator*(other: DualNum) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
public extension Flector: Mul<Flector> {
    public typedef Motor MulOutput;
    public func operator*(other: Flector) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Flector: Mul<Horizon> {
    public typedef Motor MulOutput;
    public func operator*(other: Horizon) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Flector: Mul<Line> {
    public typedef Flector MulOutput;
    public func operator*(other: Line) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public func operator*=(const Line& other) {
        *this = this.geometric_product(other);
    }
}
public extension Flector: Mul<Motor> {
    public typedef Flector MulOutput;
    public func operator*(other: Motor) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public func operator*=(const Motor& other) {
        *this = this.geometric_product(other);
    }
}
public extension Flector: Mul<MultiVector> {
    public typedef MultiVector MulOutput;
    public func operator*(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension Flector: Mul<Origin> {
    public typedef Motor MulOutput;
    public func operator*(other: Origin) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Flector: Mul<Plane> {
    public typedef Motor MulOutput;
    public func operator*(other: Plane) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Flector: Mul<Point> {
    public typedef Motor MulOutput;
    public func operator*(other: Point) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Flector: Mul<Scalar> {
    public typedef Flector MulOutput;
    public func operator*(other: Scalar) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Flector: Neg {
    public typedef Flector NegOutput;
    public func neg() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Flector: Not {
    public typedef Flector NotOutput;
    public func not() -> Flector {
        return this.right_dual();
    }
}
public extension Flector: Sub<AntiScalar> {
    public typedef MultiVector SubOutput;
    public func operator-(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Sub<DualNum> {
    public typedef MultiVector SubOutput;
    public func operator-(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Sub<Flector> {
    public typedef Flector SubOutput;
    public func operator-(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group1
        );
    }
}
public extension Flector {
    public func operator-=(const Flector& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group1
        );
    }
}
public extension Flector: Sub<Horizon> {
    public typedef Flector SubOutput;
    public func operator-(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Flector {
    public func operator-=(const Horizon& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Flector: Sub<Line> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Sub<Motor> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Sub<MultiVector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0 - other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 - other.group4
        );
    }
}
public extension Flector: Sub<Origin> {
    public typedef Flector SubOutput;
    public func operator-(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public func operator-=(const Origin& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Sub<Plane> {
    public typedef Flector SubOutput;
    public func operator-(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group0
        );
    }
}
public extension Flector {
    public func operator-=(const Plane& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group0
        );
    }
}
public extension Flector: Sub<Point> {
    public typedef Flector SubOutput;
    public func operator-(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public func operator-=(const Point& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector: Sub<Scalar> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
