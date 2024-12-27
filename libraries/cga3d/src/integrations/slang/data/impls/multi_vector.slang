using traits::GeometricProduct;
using traits::RightDual;
extension MultiVector: Add<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiCircleRotor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiDipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + float4(other.group3.xyz, other.e4), 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiDipoleInversion& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + float4(other.group3.xyz, other.e4), 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<AntiDualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, other.e3215 + this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiDualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, other.e3215 + this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiFlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, other.e321 + this.e321), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiFlatPoint& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, other.e321 + this.e321), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<AntiFlector> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiFlector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz + this.group1.xyz), this.e4), 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, other.e321 + this.e321), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiFlector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz + this.group1.xyz), this.e4), 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, other.e321 + this.e321), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<AntiLine> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz + this.group3.xyz), this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz + this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiLine& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz + this.group3.xyz), this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz + this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<AntiMotor> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz + this.group3.xyz), this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, other.e3215 + this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiMotor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz + this.group3.xyz), this.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, other.e3215 + this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<AntiPlane> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiPlane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz + this.group1.xyz), this.e4), 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiPlane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz + this.group1.xyz), this.e4), 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const AntiScalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<Circle> {
    associatedtype Output = MultiVector;
    func operator+(other: Circle) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz + this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Circle& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz + this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<CircleRotor> {
    associatedtype Output = MultiVector;
    func operator+(other: CircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const CircleRotor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz + this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<Dipole> {
    associatedtype Output = MultiVector;
    func operator+(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Dipole& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<DipoleInversion> {
    associatedtype Output = MultiVector;
    func operator+(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 + this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const DipoleInversion& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz + this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 + this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<DualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const DualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<FlatPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group0 + this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const FlatPoint& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group0 + this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<Flector> {
    associatedtype Output = MultiVector;
    func operator+(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group0 + this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 + this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Flector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group0 + this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 + this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<Line> {
    associatedtype Output = MultiVector;
    func operator+(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group6.xyz), this.e321), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz + this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Line& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group6.xyz), this.e321), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz + this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<Motor> {
    associatedtype Output = MultiVector;
    func operator+(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group6.xyz), this.e321), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Motor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group6.xyz), this.e321), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group3 + this.group3, 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz + this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz + this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group6 + this.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group7.xyz + this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group8.xyz + this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group9 + this.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 + this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const MultiVector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5 + this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            other.group3 + this.group3, 
            /* e41, e42, e43, 0 */
            float4(other.group4.xyz + this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group5.xyz + this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            other.group6 + this.group6, 
            /* e423, e431, e412, 0 */
            float4(other.group7.xyz + this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group8.xyz + this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group9 + this.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 + this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<Plane> {
    associatedtype Output = MultiVector;
    func operator+(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Plane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<RoundPoint> {
    associatedtype Output = MultiVector;
    func operator+(other: RoundPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 + other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const RoundPoint& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 + other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<Scalar> {
    associatedtype Output = MultiVector;
    func operator+(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Scalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<Sphere> {
    associatedtype Output = MultiVector;
    func operator+(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 + other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const Sphere& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 + other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<VersorEven> {
    associatedtype Output = MultiVector;
    func operator+(other: VersorEven) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + other.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5 + other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 + other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz + other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const VersorEven& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 + other.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5 + other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 + other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz + other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz + other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Add<VersorOdd> {
    associatedtype Output = MultiVector;
    func operator+(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz + other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + other.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 + other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator+=(const VersorOdd& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz + other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz + other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + other.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 + other.e1234, 0.0, 0.0, 0.0)
        );
    }
}

impl From<AntiCircleRotor> for MultiVector {
    fn from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_anti_circle_rotor.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_anti_circle_rotor.e15, from_anti_circle_rotor.e25, from_anti_circle_rotor.e35, from_anti_circle_rotor.e45), 
            /* e41, e42, e43, 0 */
            float4(from_anti_circle_rotor.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_anti_circle_rotor.group1.xyz, 0.0), 
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

impl From<AntiDipoleInversion> for MultiVector {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(from_anti_dipole_inversion.e1, from_anti_dipole_inversion.e2, from_anti_dipole_inversion.e3, from_anti_dipole_inversion.e4), 
            /* e5, 0, 0, 0 */
            float4(from_anti_dipole_inversion.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            from_anti_dipole_inversion.group1, 
            /* e423, e431, e412, 0 */
            float4(from_anti_dipole_inversion.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(from_anti_dipole_inversion.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<AntiDualNum> for MultiVector {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_anti_dual_num.scalar, 0.0), 0.0, 0.0), 
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
            float4(float3(0.0), from_anti_dual_num.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<AntiFlatPoint> for MultiVector {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
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
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(from_anti_flat_point.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<AntiFlector> for MultiVector {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(from_anti_flector.group1.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(from_anti_flector.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flector.e321), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(from_anti_flector.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<AntiLine> for MultiVector {
    fn from(from_anti_line: AntiLine) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_anti_line.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(from_anti_line.group0.xyz, 0.0), 
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

impl From<AntiMotor> for MultiVector {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_anti_motor.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_anti_motor.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(from_anti_motor.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), from_anti_motor.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<AntiPlane> for MultiVector {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(from_anti_plane.group0.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(from_anti_plane.e5, 0.0, 0.0, 0.0), 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_anti_scalar.e12345), 0.0, 0.0), 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<Circle> for MultiVector {
    fn from(from_circle: Circle) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
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
            from_circle.group1, 
            /* e423, e431, e412, 0 */
            float4(from_circle.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(from_circle.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<CircleRotor> for MultiVector {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_circle_rotor.e12345), 0.0, 0.0), 
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
            from_circle_rotor.group1, 
            /* e423, e431, e412, 0 */
            float4(from_circle_rotor.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(from_circle_rotor.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<Dipole> for MultiVector {
    fn from(from_dipole: Dipole) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_dipole.group2.xyz, from_dipole.e45), 
            /* e41, e42, e43, 0 */
            float4(from_dipole.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_dipole.group1.xyz, 0.0), 
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

impl From<DipoleInversion> for MultiVector {
    fn from(from_dipole_inversion: DipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_dipole_inversion.e15, from_dipole_inversion.e25, from_dipole_inversion.e35, from_dipole_inversion.e45), 
            /* e41, e42, e43, 0 */
            float4(from_dipole_inversion.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_dipole_inversion.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_dipole_inversion.group3, 
            /* e1234, 0, 0, 0 */
            float4(from_dipole_inversion.e1234, 0.0, 0.0, 0.0)
        );
    }
}

impl From<DualNum> for MultiVector {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_dual_num.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(from_dual_num.e5, 0.0, 0.0, 0.0), 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<FlatPoint> for MultiVector {
    fn from(from_flat_point: FlatPoint) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            from_flat_point.group0, 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<Flector> for MultiVector {
    fn from(from_flector: Flector) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            from_flector.group0, 
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
            from_flector.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<Line> for MultiVector {
    fn from(from_line: Line) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
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
            float4(from_line.group0.xyz, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(from_line.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<Motor> for MultiVector {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_motor.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(from_motor.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_motor.group0.xyz, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(from_motor.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<Plane> for MultiVector {
    fn from(from_plane: Plane) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
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
            from_plane.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<RoundPoint> for MultiVector {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            from_round_point.group0, 
            /* e5, 0, 0, 0 */
            float4(from_round_point.e5, 0.0, 0.0, 0.0), 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<Scalar> for MultiVector {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_scalar.scalar, 0.0), 0.0, 0.0), 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<Sphere> for MultiVector {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
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
            from_sphere.group0, 
            /* e1234, 0, 0, 0 */
            float4(from_sphere.e1234, 0.0, 0.0, 0.0)
        );
    }
}

impl From<VersorEven> for MultiVector {
    fn from(from_versor_even: VersorEven) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_versor_even.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            from_versor_even.group3, 
            /* e5, 0, 0, 0 */
            float4(from_versor_even.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            from_versor_even.group1, 
            /* e423, e431, e412, 0 */
            float4(from_versor_even.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(from_versor_even.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}

impl From<VersorOdd> for MultiVector {
    fn from(from_versor_odd: VersorOdd) -> Self {
        use crate::elements::*;
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_versor_odd.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_versor_odd.e15, from_versor_odd.e25, from_versor_odd.e35, from_versor_odd.e45), 
            /* e41, e42, e43, 0 */
            float4(from_versor_odd.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_versor_odd.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_versor_odd.group3, 
            /* e1234, 0, 0, 0 */
            float4(from_versor_odd.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Mul<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiCircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiCircleRotor& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiDipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiDipoleInversion& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<AntiDualNum> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiDualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiDualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiFlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiFlatPoint& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<AntiFlector> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiFlector) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiFlector& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<AntiLine> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiLine) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiLine& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<AntiMotor> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiMotor) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiMotor& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<AntiPlane> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiPlane) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiPlane& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Circle> {
    associatedtype Output = MultiVector;
    func operator*(other: Circle) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Circle& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<CircleRotor> {
    associatedtype Output = MultiVector;
    func operator*(other: CircleRotor) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const CircleRotor& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Dipole> {
    associatedtype Output = MultiVector;
    func operator*(other: Dipole) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Dipole& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<DipoleInversion> {
    associatedtype Output = MultiVector;
    func operator*(other: DipoleInversion) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const DipoleInversion& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<DualNum> {
    associatedtype Output = MultiVector;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<FlatPoint> {
    associatedtype Output = MultiVector;
    func operator*(other: FlatPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const FlatPoint& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Flector> {
    associatedtype Output = MultiVector;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Flector& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Line> {
    associatedtype Output = MultiVector;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Line& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Motor> {
    associatedtype Output = MultiVector;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Motor& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const MultiVector& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Plane> {
    associatedtype Output = MultiVector;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Plane& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<RoundPoint> {
    associatedtype Output = MultiVector;
    func operator*(other: RoundPoint) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const RoundPoint& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Scalar> {
    associatedtype Output = MultiVector;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<Sphere> {
    associatedtype Output = MultiVector;
    func operator*(other: Sphere) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const Sphere& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<VersorEven> {
    associatedtype Output = MultiVector;
    func operator*(other: VersorEven) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const VersorEven& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Mul<VersorOdd> {
    associatedtype Output = MultiVector;
    func operator*(other: VersorOdd) -> Output {
        return this.geometric_product(other);
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator*=(const VersorOdd& other) {
        *this = this.geometric_product(other);
    }
}
extension MultiVector: Neg {
    associatedtype Output = MultiVector;
    func neg() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Not {
    associatedtype Output = MultiVector;
    func not() -> Output {
        return this.right_dual();
    }
}
extension MultiVector: Sub<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiCircleRotor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiDipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - float4(other.group3.xyz, other.e4), 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiDipoleInversion& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - float4(other.group3.xyz, other.e4), 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<AntiDualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiDualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiFlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 + float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiFlatPoint& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 + float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<AntiFlector> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiFlector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, 0.0) + this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 + float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiFlector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, 0.0) + this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 + float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<AntiLine> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiLine& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<AntiMotor> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiMotor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, 0.0) + this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<AntiPlane> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiPlane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, 0.0) + this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiPlane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1 * -1.0, other.e2 * -1.0, other.e3 * -1.0, 0.0) + this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const AntiScalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<Circle> {
    associatedtype Output = MultiVector;
    func operator-(other: Circle) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Circle& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<CircleRotor> {
    associatedtype Output = MultiVector;
    func operator-(other: CircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const CircleRotor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<Dipole> {
    associatedtype Output = MultiVector;
    func operator-(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Dipole& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<DipoleInversion> {
    associatedtype Output = MultiVector;
    func operator-(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const DipoleInversion& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<DualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const DualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<FlatPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const FlatPoint& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<Flector> {
    associatedtype Output = MultiVector;
    func operator-(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group1, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Flector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group1, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<Line> {
    associatedtype Output = MultiVector;
    func operator-(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Line& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<Motor> {
    associatedtype Output = MultiVector;
    func operator-(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Motor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e415 * -1.0, other.e425 * -1.0, other.e435 * -1.0, 0.0) + this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - other.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const MultiVector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy - other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - other.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<Plane> {
    associatedtype Output = MultiVector;
    func operator-(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Plane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<RoundPoint> {
    associatedtype Output = MultiVector;
    func operator-(other: RoundPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const RoundPoint& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<Scalar> {
    associatedtype Output = MultiVector;
    func operator-(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Scalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<Sphere> {
    associatedtype Output = MultiVector;
    func operator-(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const Sphere& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<VersorEven> {
    associatedtype Output = MultiVector;
    func operator-(other: VersorEven) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const VersorEven& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, other.e12345 * -1.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 - other.group3, 
            /* e5, 0, 0, 0 */
            float4(this.e5 - other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz - other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz - other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Sub<VersorOdd> {
    associatedtype Output = MultiVector;
    func operator-(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    associatedtype Output = MultiVector;
    func operator-=(const VersorOdd& other) {
        *this = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * -1.0, 0.0) + this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 - float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz - other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz - other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
