using traits::GeometricProduct;
using traits::RightDual;
extension AntiLine: Add<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func operator+(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz + other.group1.xyz), other.e45), 
            /* e15, e25, e35, scalar */
            float4((this.group1.xyz + other.group2.xyz), other.scalar)
        );
    }
}
extension AntiLine: Add<AntiDipoleInversion> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func operator+(other: AntiDualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, other.e3215)
        );
    }
}
extension AntiLine: Add<AntiFlatPoint> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<AntiFlector> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<AntiLine> {
    associatedtype Output = AntiLine;
    func operator+(other: AntiLine) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz + this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine {
    associatedtype Output = AntiLine;
    func operator+=(const AntiLine& other) {
        *this = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz + this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: Add<AntiMotor> {
    associatedtype Output = AntiMotor;
    func operator+(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((this.group0.xyz + other.group0.xyz), other.scalar), 
            /* e15, e25, e35, e3215 */
            float4((this.group1.xyz + other.group1.xyz), other.e3215)
        );
    }
}
extension AntiLine: Add<AntiPlane> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<AntiScalar> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<Circle> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<CircleRotor> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<Dipole> {
    associatedtype Output = Dipole;
    func operator+(other: Dipole) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz + other.group1.xyz), other.e45), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz + other.group2.xyz, 0.0)
        );
    }
}
extension AntiLine: Add<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func operator+(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz + other.group1.xyz), other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group2.xyz), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group3
        );
    }
}
extension AntiLine: Add<DualNum> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<FlatPoint> {
    associatedtype Output = Dipole;
    func operator+(other: FlatPoint) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz + other.group0.xyz, 0.0)
        );
    }
}
extension AntiLine: Add<Flector> {
    associatedtype Output = DipoleInversion;
    func operator+(other: Flector) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1
        );
    }
}
extension AntiLine: Add<Line> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xyz, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<Motor> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xyz, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<MultiVector> {
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
            float4((this.group1.xyz + other.group3.xyz), other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz + other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: Add<Plane> {
    associatedtype Output = DipoleInversion;
    func operator+(other: Plane) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0
        );
    }
}
extension AntiLine: Add<RoundPoint> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<Scalar> {
    associatedtype Output = AntiMotor;
    func operator+(other: Scalar) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: Add<Sphere> {
    associatedtype Output = DipoleInversion;
    func operator+(other: Sphere) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group0
        );
    }
}
extension AntiLine: Add<VersorEven> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Add<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator+(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0, 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz + other.group1.xyz), other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group2.xyz), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group3
        );
    }
}
extension AntiLine: Mul<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<AntiDualNum> {
    associatedtype Output = AntiLine;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine {
    associatedtype Output = AntiLine;
    func operator*=(const AntiDualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiLine: Mul<AntiFlatPoint> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<AntiFlector> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<AntiLine> {
    associatedtype Output = AntiMotor;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<AntiMotor> {
    associatedtype Output = AntiMotor;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<AntiPlane> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<AntiScalar> {
    associatedtype Output = Line;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<Circle> {
    associatedtype Output = VersorEven;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<Dipole> {
    associatedtype Output = VersorOdd;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<DualNum> {
    associatedtype Output = Line;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<FlatPoint> {
    associatedtype Output = Flector;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<Flector> {
    associatedtype Output = Flector;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<Line> {
    associatedtype Output = Motor;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<Motor> {
    associatedtype Output = Motor;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<Plane> {
    associatedtype Output = Flector;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<Scalar> {
    associatedtype Output = AntiLine;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine {
    associatedtype Output = AntiLine;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiLine: Mul<Sphere> {
    associatedtype Output = DipoleInversion;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<VersorEven> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Mul<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiLine: Neg {
    associatedtype Output = AntiLine;
    func neg() -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
    }
}
extension AntiLine: Not {
    associatedtype Output = Line;
    func not() -> Output {
        return this.right_dual();
    }
}
extension AntiLine: Sub<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func operator-(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4((this.group1.xyz - other.group2.xyz), other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension AntiLine: Sub<AntiDipoleInversion> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func operator-(other: AntiDualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar * -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, other.e3215 * -1.0)
        );
    }
}
extension AntiLine: Sub<AntiFlatPoint> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<AntiFlector> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<AntiLine> {
    associatedtype Output = AntiLine;
    func operator-(other: AntiLine) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz - other.group1.xyz, 0.0)
        );
    }
}
extension AntiLine {
    associatedtype Output = AntiLine;
    func operator-=(const AntiLine& other) {
        *this = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz - other.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: Sub<AntiMotor> {
    associatedtype Output = AntiMotor;
    func operator-(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((this.group0.xyz - other.group0.xyz), other.scalar) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4((this.group1.xyz - other.group1.xyz), other.e3215) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension AntiLine: Sub<AntiPlane> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<AntiScalar> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<Circle> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<CircleRotor> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<Dipole> {
    associatedtype Output = Dipole;
    func operator-(other: Dipole) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz - other.group2.xyz, 0.0)
        );
    }
}
extension AntiLine: Sub<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func operator-(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0)
        );
    }
}
extension AntiLine: Sub<DualNum> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<FlatPoint> {
    associatedtype Output = Dipole;
    func operator-(other: FlatPoint) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45 * -1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz - other.group0.xyz, 0.0)
        );
    }
}
extension AntiLine: Sub<Flector> {
    associatedtype Output = DipoleInversion;
    func operator-(other: Flector) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(-1.0)
        );
    }
}
extension AntiLine: Sub<Line> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<Motor> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<MultiVector> {
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
            float4((this.group1.xyz - other.group3.xyz), other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz - other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group6 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group7.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group8.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group9 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: Sub<Plane> {
    associatedtype Output = DipoleInversion;
    func operator-(other: Plane) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0)
        );
    }
}
extension AntiLine: Sub<RoundPoint> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<Scalar> {
    associatedtype Output = AntiMotor;
    func operator-(other: Scalar) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar * -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: Sub<Sphere> {
    associatedtype Output = DipoleInversion;
    func operator-(other: Sphere) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0)
        );
    }
}
extension AntiLine: Sub<VersorEven> {
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
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Sub<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator-(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(-1.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0)
        );
    }
}
