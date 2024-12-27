using traits::GeometricProduct;
using traits::RightDual;
extension DipoleInversion: Add<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz + this.group0.xyz), other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Add<AntiDipoleInversion> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, other.e3215 + this.e3215)
        );
    }
}
extension DipoleInversion: Add<AntiFlatPoint> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<AntiFlector> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<AntiLine> {
    associatedtype Output = DipoleInversion;
    func operator+(other: AntiLine) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz + this.group1.xyz), this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator+=(const AntiLine& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz + this.group1.xyz), this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Add<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz + this.group1.xyz), this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, other.e3215 + this.e3215)
        );
    }
}
extension DipoleInversion: Add<AntiPlane> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<AntiScalar> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<Circle> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<CircleRotor> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<Dipole> {
    associatedtype Output = DipoleInversion;
    func operator+(other: Dipole) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator+=(const Dipole& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Add<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func operator+(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            other.group2 + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group3
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator+=(const DipoleInversion& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            other.group2 + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group3
        );
    }
}
extension DipoleInversion: Add<DualNum> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<FlatPoint> {
    associatedtype Output = DipoleInversion;
    func operator+(other: FlatPoint) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator+=(const FlatPoint& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Add<Flector> {
    associatedtype Output = DipoleInversion;
    func operator+(other: Flector) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + other.group1
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator+=(const Flector& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + other.group1
        );
    }
}
extension DipoleInversion: Add<Line> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<Motor> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<MultiVector> {
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
            this.group3 + other.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 + other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<Plane> {
    associatedtype Output = DipoleInversion;
    func operator+(other: Plane) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + other.group0
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator+=(const Plane& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + other.group0
        );
    }
}
extension DipoleInversion: Add<RoundPoint> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<Scalar> {
    associatedtype Output = VersorOdd;
    func operator+(other: Scalar) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Add<Sphere> {
    associatedtype Output = DipoleInversion;
    func operator+(other: Sphere) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(float3(0.0), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + other.group0
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator+=(const Sphere& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(float3(0.0), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + other.group0
        );
    }
}
extension DipoleInversion: Add<VersorEven> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Add<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator+(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4((this.group0.xyz + other.group0.xyz), other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1 + other.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 + other.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + other.group3
        );
    }
}

impl From<AntiLine> for DipoleInversion {
    fn from(from_anti_line: AntiLine) -> Self {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(from_anti_line.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}

impl From<Dipole> for DipoleInversion {
    fn from(from_dipole: Dipole) -> Self {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(from_dipole.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            from_dipole.group1, 
            /* e15, e25, e35, e1234 */
            float4(from_dipole.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}

impl From<FlatPoint> for DipoleInversion {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flat_point.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flat_point.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}

impl From<Flector> for DipoleInversion {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flector.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flector.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_flector.group1
        );
    }
}

impl From<Plane> for DipoleInversion {
    fn from(from_plane: Plane) -> Self {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_plane.group0
        );
    }
}

impl From<Sphere> for DipoleInversion {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), from_sphere.e1234), 
            /* e4235, e4315, e4125, e3215 */
            from_sphere.group0
        );
    }
}
extension DipoleInversion: Mul<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<AntiFlatPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<AntiFlector> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<AntiLine> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<AntiPlane> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<AntiScalar> {
    associatedtype Output = AntiDipoleInversion;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<Circle> {
    associatedtype Output = VersorEven;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<Dipole> {
    associatedtype Output = VersorOdd;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<DualNum> {
    associatedtype Output = VersorEven;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<FlatPoint> {
    associatedtype Output = VersorOdd;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<Flector> {
    associatedtype Output = VersorOdd;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<Line> {
    associatedtype Output = VersorEven;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<Motor> {
    associatedtype Output = VersorEven;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<Plane> {
    associatedtype Output = VersorOdd;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<RoundPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<Scalar> {
    associatedtype Output = DipoleInversion;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<Sphere> {
    associatedtype Output = VersorOdd;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<VersorEven> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Mul<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension DipoleInversion: Neg {
    associatedtype Output = DipoleInversion;
    func neg() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
extension DipoleInversion: Not {
    associatedtype Output = AntiDipoleInversion;
    func not() -> Output {
        return this.right_dual();
    }
}
extension DipoleInversion: Sub<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4((this.group0.xyz - other.group0.xyz), other.scalar) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Sub<AntiDipoleInversion> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension DipoleInversion: Sub<AntiFlatPoint> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<AntiFlector> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<AntiLine> {
    associatedtype Output = DipoleInversion;
    func operator-(other: AntiLine) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator-=(const AntiLine& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Sub<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension DipoleInversion: Sub<AntiPlane> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<AntiScalar> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<Circle> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<CircleRotor> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<Dipole> {
    associatedtype Output = DipoleInversion;
    func operator-(other: Dipole) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator-=(const Dipole& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Sub<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func operator-(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 - other.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group3
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator-=(const DipoleInversion& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 - other.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group3
        );
    }
}
extension DipoleInversion: Sub<DualNum> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<FlatPoint> {
    associatedtype Output = DipoleInversion;
    func operator-(other: FlatPoint) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator-=(const FlatPoint& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Sub<Flector> {
    associatedtype Output = DipoleInversion;
    func operator-(other: Flector) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group1
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator-=(const Flector& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group1
        );
    }
}
extension DipoleInversion: Sub<Line> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<Motor> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<MultiVector> {
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
            this.group3 - other.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<Plane> {
    associatedtype Output = DipoleInversion;
    func operator-(other: Plane) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group0
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator-=(const Plane& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group0
        );
    }
}
extension DipoleInversion: Sub<RoundPoint> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<Scalar> {
    associatedtype Output = VersorOdd;
    func operator-(other: Scalar) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DipoleInversion: Sub<Sphere> {
    associatedtype Output = DipoleInversion;
    func operator-(other: Sphere) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group0
        );
    }
}
extension DipoleInversion {
    associatedtype Output = DipoleInversion;
    func operator-=(const Sphere& other) {
        *this = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group0
        );
    }
}
extension DipoleInversion: Sub<VersorEven> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Sub<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator-(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4((this.group0.xyz - other.group0.xyz), other.scalar) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 - other.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group3
        );
    }
}

impl TryFrom<AntiCircleRotor> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(anti_circle_rotor.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            anti_circle_rotor.group1, 
            /* e15, e25, e35, e1234 */
            float4(anti_circle_rotor.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        ));
    }
}

impl TryFrom<AntiDualNum> for DipoleInversion {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_dual_num.e3215)
        ));
    }
}

impl TryFrom<AntiMotor> for DipoleInversion {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(anti_motor.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_motor.e3215)
        ));
    }
}

impl TryFrom<MultiVector> for DipoleInversion {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[17];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[18];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[19];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[21];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[22];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[23];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[24];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[25];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[26];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(multi_vector.group4.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(multi_vector.group5.xyz, multi_vector.e45), 
            /* e15, e25, e35, e1234 */
            float4(multi_vector.e15, multi_vector.e25, multi_vector.e35, multi_vector.e1234), 
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9
        ));
    }
}

impl TryFrom<VersorOdd> for DipoleInversion {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(versor_odd.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            versor_odd.group1, 
            /* e15, e25, e35, e1234 */
            versor_odd.group2, 
            /* e4235, e4315, e4125, e3215 */
            versor_odd.group3
        ));
    }
}
