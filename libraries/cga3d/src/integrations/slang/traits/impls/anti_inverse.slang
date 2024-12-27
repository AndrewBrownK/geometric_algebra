extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> AntiCircleRotor {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.anti_inverse();
        return *this;
    }
}
extension AntiCircleRotor: AntiInverse {
    associatedtype Output = AntiCircleRotor;
    func anti_inverse() -> Output {
        AntiCircleRotor anti_reverse = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(powi(this.e45, 2) - powi(this.e23, 2) - powi(this.e31, 2) - powi(this.e12, 2) - powi(this.scalar, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35), 0.0, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * anti_reverse.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e12345) * anti_reverse.group2
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> AntiDipoleInversion {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.anti_inverse();
        return *this;
    }
}
extension AntiDipoleInversion: AntiInverse {
    associatedtype Output = AntiDipoleInversion;
    func anti_inverse() -> Output {
        AntiDipoleInversion anti_reverse = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e4 * this.e5) + powi(this.e415, 2) + powi(this.e425, 2) + powi(this.e435, 2) - powi(this.e321, 2) - powi(this.e1, 2) - powi(this.e2, 2) - powi(this.e3, 2), 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * anti_reverse.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * anti_reverse.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e12345) * anti_reverse.group2, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * anti_reverse.group3
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> AntiDualNum {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.anti_inverse();
        return *this;
    }
}
extension AntiDualNum: AntiInverse {
    associatedtype Output = AntiDualNum;
    func anti_inverse() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(powi(this.scalar, -2) * -1.0) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> AntiFlatPoint {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.anti_inverse();
        return *this;
    }
}
extension AntiFlatPoint: AntiInverse {
    associatedtype Output = AntiFlatPoint;
    func anti_inverse() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(powi(this.e321, -2) * -1.0) * float4(this.e235 * -1.0, this.e315 * -1.0, this.e125 * -1.0, this.e321 * -1.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> AntiFlector {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.anti_inverse();
        return *this;
    }
}
extension AntiFlector: AntiInverse {
    associatedtype Output = AntiFlector;
    func anti_inverse() -> Output {
        AntiFlector anti_reverse = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-powi(this.e321, 2) - powi(this.e1, 2) - powi(this.e2, 2) - powi(this.e3, 2), 0.0, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * anti_reverse.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * anti_reverse.group1
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> AntiLine {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.anti_inverse();
        return *this;
    }
}
extension AntiLine: AntiInverse {
    associatedtype Output = AntiLine;
    func anti_inverse() -> Output {
        AntiLine anti_reverse = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-powi(this.e23, 2) - powi(this.e31, 2) - powi(this.e12, 2), 0.0, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(other.e12345) * anti_reverse.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(other.e12345) * anti_reverse.group1.xyz, 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> AntiMotor {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.anti_inverse();
        return *this;
    }
}
extension AntiMotor: AntiInverse {
    associatedtype Output = AntiMotor;
    func anti_inverse() -> Output {
        AntiMotor anti_reverse = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-powi(this.e23, 2) - powi(this.e31, 2) - powi(this.e12, 2) - powi(this.scalar, 2), 0.0, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * anti_reverse.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * anti_reverse.group1
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> AntiPlane {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.anti_inverse();
        return *this;
    }
}
extension AntiPlane: AntiInverse {
    associatedtype Output = AntiPlane;
    func anti_inverse() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-powi(this.e1, 2) - powi(this.e2, 2) - powi(this.e3, 2)) * this.group0
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> AntiScalar {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_inverse();
        return *this;
    }
}
extension AntiScalar: AntiInverse {
    associatedtype Output = AntiScalar;
    func anti_inverse() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> Circle {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> Circle& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Circle: AntiInverse {
    associatedtype Output = Circle;
    func anti_inverse() -> Output {
        Circle anti_reverse = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + powi(this.e415, 2) + powi(this.e425, 2) + powi(this.e435, 2) - powi(this.e321, 2), 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * anti_reverse.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * anti_reverse.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * anti_reverse.group2.xyz, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> CircleRotor {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.anti_inverse();
        return *this;
    }
}
extension CircleRotor: AntiInverse {
    associatedtype Output = CircleRotor;
    func anti_inverse() -> Output {
        CircleRotor anti_reverse = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + powi(this.e415, 2) + powi(this.e425, 2) + powi(this.e435, 2) + powi(this.e12345, 2) - powi(this.e321, 2), 0.0, 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * anti_reverse.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * anti_reverse.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e12345) * anti_reverse.group2
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> Dipole {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> Dipole& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Dipole: AntiInverse {
    associatedtype Output = Dipole;
    func anti_inverse() -> Output {
        Dipole anti_reverse = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(powi(this.e45, 2) - powi(this.e23, 2) - powi(this.e31, 2) - powi(this.e12, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35), 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * anti_reverse.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(other.e12345) * anti_reverse.group2.xyz, 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> DipoleInversion {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.anti_inverse();
        return *this;
    }
}
extension DipoleInversion: AntiInverse {
    associatedtype Output = DipoleInversion;
    func anti_inverse() -> Output {
        DipoleInversion anti_reverse = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(powi(this.e45, 2) + powi(this.e4235, 2) + powi(this.e4315, 2) + powi(this.e4125, 2) - powi(this.e23, 2) - powi(this.e31, 2) - powi(this.e12, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * anti_reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * anti_reverse.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * anti_reverse.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * anti_reverse.group3
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> DualNum {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.anti_inverse();
        return *this;
    }
}
extension DualNum: AntiInverse {
    associatedtype Output = DualNum;
    func anti_inverse() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(powi(this.e12345, -2)) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> FlatPoint {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.anti_inverse();
        return *this;
    }
}
extension FlatPoint: AntiInverse {
    associatedtype Output = FlatPoint;
    func anti_inverse() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(powi(this.e45, -2)) * float4(this.e15 * -1.0, this.e25 * -1.0, this.e35 * -1.0, this.e45 * -1.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> Flector {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> Flector& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Flector: AntiInverse {
    associatedtype Output = Flector;
    func anti_inverse() -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(powi(this.e45, 2) + powi(this.e4235, 2) + powi(this.e4315, 2) + powi(this.e4125, 2), 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * anti_reverse.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * anti_reverse.group1
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> Line {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> Line& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Line: AntiInverse {
    associatedtype Output = Line;
    func anti_inverse() -> Output {
        Line anti_reverse = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(powi(this.e415, 2) + powi(this.e425, 2) + powi(this.e435, 2), 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(other.e12345) * anti_reverse.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * anti_reverse.group1.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> Motor {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> Motor& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Motor: AntiInverse {
    associatedtype Output = Motor;
    func anti_inverse() -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(powi(this.e415, 2) + powi(this.e425, 2) + powi(this.e435, 2) + powi(this.e12345, 2), 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * anti_reverse.group0, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * anti_reverse.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> MultiVector {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.anti_inverse();
        return *this;
    }
}
extension MultiVector: AntiInverse {
    associatedtype Output = MultiVector;
    func anti_inverse() -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
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
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(2.0*(this.e4 * this.e5) + 2.0 * (this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + powi(this.e12345, 2) + powi(this.e45, 2) + powi(this.e415, 2) + powi(this.e425, 2) + powi(this.e435, 2) + powi(this.e4235, 2) + powi(this.e4315, 2) + powi(this.e4125, 2) - powi(this.scalar, 2) - powi(this.e1, 2) - powi(this.e2, 2) - powi(this.e3, 2) - powi(this.e23, 2) - powi(this.e31, 2) - powi(this.e12, 2) - powi(this.e321, 2) - 2.0 * (this.e15 * this.e41) - 2.0 * (this.e25 * this.e42) - 2.0 * (this.e35 * this.e43) - 2.0 * (this.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.e12345) * anti_reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * anti_reverse.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e12345 * anti_reverse.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * anti_reverse.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * anti_reverse.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e12345) * anti_reverse.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * anti_reverse.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * anti_reverse.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * anti_reverse.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * anti_reverse.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e12345 * anti_reverse.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> Plane {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> Plane& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Plane: AntiInverse {
    associatedtype Output = Plane;
    func anti_inverse() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(powi(this.e4235, 2) + powi(this.e4315, 2) + powi(this.e4125, 2)) * this.group0
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> RoundPoint {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.anti_inverse();
        return *this;
    }
}
extension RoundPoint: AntiInverse {
    associatedtype Output = RoundPoint;
    func anti_inverse() -> Output {
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(2.0*(this.e4 * this.e5) - powi(this.e1, 2) - powi(this.e2, 2) - powi(this.e3, 2), 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> Scalar {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Scalar: AntiInverse {
    associatedtype Output = Scalar;
    func anti_inverse() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(1.0/this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> Sphere {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> Sphere& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Sphere: AntiInverse {
    associatedtype Output = Sphere;
    func anti_inverse() -> Output {
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(powi(this.e4235, 2) + powi(this.e4315, 2) + powi(this.e4125, 2) - 2.0 * (this.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> VersorEven {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.anti_inverse();
        return *this;
    }
}
extension VersorEven: AntiInverse {
    associatedtype Output = VersorEven;
    func anti_inverse() -> Output {
        VersorEven anti_reverse = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e5 * this.e4) + powi(this.e12345, 2) + powi(this.e415, 2) + powi(this.e425, 2) + powi(this.e435, 2) - powi(this.e321, 2) - powi(this.e1, 2) - powi(this.e2, 2) - powi(this.e3, 2), 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * anti_reverse.group0, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * anti_reverse.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * anti_reverse.group2, 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * anti_reverse.group3
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: AntiInversePrefixOrPostfix) -> VersorOdd {
        return this.anti_inverse();
    }
    func operator/=(const AntiInversePrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.anti_inverse();
        return *this;
    }
}
extension VersorOdd: AntiInverse {
    associatedtype Output = VersorOdd;
    func anti_inverse() -> Output {
        VersorOdd anti_reverse = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(powi(this.e45, 2) + powi(this.e4235, 2) + powi(this.e4315, 2) + powi(this.e4125, 2) - powi(this.scalar, 2) - powi(this.e23, 2) - powi(this.e31, 2) - powi(this.e12, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.e12345) * anti_reverse.group0, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * anti_reverse.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * anti_reverse.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * anti_reverse.group3
        );
    }
}
