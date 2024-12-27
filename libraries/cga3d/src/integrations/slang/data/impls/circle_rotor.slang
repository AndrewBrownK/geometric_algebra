using traits::GeometricProduct;
using traits::RightDual;
extension CircleRotor: Add<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Add<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator+(other: AntiDipoleInversion) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz + this.group0.xyz), this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz + this.group2.xyz), other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.e1, other.e2, other.e3, other.e4)
        );
    }
}
extension CircleRotor: Add<AntiDualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
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
extension CircleRotor: Add<AntiFlatPoint> {
    associatedtype Output = CircleRotor;
    func operator+(other: AntiFlatPoint) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e12345 */
            float4((other.group0.xyz + this.group2.xyz), this.e12345)
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator+=(const AntiFlatPoint& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e12345 */
            float4((other.group0.xyz + this.group2.xyz), this.e12345)
        );
    }
}
extension CircleRotor: Add<AntiFlector> {
    associatedtype Output = VersorEven;
    func operator+(other: AntiFlector) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e5 */
            float4((other.group0.xyz + this.group2.xyz), other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, 0.0)
        );
    }
}
extension CircleRotor: Add<AntiLine> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Add<AntiMotor> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Add<AntiPlane> {
    associatedtype Output = VersorEven;
    func operator+(other: AntiPlane) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, 0.0)
        );
    }
}
extension CircleRotor: Add<AntiScalar> {
    associatedtype Output = CircleRotor;
    func operator+(other: AntiScalar) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 + float4(float3(0.0), other.e12345)
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator+=(const AntiScalar& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 + float4(float3(0.0), other.e12345)
        );
    }
}
extension CircleRotor: Add<Circle> {
    associatedtype Output = CircleRotor;
    func operator+(other: Circle) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e12345 */
            float4((other.group2.xyz + this.group2.xyz), this.e12345)
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator+=(const Circle& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e12345 */
            float4((other.group2.xyz + this.group2.xyz), this.e12345)
        );
    }
}
extension CircleRotor: Add<CircleRotor> {
    associatedtype Output = CircleRotor;
    func operator+(other: CircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e12345 */
            other.group2 + this.group2
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator+=(const CircleRotor& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e12345 */
            other.group2 + this.group2
        );
    }
}
extension CircleRotor: Add<Dipole> {
    associatedtype Output = MultiVector;
    func operator+(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Add<DipoleInversion> {
    associatedtype Output = MultiVector;
    func operator+(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Add<DualNum> {
    associatedtype Output = VersorEven;
    func operator+(other: DualNum) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345 + other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension CircleRotor: Add<FlatPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Add<Flector> {
    associatedtype Output = MultiVector;
    func operator+(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Add<Line> {
    associatedtype Output = CircleRotor;
    func operator+(other: Line) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group1.xyz), this.e321), 
            /* e235, e315, e125, e12345 */
            float4((other.group1.xyz + this.group2.xyz), this.e12345)
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator+=(const Line& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group1.xyz), this.e321), 
            /* e235, e315, e125, e12345 */
            float4((other.group1.xyz + this.group2.xyz), this.e12345)
        );
    }
}
extension CircleRotor: Add<Motor> {
    associatedtype Output = VersorEven;
    func operator+(other: Motor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345 + other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz + other.group1.xyz), other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension CircleRotor: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345) + other.group0.xy, 0.0, 0.0), 
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
extension CircleRotor: Add<Plane> {
    associatedtype Output = MultiVector;
    func operator+(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
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
extension CircleRotor: Add<RoundPoint> {
    associatedtype Output = VersorEven;
    func operator+(other: RoundPoint) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            other.group0
        );
    }
}
extension CircleRotor: Add<Scalar> {
    associatedtype Output = MultiVector;
    func operator+(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
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
extension CircleRotor: Add<Sphere> {
    associatedtype Output = MultiVector;
    func operator+(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
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
extension CircleRotor: Add<VersorEven> {
    associatedtype Output = VersorEven;
    func operator+(other: VersorEven) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 + float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + other.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz + other.group2.xyz), other.e5), 
            /* e1, e2, e3, e4 */
            other.group3
        );
    }
}
extension CircleRotor: Add<VersorOdd> {
    associatedtype Output = MultiVector;
    func operator+(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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

impl From<AntiFlatPoint> for CircleRotor {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125, e12345 */
            float4(from_anti_flat_point.group0.xyz, 0.0)
        );
    }
}

impl From<AntiScalar> for CircleRotor {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), from_anti_scalar.e12345)
        );
    }
}

impl From<Circle> for CircleRotor {
    fn from(from_circle: Circle) -> Self {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(from_circle.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            from_circle.group1, 
            /* e235, e315, e125, e12345 */
            float4(from_circle.group2.xyz, 0.0)
        );
    }
}

