using traits::GeometricProduct;
using traits::RightDual;
extension AntiDipoleInversion: Add<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func operator+(other: AntiDipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e4 */
            other.group2 + this.group2, 
            /* e1, e2, e3, e5 */
            other.group3 + this.group3
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator+=(const AntiDipoleInversion& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e4 */
            other.group2 + this.group2, 
            /* e1, e2, e3, e5 */
            other.group3 + this.group3
        );
    }
}
extension AntiDipoleInversion: Add<AntiDualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<AntiFlatPoint> {
    associatedtype Output = AntiDipoleInversion;
    func operator+(other: AntiFlatPoint) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e4 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator+=(const AntiFlatPoint& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e4 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion: Add<AntiFlector> {
    associatedtype Output = AntiDipoleInversion;
    func operator+(other: AntiFlector) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e4 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3 + other.group1
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator+=(const AntiFlector& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e4 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3 + other.group1
        );
    }
}
extension AntiDipoleInversion: Add<AntiLine> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<AntiMotor> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<AntiPlane> {
    associatedtype Output = AntiDipoleInversion;
    func operator+(other: AntiPlane) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2, 
            /* e1, e2, e3, e5 */
            this.group3 + other.group0
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator+=(const AntiPlane& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2, 
            /* e1, e2, e3, e5 */
            this.group3 + other.group0
        );
    }
}
extension AntiDipoleInversion: Add<AntiScalar> {
    associatedtype Output = VersorEven;
    func operator+(other: AntiScalar) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.e235, this.e315, this.e125, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
extension AntiDipoleInversion: Add<Circle> {
    associatedtype Output = AntiDipoleInversion;
    func operator+(other: Circle) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz + other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + other.group1, 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz + this.group2.xyz), this.e4), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator+=(const Circle& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz + other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + other.group1, 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz + this.group2.xyz), this.e4), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion: Add<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator+(other: CircleRotor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz + other.group0.xyz), other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + other.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz + other.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
