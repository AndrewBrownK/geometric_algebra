using traits;
using traits;
public extension MultiVector: Add<AntiScalar> {
    public typedef MultiVector AddOutput;
    public func operator+(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator+=(const AntiScalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Add<DualNum> {
    public typedef MultiVector AddOutput;
    public func operator+(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator+=(const DualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Add<Flector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 + this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group4
        );
    }
}
public extension MultiVector {
    public func operator+=(const Flector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 + this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group4
        );
    }
}
public extension MultiVector: Add<Horizon> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321)
        );
    }
}
public extension MultiVector {
    public func operator+=(const Horizon& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321)
        );
    }
}
public extension MultiVector: Add<Line> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            other.group0.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator+=(const Line& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            other.group0.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Add<Motor> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz + other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz + other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator+=(const Motor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz + other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz + other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Add<MultiVector> {
    public typedef MultiVector AddOutput;
    public func operator+(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e41, e42, e43 */
            other.group2.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 + this.group4
        );
    }
}
public extension MultiVector {
    public func operator+=(const MultiVector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e41, e42, e43 */
            other.group2.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 + this.group4
        );
    }
}
public extension MultiVector: Add<Origin> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator+=(const Origin& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Add<Plane> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + other.group0
        );
    }
}
public extension MultiVector {
    public func operator+=(const Plane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + other.group0
        );
    }
}
public extension MultiVector: Add<Point> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator+=(const Point& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Add<Scalar> {
    public typedef MultiVector AddOutput;
    public func operator+(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator+=(const Scalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Mul<AntiScalar> {
    public typedef MultiVector MulOutput;
    public func operator*(other: AntiScalar) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<DualNum> {
    public typedef MultiVector MulOutput;
    public func operator*(other: DualNum) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<Flector> {
    public typedef MultiVector MulOutput;
    public func operator*(other: Flector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const Flector& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<Horizon> {
    public typedef MultiVector MulOutput;
    public func operator*(other: Horizon) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const Horizon& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<Line> {
    public typedef MultiVector MulOutput;
    public func operator*(other: Line) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const Line& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<Motor> {
    public typedef MultiVector MulOutput;
    public func operator*(other: Motor) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const Motor& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<MultiVector> {
    public typedef MultiVector MulOutput;
    public func operator*(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const MultiVector& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<Origin> {
    public typedef MultiVector MulOutput;
    public func operator*(other: Origin) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const Origin& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<Plane> {
    public typedef MultiVector MulOutput;
    public func operator*(other: Plane) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const Plane& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<Point> {
    public typedef MultiVector MulOutput;
    public func operator*(other: Point) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const Point& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Mul<Scalar> {
    public typedef MultiVector MulOutput;
    public func operator*(other: Scalar) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector: Neg {
    public typedef MultiVector NegOutput;
    public func neg() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
public extension MultiVector: Not {
    public typedef MultiVector NotOutput;
    public func not() -> MultiVector {
        return this.right_dual();
    }
}
public extension MultiVector: Sub<AntiScalar> {
    public typedef MultiVector SubOutput;
    public func operator-(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator-=(const AntiScalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Sub<DualNum> {
    public typedef MultiVector SubOutput;
    public func operator-(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator-=(const DualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Sub<Flector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group1
        );
    }
}
public extension MultiVector {
    public func operator-=(const Flector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group1
        );
    }
}
public extension MultiVector: Sub<Horizon> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension MultiVector {
    public func operator-=(const Horizon& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension MultiVector: Sub<Line> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator-=(const Line& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Sub<Motor> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator-=(const Motor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Sub<MultiVector> {
    public typedef MultiVector SubOutput;
    public func operator-(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group4
        );
    }
}
public extension MultiVector {
    public func operator-=(const MultiVector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group4
        );
    }
}
public extension MultiVector: Sub<Origin> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator-=(const Origin& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Sub<Plane> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group0
        );
    }
}
public extension MultiVector {
    public func operator-=(const Plane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group0
        );
    }
}
public extension MultiVector: Sub<Point> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator-=(const Point& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector: Sub<Scalar> {
    public typedef MultiVector SubOutput;
    public func operator-(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator-=(const Scalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
