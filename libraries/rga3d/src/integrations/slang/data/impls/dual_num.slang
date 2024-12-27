using traits::GeometricProduct;
using traits::RightDual;
extension DualNum: Add<AntiScalar> {
    associatedtype Output = DualNum;
    func operator+(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum {
    associatedtype Output = DualNum;
    func operator+=(const AntiScalar& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: Add<DualNum> {
    associatedtype Output = DualNum;
    func operator+(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum {
    associatedtype Output = DualNum;
    func operator+=(const DualNum& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: Add<Flector> {
    associatedtype Output = MultiVector;
    func operator+(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
extension DualNum: Add<Horizon> {
    associatedtype Output = MultiVector;
    func operator+(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
extension DualNum: Add<Line> {
    associatedtype Output = Motor;
    func operator+(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar)
        );
    }
}
extension DualNum: Add<Motor> {
    associatedtype Output = Motor;
    func operator+(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234 + other.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar + other.scalar)
        );
    }
}
extension DualNum: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy + other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
extension DualNum: Add<Origin> {
    associatedtype Output = MultiVector;
    func operator+(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension DualNum: Add<Plane> {
    associatedtype Output = MultiVector;
    func operator+(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
extension DualNum: Add<Point> {
    associatedtype Output = MultiVector;
    func operator+(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension DualNum: Add<Scalar> {
    associatedtype Output = DualNum;
    func operator+(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum {
    associatedtype Output = DualNum;
    func operator+=(const Scalar& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: Mul<AntiScalar> {
    associatedtype Output = AntiScalar;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum: Mul<DualNum> {
    associatedtype Output = DualNum;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum {
    associatedtype Output = DualNum;
    func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension DualNum: Mul<Flector> {
    associatedtype Output = Flector;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum: Mul<Horizon> {
    associatedtype Output = Flector;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum: Mul<Line> {
    associatedtype Output = Line;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum: Mul<Motor> {
    associatedtype Output = Motor;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum: Mul<Origin> {
    associatedtype Output = Origin;
    func operator*(other: Origin) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum: Mul<Plane> {
    associatedtype Output = Flector;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum: Mul<Point> {
    associatedtype Output = Flector;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum: Mul<Scalar> {
    associatedtype Output = DualNum;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension DualNum {
    associatedtype Output = DualNum;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension DualNum: Neg {
    associatedtype Output = DualNum;
    func neg() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0)
        );
    }
}
extension DualNum: Not {
    associatedtype Output = AntiScalar;
    func not() -> Output {
        return this.right_dual();
    }
}
extension DualNum: Sub<AntiScalar> {
    associatedtype Output = DualNum;
    func operator-(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234 * -1.0) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum {
    associatedtype Output = DualNum;
    func operator-=(const AntiScalar& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234 * -1.0) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: Sub<DualNum> {
    associatedtype Output = DualNum;
    func operator-(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum {
    associatedtype Output = DualNum;
    func operator-=(const DualNum& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: Sub<Flector> {
    associatedtype Output = MultiVector;
    func operator-(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
extension DualNum: Sub<Horizon> {
    associatedtype Output = MultiVector;
    func operator-(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension DualNum: Sub<Line> {
    associatedtype Output = Motor;
    func operator-(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension DualNum: Sub<Motor> {
    associatedtype Output = Motor;
    func operator-(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234 - other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension DualNum: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
extension DualNum: Sub<Origin> {
    associatedtype Output = MultiVector;
    func operator-(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension DualNum: Sub<Plane> {
    associatedtype Output = MultiVector;
    func operator-(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
extension DualNum: Sub<Point> {
    associatedtype Output = MultiVector;
    func operator-(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension DualNum: Sub<Scalar> {
    associatedtype Output = DualNum;
    func operator-(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum {
    associatedtype Output = DualNum;
    func operator-=(const Scalar& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0)
        );
    }
}