impl From<Line> for CircleRotor {
    fn from(from_line: Line) -> Self {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e12345 */
            float4(from_line.group1.xyz, 0.0)
        );
    }
}
extension CircleRotor: Mul<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<AntiDualNum> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<AntiFlatPoint> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<AntiFlector> {
    associatedtype Output = VersorOdd;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<AntiLine> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<AntiMotor> {
    associatedtype Output = VersorEven;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<AntiPlane> {
    associatedtype Output = DipoleInversion;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<AntiScalar> {
    associatedtype Output = AntiCircleRotor;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<Circle> {
    associatedtype Output = VersorOdd;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<CircleRotor> {
    associatedtype Output = VersorOdd;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<Dipole> {
    associatedtype Output = VersorEven;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<DipoleInversion> {
    associatedtype Output = VersorEven;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<DualNum> {
    associatedtype Output = VersorOdd;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<FlatPoint> {
    associatedtype Output = VersorEven;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<Flector> {
    associatedtype Output = VersorEven;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<Line> {
    associatedtype Output = VersorOdd;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<Motor> {
    associatedtype Output = VersorOdd;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<Plane> {
    associatedtype Output = AntiDipoleInversion;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<RoundPoint> {
    associatedtype Output = DipoleInversion;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<Scalar> {
    associatedtype Output = CircleRotor;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension CircleRotor: Mul<Sphere> {
    associatedtype Output = AntiDipoleInversion;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<VersorEven> {
    associatedtype Output = VersorOdd;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Mul<VersorOdd> {
    associatedtype Output = VersorEven;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension CircleRotor: Neg {
    associatedtype Output = CircleRotor;
    func neg() -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0)
        );
    }
}
extension CircleRotor: Not {
    associatedtype Output = AntiCircleRotor;
    func not() -> Output {
        return this.right_dual();
    }
}
extension CircleRotor: Sub<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Sub<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func operator-(other: AntiDipoleInversion) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0, 0.0) + float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz - other.group2.xyz), other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e4) * float4(-1.0)
        );
    }
}
extension CircleRotor: Sub<AntiDualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
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
extension CircleRotor: Sub<AntiFlatPoint> {
    associatedtype Output = CircleRotor;
    func operator-(other: AntiFlatPoint) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e12345 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator-=(const AntiFlatPoint& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e12345 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2
        );
    }
}
extension CircleRotor: Sub<AntiFlector> {
    associatedtype Output = VersorEven;
    func operator-(other: AntiFlector) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz - other.group0.xyz), other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension CircleRotor: Sub<AntiLine> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Sub<AntiMotor> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Sub<AntiPlane> {
    associatedtype Output = VersorEven;
    func operator-(other: AntiPlane) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension CircleRotor: Sub<AntiScalar> {
    associatedtype Output = CircleRotor;
    func operator-(other: AntiScalar) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 + float4(float3(0.0), other.e12345 * -1.0)
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator-=(const AntiScalar& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 + float4(float3(0.0), other.e12345 * -1.0)
        );
    }
}
extension CircleRotor: Sub<Circle> {
    associatedtype Output = CircleRotor;
    func operator-(other: Circle) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator-=(const Circle& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2
        );
    }
}
extension CircleRotor: Sub<CircleRotor> {
    associatedtype Output = CircleRotor;
    func operator-(other: CircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 - other.group2
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator-=(const CircleRotor& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 - other.group2
        );
    }
}
extension CircleRotor: Sub<Dipole> {
    associatedtype Output = MultiVector;
    func operator-(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Sub<DipoleInversion> {
    associatedtype Output = MultiVector;
    func operator-(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Sub<DualNum> {
    associatedtype Output = VersorEven;
    func operator-(other: DualNum) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345 - other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension CircleRotor: Sub<FlatPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Sub<Flector> {
    associatedtype Output = MultiVector;
    func operator-(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
extension CircleRotor: Sub<Line> {
    associatedtype Output = CircleRotor;
    func operator-(other: Line) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2
        );
    }
}
extension CircleRotor {
    associatedtype Output = CircleRotor;
    func operator-=(const Line& other) {
        *this = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0, 0.0) + this.group2
        );
    }
}
extension CircleRotor: Sub<Motor> {
    associatedtype Output = VersorEven;
    func operator-(other: Motor) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345 - other.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz - other.group1.xyz), other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension CircleRotor: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345 - other.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
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
extension CircleRotor: Sub<Plane> {
    associatedtype Output = MultiVector;
    func operator-(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
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
extension CircleRotor: Sub<RoundPoint> {
    associatedtype Output = VersorEven;
    func operator-(other: RoundPoint) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0)
        );
    }
}
extension CircleRotor: Sub<Scalar> {
    associatedtype Output = MultiVector;
    func operator-(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
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
extension CircleRotor: Sub<Sphere> {
    associatedtype Output = MultiVector;
    func operator-(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
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
extension CircleRotor: Sub<VersorEven> {
    associatedtype Output = VersorEven;
    func operator-(other: VersorEven) -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345) - other.group0, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz - other.group2.xyz), other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e4 */
            other.group3 * float4(-1.0)
        );
    }
}
extension CircleRotor: Sub<VersorOdd> {
    associatedtype Output = MultiVector;
    func operator-(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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

impl TryFrom<AntiDipoleInversion> for CircleRotor {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(anti_dipole_inversion.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            anti_dipole_inversion.group1, 
            /* e235, e315, e125, e12345 */
            float4(anti_dipole_inversion.group2.xyz, 0.0)
        ));
    }
}

impl TryFrom<AntiFlector> for CircleRotor {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_flector.e321), 
            /* e235, e315, e125, e12345 */
            float4(anti_flector.group0.xyz, 0.0)
        ));
    }
}

impl TryFrom<DualNum> for CircleRotor {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), dual_num.e12345)
        ));
    }
}

impl TryFrom<Motor> for CircleRotor {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(motor.group0.xyz, 0.0), 
            /* e235, e315, e125, e12345 */
            float4(motor.e235, motor.e315, motor.e125, motor.e12345)
        ));
    }
}

impl TryFrom<MultiVector> for CircleRotor {
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
            let mut error = "Elements from MultiVector do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(multi_vector.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125, e12345 */
            float4(multi_vector.group8.xyz, multi_vector.e12345)
        ));
    }
}

impl TryFrom<VersorEven> for CircleRotor {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(versor_even.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            versor_even.group1, 
            /* e235, e315, e125, e12345 */
            float4(versor_even.e235, versor_even.e315, versor_even.e125, versor_even.e12345)
        ));
    }
}
