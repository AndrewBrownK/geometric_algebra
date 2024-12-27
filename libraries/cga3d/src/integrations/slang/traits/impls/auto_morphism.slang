extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiCircleRotor {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiCircleRotor: AutoMorphism {
    associatedtype Output = AntiCircleRotor;
    func auto_morphism() -> Output {
        return this;
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiDipoleInversion {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiDipoleInversion: AutoMorphism {
    associatedtype Output = AntiDipoleInversion;
    func auto_morphism() -> Output {
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
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiDualNum {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiDualNum: AutoMorphism {
    associatedtype Output = AntiDualNum;
    func auto_morphism() -> Output {
        return this;
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiFlatPoint {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiFlatPoint: AutoMorphism {
    associatedtype Output = AntiFlatPoint;
    func auto_morphism() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiFlector {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiFlector: AutoMorphism {
    associatedtype Output = AntiFlector;
    func auto_morphism() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiLine {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiLine: AutoMorphism {
    associatedtype Output = AntiLine;
    func auto_morphism() -> Output {
        return this;
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiMotor {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiMotor: AutoMorphism {
    associatedtype Output = AntiMotor;
    func auto_morphism() -> Output {
        return this;
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiPlane {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiPlane: AutoMorphism {
    associatedtype Output = AntiPlane;
    func auto_morphism() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> AntiScalar {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.auto_morphism();
        return *this;
    }
}
extension AntiScalar: AutoMorphism {
    associatedtype Output = AntiScalar;
    func auto_morphism() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Circle {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Circle& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Circle: AutoMorphism {
    associatedtype Output = Circle;
    func auto_morphism() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> CircleRotor {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.auto_morphism();
        return *this;
    }
}
extension CircleRotor: AutoMorphism {
    associatedtype Output = CircleRotor;
    func auto_morphism() -> Output {
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
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Dipole {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Dipole& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Dipole: AutoMorphism {
    associatedtype Output = Dipole;
    func auto_morphism() -> Output {
        return this;
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> DipoleInversion {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.auto_morphism();
        return *this;
    }
}
extension DipoleInversion: AutoMorphism {
    associatedtype Output = DipoleInversion;
    func auto_morphism() -> Output {
        return this;
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> DualNum {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.auto_morphism();
        return *this;
    }
}
extension DualNum: AutoMorphism {
    associatedtype Output = DualNum;
    func auto_morphism() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> FlatPoint {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.auto_morphism();
        return *this;
    }
}
extension FlatPoint: AutoMorphism {
    associatedtype Output = FlatPoint;
    func auto_morphism() -> Output {
        return this;
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Flector {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Flector& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Flector: AutoMorphism {
    associatedtype Output = Flector;
    func auto_morphism() -> Output {
        return this;
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Line {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Line& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Line: AutoMorphism {
    associatedtype Output = Line;
    func auto_morphism() -> Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Motor {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Motor& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Motor: AutoMorphism {
    associatedtype Output = Motor;
    func auto_morphism() -> Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> MultiVector {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.auto_morphism();
        return *this;
    }
}
extension MultiVector: AutoMorphism {
    associatedtype Output = MultiVector;
    func auto_morphism() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy * float2(1.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
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
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Plane {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Plane& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Plane: AutoMorphism {
    associatedtype Output = Plane;
    func auto_morphism() -> Output {
        return this;
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> RoundPoint {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.auto_morphism();
        return *this;
    }
}
extension RoundPoint: AutoMorphism {
    associatedtype Output = RoundPoint;
    func auto_morphism() -> Output {
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
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Scalar {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Scalar: AutoMorphism {
    associatedtype Output = Scalar;
    func auto_morphism() -> Output {
        return this;
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> Sphere {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> Sphere& {
        this = this.auto_morphism();
        return *this;
    }
}
extension Sphere: AutoMorphism {
    associatedtype Output = Sphere;
    func auto_morphism() -> Output {
        return this;
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> VersorEven {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.auto_morphism();
        return *this;
    }
}
extension VersorEven: AutoMorphism {
    associatedtype Output = VersorEven;
    func auto_morphism() -> Output {
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
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: AutoMorphismPrefixOrPostfix) -> VersorOdd {
        return this.auto_morphism();
    }
    func operator/=(const AutoMorphismPrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.auto_morphism();
        return *this;
    }
}
extension VersorOdd: AutoMorphism {
    associatedtype Output = VersorOdd;
    func auto_morphism() -> Output {
        return this;
    }
}
