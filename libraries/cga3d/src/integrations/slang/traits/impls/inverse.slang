extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> AntiCircleRotor {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.inverse();
        return *this;
    }
}
extension AntiCircleRotor: Inverse {
    associatedtype Output = AntiCircleRotor;
    func inverse() -> Output {
        AntiCircleRotor reverse = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) + powi(this.scalar, 2) - powi(this.e45, 2), 0.0, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * reverse.group1, 
            /* e15, e25, e35, scalar */
            float4(other.scalar) * reverse.group2
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> AntiDipoleInversion {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.inverse();
        return *this;
    }
}
extension AntiDipoleInversion: Inverse {
    associatedtype Output = AntiDipoleInversion;
    func inverse() -> Output {
        AntiDipoleInversion reverse = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(this.e321, 2) + powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2) - powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e4 * this.e5), 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * reverse.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group1, 
            /* e235, e315, e125, e4 */
            float4(other.scalar) * reverse.group2, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * reverse.group3
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> AntiDualNum {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.inverse();
        return *this;
    }
}
extension AntiDualNum: Inverse {
    associatedtype Output = AntiDualNum;
    func inverse() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(powi(this.scalar, -2)) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> AntiFlatPoint {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.inverse();
        return *this;
    }
}
extension AntiFlatPoint: Inverse {
    associatedtype Output = AntiFlatPoint;
    func inverse() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(powi(this.e321, -2)) * float4(this.e235 * -1.0, this.e315 * -1.0, this.e125 * -1.0, this.e321 * -1.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> AntiFlector {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.inverse();
        return *this;
    }
}
extension AntiFlector: Inverse {
    associatedtype Output = AntiFlector;
    func inverse() -> Output {
        AntiFlector reverse = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(this.e321, 2) + powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2), 0.0, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * reverse.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * reverse.group1
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> AntiLine {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.inverse();
        return *this;
    }
}
extension AntiLine: Inverse {
    associatedtype Output = AntiLine;
    func inverse() -> Output {
        AntiLine reverse = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2), 0.0, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * reverse.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(other.scalar) * reverse.group1.xyz, 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> AntiMotor {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.inverse();
        return *this;
    }
}
extension AntiMotor: Inverse {
    associatedtype Output = AntiMotor;
    func inverse() -> Output {
        AntiMotor reverse = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) + powi(this.scalar, 2), 0.0, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * reverse.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * reverse.group1
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> AntiPlane {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.inverse();
        return *this;
    }
}
extension AntiPlane: Inverse {
    associatedtype Output = AntiPlane;
    func inverse() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2)) * this.group0
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> AntiScalar {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.inverse();
        return *this;
    }
}
extension AntiScalar: Inverse {
    associatedtype Output = AntiScalar;
    func inverse() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Circle {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Circle& {
        this = this.inverse();
        return *this;
    }
}
extension Circle: Inverse {
    associatedtype Output = Circle;
    func inverse() -> Output {
        Circle reverse = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(this.e321, 2) - powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125), 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * reverse.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * reverse.group2.xyz, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> CircleRotor {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.inverse();
        return *this;
    }
}
extension CircleRotor: Inverse {
    associatedtype Output = CircleRotor;
    func inverse() -> Output {
        CircleRotor reverse = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(this.e321, 2) - powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - powi(this.e12345, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125), 0.0, 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * reverse.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.scalar) * reverse.group2
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Dipole {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Dipole& {
        this = this.inverse();
        return *this;
    }
}
extension Dipole: Inverse {
    associatedtype Output = Dipole;
    func inverse() -> Output {
        Dipole reverse = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) - powi(this.e45, 2), 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * reverse.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(other.scalar) * reverse.group2.xyz, 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> DipoleInversion {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.inverse();
        return *this;
    }
}
extension DipoleInversion: Inverse {
    associatedtype Output = DipoleInversion;
    func inverse() -> Output {
        DipoleInversion reverse = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) - powi(this.e45, 2) - powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2), 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * reverse.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * reverse.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * reverse.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * reverse.group3
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> DualNum {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.inverse();
        return *this;
    }
}
extension DualNum: Inverse {
    associatedtype Output = DualNum;
    func inverse() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(powi(this.e12345, -2) * -1.0) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> FlatPoint {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.inverse();
        return *this;
    }
}
extension FlatPoint: Inverse {
    associatedtype Output = FlatPoint;
    func inverse() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(powi(this.e45, -2) * -1.0) * float4(this.e15 * -1.0, this.e25 * -1.0, this.e35 * -1.0, this.e45 * -1.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Flector {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Flector& {
        this = this.inverse();
        return *this;
    }
}
extension Flector: Inverse {
    associatedtype Output = Flector;
    func inverse() -> Output {
        Flector reverse = Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-powi(this.e45, 2) - powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2), 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * reverse.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * reverse.group1
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Line {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Line& {
        this = this.inverse();
        return *this;
    }
}
extension Line: Inverse {
    associatedtype Output = Line;
    func inverse() -> Output {
        Line reverse = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2), 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(other.scalar) * reverse.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * reverse.group1.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Motor {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Motor& {
        this = this.inverse();
        return *this;
    }
}
extension Motor: Inverse {
    associatedtype Output = Motor;
    func inverse() -> Output {
        Motor reverse = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - powi(this.e12345, 2), 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * reverse.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * reverse.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> MultiVector {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.inverse();
        return *this;
    }
}
extension MultiVector: Inverse {
    associatedtype Output = MultiVector;
    func inverse() -> Output {
        MultiVector reverse = MultiVector.from_groups(
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
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(2.0*(this.e15 * this.e41) + 2.0 * (this.e25 * this.e42) + 2.0 * (this.e35 * this.e43) + 2.0 * (this.e3215 * this.e1234) + powi(this.scalar, 2) + powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) + powi(this.e321, 2) - powi(this.e12345, 2) - powi(this.e45, 2) - powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2) - 2.0 * (this.e4 * this.e5) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125), 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar) * reverse.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group1, 
            /* e5, 0, 0, 0 */
            float4(reverse.e5 * other.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * reverse.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * reverse.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * reverse.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * reverse.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * reverse.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * reverse.group9, 
            /* e1234, 0, 0, 0 */
            float4(reverse.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Plane {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Plane& {
        this = this.inverse();
        return *this;
    }
}
extension Plane: Inverse {
    associatedtype Output = Plane;
    func inverse() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2)) * this.group0
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> RoundPoint {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.inverse();
        return *this;
    }
}
extension RoundPoint: Inverse {
    associatedtype Output = RoundPoint;
    func inverse() -> Output {
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2) - 2.0 * (this.e4 * this.e5), 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Scalar {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.inverse();
        return *this;
    }
}
extension Scalar: Inverse {
    associatedtype Output = Scalar;
    func inverse() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(1.0/this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Sphere {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Sphere& {
        this = this.inverse();
        return *this;
    }
}
extension Sphere: Inverse {
    associatedtype Output = Sphere;
    func inverse() -> Output {
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(2.0*(this.e3215 * this.e1234) - powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2), 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> VersorEven {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.inverse();
        return *this;
    }
}
extension VersorEven: Inverse {
    associatedtype Output = VersorEven;
    func inverse() -> Output {
        VersorEven reverse = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(this.e321, 2) + powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2) - powi(this.e12345, 2) - powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e5 * this.e4), 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * reverse.group0, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * reverse.group2, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group3
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> VersorOdd {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.inverse();
        return *this;
    }
}
extension VersorOdd: Inverse {
    associatedtype Output = VersorOdd;
    func inverse() -> Output {
        VersorOdd reverse = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        Scalar other = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + powi(this.scalar, 2) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) - powi(this.e45, 2) - powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2), 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * reverse.group0, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * reverse.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * reverse.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * reverse.group3
        );
    }
}
