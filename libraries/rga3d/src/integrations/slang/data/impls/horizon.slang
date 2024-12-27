using traits::GeometricProduct;
using traits::RightDual;
extension Horizon: Add<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Add<DualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Add<Flector> {
    associatedtype Output = Flector;
    func operator+(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 + float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Add<Horizon> {
    associatedtype Output = Horizon;
    func operator+(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(other.e321 + this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon {
    associatedtype Output = Horizon;
    func operator+=(const Horizon& other) {
        *this = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(other.e321 + this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: Add<Line> {
    associatedtype Output = MultiVector;
    func operator+(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Add<Motor> {
    associatedtype Output = MultiVector;
    func operator+(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, other.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 + float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Add<Origin> {
    associatedtype Output = Flector;
    func operator+(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Add<Plane> {
    associatedtype Output = Plane;
    func operator+(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            other.group0 + float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Add<Point> {
    associatedtype Output = Flector;
    func operator+(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Add<Scalar> {
    associatedtype Output = MultiVector;
    func operator+(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Mul<AntiScalar> {
    associatedtype Output = Origin;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<DualNum> {
    associatedtype Output = Flector;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<Flector> {
    associatedtype Output = Motor;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<Horizon> {
    associatedtype Output = Scalar;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<Line> {
    associatedtype Output = Flector;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<Motor> {
    associatedtype Output = Flector;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<Origin> {
    associatedtype Output = AntiScalar;
    func operator*(other: Origin) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<Plane> {
    associatedtype Output = Motor;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<Point> {
    associatedtype Output = Motor;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon: Mul<Scalar> {
    associatedtype Output = Horizon;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Horizon {
    associatedtype Output = Horizon;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Horizon: Neg {
    associatedtype Output = Horizon;
    func neg() -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: Not {
    associatedtype Output = Origin;
    func not() -> Output {
        return this.right_dual();
    }
}
extension Horizon: Sub<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, other.e1234) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Sub<DualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Sub<Flector> {
    associatedtype Output = Flector;
    func operator-(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            float4(other.group1.xyz, this.e321 - other.e321) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Horizon: Sub<Horizon> {
    associatedtype Output = Horizon;
    func operator-(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 - other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon {
    associatedtype Output = Horizon;
    func operator-=(const Horizon& other) {
        *this = Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 - other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: Sub<Line> {
    associatedtype Output = MultiVector;
    func operator-(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Sub<Motor> {
    associatedtype Output = MultiVector;
    func operator-(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, other.e1234) * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.group4.xyz, this.e321 - other.e321) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Horizon: Sub<Origin> {
    associatedtype Output = Flector;
    func operator-(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Sub<Plane> {
    associatedtype Output = Plane;
    func operator-(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.group0.xyz, this.e321 - other.e321) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Horizon: Sub<Point> {
    associatedtype Output = Flector;
    func operator-(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon: Sub<Scalar> {
    associatedtype Output = MultiVector;
    func operator-(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
