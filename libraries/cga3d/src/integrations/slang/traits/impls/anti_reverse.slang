extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiCircleRotor {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiCircleRotor: AntiReverse {
    associatedtype Output = AntiCircleRotor;
    func anti_reverse() -> Output {
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
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiDipoleInversion {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiDipoleInversion: AntiReverse {
    associatedtype Output = AntiDipoleInversion;
    func anti_reverse() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiDualNum {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiDualNum: AntiReverse {
    associatedtype Output = AntiDualNum;
    func anti_reverse() -> Output {
        return this;
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiFlatPoint {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiFlatPoint: AntiReverse {
    associatedtype Output = AntiFlatPoint;
    func anti_reverse() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiFlector {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiFlector: AntiReverse {
    associatedtype Output = AntiFlector;
    func anti_reverse() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiLine {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiLine: AntiReverse {
    associatedtype Output = AntiLine;
    func anti_reverse() -> Output {
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
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiMotor {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiMotor: AntiReverse {
    associatedtype Output = AntiMotor;
    func anti_reverse() -> Output {
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
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiPlane {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiPlane: AntiReverse {
    associatedtype Output = AntiPlane;
    func anti_reverse() -> Output {
        return this;
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiScalar {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiScalar: AntiReverse {
    associatedtype Output = AntiScalar;
    func anti_reverse() -> Output {
        return this;
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Circle {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Circle& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Circle: AntiReverse {
    associatedtype Output = Circle;
    func anti_reverse() -> Output {
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
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> CircleRotor {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.anti_reverse();
        return *this;
    }
}
extension CircleRotor: AntiReverse {
    associatedtype Output = CircleRotor;
    func anti_reverse() -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Dipole {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Dipole& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Dipole: AntiReverse {
    associatedtype Output = Dipole;
    func anti_reverse() -> Output {
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
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> DipoleInversion {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.anti_reverse();
        return *this;
    }
}
extension DipoleInversion: AntiReverse {
    associatedtype Output = DipoleInversion;
    func anti_reverse() -> Output {
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
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> DualNum {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.anti_reverse();
        return *this;
    }
}
extension DualNum: AntiReverse {
    associatedtype Output = DualNum;
    func anti_reverse() -> Output {
        return this;
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> FlatPoint {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.anti_reverse();
        return *this;
    }
}
extension FlatPoint: AntiReverse {
    associatedtype Output = FlatPoint;
    func anti_reverse() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Flector {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Flector& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Flector: AntiReverse {
    associatedtype Output = Flector;
    func anti_reverse() -> Output {
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
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Line {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Line& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Line: AntiReverse {
    associatedtype Output = Line;
    func anti_reverse() -> Output {
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
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Motor {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Motor& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Motor: AntiReverse {
    associatedtype Output = Motor;
    func anti_reverse() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> MultiVector {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.anti_reverse();
        return *this;
    }
}
extension MultiVector: AntiReverse {
    associatedtype Output = MultiVector;
    func anti_reverse() -> Output {
        return MultiVector.from_groups(
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
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Plane {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Plane& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Plane: AntiReverse {
    associatedtype Output = Plane;
    func anti_reverse() -> Output {
        return this;
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> RoundPoint {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.anti_reverse();
        return *this;
    }
}
extension RoundPoint: AntiReverse {
    associatedtype Output = RoundPoint;
    func anti_reverse() -> Output {
        return this;
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Scalar {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Scalar: AntiReverse {
    associatedtype Output = Scalar;
    func anti_reverse() -> Output {
        return this;
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Sphere {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Sphere& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Sphere: AntiReverse {
    associatedtype Output = Sphere;
    func anti_reverse() -> Output {
        return this;
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> VersorEven {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.anti_reverse();
        return *this;
    }
}
extension VersorEven: AntiReverse {
    associatedtype Output = VersorEven;
    func anti_reverse() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> VersorOdd {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.anti_reverse();
        return *this;
    }
}
extension VersorOdd: AntiReverse {
    associatedtype Output = VersorOdd;
    func anti_reverse() -> Output {
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
