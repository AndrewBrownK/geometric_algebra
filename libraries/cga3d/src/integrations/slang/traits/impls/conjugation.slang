extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiCircleRotor {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.conjugation();
        return *this;
    }
}
extension AntiCircleRotor: Conjugation {
    associatedtype Output = AntiCircleRotor;
    func conjugation() -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiDipoleInversion {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.conjugation();
        return *this;
    }
}
extension AntiDipoleInversion: Conjugation {
    associatedtype Output = AntiDipoleInversion;
    func conjugation() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiDualNum {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.conjugation();
        return *this;
    }
}
extension AntiDualNum: Conjugation {
    associatedtype Output = AntiDualNum;
    func conjugation() -> Output {
        return this;
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiFlatPoint {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.conjugation();
        return *this;
    }
}
extension AntiFlatPoint: Conjugation {
    associatedtype Output = AntiFlatPoint;
    func conjugation() -> Output {
        return this;
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiFlector {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.conjugation();
        return *this;
    }
}
extension AntiFlector: Conjugation {
    associatedtype Output = AntiFlector;
    func conjugation() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0, 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiLine {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.conjugation();
        return *this;
    }
}
extension AntiLine: Conjugation {
    associatedtype Output = AntiLine;
    func conjugation() -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiMotor {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.conjugation();
        return *this;
    }
}
extension AntiMotor: Conjugation {
    associatedtype Output = AntiMotor;
    func conjugation() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiPlane {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.conjugation();
        return *this;
    }
}
extension AntiPlane: Conjugation {
    associatedtype Output = AntiPlane;
    func conjugation() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> AntiScalar {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.conjugation();
        return *this;
    }
}
extension AntiScalar: Conjugation {
    associatedtype Output = AntiScalar;
    func conjugation() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Circle {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Circle& {
        this = this.conjugation();
        return *this;
    }
}
extension Circle: Conjugation {
    associatedtype Output = Circle;
    func conjugation() -> Output {
        return this;
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> CircleRotor {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.conjugation();
        return *this;
    }
}
extension CircleRotor: Conjugation {
    associatedtype Output = CircleRotor;
    func conjugation() -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Dipole {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Dipole& {
        this = this.conjugation();
        return *this;
    }
}
extension Dipole: Conjugation {
    associatedtype Output = Dipole;
    func conjugation() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> DipoleInversion {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.conjugation();
        return *this;
    }
}
extension DipoleInversion: Conjugation {
    associatedtype Output = DipoleInversion;
    func conjugation() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> DualNum {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.conjugation();
        return *this;
    }
}
extension DualNum: Conjugation {
    associatedtype Output = DualNum;
    func conjugation() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> FlatPoint {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.conjugation();
        return *this;
    }
}
extension FlatPoint: Conjugation {
    associatedtype Output = FlatPoint;
    func conjugation() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Flector {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Flector& {
        this = this.conjugation();
        return *this;
    }
}
extension Flector: Conjugation {
    associatedtype Output = Flector;
    func conjugation() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Line {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Line& {
        this = this.conjugation();
        return *this;
    }
}
extension Line: Conjugation {
    associatedtype Output = Line;
    func conjugation() -> Output {
        return this;
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Motor {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Motor& {
        this = this.conjugation();
        return *this;
    }
}
extension Motor: Conjugation {
    associatedtype Output = Motor;
    func conjugation() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> MultiVector {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.conjugation();
        return *this;
    }
}
extension MultiVector: Conjugation {
    associatedtype Output = MultiVector;
    func conjugation() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy * float2(1.0, -1.0), 0.0, 0.0), 
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
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Plane {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Plane& {
        this = this.conjugation();
        return *this;
    }
}
extension Plane: Conjugation {
    associatedtype Output = Plane;
    func conjugation() -> Output {
        return this;
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> RoundPoint {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.conjugation();
        return *this;
    }
}
extension RoundPoint: Conjugation {
    associatedtype Output = RoundPoint;
    func conjugation() -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Scalar {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.conjugation();
        return *this;
    }
}
extension Scalar: Conjugation {
    associatedtype Output = Scalar;
    func conjugation() -> Output {
        return this;
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> Sphere {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> Sphere& {
        this = this.conjugation();
        return *this;
    }
}
extension Sphere: Conjugation {
    associatedtype Output = Sphere;
    func conjugation() -> Output {
        return this;
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> VersorEven {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.conjugation();
        return *this;
    }
}
extension VersorEven: Conjugation {
    associatedtype Output = VersorEven;
    func conjugation() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group3 * float4(-1.0)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: ConjugationPrefixOrPostfix) -> VersorOdd {
        return this.conjugation();
    }
    func operator/=(const ConjugationPrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.conjugation();
        return *this;
    }
}
extension VersorOdd: Conjugation {
    associatedtype Output = VersorOdd;
    func conjugation() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
