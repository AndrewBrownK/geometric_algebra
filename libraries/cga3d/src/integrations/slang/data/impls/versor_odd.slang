using traits::GeometricProduct;
using traits::RightDual;
extension VersorOdd: Add<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const AntiCircleRotor& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Add<AntiDipoleInversion> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar + this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, other.e3215 + this.e3215)
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const AntiDualNum& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar + this.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, other.e3215 + this.e3215)
        );
    }
}
extension VersorOdd: Add<AntiFlatPoint> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<AntiFlector> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<AntiLine> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiLine) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz + this.group1.xyz), this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const AntiLine& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz + this.group1.xyz), this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Add<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator+(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar + this.scalar), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz + this.group1.xyz), this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, other.e3215 + this.e3215)
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const AntiMotor& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, other.scalar + this.scalar), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz + this.group1.xyz), this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, other.e3215 + this.e3215)
        );
    }
}
extension VersorOdd: Add<AntiPlane> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<AntiScalar> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<Circle> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<CircleRotor> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<Dipole> {
    associatedtype Output = VersorOdd;
    func operator+(other: Dipole) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz + this.group0.xyz), this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const Dipole& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz + this.group0.xyz), this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Add<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator+(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz + this.group0.xyz), this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            other.group2 + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const DipoleInversion& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz + this.group0.xyz), this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            other.group2 + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group3
        );
    }
}
extension VersorOdd: Add<DualNum> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<FlatPoint> {
    associatedtype Output = VersorOdd;
    func operator+(other: FlatPoint) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            float4(this.group1.xyz, other.e45 + this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group0.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const FlatPoint& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            float4(this.group1.xyz, other.e45 + this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group0.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Add<Flector> {
    associatedtype Output = VersorOdd;
    func operator+(other: Flector) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            float4(this.group1.xyz, other.e45 + this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group0.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 + this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const Flector& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            float4(this.group1.xyz, other.e45 + this.e45), 
            /* e15, e25, e35, e1234 */
            float4((other.group0.xyz + this.group2.xyz), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 + this.group3
        );
    }
}
extension VersorOdd: Add<Line> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<Motor> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<MultiVector> {
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
            float4(other.group4.xyz + this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz + this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group9 + this.group3, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 + this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<Plane> {
    associatedtype Output = VersorOdd;
    func operator+(other: Plane) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group0 + this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const Plane& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group0 + this.group3
        );
    }
}
extension VersorOdd: Add<RoundPoint> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<Scalar> {
    associatedtype Output = VersorOdd;
    func operator+(other: Scalar) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const Scalar& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Add<Sphere> {
    associatedtype Output = VersorOdd;
    func operator+(other: Sphere) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(float3(0.0), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 + this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const Sphere& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(float3(0.0), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 + this.group3
        );
    }
}
extension VersorOdd: Add<VersorEven> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Add<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator+(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 + this.group0, 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            other.group2 + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator+=(const VersorOdd& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 + this.group0, 
            /* e23, e31, e12, e45 */
            other.group1 + this.group1, 
            /* e15, e25, e35, e1234 */
            other.group2 + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group3
        );
    }
}
extension VersorOdd: Mul<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const AntiCircleRotor& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const AntiDualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<AntiFlatPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<AntiFlector> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<AntiLine> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const AntiLine& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const AntiMotor& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<AntiPlane> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<AntiScalar> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<Circle> {
    associatedtype Output = VersorEven;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<Dipole> {
    associatedtype Output = VersorOdd;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const Dipole& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const DipoleInversion& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<DualNum> {
    associatedtype Output = VersorEven;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<FlatPoint> {
    associatedtype Output = VersorOdd;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const FlatPoint& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<Flector> {
    associatedtype Output = VersorOdd;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const Flector& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<Line> {
    associatedtype Output = VersorEven;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<Motor> {
    associatedtype Output = VersorEven;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<Plane> {
    associatedtype Output = VersorOdd;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const Plane& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<RoundPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<Scalar> {
    associatedtype Output = VersorOdd;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<Sphere> {
    associatedtype Output = VersorOdd;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const Sphere& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Mul<VersorEven> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd: Mul<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator*=(const VersorOdd& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorOdd: Neg {
    associatedtype Output = VersorOdd;
    func neg() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
extension VersorOdd: Not {
    associatedtype Output = VersorEven;
    func not() -> Output {
        return this.right_dual();
    }
}
extension VersorOdd: Sub<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 - float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const AntiCircleRotor& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 - float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Sub<AntiDipoleInversion> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const AntiDualNum& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension VersorOdd: Sub<AntiFlatPoint> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<AntiFlector> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<AntiLine> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiLine) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const AntiLine& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Sub<AntiMotor> {
    associatedtype Output = VersorOdd;
    func operator-(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const AntiMotor& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 + float4(float3(0.0), other.e3215 * -1.0)
        );
    }
}
extension VersorOdd: Sub<AntiPlane> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<AntiScalar> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<Circle> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<CircleRotor> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<Dipole> {
    associatedtype Output = VersorOdd;
    func operator-(other: Dipole) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const Dipole& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Sub<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator-(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 - other.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const DipoleInversion& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 - other.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group3
        );
    }
}
extension VersorOdd: Sub<DualNum> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<FlatPoint> {
    associatedtype Output = VersorOdd;
    func operator-(other: FlatPoint) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const FlatPoint& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Sub<Flector> {
    associatedtype Output = VersorOdd;
    func operator-(other: Flector) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group1
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const Flector& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 + float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group1
        );
    }
}
extension VersorOdd: Sub<Line> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<Motor> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<MultiVector> {
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
            this.group3 - other.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<Plane> {
    associatedtype Output = VersorOdd;
    func operator-(other: Plane) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group0
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const Plane& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group0
        );
    }
}
extension VersorOdd: Sub<RoundPoint> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<Scalar> {
    associatedtype Output = VersorOdd;
    func operator-(other: Scalar) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const Scalar& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 + float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd: Sub<Sphere> {
    associatedtype Output = VersorOdd;
    func operator-(other: Sphere) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group0
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const Sphere& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group0
        );
    }
}
extension VersorOdd: Sub<VersorEven> {
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
            this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Sub<VersorOdd> {
    associatedtype Output = VersorOdd;
    func operator-(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 - other.group0, 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 - other.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group3
        );
    }
}
extension VersorOdd {
    associatedtype Output = VersorOdd;
    func operator-=(const VersorOdd& other) {
        *this = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 - other.group0, 
            /* e23, e31, e12, e45 */
            this.group1 - other.group1, 
            /* e15, e25, e35, e1234 */
            this.group2 - other.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group3 - other.group3
        );
    }
}
