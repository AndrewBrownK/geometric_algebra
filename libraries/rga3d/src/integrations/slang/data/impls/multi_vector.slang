using traits::GeometricProduct;
using traits::RightDual;
extension MultiVector: Add<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiScalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Add<DualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const DualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Add<Flector> {
    associatedtype Output = MultiVector;
    func operator+(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 + this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group1 + this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Flector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 + this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group1 + this.group4
        );
    }
}
extension MultiVector: Add<Horizon> {
    associatedtype Output = MultiVector;
    func operator+(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Horizon& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321)
        );
    }
}
extension MultiVector: Add<Line> {
    associatedtype Output = MultiVector;
    func operator+(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz + this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Line& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz + this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Add<Motor> {
    associatedtype Output = MultiVector;
    func operator+(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, other.e1234) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz + other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz + other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Motor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, other.e1234) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz + other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz + other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz + this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz + this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 + this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const MultiVector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz + this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz + this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 + this.group4
        );
    }
}
extension MultiVector: Add<Origin> {
    associatedtype Output = MultiVector;
    func operator+(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4), 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Origin& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4), 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Add<Plane> {
    associatedtype Output = MultiVector;
    func operator+(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 + other.group0
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Plane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 + other.group0
        );
    }
}
extension MultiVector: Add<Point> {
    associatedtype Output = MultiVector;
    func operator+(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Point& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Add<Scalar> {
    associatedtype Output = MultiVector;
    func operator+(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Scalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Mul<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<DualNum> {
    associatedtype Output = MultiVector;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Flector> {
    associatedtype Output = MultiVector;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Flector& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Horizon> {
    associatedtype Output = MultiVector;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Horizon& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Line> {
    associatedtype Output = MultiVector;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Line& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Motor> {
    associatedtype Output = MultiVector;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Motor& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const MultiVector& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Origin> {
    associatedtype Output = MultiVector;
    func operator*(other: Origin) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Origin& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Plane> {
    associatedtype Output = MultiVector;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Plane& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Point> {
    associatedtype Output = MultiVector;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Point& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Scalar> {
    associatedtype Output = MultiVector;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Neg {
    associatedtype Output = MultiVector;
    func neg() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
extension MultiVector: Not {
    associatedtype Output = MultiVector;
    func not() -> Output {
        return this.right_dual();
    }
}
extension MultiVector: Sub<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiScalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Sub<DualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const DualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Sub<Flector> {
    associatedtype Output = MultiVector;
    func operator-(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 - other.group1
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Flector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 - other.group1
        );
    }
}
extension MultiVector: Sub<Horizon> {
    associatedtype Output = MultiVector;
    func operator-(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Horizon& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension MultiVector: Sub<Line> {
    associatedtype Output = MultiVector;
    func operator-(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz - other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Line& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz - other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Sub<Motor> {
    associatedtype Output = MultiVector;
    func operator-(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * -1.0, other.e1234 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz - other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Motor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * -1.0, other.e1234 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz - other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz - other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz - other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 - other.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const MultiVector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz - other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz - other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 - other.group4
        );
    }
}
extension MultiVector: Sub<Origin> {
    associatedtype Output = MultiVector;
    func operator-(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Origin& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Sub<Plane> {
    associatedtype Output = MultiVector;
    func operator-(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 - other.group0
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Plane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 - other.group0
        );
    }
}
extension MultiVector: Sub<Point> {
    associatedtype Output = MultiVector;
    func operator-(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Point& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector: Sub<Scalar> {
    associatedtype Output = MultiVector;
    func operator-(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Scalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
