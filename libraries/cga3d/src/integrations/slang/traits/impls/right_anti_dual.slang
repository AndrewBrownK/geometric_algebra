extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> CircleRotor {
        return this.right_anti_dual();
    }
}
extension AntiCircleRotor: RightAntiDual {
    associatedtype Output = CircleRotor;
    func right_anti_dual() -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> DipoleInversion {
        return this.right_anti_dual();
    }
}
extension AntiDipoleInversion: RightAntiDual {
    associatedtype Output = DipoleInversion;
    func right_anti_dual() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> DualNum {
        return this.right_anti_dual();
    }
}
extension AntiDualNum: RightAntiDual {
    associatedtype Output = DualNum;
    func right_anti_dual() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> FlatPoint {
        return this.right_anti_dual();
    }
}
extension AntiFlatPoint: RightAntiDual {
    associatedtype Output = FlatPoint;
    func right_anti_dual() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> Flector {
        return this.right_anti_dual();
    }
}
extension AntiFlector: RightAntiDual {
    associatedtype Output = Flector;
    func right_anti_dual() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> Line {
        return this.right_anti_dual();
    }
}
extension AntiLine: RightAntiDual {
    associatedtype Output = Line;
    func right_anti_dual() -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> Motor {
        return this.right_anti_dual();
    }
}
extension AntiMotor: RightAntiDual {
    associatedtype Output = Motor;
    func right_anti_dual() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> Plane {
        return this.right_anti_dual();
    }
}
extension AntiPlane: RightAntiDual {
    associatedtype Output = Plane;
    func right_anti_dual() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> Scalar {
        return this.right_anti_dual();
    }
}
extension AntiScalar: RightAntiDual {
    associatedtype Output = Scalar;
    func right_anti_dual() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> Dipole {
        return this.right_anti_dual();
    }
}
extension Circle: RightAntiDual {
    associatedtype Output = Dipole;
    func right_anti_dual() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> AntiCircleRotor {
        return this.right_anti_dual();
    }
}
extension CircleRotor: RightAntiDual {
    associatedtype Output = AntiCircleRotor;
    func right_anti_dual() -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> Circle {
        return this.right_anti_dual();
    }
}
extension Dipole: RightAntiDual {
    associatedtype Output = Circle;
    func right_anti_dual() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> AntiDipoleInversion {
        return this.right_anti_dual();
    }
}
extension DipoleInversion: RightAntiDual {
    associatedtype Output = AntiDipoleInversion;
    func right_anti_dual() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> AntiDualNum {
        return this.right_anti_dual();
    }
}
extension DualNum: RightAntiDual {
    associatedtype Output = AntiDualNum;
    func right_anti_dual() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> AntiFlatPoint {
        return this.right_anti_dual();
    }
}
extension FlatPoint: RightAntiDual {
    associatedtype Output = AntiFlatPoint;
    func right_anti_dual() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> AntiFlector {
        return this.right_anti_dual();
    }
}
extension Flector: RightAntiDual {
    associatedtype Output = AntiFlector;
    func right_anti_dual() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> AntiLine {
        return this.right_anti_dual();
    }
}
extension Line: RightAntiDual {
    associatedtype Output = AntiLine;
    func right_anti_dual() -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> AntiMotor {
        return this.right_anti_dual();
    }
}
extension Motor: RightAntiDual {
    associatedtype Output = AntiMotor;
    func right_anti_dual() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> MultiVector {
        return this.right_anti_dual();
    }
    func operator/=(const RightAntiDualPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.right_anti_dual();
        return *this;
    }
}
extension MultiVector: RightAntiDual {
    associatedtype Output = MultiVector;
    func right_anti_dual() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group9.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(this.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group8.xyz, this.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group5.xyz, this.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> AntiPlane {
        return this.right_anti_dual();
    }
}
extension Plane: RightAntiDual {
    associatedtype Output = AntiPlane;
    func right_anti_dual() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> Sphere {
        return this.right_anti_dual();
    }
}
extension RoundPoint: RightAntiDual {
    associatedtype Output = Sphere;
    func right_anti_dual() -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> AntiScalar {
        return this.right_anti_dual();
    }
}
extension Scalar: RightAntiDual {
    associatedtype Output = AntiScalar;
    func right_anti_dual() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> RoundPoint {
        return this.right_anti_dual();
    }
}
extension Sphere: RightAntiDual {
    associatedtype Output = RoundPoint;
    func right_anti_dual() -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(this.e3215, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> VersorOdd {
        return this.right_anti_dual();
    }
}
extension VersorEven: RightAntiDual {
    associatedtype Output = VersorOdd;
    func right_anti_dual() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, this.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: RightAntiDualPrefixOrPostfix) -> VersorEven {
        return this.right_anti_dual();
    }
}
extension VersorOdd: RightAntiDual {
    associatedtype Output = VersorEven;
    func right_anti_dual() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
