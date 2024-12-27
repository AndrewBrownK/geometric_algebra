using traits::GeometricProduct;
using traits::RightDual;
extension AntiCircleRotor: Add<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func operator+(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, scalar */
            other.group2 + this.group2
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator+=(const AntiCircleRotor& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, scalar */
            other.group2 + this.group2
        );
    }
}
extension AntiCircleRotor: Add<AntiDipoleInversion> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar + other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215)
        );
    }
}
extension AntiCircleRotor: Add<AntiFlatPoint> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<AntiFlector> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<AntiLine> {
    associatedtype Output = AntiCircleRotor;
    func operator+(other: AntiLine) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz + this.group1.xyz), this.e45), 
            /* e15, e25, e35, scalar */
            float4((other.group1.xyz + this.group2.xyz), this.scalar)
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator+=(const AntiLine& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz + this.group1.xyz), this.e45), 
            /* e15, e25, e35, scalar */
            float4((other.group1.xyz + this.group2.xyz), this.scalar)
        );
    }
}
extension AntiCircleRotor: Add<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar + other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz + other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215)
        );
    }
}
extension AntiCircleRotor: Add<AntiPlane> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<AntiScalar> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<Circle> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<CircleRotor> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func operator+(other: Dipole) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz + other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + other.group1, 
            /* e15, e25, e35, scalar */
            float4((other.group2.xyz + this.group2.xyz), this.scalar)
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator+=(const Dipole& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz + other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + other.group1, 
            /* e15, e25, e35, scalar */
            float4((other.group2.xyz + this.group2.xyz), this.scalar)
        );
    }
}
extension AntiCircleRotor: Add<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator+(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4((this.group0.xyz + other.group0.xyz), this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1 + other.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz + other.group2.xyz), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group3
        );
    }
}
extension AntiCircleRotor: Add<DualNum> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<FlatPoint> {
    associatedtype Output = AntiCircleRotor;
    func operator+(other: FlatPoint) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45), 
            /* e15, e25, e35, scalar */
            this.group2 + float4(other.group0.xyz, 0.0)
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator+=(const FlatPoint& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45), 
            /* e15, e25, e35, scalar */
            this.group2 + float4(other.group0.xyz, 0.0)
        );
    }
}
extension AntiCircleRotor: Add<Flector> {
    associatedtype Output = VersorOdd;
    func operator+(other: Flector) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz + other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1
        );
    }
}
extension AntiCircleRotor: Add<Line> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<Motor> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<MultiVector> {
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
            other.group3 + float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz + other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz + this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<Plane> {
    associatedtype Output = VersorOdd;
    func operator+(other: Plane) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0
        );
    }
}
extension AntiCircleRotor: Add<RoundPoint> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<Scalar> {
    associatedtype Output = AntiCircleRotor;
    func operator+(other: Scalar) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, scalar */
            this.group2 + float4(float3(0.0), other.scalar)
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator+=(const Scalar& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, scalar */
            this.group2 + float4(float3(0.0), other.scalar)
        );
    }
}
extension AntiCircleRotor: Add<Sphere> {
    associatedtype Output = VersorOdd;
    func operator+(other: Sphere) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group0
        );
    }
}
extension AntiCircleRotor: Add<VersorEven> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Add<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator+(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 + float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1 + other.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz + other.group2.xyz), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group3
        );
    }
}
extension AntiCircleRotor: Mul<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<AntiFlatPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<AntiFlector> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<AntiLine> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<AntiPlane> {
    associatedtype Output = AntiDipoleInversion;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<AntiScalar> {
    associatedtype Output = CircleRotor;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<Circle> {
    associatedtype Output = VersorEven;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<Dipole> {
    associatedtype Output = VersorOdd;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<DualNum> {
    associatedtype Output = VersorEven;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<FlatPoint> {
    associatedtype Output = VersorOdd;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<Flector> {
    associatedtype Output = VersorOdd;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<Line> {
    associatedtype Output = VersorEven;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<Motor> {
    associatedtype Output = VersorEven;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<Plane> {
    associatedtype Output = DipoleInversion;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<Scalar> {
    associatedtype Output = AntiCircleRotor;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<Sphere> {
    associatedtype Output = DipoleInversion;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<VersorEven> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Mul<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiCircleRotor: Neg {
    associatedtype Output = AntiCircleRotor;
    func neg() -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0)
        );
    }
}
extension AntiCircleRotor: Not {
    associatedtype Output = CircleRotor;
    func not() -> Output {
        return this.right_dual();
    }
}
extension AntiCircleRotor: Sub<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func operator-(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, scalar */
            this.group2 - other.group2
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator-=(const AntiCircleRotor& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, scalar */
            this.group2 - other.group2
        );
    }
}
extension AntiCircleRotor: Sub<AntiDipoleInversion> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar - other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension AntiCircleRotor: Sub<AntiFlatPoint> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<AntiFlector> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<AntiLine> {
    associatedtype Output = AntiCircleRotor;
    func operator-(other: AntiLine) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator-=(const AntiLine& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2
        );
    }
}
extension AntiCircleRotor: Sub<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar - other.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz - other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension AntiCircleRotor: Sub<AntiPlane> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<AntiScalar> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<Circle> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<CircleRotor> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func operator-(other: Dipole) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator-=(const Dipole& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2
        );
    }
}
extension AntiCircleRotor: Sub<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator-(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz - other.group2.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0)
        );
    }
}
extension AntiCircleRotor: Sub<DualNum> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<FlatPoint> {
    associatedtype Output = AntiCircleRotor;
    func operator-(other: FlatPoint) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, scalar */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator-=(const FlatPoint& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, scalar */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2
        );
    }
}
extension AntiCircleRotor: Sub<Flector> {
    associatedtype Output = VersorOdd;
    func operator-(other: Flector) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz - other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(-1.0)
        );
    }
}
extension AntiCircleRotor: Sub<Line> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<Motor> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<MultiVector> {
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
            float4(this.group2.xyz, this.e45) - other.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz - other.group5.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<Plane> {
    associatedtype Output = VersorOdd;
    func operator-(other: Plane) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0)
        );
    }
}
extension AntiCircleRotor: Sub<RoundPoint> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<Scalar> {
    associatedtype Output = AntiCircleRotor;
    func operator-(other: Scalar) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, scalar */
            this.group2 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
extension AntiCircleRotor {
    associatedtype Output = AntiCircleRotor;
    func operator-=(const Scalar& other) {
        *this = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, scalar */
            this.group2 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
extension AntiCircleRotor: Sub<Sphere> {
    associatedtype Output = VersorOdd;
    func operator-(other: Sphere) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0)
        );
    }
}
extension AntiCircleRotor: Sub<VersorEven> {
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
            float4(this.e15, this.e25, this.e35, this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: Sub<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator-(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.scalar) - other.group0, 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz - other.group2.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0)
        );
    }
}
