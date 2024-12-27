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
