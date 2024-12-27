using traits::GeometricProduct;
using traits::RightDual;
extension AntiDualNum: Add<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, other.scalar + this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiDualNum: Add<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiDipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1, other.e2, other.e3, other.e4), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func operator+(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum {
    associatedtype Output = AntiDualNum;
    func operator+=(const AntiDualNum& other) {
        *this = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Add<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiFlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<AntiFlector> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiFlector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<AntiLine> {
    associatedtype Output = AntiMotor;
    func operator+(other: AntiLine) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.group0.xyz, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(other.group1.xyz, this.e3215)
        );
    }
}
extension AntiDualNum: Add<AntiMotor> {
    associatedtype Output = AntiMotor;
    func operator+(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.group0.xyz, this.scalar + other.scalar), 
            /* e15, e25, e35, e3215 */
            float4(other.group1.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiDualNum: Add<AntiPlane> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiPlane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<Circle> {
    associatedtype Output = MultiVector;
    func operator+(other: Circle) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<CircleRotor> {
    associatedtype Output = MultiVector;
    func operator+(other: CircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<Dipole> {
    associatedtype Output = VersorOdd;
    func operator+(other: Dipole) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiDualNum: Add<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator+(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiDualNum: Add<DualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<FlatPoint> {
    associatedtype Output = VersorOdd;
    func operator+(other: FlatPoint) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiDualNum: Add<Flector> {
    associatedtype Output = VersorOdd;
    func operator+(other: Flector) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiDualNum: Add<Line> {
    associatedtype Output = MultiVector;
    func operator+(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<Motor> {
    associatedtype Output = MultiVector;
    func operator+(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0) + other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group3, 
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
            float4(other.group9.xyz, this.e3215 + other.e3215), 
            /* e1234, 0, 0, 0 */
            float4(other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Add<Plane> {
    associatedtype Output = VersorOdd;
    func operator+(other: Plane) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiDualNum: Add<RoundPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: RoundPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<Scalar> {
    associatedtype Output = AntiDualNum;
    func operator+(other: Scalar) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(0.0, other.scalar) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum {
    associatedtype Output = AntiDualNum;
    func operator+=(const Scalar& other) {
        *this = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(0.0, other.scalar) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Add<Sphere> {
    associatedtype Output = VersorOdd;
    func operator+(other: Sphere) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiDualNum: Add<VersorEven> {
    associatedtype Output = MultiVector;
    func operator+(other: VersorEven) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group3, 
            /* e5, 0, 0, 0 */
            float4(other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Add<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator+(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar + other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiDualNum: Mul<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum {
    associatedtype Output = AntiDualNum;
    func operator*=(const AntiDualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<AntiFlatPoint> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<AntiFlector> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<AntiLine> {
    associatedtype Output = AntiLine;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<AntiMotor> {
    associatedtype Output = AntiMotor;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<AntiPlane> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<AntiScalar> {
    associatedtype Output = DualNum;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<Circle> {
    associatedtype Output = AntiDipoleInversion;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<Dipole> {
    associatedtype Output = DipoleInversion;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<DualNum> {
    associatedtype Output = DualNum;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<FlatPoint> {
    associatedtype Output = Flector;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<Flector> {
    associatedtype Output = Flector;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<Line> {
    associatedtype Output = Line;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<Motor> {
    associatedtype Output = Motor;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<Plane> {
    associatedtype Output = Flector;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<RoundPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<Scalar> {
    associatedtype Output = AntiDualNum;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum {
    associatedtype Output = AntiDualNum;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<Sphere> {
    associatedtype Output = VersorOdd;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<VersorEven> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Mul<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDualNum: Neg {
    associatedtype Output = AntiDualNum;
    func neg() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Not {
    associatedtype Output = DualNum;
    func not() -> Output {
        return this.right_dual();
    }
}
extension AntiDualNum: Sub<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiDualNum: Sub<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiDipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e4) * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func operator-(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum {
    associatedtype Output = AntiDualNum;
    func operator-=(const AntiDualNum& other) {
        *this = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Sub<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiFlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<AntiFlector> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiFlector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<AntiLine> {
    associatedtype Output = AntiMotor;
    func operator-(other: AntiLine) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(other.group1.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDualNum: Sub<AntiMotor> {
    associatedtype Output = AntiMotor;
    func operator-(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.group0.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(other.group1.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDualNum: Sub<AntiPlane> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiPlane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<Circle> {
    associatedtype Output = MultiVector;
    func operator-(other: Circle) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<CircleRotor> {
    associatedtype Output = MultiVector;
    func operator-(other: CircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<Dipole> {
    associatedtype Output = VersorOdd;
    func operator-(other: Dipole) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiDualNum: Sub<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator-(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDualNum: Sub<DualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<FlatPoint> {
    associatedtype Output = VersorOdd;
    func operator-(other: FlatPoint) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiDualNum: Sub<Flector> {
    associatedtype Output = VersorOdd;
    func operator-(other: Flector) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDualNum: Sub<Line> {
    associatedtype Output = MultiVector;
    func operator-(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<Motor> {
    associatedtype Output = MultiVector;
    func operator-(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar - other.scalar, other.e12345) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group3 * float4(-1.0), 
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
            float4(other.group9.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Sub<Plane> {
    associatedtype Output = VersorOdd;
    func operator-(other: Plane) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDualNum: Sub<RoundPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: RoundPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<Scalar> {
    associatedtype Output = AntiDualNum;
    func operator-(other: Scalar) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(0.0, other.scalar * -1.0) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum {
    associatedtype Output = AntiDualNum;
    func operator-=(const Scalar& other) {
        *this = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(0.0, other.scalar * -1.0) + this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Sub<Sphere> {
    associatedtype Output = VersorOdd;
    func operator-(other: Sphere) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDualNum: Sub<VersorEven> {
    associatedtype Output = MultiVector;
    func operator-(other: VersorEven) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, other.e12345) * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group3 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Sub<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator-(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
