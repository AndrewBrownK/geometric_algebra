using traits::GeometricProduct;
using traits::RightDual;
extension AntiMotor: Add<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, other.scalar + this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiMotor: Add<AntiDipoleInversion> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func operator+(other: AntiDualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar + this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, other.e3215 + this.e3215)
        );
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator+=(const AntiDualNum& other) {
        *this = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar + this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, other.e3215 + this.e3215)
        );
    }
}
extension AntiMotor: Add<AntiFlatPoint> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<AntiFlector> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<AntiLine> {
    associatedtype Output = AntiMotor;
    func operator+(other: AntiLine) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz + this.group0.xyz), this.scalar), 
            /* e15, e25, e35, e3215 */
            float4((other.group1.xyz + this.group1.xyz), this.e3215)
        );
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator+=(const AntiLine& other) {
        *this = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz + this.group0.xyz), this.scalar), 
            /* e15, e25, e35, e3215 */
            float4((other.group1.xyz + this.group1.xyz), this.e3215)
        );
    }
}
extension AntiMotor: Add<AntiMotor> {
    associatedtype Output = AntiMotor;
    func operator+(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 + this.group0, 
            /* e15, e25, e35, e3215 */
            other.group1 + this.group1
        );
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator+=(const AntiMotor& other) {
        *this = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 + this.group0, 
            /* e15, e25, e35, e3215 */
            other.group1 + this.group1
        );
    }
}
extension AntiMotor: Add<AntiPlane> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<AntiScalar> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<Circle> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<CircleRotor> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<Dipole> {
    associatedtype Output = VersorOdd;
    func operator+(other: Dipole) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz + other.group1.xyz), other.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiMotor: Add<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator+(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz + other.group1.xyz), other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group2.xyz), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiMotor: Add<DualNum> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<FlatPoint> {
    associatedtype Output = VersorOdd;
    func operator+(other: FlatPoint) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiMotor: Add<Flector> {
    associatedtype Output = VersorOdd;
    func operator+(other: Flector) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiMotor: Add<Line> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<Motor> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<MultiVector> {
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
            float4((this.group1.xyz + other.group3.xyz), other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz + this.group0.xyz, 0.0), 
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
extension AntiMotor: Add<Plane> {
    associatedtype Output = VersorOdd;
    func operator+(other: Plane) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiMotor: Add<RoundPoint> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<Scalar> {
    associatedtype Output = AntiMotor;
    func operator+(other: Scalar) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 + float4(float3(0.0), other.scalar), 
            /* e15, e25, e35, e3215 */
            this.group1
        );
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator+=(const Scalar& other) {
        *this = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 + float4(float3(0.0), other.scalar), 
            /* e15, e25, e35, e3215 */
            this.group1
        );
    }
}
extension AntiMotor: Add<Sphere> {
    associatedtype Output = VersorOdd;
    func operator+(other: Sphere) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiMotor: Add<VersorEven> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Add<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator+(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar + other.scalar), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz + other.group1.xyz), other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group2.xyz), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 + other.e3215)
        );
    }
}
extension AntiMotor: Mul<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator*=(const AntiDualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiMotor: Mul<AntiFlatPoint> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<AntiFlector> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<AntiLine> {
    associatedtype Output = AntiMotor;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator*=(const AntiLine& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiMotor: Mul<AntiMotor> {
    associatedtype Output = AntiMotor;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator*=(const AntiMotor& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiMotor: Mul<AntiPlane> {
    associatedtype Output = AntiFlector;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<AntiScalar> {
    associatedtype Output = Motor;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<Circle> {
    associatedtype Output = VersorEven;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<Dipole> {
    associatedtype Output = VersorOdd;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<DualNum> {
    associatedtype Output = Motor;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<FlatPoint> {
    associatedtype Output = Flector;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<Flector> {
    associatedtype Output = Flector;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<Line> {
    associatedtype Output = Motor;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<Motor> {
    associatedtype Output = Motor;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<Plane> {
    associatedtype Output = Flector;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<RoundPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<Scalar> {
    associatedtype Output = AntiMotor;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiMotor: Mul<Sphere> {
    associatedtype Output = VersorOdd;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<VersorEven> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Mul<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiMotor: Neg {
    associatedtype Output = AntiMotor;
    func neg() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
extension AntiMotor: Not {
    associatedtype Output = Motor;
    func not() -> Output {
        return this.right_dual();
    }
}
extension AntiMotor: Sub<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiMotor: Sub<AntiDipoleInversion> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func operator-(other: AntiDualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator-=(const AntiDualNum& other) {
        *this = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension AntiMotor: Sub<AntiFlatPoint> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<AntiFlector> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<AntiLine> {
    associatedtype Output = AntiMotor;
    func operator-(other: AntiLine) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group1
        );
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator-=(const AntiLine& other) {
        *this = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group1
        );
    }
}
extension AntiMotor: Sub<AntiMotor> {
    associatedtype Output = AntiMotor;
    func operator-(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 - other.group0, 
            /* e15, e25, e35, e3215 */
            this.group1 - other.group1
        );
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator-=(const AntiMotor& other) {
        *this = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 - other.group0, 
            /* e15, e25, e35, e3215 */
            this.group1 - other.group1
        );
    }
}
extension AntiMotor: Sub<AntiPlane> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<AntiScalar> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<Circle> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<CircleRotor> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<Dipole> {
    associatedtype Output = VersorOdd;
    func operator-(other: Dipole) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiMotor: Sub<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator-(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiMotor: Sub<DualNum> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<FlatPoint> {
    associatedtype Output = VersorOdd;
    func operator-(other: FlatPoint) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiMotor: Sub<Flector> {
    associatedtype Output = VersorOdd;
    func operator-(other: Flector) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiMotor: Sub<Line> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<Motor> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<MultiVector> {
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
            float4(other.group9.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: Sub<Plane> {
    associatedtype Output = VersorOdd;
    func operator-(other: Plane) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiMotor: Sub<RoundPoint> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<Scalar> {
    associatedtype Output = AntiMotor;
    func operator-(other: Scalar) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1
        );
    }
}
extension AntiMotor {
    associatedtype Output = AntiMotor;
    func operator-=(const Scalar& other) {
        *this = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1
        );
    }
}
extension AntiMotor: Sub<Sphere> {
    associatedtype Output = VersorOdd;
    func operator-(other: Sphere) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiMotor: Sub<VersorEven> {
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
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Sub<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator-(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 - other.e3215) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
