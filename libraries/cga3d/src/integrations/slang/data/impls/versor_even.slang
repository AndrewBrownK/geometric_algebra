using traits::GeometricProduct;
using traits::RightDual;
extension VersorEven: Add<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Add<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator+(other: AntiDipoleInversion) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz + this.group0.xyz), this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(other.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            this.group3 + float4(other.group3.xyz, other.e4)
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const AntiDipoleInversion& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz + this.group0.xyz), this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(other.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            this.group3 + float4(other.group3.xyz, other.e4)
        );
    }
}
extension VersorEven: Add<AntiDualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Add<AntiFlatPoint> {
    associatedtype Output = VersorEven;
    func operator+(other: AntiFlatPoint) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e5 */
            float4((other.group0.xyz + this.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const AntiFlatPoint& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e5 */
            float4((other.group0.xyz + this.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Add<AntiFlector> {
    associatedtype Output = VersorEven;
    func operator+(other: AntiFlector) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e5 */
            this.group2 + float4(other.group0.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz + this.group3.xyz), this.e4)
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const AntiFlector& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e5 */
            this.group2 + float4(other.group0.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz + this.group3.xyz), this.e4)
        );
    }
}
extension VersorEven: Add<AntiLine> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Add<AntiMotor> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Add<AntiPlane> {
    associatedtype Output = VersorEven;
    func operator+(other: AntiPlane) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz + this.group3.xyz), this.e4)
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const AntiPlane& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz + this.group3.xyz), this.e4)
        );
    }
}
extension VersorEven: Add<AntiScalar> {
    associatedtype Output = VersorEven;
    func operator+(other: AntiScalar) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(float3(0.0), other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const AntiScalar& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(float3(0.0), other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Add<Circle> {
    associatedtype Output = VersorEven;
    func operator+(other: Circle) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz + this.group0.xyz), this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz + this.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const Circle& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz + this.group0.xyz), this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz + this.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Add<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator+(other: CircleRotor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz + this.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const CircleRotor& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz + this.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Add<Dipole> {
    associatedtype Output = MultiVector;
    func operator+(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Add<DipoleInversion> {
    associatedtype Output = MultiVector;
    func operator+(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Add<DualNum> {
    associatedtype Output = VersorEven;
    func operator+(other: DualNum) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 + this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const DualNum& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 + this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Add<FlatPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Add<Flector> {
    associatedtype Output = MultiVector;
    func operator+(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Add<Line> {
    associatedtype Output = VersorEven;
    func operator+(other: Line) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group1.xyz), this.e321), 
            /* e235, e315, e125, e5 */
            float4((other.group1.xyz + this.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const Line& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group1.xyz), this.e321), 
            /* e235, e315, e125, e5 */
            float4((other.group1.xyz + this.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Add<Motor> {
    associatedtype Output = VersorEven;
    func operator+(other: Motor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 + this.e12345), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group1.xyz), this.e321), 
            /* e235, e315, e125, e5 */
            other.group1 + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const Motor& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 + this.e12345), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group1.xyz), this.e321), 
            /* e235, e315, e125, e5 */
            other.group1 + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345) + other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 + this.group3, 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group3, 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group6 + this.group1, 
            /* e423, e431, e412, 0 */
            float4(other.group7.xyz + this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group8.xyz + this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Add<Plane> {
    associatedtype Output = MultiVector;
    func operator+(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Add<RoundPoint> {
    associatedtype Output = VersorEven;
    func operator+(other: RoundPoint) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(float3(0.0), other.e5), 
            /* e1, e2, e3, e4 */
            other.group0 + this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const RoundPoint& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(float3(0.0), other.e5), 
            /* e1, e2, e3, e4 */
            other.group0 + this.group3
        );
    }
}
extension VersorEven: Add<Scalar> {
    associatedtype Output = MultiVector;
    func operator+(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Add<Sphere> {
    associatedtype Output = MultiVector;
    func operator+(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Add<VersorEven> {
    associatedtype Output = VersorEven;
    func operator+(other: VersorEven) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 + this.group0, 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            other.group2 + this.group2, 
            /* e1, e2, e3, e4 */
            other.group3 + this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator+=(const VersorEven& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 + this.group0, 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            other.group2 + this.group2, 
            /* e1, e2, e3, e4 */
            other.group3 + this.group3
        );
    }
}
extension VersorEven: Add<VersorOdd> {
    associatedtype Output = MultiVector;
    func operator+(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234, 0.0, 0.0, 0.0)
        );
    }
}

impl From<AntiDipoleInversion> for VersorEven {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(from_anti_dipole_inversion.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            from_anti_dipole_inversion.group1, 
            /* e235, e315, e125, e5 */
            float4(from_anti_dipole_inversion.e235, from_anti_dipole_inversion.e315, from_anti_dipole_inversion.e125, from_anti_dipole_inversion.e5), 
            /* e1, e2, e3, e4 */
            float4(from_anti_dipole_inversion.e1, from_anti_dipole_inversion.e2, from_anti_dipole_inversion.e3, from_anti_dipole_inversion.e4)
        );
    }
}

impl From<AntiFlatPoint> for VersorEven {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125, e5 */
            float4(from_anti_flat_point.group0.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}

impl From<AntiFlector> for VersorEven {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flector.e321), 
            /* e235, e315, e125, e5 */
            float4(from_anti_flector.e235, from_anti_flector.e315, from_anti_flector.e125, from_anti_flector.e5), 
            /* e1, e2, e3, e4 */
            float4(from_anti_flector.group1.xyz, 0.0)
        );
    }
}

impl From<AntiPlane> for VersorEven {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_anti_plane.e5), 
            /* e1, e2, e3, e4 */
            float4(from_anti_plane.group0.xyz, 0.0)
        );
    }
}

impl From<AntiScalar> for VersorEven {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), from_anti_scalar.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}

impl From<Circle> for VersorEven {
    fn from(from_circle: Circle) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(from_circle.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            from_circle.group1, 
            /* e235, e315, e125, e5 */
            float4(from_circle.group2.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}

impl From<CircleRotor> for VersorEven {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(from_circle_rotor.group0.xyz, from_circle_rotor.e12345), 
            /* e415, e425, e435, e321 */
            from_circle_rotor.group1, 
            /* e235, e315, e125, e5 */
            float4(from_circle_rotor.group2.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}

impl From<DualNum> for VersorEven {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), from_dual_num.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_dual_num.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}

impl From<Line> for VersorEven {
    fn from(from_line: Line) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(from_line.group1.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}

impl From<Motor> for VersorEven {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), from_motor.e12345), 
            /* e415, e425, e435, e321 */
            float4(from_motor.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            from_motor.group1, 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}

impl From<RoundPoint> for VersorEven {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_round_point.e5), 
            /* e1, e2, e3, e4 */
            from_round_point.group0
        );
    }
}
extension VersorEven: Mul<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const AntiCircleRotor& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<AntiDualNum> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const AntiDualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<AntiFlatPoint> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<AntiFlector> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<AntiLine> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const AntiLine& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<AntiMotor> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const AntiMotor& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<AntiPlane> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<AntiScalar> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<Circle> {
    associatedtype Output = VersorOdd;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<CircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<Dipole> {
    associatedtype Output = VersorEven;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const Dipole& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<DipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const DipoleInversion& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<DualNum> {
    associatedtype Output = VersorOdd;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<FlatPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const FlatPoint& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<Flector> {
    associatedtype Output = VersorEven;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const Flector& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<Line> {
    associatedtype Output = VersorOdd;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<Motor> {
    associatedtype Output = VersorOdd;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<Plane> {
    associatedtype Output = VersorEven;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const Plane& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<RoundPoint> {
    associatedtype Output = VersorOdd;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<Scalar> {
    associatedtype Output = VersorEven;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<Sphere> {
    associatedtype Output = VersorEven;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const Sphere& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Mul<VersorEven> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven: Mul<VersorOdd> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator*=(const VersorOdd& other) {
        *this = this.geometric_product(other);
    }
}
extension VersorEven: Neg {
    associatedtype Output = VersorEven;
    func neg() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e4 */
            this.group3 * float4(-1.0)
        );
    }
}
extension VersorEven: Not {
    associatedtype Output = VersorOdd;
    func not() -> Output {
        return this.right_dual();
    }
}
extension VersorEven: Sub<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45) * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Sub<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator-(other: AntiDipoleInversion) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, 0.0) + this.group0, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            this.group2 - float4(other.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            this.group3 - float4(other.group3.xyz, other.e4)
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const AntiDipoleInversion& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, 0.0) + this.group0, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            this.group2 - float4(other.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            this.group3 - float4(other.group3.xyz, other.e4)
        );
    }
}
extension VersorEven: Sub<AntiDualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Sub<AntiFlatPoint> {
    associatedtype Output = VersorEven;
    func operator-(other: AntiFlatPoint) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const AntiFlatPoint& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Sub<AntiFlector> {
    associatedtype Output = VersorEven;
    func operator-(other: AntiFlector) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            this.group2 - float4(other.group0.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, 0.0) + this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const AntiFlector& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            this.group2 - float4(other.group0.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, 0.0) + this.group3
        );
    }
}
extension VersorEven: Sub<AntiLine> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Sub<AntiMotor> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Sub<AntiPlane> {
    associatedtype Output = VersorEven;
    func operator-(other: AntiPlane) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(float3(0.0), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, 0.0) + this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const AntiPlane& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(float3(0.0), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, 0.0) + this.group3
        );
    }
}
extension VersorEven: Sub<AntiScalar> {
    associatedtype Output = VersorEven;
    func operator-(other: AntiScalar) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const AntiScalar& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Sub<Circle> {
    associatedtype Output = VersorEven;
    func operator-(other: Circle) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, 0.0) + this.group0, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const Circle& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, 0.0) + this.group0, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Sub<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator-(other: CircleRotor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 - float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const CircleRotor& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 - float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Sub<Dipole> {
    associatedtype Output = MultiVector;
    func operator-(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45) * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Sub<DipoleInversion> {
    associatedtype Output = MultiVector;
    func operator-(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45) * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Sub<DualNum> {
    associatedtype Output = VersorEven;
    func operator-(other: DualNum) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(float3(0.0), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const DualNum& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(float3(0.0), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Sub<FlatPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Sub<Flector> {
    associatedtype Output = MultiVector;
    func operator-(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Sub<Line> {
    associatedtype Output = VersorEven;
    func operator-(other: Line) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const Line& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Sub<Motor> {
    associatedtype Output = VersorEven;
    func operator-(other: Motor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 - other.group1, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const Motor& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 + float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 - other.group1, 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorEven: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345 - other.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3 - other.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group3 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz - other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz - other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group9 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Sub<Plane> {
    associatedtype Output = MultiVector;
    func operator-(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Sub<RoundPoint> {
    associatedtype Output = VersorEven;
    func operator-(other: RoundPoint) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(float3(0.0), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            this.group3 - other.group0
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const RoundPoint& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 + float4(float3(0.0), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            this.group3 - other.group0
        );
    }
}
extension VersorEven: Sub<Scalar> {
    associatedtype Output = MultiVector;
    func operator-(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Sub<Sphere> {
    associatedtype Output = MultiVector;
    func operator-(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Sub<VersorEven> {
    associatedtype Output = VersorEven;
    func operator-(other: VersorEven) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 - other.group0, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            this.group2 - other.group2, 
            /* e1, e2, e3, e4 */
            this.group3 - other.group3
        );
    }
}
extension VersorEven {
    associatedtype Output = VersorEven;
    func operator-=(const VersorEven& other) {
        *this = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 - other.group0, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            this.group2 - other.group2, 
            /* e1, e2, e3, e4 */
            this.group3 - other.group3
        );
    }
}
extension VersorEven: Sub<VersorOdd> {
    associatedtype Output = MultiVector;
    func operator-(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45) * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}

impl TryFrom<MultiVector> for VersorEven {
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
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[16];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into VersorEven { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(multi_vector.group7.xyz, multi_vector.e12345), 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125, e5 */
            float4(multi_vector.group8.xyz, multi_vector.e5), 
            /* e1, e2, e3, e4 */
            multi_vector.group1
        ));
    }
}
