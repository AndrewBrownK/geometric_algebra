using traits;
using traits;
public extension Motor: Add<AntiScalar> {
    public typedef Motor AddOutput;
    public func operator+(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
public extension Motor {
    public func operator+=(const AntiScalar& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
public extension Motor: Add<DualNum> {
    public typedef Motor AddOutput;
    public func operator+(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 + this.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar + this.scalar)
        );
    }
}
public extension Motor {
    public func operator+=(const DualNum& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 + this.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar + this.scalar)
        );
    }
}
public extension Motor: Add<Flector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Add<Horizon> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Add<Line> {
    public typedef Motor AddOutput;
    public func operator+(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz + this.group0.xyz), this.e1234), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz + this.group1.xyz), this.scalar)
        );
    }
}
public extension Motor {
    public func operator+=(const Line& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz + this.group0.xyz), this.e1234), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz + this.group1.xyz), this.scalar)
        );
    }
}
public extension Motor: Add<Motor> {
    public typedef Motor AddOutput;
    public func operator+(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 + this.group0, 
            /* e23, e31, e12, scalar */
            other.group1 + this.group1
        );
    }
}
public extension Motor {
    public func operator+=(const Motor& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 + this.group0, 
            /* e23, e31, e12, scalar */
            other.group1 + this.group1
        );
    }
}
public extension Motor: Add<MultiVector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234) + other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz + this.group0.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz + this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Motor: Add<Origin> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Add<Plane> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Add<Point> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Add<Scalar> {
    public typedef Motor AddOutput;
    public func operator+(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar)
        );
    }
}
public extension Motor {
    public func operator+=(const Scalar& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar)
        );
    }
}
public extension Motor: Mul<AntiScalar> {
    public typedef Motor MulOutput;
    public func operator*(other: AntiScalar) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public func operator*=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Motor: Mul<DualNum> {
    public typedef Motor MulOutput;
    public func operator*(other: DualNum) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
public extension Motor: Mul<Flector> {
    public typedef Flector MulOutput;
    public func operator*(other: Flector) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Motor: Mul<Horizon> {
    public typedef Flector MulOutput;
    public func operator*(other: Horizon) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Motor: Mul<Line> {
    public typedef Motor MulOutput;
    public func operator*(other: Line) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public func operator*=(const Line& other) {
        *this = this.geometric_product(other);
    }
}
public extension Motor: Mul<Motor> {
    public typedef Motor MulOutput;
    public func operator*(other: Motor) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public func operator*=(const Motor& other) {
        *this = this.geometric_product(other);
    }
}
public extension Motor: Mul<MultiVector> {
    public typedef MultiVector MulOutput;
    public func operator*(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension Motor: Mul<Origin> {
    public typedef Flector MulOutput;
    public func operator*(other: Origin) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Motor: Mul<Plane> {
    public typedef Flector MulOutput;
    public func operator*(other: Plane) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Motor: Mul<Point> {
    public typedef Flector MulOutput;
    public func operator*(other: Point) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Motor: Mul<Scalar> {
    public typedef Motor MulOutput;
    public func operator*(other: Scalar) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Motor: Neg {
    public typedef Motor NegOutput;
    public func neg() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Motor: Not {
    public typedef Motor NotOutput;
    public func not() -> Motor {
        return this.right_dual();
    }
}
public extension Motor: Sub<AntiScalar> {
    public typedef Motor SubOutput;
    public func operator-(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
public extension Motor {
    public func operator-=(const AntiScalar& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
public extension Motor: Sub<DualNum> {
    public typedef Motor SubOutput;
    public func operator-(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
public extension Motor {
    public func operator-=(const DualNum& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
public extension Motor: Sub<Flector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Sub<Horizon> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Sub<Line> {
    public typedef Motor SubOutput;
    public func operator-(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1
        );
    }
}
public extension Motor {
    public func operator-=(const Line& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1
        );
    }
}
public extension Motor: Sub<Motor> {
    public typedef Motor SubOutput;
    public func operator-(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 - other.group0, 
            /* e23, e31, e12, scalar */
            this.group1 - other.group1
        );
    }
}
public extension Motor {
    public func operator-=(const Motor& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 - other.group0, 
            /* e23, e31, e12, scalar */
            this.group1 - other.group1
        );
    }
}
public extension Motor: Sub<MultiVector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234) - other.group0.xy, 
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
public extension Motor: Sub<Origin> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Sub<Plane> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Sub<Point> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
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
public extension Motor: Sub<Scalar> {
    public typedef Motor SubOutput;
    public func operator-(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
public extension Motor {
    public func operator-=(const Scalar& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