extension AntiDipoleInversion: Add<Dipole> {
    associatedtype Output = MultiVector;
    func operator+(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<DipoleInversion> {
    associatedtype Output = MultiVector;
    func operator+(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<DualNum> {
    associatedtype Output = VersorEven;
    func operator+(other: DualNum) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5 + other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
extension AntiDipoleInversion: Add<FlatPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<Flector> {
    associatedtype Output = MultiVector;
    func operator+(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<Line> {
    associatedtype Output = AntiDipoleInversion;
    func operator+(other: Line) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group1.xyz), this.e321), 
            /* e235, e315, e125, e4 */
            float4((other.group1.xyz + this.group2.xyz), this.e4), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator+=(const Line& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group1.xyz), this.e321), 
            /* e235, e315, e125, e4 */
            float4((other.group1.xyz + this.group2.xyz), this.e4), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion: Add<Motor> {
    associatedtype Output = VersorEven;
    func operator+(other: Motor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            other.group1 + float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
extension AntiDipoleInversion: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 + float4(this.group3.xyz, this.e4), 
            /* e5, 0, 0, 0 */
            float4(this.e5 + other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group3, 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + other.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz + other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group8.xyz + this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: Add<Plane> {
    associatedtype Output = MultiVector;
    func operator+(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func operator+(other: RoundPoint) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2 + float4(float3(0.0), other.e4), 
            /* e1, e2, e3, e5 */
            this.group3 + float4(other.group0.xyz, other.e5)
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator+=(const RoundPoint& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2 + float4(float3(0.0), other.e4), 
            /* e1, e2, e3, e5 */
            this.group3 + float4(other.group0.xyz, other.e5)
        );
    }
}
extension AntiDipoleInversion: Add<Scalar> {
    associatedtype Output = MultiVector;
    func operator+(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<Sphere> {
    associatedtype Output = MultiVector;
    func operator+(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Add<VersorEven> {
    associatedtype Output = VersorEven;
    func operator+(other: VersorEven) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz + other.group0.xyz), other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + other.group1, 
            /* e235, e315, e125, e5 */
            other.group2 + float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            other.group3 + float4(this.group3.xyz, this.e4)
        );
    }
}
extension AntiDipoleInversion: Add<VersorOdd> {
    associatedtype Output = MultiVector;
    func operator+(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Mul<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<AntiDualNum> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<AntiFlatPoint> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<AntiFlector> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<AntiLine> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<AntiMotor> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<AntiPlane> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<AntiScalar> {
    associatedtype Output = DipoleInversion;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<Circle> {
    associatedtype Output = VersorOdd;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<CircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<Dipole> {
    associatedtype Output = VersorEven;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<DipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<DualNum> {
    associatedtype Output = VersorOdd;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<FlatPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<Flector> {
    associatedtype Output = VersorEven;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<Line> {
    associatedtype Output = VersorOdd;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<Motor> {
    associatedtype Output = VersorOdd;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<Plane> {
    associatedtype Output = VersorEven;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<RoundPoint> {
    associatedtype Output = VersorOdd;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<Scalar> {
    associatedtype Output = AntiDipoleInversion;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<Sphere> {
    associatedtype Output = VersorEven;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<VersorEven> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Mul<VersorOdd> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiDipoleInversion: Neg {
    associatedtype Output = AntiDipoleInversion;
    func neg() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0)
        );
    }
}
extension AntiDipoleInversion: Not {
    associatedtype Output = DipoleInversion;
    func not() -> Output {
        return this.right_dual();
    }
}
extension AntiDipoleInversion: Sub<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func operator-(other: AntiDipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e4 */
            this.group2 - other.group2, 
            /* e1, e2, e3, e5 */
            this.group3 - other.group3
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator-=(const AntiDipoleInversion& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e4 */
            this.group2 - other.group2, 
            /* e1, e2, e3, e5 */
            this.group3 - other.group3
        );
    }
}
extension AntiDipoleInversion: Sub<AntiDualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<AntiFlatPoint> {
    associatedtype Output = AntiDipoleInversion;
    func operator-(other: AntiFlatPoint) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator-=(const AntiFlatPoint& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion: Sub<AntiFlector> {
    associatedtype Output = AntiDipoleInversion;
    func operator-(other: AntiFlector) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e5 */
            this.group3 - other.group1
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator-=(const AntiFlector& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e5 */
            this.group3 - other.group1
        );
    }
}
extension AntiDipoleInversion: Sub<AntiLine> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<AntiMotor> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<AntiPlane> {
    associatedtype Output = AntiDipoleInversion;
    func operator-(other: AntiPlane) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2, 
            /* e1, e2, e3, e5 */
            this.group3 - other.group0
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator-=(const AntiPlane& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2, 
            /* e1, e2, e3, e5 */
            this.group3 - other.group0
        );
    }
}
extension AntiDipoleInversion: Sub<AntiScalar> {
    associatedtype Output = VersorEven;
    func operator-(other: AntiScalar) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.e235, this.e315, this.e125, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
extension AntiDipoleInversion: Sub<Circle> {
    associatedtype Output = AntiDipoleInversion;
    func operator-(other: Circle) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator-=(const Circle& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion: Sub<CircleRotor> {
    associatedtype Output = VersorEven;
    func operator-(other: CircleRotor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz - other.group0.xyz), other.e12345) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
extension AntiDipoleInversion: Sub<Dipole> {
    associatedtype Output = MultiVector;
    func operator-(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<DipoleInversion> {
    associatedtype Output = MultiVector;
    func operator-(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<DualNum> {
    associatedtype Output = VersorEven;
    func operator-(other: DualNum) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
extension AntiDipoleInversion: Sub<FlatPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<Flector> {
    associatedtype Output = MultiVector;
    func operator-(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<Line> {
    associatedtype Output = AntiDipoleInversion;
    func operator-(other: Line) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator-=(const Line& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2, 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDipoleInversion: Sub<Motor> {
    associatedtype Output = VersorEven;
    func operator-(other: Motor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5) - other.group1, 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
extension AntiDipoleInversion: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e4) - other.group1, 
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
extension AntiDipoleInversion: Sub<Plane> {
    associatedtype Output = MultiVector;
    func operator-(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func operator-(other: RoundPoint) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2 + float4(float3(0.0), other.e4 * -1.0), 
            /* e1, e2, e3, e5 */
            this.group3 - float4(other.group0.xyz, other.e5)
        );
    }
}
extension AntiDipoleInversion {
    associatedtype Output = AntiDipoleInversion;
    func operator-=(const RoundPoint& other) {
        *this = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2 + float4(float3(0.0), other.e4 * -1.0), 
            /* e1, e2, e3, e5 */
            this.group3 - float4(other.group0.xyz, other.e5)
        );
    }
}
extension AntiDipoleInversion: Sub<Scalar> {
    associatedtype Output = MultiVector;
    func operator-(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<Sphere> {
    associatedtype Output = MultiVector;
    func operator-(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
extension AntiDipoleInversion: Sub<VersorEven> {
    associatedtype Output = VersorEven;
    func operator-(other: VersorEven) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz - other.group0.xyz), other.e12345) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5) - other.group2, 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e4) - other.group3
        );
    }
}
extension AntiDipoleInversion: Sub<VersorOdd> {
    associatedtype Output = MultiVector;
    func operator-(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
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
