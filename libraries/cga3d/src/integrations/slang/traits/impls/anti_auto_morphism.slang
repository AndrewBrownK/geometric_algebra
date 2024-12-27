extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiCircleRotor {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiCircleRotor: AntiAutoMorphism {
    associatedtype Output = AntiCircleRotor;
    func anti_auto_morphism() -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiDipoleInversion {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiDipoleInversion: AntiAutoMorphism {
    associatedtype Output = AntiDipoleInversion;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiDualNum {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiDualNum: AntiAutoMorphism {
    associatedtype Output = AntiDualNum;
    func anti_auto_morphism() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiFlatPoint {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiFlatPoint: AntiAutoMorphism {
    associatedtype Output = AntiFlatPoint;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiFlector {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiFlector: AntiAutoMorphism {
    associatedtype Output = AntiFlector;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiLine {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiLine: AntiAutoMorphism {
    associatedtype Output = AntiLine;
    func anti_auto_morphism() -> Output {
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
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiMotor {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiMotor: AntiAutoMorphism {
    associatedtype Output = AntiMotor;
    func anti_auto_morphism() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiPlane {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiPlane: AntiAutoMorphism {
    associatedtype Output = AntiPlane;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> AntiScalar {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension AntiScalar: AntiAutoMorphism {
    associatedtype Output = AntiScalar;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Circle {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Circle& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Circle: AntiAutoMorphism {
    associatedtype Output = Circle;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> CircleRotor {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension CircleRotor: AntiAutoMorphism {
    associatedtype Output = CircleRotor;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Dipole {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Dipole& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Dipole: AntiAutoMorphism {
    associatedtype Output = Dipole;
    func anti_auto_morphism() -> Output {
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
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> DipoleInversion {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension DipoleInversion: AntiAutoMorphism {
    associatedtype Output = DipoleInversion;
    func anti_auto_morphism() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> DualNum {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension DualNum: AntiAutoMorphism {
    associatedtype Output = DualNum;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> FlatPoint {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension FlatPoint: AntiAutoMorphism {
    associatedtype Output = FlatPoint;
    func anti_auto_morphism() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Flector {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Flector& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Flector: AntiAutoMorphism {
    associatedtype Output = Flector;
    func anti_auto_morphism() -> Output {
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
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Line {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Line& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Line: AntiAutoMorphism {
    associatedtype Output = Line;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Motor {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Motor& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Motor: AntiAutoMorphism {
    associatedtype Output = Motor;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> MultiVector {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension MultiVector: AntiAutoMorphism {
    associatedtype Output = MultiVector;
    func anti_auto_morphism() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy * float2(-1.0, 1.0), 0.0, 0.0), 
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
            this.group6, 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Plane {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Plane& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Plane: AntiAutoMorphism {
    associatedtype Output = Plane;
    func anti_auto_morphism() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> RoundPoint {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension RoundPoint: AntiAutoMorphism {
    associatedtype Output = RoundPoint;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Scalar {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Scalar: AntiAutoMorphism {
    associatedtype Output = Scalar;
    func anti_auto_morphism() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> Sphere {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> Sphere& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension Sphere: AntiAutoMorphism {
    associatedtype Output = Sphere;
    func anti_auto_morphism() -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> VersorEven {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension VersorEven: AntiAutoMorphism {
    associatedtype Output = VersorEven;
    func anti_auto_morphism() -> Output {
        return this;
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: AntiAutoMorphismPrefixOrPostfix) -> VersorOdd {
        return this.anti_auto_morphism();
    }
    func operator/=(const AntiAutoMorphismPrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.anti_auto_morphism();
        return *this;
    }
}
extension VersorOdd: AntiAutoMorphism {
    associatedtype Output = VersorOdd;
    func anti_auto_morphism() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
