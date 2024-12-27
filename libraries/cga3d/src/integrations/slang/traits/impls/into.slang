extension MultiVector: From<AntiCircleRotor> {
    associatedtype Self = MultiVector:
    static func from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_anti_circle_rotor.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_anti_circle_rotor.e15, from_anti_circle_rotor.e25, from_anti_circle_rotor.e35, from_anti_circle_rotor.e45), 
            /* e41, e42, e43, 0 */
            float4(from_anti_circle_rotor.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_anti_circle_rotor.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiCircleRotor: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<AntiCircleRotor> {
    associatedtype Self = VersorOdd:
    static func from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(from_anti_circle_rotor.group0.xyz, from_anti_circle_rotor.scalar), 
            /* e23, e31, e12, e45 */
            from_anti_circle_rotor.group1, 
            /* e15, e25, e35, e1234 */
            float4(from_anti_circle_rotor.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension AntiCircleRotor: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension MultiVector: From<AntiDipoleInversion> {
    associatedtype Self = MultiVector:
    static func from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(from_anti_dipole_inversion.e1, from_anti_dipole_inversion.e2, from_anti_dipole_inversion.e3, from_anti_dipole_inversion.e4), 
            /* e5, 0, 0, 0 */
            float4(from_anti_dipole_inversion.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            from_anti_dipole_inversion.group1, 
            /* e423, e431, e412, 0 */
            float4(from_anti_dipole_inversion.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(from_anti_dipole_inversion.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDipoleInversion: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<AntiDipoleInversion> {
    associatedtype Self = VersorEven:
    static func from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(from_anti_dipole_inversion.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            from_anti_dipole_inversion.group1, 
            /* e235, e315, e125, e5 */
            float4(from_anti_dipole_inversion.e235, from_anti_dipole_inversion.e315, from_anti_dipole_inversion.e125, from_anti_dipole_inversion.e5), 
            /* e1, e2, e3, e4 */
            float4(from_anti_dipole_inversion.e1, from_anti_dipole_inversion.e2, from_anti_dipole_inversion.e3, from_anti_dipole_inversion.e4)
        );
    }
}
extension AntiDipoleInversion: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension AntiMotor: From<AntiDualNum> {
    associatedtype Self = AntiMotor:
    static func from(from_anti_dual_num: AntiDualNum) -> Self {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_anti_dual_num.scalar), 
            /* e15, e25, e35, e3215 */
            float4(float3(0.0), from_anti_dual_num.e3215)
        );
    }
}
extension AntiDualNum: Into<AntiMotor> {
    public func into() -> AntiMotor {
        return AntiMotor.from(this);
    }
}
extension MultiVector: From<AntiDualNum> {
    associatedtype Self = MultiVector:
    static func from(from_anti_dual_num: AntiDualNum) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_anti_dual_num.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), from_anti_dual_num.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<AntiDualNum> {
    associatedtype Self = VersorOdd:
    static func from(from_anti_dual_num: AntiDualNum) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), from_anti_dual_num.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), from_anti_dual_num.e3215)
        );
    }
}
extension AntiDualNum: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension AntiDipoleInversion: From<AntiFlatPoint> {
    associatedtype Self = AntiDipoleInversion:
    static func from(from_anti_flat_point: AntiFlatPoint) -> Self {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125, e4 */
            float4(from_anti_flat_point.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension AntiFlatPoint: Into<AntiDipoleInversion> {
    public func into() -> AntiDipoleInversion {
        return AntiDipoleInversion.from(this);
    }
}
extension AntiFlector: From<AntiFlatPoint> {
    associatedtype Self = AntiFlector:
    static func from(from_anti_flat_point: AntiFlatPoint) -> Self {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            from_anti_flat_point.group0, 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension AntiFlatPoint: Into<AntiFlector> {
    public func into() -> AntiFlector {
        return AntiFlector.from(this);
    }
}
extension Circle: From<AntiFlatPoint> {
    associatedtype Self = Circle:
    static func from(from_anti_flat_point: AntiFlatPoint) -> Self {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125, 0 */
            float4(from_anti_flat_point.group0.xyz, 0.0)
        );
    }
}
extension AntiFlatPoint: Into<Circle> {
    public func into() -> Circle {
        return Circle.from(this);
    }
}
extension CircleRotor: From<AntiFlatPoint> {
    associatedtype Self = CircleRotor:
    static func from(from_anti_flat_point: AntiFlatPoint) -> Self {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125, e12345 */
            float4(from_anti_flat_point.group0.xyz, 0.0)
        );
    }
}
extension AntiFlatPoint: Into<CircleRotor> {
    public func into() -> CircleRotor {
        return CircleRotor.from(this);
    }
}
extension MultiVector: From<AntiFlatPoint> {
    associatedtype Self = MultiVector:
    static func from(from_anti_flat_point: AntiFlatPoint) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(from_anti_flat_point.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlatPoint: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<AntiFlatPoint> {
    associatedtype Self = VersorEven:
    static func from(from_anti_flat_point: AntiFlatPoint) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125, e5 */
            float4(from_anti_flat_point.group0.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension AntiFlatPoint: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension AntiDipoleInversion: From<AntiFlector> {
    associatedtype Self = AntiDipoleInversion:
    static func from(from_anti_flector: AntiFlector) -> Self {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flector.e321), 
            /* e235, e315, e125, e4 */
            float4(from_anti_flector.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            from_anti_flector.group1
        );
    }
}
extension AntiFlector: Into<AntiDipoleInversion> {
    public func into() -> AntiDipoleInversion {
        return AntiDipoleInversion.from(this);
    }
}
extension MultiVector: From<AntiFlector> {
    associatedtype Self = MultiVector:
    static func from(from_anti_flector: AntiFlector) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(from_anti_flector.group1.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(from_anti_flector.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flector.e321), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(from_anti_flector.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlector: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<AntiFlector> {
    associatedtype Self = VersorEven:
    static func from(from_anti_flector: AntiFlector) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flector.e321), 
            /* e235, e315, e125, e5 */
            float4(from_anti_flector.e235, from_anti_flector.e315, from_anti_flector.e125, from_anti_flector.e5), 
            /* e1, e2, e3, e4 */
            float4(from_anti_flector.group1.xyz, 0.0)
        );
    }
}
extension AntiFlector: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension AntiCircleRotor: From<AntiLine> {
    associatedtype Self = AntiCircleRotor:
    static func from(from_anti_line: AntiLine) -> Self {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, scalar */
            float4(from_anti_line.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: Into<AntiCircleRotor> {
    public func into() -> AntiCircleRotor {
        return AntiCircleRotor.from(this);
    }
}
extension AntiMotor: From<AntiLine> {
    associatedtype Self = AntiMotor:
    static func from(from_anti_line: AntiLine) -> Self {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, e3215 */
            float4(from_anti_line.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: Into<AntiMotor> {
    public func into() -> AntiMotor {
        return AntiMotor.from(this);
    }
}
extension Dipole: From<AntiLine> {
    associatedtype Self = Dipole:
    static func from(from_anti_line: AntiLine) -> Self {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(from_anti_line.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: Into<Dipole> {
    public func into() -> Dipole {
        return Dipole.from(this);
    }
}
extension DipoleInversion: From<AntiLine> {
    associatedtype Self = DipoleInversion:
    static func from(from_anti_line: AntiLine) -> Self {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(from_anti_line.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension AntiLine: Into<DipoleInversion> {
    public func into() -> DipoleInversion {
        return DipoleInversion.from(this);
    }
}
extension MultiVector: From<AntiLine> {
    associatedtype Self = MultiVector:
    static func from(from_anti_line: AntiLine) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_anti_line.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<AntiLine> {
    associatedtype Self = VersorOdd:
    static func from(from_anti_line: AntiLine) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(from_anti_line.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension AntiLine: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension MultiVector: From<AntiMotor> {
    associatedtype Self = MultiVector:
    static func from(from_anti_motor: AntiMotor) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_anti_motor.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_anti_motor.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(from_anti_motor.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), from_anti_motor.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<AntiMotor> {
    associatedtype Self = VersorOdd:
    static func from(from_anti_motor: AntiMotor) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), from_anti_motor.scalar), 
            /* e23, e31, e12, e45 */
            float4(from_anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(from_anti_motor.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), from_anti_motor.e3215)
        );
    }
}
extension AntiMotor: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension AntiDipoleInversion: From<AntiPlane> {
    associatedtype Self = AntiDipoleInversion:
    static func from(from_anti_plane: AntiPlane) -> Self {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            from_anti_plane.group0
        );
    }
}
extension AntiPlane: Into<AntiDipoleInversion> {
    public func into() -> AntiDipoleInversion {
        return AntiDipoleInversion.from(this);
    }
}
extension AntiFlector: From<AntiPlane> {
    associatedtype Self = AntiFlector:
    static func from(from_anti_plane: AntiPlane) -> Self {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            from_anti_plane.group0
        );
    }
}
extension AntiPlane: Into<AntiFlector> {
    public func into() -> AntiFlector {
        return AntiFlector.from(this);
    }
}
extension MultiVector: From<AntiPlane> {
    associatedtype Self = MultiVector:
    static func from(from_anti_plane: AntiPlane) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(from_anti_plane.group0.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(from_anti_plane.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiPlane: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension RoundPoint: From<AntiPlane> {
    associatedtype Self = RoundPoint:
    static func from(from_anti_plane: AntiPlane) -> Self {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(from_anti_plane.group0.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(from_anti_plane.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: Into<RoundPoint> {
    public func into() -> RoundPoint {
        return RoundPoint.from(this);
    }
}
extension VersorEven: From<AntiPlane> {
    associatedtype Self = VersorEven:
    static func from(from_anti_plane: AntiPlane) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_anti_plane.e5), 
            /* e1, e2, e3, e4 */
            float4(from_anti_plane.group0.xyz, 0.0)
        );
    }
}
extension AntiPlane: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension CircleRotor: From<AntiScalar> {
    associatedtype Self = CircleRotor:
    static func from(from_anti_scalar: AntiScalar) -> Self {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), from_anti_scalar.e12345)
        );
    }
}
extension AntiScalar: Into<CircleRotor> {
    public func into() -> CircleRotor {
        return CircleRotor.from(this);
    }
}
extension DualNum: From<AntiScalar> {
    associatedtype Self = DualNum:
    static func from(from_anti_scalar: AntiScalar) -> Self {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(0.0, from_anti_scalar.e12345), 0.0, 0.0)
        );
    }
}
extension AntiScalar: Into<DualNum> {
    public func into() -> DualNum {
        return DualNum.from(this);
    }
}
extension Motor: From<AntiScalar> {
    associatedtype Self = Motor:
    static func from(from_anti_scalar: AntiScalar) -> Self {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), from_anti_scalar.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
extension MultiVector: From<AntiScalar> {
    associatedtype Self = MultiVector:
    static func from(from_anti_scalar: AntiScalar) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_anti_scalar.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<AntiScalar> {
    associatedtype Self = VersorEven:
    static func from(from_anti_scalar: AntiScalar) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), from_anti_scalar.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension AntiDipoleInversion: From<Circle> {
    associatedtype Self = AntiDipoleInversion:
    static func from(from_circle: Circle) -> Self {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(from_circle.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            from_circle.group1, 
            /* e235, e315, e125, e4 */
            float4(from_circle.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension Circle: Into<AntiDipoleInversion> {
    public func into() -> AntiDipoleInversion {
        return AntiDipoleInversion.from(this);
    }
}
extension CircleRotor: From<Circle> {
    associatedtype Self = CircleRotor:
    static func from(from_circle: Circle) -> Self {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(from_circle.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            from_circle.group1, 
            /* e235, e315, e125, e12345 */
            float4(from_circle.group2.xyz, 0.0)
        );
    }
}
extension Circle: Into<CircleRotor> {
    public func into() -> CircleRotor {
        return CircleRotor.from(this);
    }
}
extension MultiVector: From<Circle> {
    associatedtype Self = MultiVector:
    static func from(from_circle: Circle) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            from_circle.group1, 
            /* e423, e431, e412, 0 */
            float4(from_circle.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(from_circle.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Circle: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<Circle> {
    associatedtype Self = VersorEven:
    static func from(from_circle: Circle) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(from_circle.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            from_circle.group1, 
            /* e235, e315, e125, e5 */
            float4(from_circle.group2.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension Circle: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension MultiVector: From<CircleRotor> {
    associatedtype Self = MultiVector:
    static func from(from_circle_rotor: CircleRotor) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_circle_rotor.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            from_circle_rotor.group1, 
            /* e423, e431, e412, 0 */
            float4(from_circle_rotor.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(from_circle_rotor.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension CircleRotor: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<CircleRotor> {
    associatedtype Self = VersorEven:
    static func from(from_circle_rotor: CircleRotor) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(from_circle_rotor.group0.xyz, from_circle_rotor.e12345), 
            /* e415, e425, e435, e321 */
            from_circle_rotor.group1, 
            /* e235, e315, e125, e5 */
            float4(from_circle_rotor.group2.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension CircleRotor: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension AntiCircleRotor: From<Dipole> {
    associatedtype Self = AntiCircleRotor:
    static func from(from_dipole: Dipole) -> Self {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(from_dipole.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            from_dipole.group1, 
            /* e15, e25, e35, scalar */
            float4(from_dipole.group2.xyz, 0.0)
        );
    }
}
extension Dipole: Into<AntiCircleRotor> {
    public func into() -> AntiCircleRotor {
        return AntiCircleRotor.from(this);
    }
}
extension DipoleInversion: From<Dipole> {
    associatedtype Self = DipoleInversion:
    static func from(from_dipole: Dipole) -> Self {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(from_dipole.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            from_dipole.group1, 
            /* e15, e25, e35, e1234 */
            float4(from_dipole.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension Dipole: Into<DipoleInversion> {
    public func into() -> DipoleInversion {
        return DipoleInversion.from(this);
    }
}
extension MultiVector: From<Dipole> {
    associatedtype Self = MultiVector:
    static func from(from_dipole: Dipole) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_dipole.group2.xyz, from_dipole.e45), 
            /* e41, e42, e43, 0 */
            float4(from_dipole.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_dipole.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Dipole: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<Dipole> {
    associatedtype Self = VersorOdd:
    static func from(from_dipole: Dipole) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(from_dipole.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            from_dipole.group1, 
            /* e15, e25, e35, e1234 */
            float4(from_dipole.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension Dipole: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension MultiVector: From<DipoleInversion> {
    associatedtype Self = MultiVector:
    static func from(from_dipole_inversion: DipoleInversion) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_dipole_inversion.e15, from_dipole_inversion.e25, from_dipole_inversion.e35, from_dipole_inversion.e45), 
            /* e41, e42, e43, 0 */
            float4(from_dipole_inversion.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_dipole_inversion.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_dipole_inversion.group3, 
            /* e1234, 0, 0, 0 */
            float4(from_dipole_inversion.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<DipoleInversion> {
    associatedtype Self = VersorOdd:
    static func from(from_dipole_inversion: DipoleInversion) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(from_dipole_inversion.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            from_dipole_inversion.group1, 
            /* e15, e25, e35, e1234 */
            from_dipole_inversion.group2, 
            /* e4235, e4315, e4125, e3215 */
            from_dipole_inversion.group3
        );
    }
}
extension DipoleInversion: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension Motor: From<DualNum> {
    associatedtype Self = Motor:
    static func from(from_dual_num: DualNum) -> Self {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), from_dual_num.e12345), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_dual_num.e5)
        );
    }
}
extension DualNum: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
extension MultiVector: From<DualNum> {
    associatedtype Self = MultiVector:
    static func from(from_dual_num: DualNum) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_dual_num.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(from_dual_num.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension DualNum: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<DualNum> {
    associatedtype Self = VersorEven:
    static func from(from_dual_num: DualNum) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), from_dual_num.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_dual_num.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension DualNum: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension AntiCircleRotor: From<FlatPoint> {
    associatedtype Self = AntiCircleRotor:
    static func from(from_flat_point: FlatPoint) -> Self {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flat_point.e45), 
            /* e15, e25, e35, scalar */
            float4(from_flat_point.group0.xyz, 0.0)
        );
    }
}
extension FlatPoint: Into<AntiCircleRotor> {
    public func into() -> AntiCircleRotor {
        return AntiCircleRotor.from(this);
    }
}
extension Dipole: From<FlatPoint> {
    associatedtype Self = Dipole:
    static func from(from_flat_point: FlatPoint) -> Self {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flat_point.e45), 
            /* e15, e25, e35, 0 */
            float4(from_flat_point.group0.xyz, 0.0)
        );
    }
}
extension FlatPoint: Into<Dipole> {
    public func into() -> Dipole {
        return Dipole.from(this);
    }
}
extension DipoleInversion: From<FlatPoint> {
    associatedtype Self = DipoleInversion:
    static func from(from_flat_point: FlatPoint) -> Self {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flat_point.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flat_point.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension FlatPoint: Into<DipoleInversion> {
    public func into() -> DipoleInversion {
        return DipoleInversion.from(this);
    }
}
extension Flector: From<FlatPoint> {
    associatedtype Self = Flector:
    static func from(from_flat_point: FlatPoint) -> Self {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            from_flat_point.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension FlatPoint: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
extension MultiVector: From<FlatPoint> {
    associatedtype Self = MultiVector:
    static func from(from_flat_point: FlatPoint) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            from_flat_point.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension FlatPoint: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<FlatPoint> {
    associatedtype Self = VersorOdd:
    static func from(from_flat_point: FlatPoint) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flat_point.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flat_point.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension FlatPoint: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension DipoleInversion: From<Flector> {
    associatedtype Self = DipoleInversion:
    static func from(from_flector: Flector) -> Self {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flector.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flector.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_flector.group1
        );
    }
}
extension Flector: Into<DipoleInversion> {
    public func into() -> DipoleInversion {
        return DipoleInversion.from(this);
    }
}
extension MultiVector: From<Flector> {
    associatedtype Self = MultiVector:
    static func from(from_flector: Flector) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            from_flector.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_flector.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<Flector> {
    associatedtype Self = VersorOdd:
    static func from(from_flector: Flector) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flector.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flector.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_flector.group1
        );
    }
}
extension Flector: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension AntiDipoleInversion: From<Line> {
    associatedtype Self = AntiDipoleInversion:
    static func from(from_line: Line) -> Self {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e4 */
            float4(from_line.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension Line: Into<AntiDipoleInversion> {
    public func into() -> AntiDipoleInversion {
        return AntiDipoleInversion.from(this);
    }
}
extension Circle: From<Line> {
    associatedtype Self = Circle:
    static func from(from_line: Line) -> Self {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(from_line.group1.xyz, 0.0)
        );
    }
}
extension Line: Into<Circle> {
    public func into() -> Circle {
        return Circle.from(this);
    }
}
extension CircleRotor: From<Line> {
    associatedtype Self = CircleRotor:
    static func from(from_line: Line) -> Self {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e12345 */
            float4(from_line.group1.xyz, 0.0)
        );
    }
}
extension Line: Into<CircleRotor> {
    public func into() -> CircleRotor {
        return CircleRotor.from(this);
    }
}
extension Motor: From<Line> {
    associatedtype Self = Motor:
    static func from(from_line: Line) -> Self {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(from_line.group1.xyz, 0.0)
        );
    }
}
extension Line: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
extension MultiVector: From<Line> {
    associatedtype Self = MultiVector:
    static func from(from_line: Line) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(from_line.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Line: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<Line> {
    associatedtype Self = VersorEven:
    static func from(from_line: Line) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(from_line.group1.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension Line: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension MultiVector: From<Motor> {
    associatedtype Self = MultiVector:
    static func from(from_motor: Motor) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_motor.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(from_motor.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_motor.group0.xyz, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(from_motor.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Motor: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<Motor> {
    associatedtype Self = VersorEven:
    static func from(from_motor: Motor) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), from_motor.e12345), 
            /* e415, e425, e435, e321 */
            float4(from_motor.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            from_motor.group1, 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension Motor: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension DipoleInversion: From<Plane> {
    associatedtype Self = DipoleInversion:
    static func from(from_plane: Plane) -> Self {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_plane.group0
        );
    }
}
extension Plane: Into<DipoleInversion> {
    public func into() -> DipoleInversion {
        return DipoleInversion.from(this);
    }
}
extension Flector: From<Plane> {
    associatedtype Self = Flector:
    static func from(from_plane: Plane) -> Self {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_plane.group0
        );
    }
}
extension Plane: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
extension MultiVector: From<Plane> {
    associatedtype Self = MultiVector:
    static func from(from_plane: Plane) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_plane.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension Sphere: From<Plane> {
    associatedtype Self = Sphere:
    static func from(from_plane: Plane) -> Self {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            from_plane.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane: Into<Sphere> {
    public func into() -> Sphere {
        return Sphere.from(this);
    }
}
extension VersorOdd: From<Plane> {
    associatedtype Self = VersorOdd:
    static func from(from_plane: Plane) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_plane.group0
        );
    }
}
extension Plane: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension AntiDipoleInversion: From<RoundPoint> {
    associatedtype Self = AntiDipoleInversion:
    static func from(from_round_point: RoundPoint) -> Self {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), from_round_point.e4), 
            /* e1, e2, e3, e5 */
            float4(from_round_point.e1, from_round_point.e2, from_round_point.e3, from_round_point.e5)
        );
    }
}
extension RoundPoint: Into<AntiDipoleInversion> {
    public func into() -> AntiDipoleInversion {
        return AntiDipoleInversion.from(this);
    }
}
extension MultiVector: From<RoundPoint> {
    associatedtype Self = MultiVector:
    static func from(from_round_point: RoundPoint) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            from_round_point.group0, 
            /* e5, 0, 0, 0 */
            float4(from_round_point.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension RoundPoint: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorEven: From<RoundPoint> {
    associatedtype Self = VersorEven:
    static func from(from_round_point: RoundPoint) -> Self {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_round_point.e5), 
            /* e1, e2, e3, e4 */
            from_round_point.group0
        );
    }
}
extension RoundPoint: Into<VersorEven> {
    public func into() -> VersorEven {
        return VersorEven.from(this);
    }
}
extension AntiCircleRotor: From<Scalar> {
    associatedtype Self = AntiCircleRotor:
    static func from(from_scalar: Scalar) -> Self {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), from_scalar.scalar)
        );
    }
}
extension Scalar: Into<AntiCircleRotor> {
    public func into() -> AntiCircleRotor {
        return AntiCircleRotor.from(this);
    }
}
extension AntiDualNum: From<Scalar> {
    associatedtype Self = AntiDualNum:
    static func from(from_scalar: Scalar) -> Self {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(0.0, from_scalar.scalar), 0.0, 0.0)
        );
    }
}
extension Scalar: Into<AntiDualNum> {
    public func into() -> AntiDualNum {
        return AntiDualNum.from(this);
    }
}
extension AntiMotor: From<Scalar> {
    associatedtype Self = AntiMotor:
    static func from(from_scalar: Scalar) -> Self {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_scalar.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
extension Scalar: Into<AntiMotor> {
    public func into() -> AntiMotor {
        return AntiMotor.from(this);
    }
}
extension MultiVector: From<Scalar> {
    associatedtype Self = MultiVector:
    static func from(from_scalar: Scalar) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_scalar.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Scalar: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<Scalar> {
    associatedtype Self = VersorOdd:
    static func from(from_scalar: Scalar) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), from_scalar.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension Scalar: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension DipoleInversion: From<Sphere> {
    associatedtype Self = DipoleInversion:
    static func from(from_sphere: Sphere) -> Self {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), from_sphere.e1234), 
            /* e4235, e4315, e4125, e3215 */
            from_sphere.group0
        );
    }
}
extension Sphere: Into<DipoleInversion> {
    public func into() -> DipoleInversion {
        return DipoleInversion.from(this);
    }
}
extension MultiVector: From<Sphere> {
    associatedtype Self = MultiVector:
    static func from(from_sphere: Sphere) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_sphere.group0, 
            /* e1234, 0, 0, 0 */
            float4(from_sphere.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension VersorOdd: From<Sphere> {
    associatedtype Self = VersorOdd:
    static func from(from_sphere: Sphere) -> Self {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), from_sphere.e1234), 
            /* e4235, e4315, e4125, e3215 */
            from_sphere.group0
        );
    }
}
extension Sphere: Into<VersorOdd> {
    public func into() -> VersorOdd {
        return VersorOdd.from(this);
    }
}
extension MultiVector: From<VersorEven> {
    associatedtype Self = MultiVector:
    static func from(from_versor_even: VersorEven) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, from_versor_even.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            from_versor_even.group3, 
            /* e5, 0, 0, 0 */
            float4(from_versor_even.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            from_versor_even.group1, 
            /* e423, e431, e412, 0 */
            float4(from_versor_even.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(from_versor_even.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension MultiVector: From<VersorOdd> {
    associatedtype Self = MultiVector:
    static func from(from_versor_odd: VersorOdd) -> Self {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(from_versor_odd.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(from_versor_odd.e15, from_versor_odd.e25, from_versor_odd.e35, from_versor_odd.e45), 
            /* e41, e42, e43, 0 */
            float4(from_versor_odd.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_versor_odd.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_versor_odd.group3, 
            /* e1234, 0, 0, 0 */
            float4(from_versor_odd.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
