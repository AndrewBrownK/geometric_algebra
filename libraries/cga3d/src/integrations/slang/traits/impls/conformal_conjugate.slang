extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> AntiCircleRotor {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension AntiCircleRotor: ConformalConjugate {
    associatedtype Output = AntiCircleRotor;
    func conformal_conjugate() -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> AntiDipoleInversion {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension AntiDipoleInversion: ConformalConjugate {
    associatedtype Output = AntiDipoleInversion;
    func conformal_conjugate() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> AntiDualNum {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension AntiDualNum: ConformalConjugate {
    associatedtype Output = AntiDualNum;
    func conformal_conjugate() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(this.group0.xy * float2(-1.0, 1.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> AntiFlatPoint {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension AntiFlatPoint: ConformalConjugate {
    associatedtype Output = AntiFlatPoint;
    func conformal_conjugate() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> AntiFlector {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension AntiFlector: ConformalConjugate {
    associatedtype Output = AntiFlector;
    func conformal_conjugate() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> AntiLine {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension AntiLine: ConformalConjugate {
    associatedtype Output = AntiLine;
    func conformal_conjugate() -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> AntiMotor {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension AntiMotor: ConformalConjugate {
    associatedtype Output = AntiMotor;
    func conformal_conjugate() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0, 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> AntiPlane {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension AntiPlane: ConformalConjugate {
    associatedtype Output = AntiPlane;
    func conformal_conjugate() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> AntiScalar {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension AntiScalar: ConformalConjugate {
    associatedtype Output = AntiScalar;
    func conformal_conjugate() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> Circle {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> Circle& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension Circle: ConformalConjugate {
    associatedtype Output = Circle;
    func conformal_conjugate() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> CircleRotor {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension CircleRotor: ConformalConjugate {
    associatedtype Output = CircleRotor;
    func conformal_conjugate() -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> Dipole {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> Dipole& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension Dipole: ConformalConjugate {
    associatedtype Output = Dipole;
    func conformal_conjugate() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> DipoleInversion {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension DipoleInversion: ConformalConjugate {
    associatedtype Output = DipoleInversion;
    func conformal_conjugate() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> DualNum {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension DualNum: ConformalConjugate {
    associatedtype Output = DualNum;
    func conformal_conjugate() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> FlatPoint {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension FlatPoint: ConformalConjugate {
    associatedtype Output = FlatPoint;
    func conformal_conjugate() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> Flector {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> Flector& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension Flector: ConformalConjugate {
    associatedtype Output = Flector;
    func conformal_conjugate() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> Line {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> Line& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension Line: ConformalConjugate {
    associatedtype Output = Line;
    func conformal_conjugate() -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> Motor {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> Motor& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension Motor: ConformalConjugate {
    associatedtype Output = Motor;
    func conformal_conjugate() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> MultiVector {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension MultiVector: ConformalConjugate {
    associatedtype Output = MultiVector;
    func conformal_conjugate() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> Plane {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> Plane& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension Plane: ConformalConjugate {
    associatedtype Output = Plane;
    func conformal_conjugate() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> RoundPoint {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension RoundPoint: ConformalConjugate {
    associatedtype Output = RoundPoint;
    func conformal_conjugate() -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> Scalar {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension Scalar: ConformalConjugate {
    associatedtype Output = Scalar;
    func conformal_conjugate() -> Output {
        return this;
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> Sphere {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> Sphere& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension Sphere: ConformalConjugate {
    associatedtype Output = Sphere;
    func conformal_conjugate() -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> VersorEven {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension VersorEven: ConformalConjugate {
    associatedtype Output = VersorEven;
    func conformal_conjugate() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: ConformalConjugatePrefixOrPostfix) -> VersorOdd {
        return this.conformal_conjugate();
    }
    func operator/=(const ConformalConjugatePrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.conformal_conjugate();
        return *this;
    }
}
extension VersorOdd: ConformalConjugate {
    associatedtype Output = VersorOdd;
    func conformal_conjugate() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
