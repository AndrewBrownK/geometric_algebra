extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiCircleRotor {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.reverse();
        return *this;
    }
}
extension AntiCircleRotor: Reverse {
    associatedtype Output = AntiCircleRotor;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiDipoleInversion {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.reverse();
        return *this;
    }
}
extension AntiDipoleInversion: Reverse {
    associatedtype Output = AntiDipoleInversion;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiDualNum {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.reverse();
        return *this;
    }
}
extension AntiDualNum: Reverse {
    associatedtype Output = AntiDualNum;
    func reverse() -> Output {
        return this;
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiFlatPoint {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.reverse();
        return *this;
    }
}
extension AntiFlatPoint: Reverse {
    associatedtype Output = AntiFlatPoint;
    func reverse() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiFlector {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.reverse();
        return *this;
    }
}
extension AntiFlector: Reverse {
    associatedtype Output = AntiFlector;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiLine {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.reverse();
        return *this;
    }
}
extension AntiLine: Reverse {
    associatedtype Output = AntiLine;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiMotor {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.reverse();
        return *this;
    }
}
extension AntiMotor: Reverse {
    associatedtype Output = AntiMotor;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiPlane {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.reverse();
        return *this;
    }
}
extension AntiPlane: Reverse {
    associatedtype Output = AntiPlane;
    func reverse() -> Output {
        return this;
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiScalar {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.reverse();
        return *this;
    }
}
extension AntiScalar: Reverse {
    associatedtype Output = AntiScalar;
    func reverse() -> Output {
        return this;
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Circle {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Circle& {
        this = this.reverse();
        return *this;
    }
}
extension Circle: Reverse {
    associatedtype Output = Circle;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> CircleRotor {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.reverse();
        return *this;
    }
}
extension CircleRotor: Reverse {
    associatedtype Output = CircleRotor;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> Dipole {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Dipole& {
        this = this.reverse();
        return *this;
    }
}
extension Dipole: Reverse {
    associatedtype Output = Dipole;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> DipoleInversion {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.reverse();
        return *this;
    }
}
extension DipoleInversion: Reverse {
    associatedtype Output = DipoleInversion;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> DualNum {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.reverse();
        return *this;
    }
}
extension DualNum: Reverse {
    associatedtype Output = DualNum;
    func reverse() -> Output {
        return this;
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> FlatPoint {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.reverse();
        return *this;
    }
}
extension FlatPoint: Reverse {
    associatedtype Output = FlatPoint;
    func reverse() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Flector {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Flector& {
        this = this.reverse();
        return *this;
    }
}
extension Flector: Reverse {
    associatedtype Output = Flector;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> Line {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Line& {
        this = this.reverse();
        return *this;
    }
}
extension Line: Reverse {
    associatedtype Output = Line;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> Motor {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Motor& {
        this = this.reverse();
        return *this;
    }
}
extension Motor: Reverse {
    associatedtype Output = Motor;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> MultiVector {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.reverse();
        return *this;
    }
}
extension MultiVector: Reverse {
    associatedtype Output = MultiVector;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> Plane {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Plane& {
        this = this.reverse();
        return *this;
    }
}
extension Plane: Reverse {
    associatedtype Output = Plane;
    func reverse() -> Output {
        return this;
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> RoundPoint {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.reverse();
        return *this;
    }
}
extension RoundPoint: Reverse {
    associatedtype Output = RoundPoint;
    func reverse() -> Output {
        return this;
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Scalar {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.reverse();
        return *this;
    }
}
extension Scalar: Reverse {
    associatedtype Output = Scalar;
    func reverse() -> Output {
        return this;
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Sphere {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Sphere& {
        this = this.reverse();
        return *this;
    }
}
extension Sphere: Reverse {
    associatedtype Output = Sphere;
    func reverse() -> Output {
        return this;
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> VersorEven {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.reverse();
        return *this;
    }
}
extension VersorEven: Reverse {
    associatedtype Output = VersorEven;
    func reverse() -> Output {
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
    func operator/(_rhs: ReversePrefixOrPostfix) -> VersorOdd {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.reverse();
        return *this;
    }
}
extension VersorOdd: Reverse {
    associatedtype Output = VersorOdd;
    func reverse() -> Output {
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
