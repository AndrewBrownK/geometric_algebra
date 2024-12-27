extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> CircleRotor {
        return this.complement();
    }
}
extension AntiCircleRotor: Complement {
    associatedtype Output = CircleRotor;
    func complement() -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            float4(this.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> DipoleInversion {
        return this.complement();
    }
}
extension AntiDipoleInversion: Complement {
    associatedtype Output = DipoleInversion;
    func complement() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, this.e5) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> VersorEven {
        return this.complement();
    }
}
extension AntiDualNum: Complement {
    associatedtype Output = VersorEven;
    func complement() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.scalar), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> Dipole {
        return this.complement();
    }
}
extension AntiFlatPoint: Complement {
    associatedtype Output = Dipole;
    func complement() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e15, e25, e35, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> DipoleInversion {
        return this.complement();
    }
}
extension AntiFlector: Complement {
    associatedtype Output = DipoleInversion;
    func complement() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> Circle {
        return this.complement();
    }
}
extension AntiLine: Complement {
    associatedtype Output = Circle;
    func complement() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group0.xyz * float3(-1.0)), 0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> VersorEven {
        return this.complement();
    }
}
extension AntiMotor: Complement {
    associatedtype Output = VersorEven;
    func complement() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group1.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4((this.group0.xyz * float3(-1.0)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> Sphere {
        return this.complement();
    }
}
extension AntiPlane: Complement {
    associatedtype Output = Sphere;
    func complement() -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> Scalar {
        return this.complement();
    }
}
extension AntiScalar: Complement {
    associatedtype Output = Scalar;
    func complement() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> Dipole {
        return this.complement();
    }
}
extension Circle: Complement {
    associatedtype Output = Dipole;
    func complement() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> AntiCircleRotor {
        return this.complement();
    }
}
extension CircleRotor: Complement {
    associatedtype Output = AntiCircleRotor;
    func complement() -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            float4(this.group0.xyz, this.e12345) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> Circle {
        return this.complement();
    }
}
extension Dipole: Complement {
    associatedtype Output = Circle;
    func complement() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> AntiDipoleInversion {
        return this.complement();
    }
}
extension DipoleInversion: Complement {
    associatedtype Output = AntiDipoleInversion;
    func complement() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            float4(this.group0.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e4235, this.e4315, this.e4125, this.e1234)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> VersorOdd {
        return this.complement();
    }
}
extension DualNum: Complement {
    associatedtype Output = VersorOdd;
    func complement() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.e12345), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> Circle {
        return this.complement();
    }
}
extension FlatPoint: Complement {
    associatedtype Output = Circle;
    func complement() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e45 * -1.0), 
            /* e235, e315, e125, 0 */
            float4(0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> AntiDipoleInversion {
        return this.complement();
    }
}
extension Flector: Complement {
    associatedtype Output = AntiDipoleInversion;
    func complement() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e45 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), this.e3215), 
            /* e1, e2, e3, e5 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> Dipole {
        return this.complement();
    }
}
extension Line: Complement {
    associatedtype Output = Dipole;
    func complement() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35, 0 */
            float4(0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> VersorOdd {
        return this.complement();
    }
}
extension Motor: Complement {
    associatedtype Output = VersorOdd;
    func complement() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group1.xyz, this.e12345) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> MultiVector {
        return this.complement();
    }
    func operator/=(const ComplementPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.complement();
        return *this;
    }
}
extension MultiVector: Complement {
    associatedtype Output = MultiVector;
    func complement() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy.yx, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group9, 
            /* e5, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group7.xyz, this.e321) * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group8.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group6.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group5.xyz, this.e45) * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group3.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group4.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> RoundPoint {
        return this.complement();
    }
}
extension Plane: Complement {
    associatedtype Output = RoundPoint;
    func complement() -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> Sphere {
        return this.complement();
    }
}
extension RoundPoint: Complement {
    associatedtype Output = Sphere;
    func complement() -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> AntiScalar {
        return this.complement();
    }
}
extension Scalar: Complement {
    associatedtype Output = AntiScalar;
    func complement() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> RoundPoint {
        return this.complement();
    }
}
extension Sphere: Complement {
    associatedtype Output = RoundPoint;
    func complement() -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> VersorOdd {
        return this.complement();
    }
}
extension VersorEven: Complement {
    associatedtype Output = VersorOdd;
    func complement() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(this.group2.xyz, this.e12345) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, this.e5) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: ComplementPrefixOrPostfix) -> VersorEven {
        return this.complement();
    }
}
extension VersorOdd: Complement {
    associatedtype Output = VersorEven;
    func complement() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(this.group2.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
