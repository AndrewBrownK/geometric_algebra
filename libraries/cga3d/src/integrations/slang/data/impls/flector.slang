using traits::GeometricProduct;
using traits::RightDual;
extension Flector: Add<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + float4(float3(0.0), this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Add<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiDipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1, other.e2, other.e3, other.e4), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, other.e3215 + this.e3215)
        );
    }
}
extension Flector: Add<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiFlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<AntiFlector> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiFlector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<AntiLine> {
    associatedtype Output = DipoleInversion;
    func operator+(other: AntiLine) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(other.group0.xyz, this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz + this.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Add<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.group0.xyz, this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz + this.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, other.e3215 + this.e3215)
        );
    }
}
extension Flector: Add<AntiPlane> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiPlane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<Circle> {
    associatedtype Output = MultiVector;
    func operator+(other: Circle) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<CircleRotor> {
    associatedtype Output = MultiVector;
    func operator+(other: CircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<Dipole> {
    associatedtype Output = DipoleInversion;
    func operator+(other: Dipole) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 + float4(float3(0.0), this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Add<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func operator+(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 + float4(float3(0.0), this.e45), 
            /* e15, e25, e35, e1234 */
            other.group2 + float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group1
        );
    }
}
extension Flector: Add<DualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<FlatPoint> {
    associatedtype Output = Flector;
    func operator+(other: FlatPoint) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 + this.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator+=(const FlatPoint& other) {
        *this = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 + this.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Add<Flector> {
    associatedtype Output = Flector;
    func operator+(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 + this.group0, 
            /* e4235, e4315, e4125, e3215 */
            other.group1 + this.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator+=(const Flector& other) {
        *this = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 + this.group0, 
            /* e4235, e4315, e4125, e3215 */
            other.group1 + this.group1
        );
    }
}
extension Flector: Add<Line> {
    associatedtype Output = MultiVector;
    func operator+(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xyz, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<Motor> {
    associatedtype Output = MultiVector;
    func operator+(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xyz, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0 + other.group3, 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 + other.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: Add<Plane> {
    associatedtype Output = Flector;
    func operator+(other: Plane) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1 + other.group0
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator+=(const Plane& other) {
        *this = Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1 + other.group0
        );
    }
}
extension Flector: Add<RoundPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: RoundPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<Scalar> {
    associatedtype Output = VersorOdd;
    func operator+(other: Scalar) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Add<Sphere> {
    associatedtype Output = DipoleInversion;
    func operator+(other: Sphere) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 + other.group0
        );
    }
}
extension Flector: Add<VersorEven> {
    associatedtype Output = MultiVector;
    func operator+(other: VersorEven) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group3, 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Add<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator+(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0, 
            /* e23, e31, e12, e45 */
            float4(other.group1.xyz, this.e45 + other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group0.xyz + other.group2.xyz), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 + other.group3
        );
    }
}
extension Flector: Mul<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<AntiDualNum> {
    associatedtype Output = Flector;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator*=(const AntiDualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension Flector: Mul<AntiFlatPoint> {
    associatedtype Output = Motor;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<AntiFlector> {
    associatedtype Output = Motor;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<AntiLine> {
    associatedtype Output = Flector;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator*=(const AntiLine& other) {
        *this = this.geometric_product(other);
    }
}
extension Flector: Mul<AntiMotor> {
    associatedtype Output = Flector;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator*=(const AntiMotor& other) {
        *this = this.geometric_product(other);
    }
}
extension Flector: Mul<AntiPlane> {
    associatedtype Output = Motor;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<AntiScalar> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Circle> {
    associatedtype Output = VersorEven;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Dipole> {
    associatedtype Output = VersorOdd;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<DualNum> {
    associatedtype Output = AntiFlector;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<FlatPoint> {
    associatedtype Output = AntiMotor;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Flector> {
    associatedtype Output = AntiMotor;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Line> {
    associatedtype Output = AntiFlector;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Motor> {
    associatedtype Output = AntiFlector;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Plane> {
    associatedtype Output = AntiMotor;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<RoundPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Scalar> {
    associatedtype Output = Flector;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Flector: Mul<Sphere> {
    associatedtype Output = VersorOdd;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<VersorEven> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Neg {
    associatedtype Output = Flector;
    func neg() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
extension Flector: Not {
    associatedtype Output = AntiFlector;
    func not() -> Output {
        return this.right_dual();
    }
}
extension Flector: Sub<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, other.scalar) * float4(-1.0), 
            /* e23, e31, e12, e45 */
            float4(other.group1.xyz, this.e45 - other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group0.xyz - other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Sub<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiDipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e4) * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension Flector: Sub<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiFlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<AntiFlector> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiFlector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<AntiLine> {
    associatedtype Output = DipoleInversion;
    func operator-(other: AntiLine) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(other.group0.xyz, this.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group0.xyz - other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Sub<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(other.group0.xyz, this.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group0.xyz - other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension Flector: Sub<AntiPlane> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiPlane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(1.0, other.e12345) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<Circle> {
    associatedtype Output = MultiVector;
    func operator-(other: Circle) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<CircleRotor> {
    associatedtype Output = MultiVector;
    func operator-(other: CircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(1.0, other.e12345) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<Dipole> {
    associatedtype Output = DipoleInversion;
    func operator-(other: Dipole) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.group1.xyz, this.e45 - other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group0.xyz - other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Sub<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func operator-(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.group1.xyz, this.e45 - other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group0.xyz - other.group2.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 - other.group3
        );
    }
}
extension Flector: Sub<DualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(1.0, other.e12345) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<FlatPoint> {
    associatedtype Output = Flector;
    func operator-(other: FlatPoint) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 - other.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator-=(const FlatPoint& other) {
        *this = Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 - other.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Sub<Flector> {
    associatedtype Output = Flector;
    func operator-(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 - other.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1 - other.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator-=(const Flector& other) {
        *this = Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 - other.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1 - other.group1
        );
    }
}
extension Flector: Sub<Line> {
    associatedtype Output = MultiVector;
    func operator-(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<Motor> {
    associatedtype Output = MultiVector;
    func operator-(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(1.0, other.e12345) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0 - other.group3, 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group6 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group7.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group8.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 - other.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: Sub<Plane> {
    associatedtype Output = Flector;
    func operator-(other: Plane) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1 - other.group0
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator-=(const Plane& other) {
        *this = Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group1 - other.group0
        );
    }
}
extension Flector: Sub<RoundPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: RoundPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<Scalar> {
    associatedtype Output = VersorOdd;
    func operator-(other: Scalar) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Flector: Sub<Sphere> {
    associatedtype Output = DipoleInversion;
    func operator-(other: Sphere) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 - other.group0
        );
    }
}
extension Flector: Sub<VersorEven> {
    associatedtype Output = MultiVector;
    func operator-(other: VersorEven) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(1.0, other.e12345) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group3 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Sub<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator-(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(-1.0), 
            /* e23, e31, e12, e45 */
            float4(other.group1.xyz, this.e45 - other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group0.xyz - other.group2.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 - other.group3
        );
    }
}
