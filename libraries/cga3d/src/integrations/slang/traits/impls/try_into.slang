
extension AntiDualNum: TryFrom<AntiCircleRotor> {
    associatedtype Self = AntiDualNum;
    static func try_from(anti_circle_rotor: AntiCircleRotor) -> Option<Self> {
        float disallowed_e41 = anti_circle_rotor.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = anti_circle_rotor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_circle_rotor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_circle_rotor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e45 = anti_circle_rotor.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e15 = anti_circle_rotor.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = anti_circle_rotor.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = anti_circle_rotor.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(0.0, anti_circle_rotor.scalar), 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: TryInto<AntiDualNum> {
    public func try_into() -> Optional<AntiDualNum> {
        return AntiDualNum.try_from(this);
    }
}

extension AntiLine: TryFrom<AntiCircleRotor> {
    associatedtype Self = AntiLine;
    static func try_from(anti_circle_rotor: AntiCircleRotor) -> Option<Self> {
        float disallowed_e41 = anti_circle_rotor.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e45 = anti_circle_rotor.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_scalar = anti_circle_rotor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(anti_circle_rotor.group1.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(anti_circle_rotor.group2.xyz, 0.0)
        );
    }
}
extension AntiCircleRotor: TryInto<AntiLine> {
    public func try_into() -> Optional<AntiLine> {
        return AntiLine.try_from(this);
    }
}

extension AntiMotor: TryFrom<AntiCircleRotor> {
    associatedtype Self = AntiMotor;
    static func try_from(anti_circle_rotor: AntiCircleRotor) -> Option<Self> {
        float disallowed_e41 = anti_circle_rotor.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e45 = anti_circle_rotor.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(anti_circle_rotor.e23, anti_circle_rotor.e31, anti_circle_rotor.e12, anti_circle_rotor.scalar), 
            /* e15, e25, e35, e3215 */
            float4(anti_circle_rotor.group2.xyz, 0.0)
        );
    }
}
extension AntiCircleRotor: TryInto<AntiMotor> {
    public func try_into() -> Optional<AntiMotor> {
        return AntiMotor.try_from(this);
    }
}

extension Dipole: TryFrom<AntiCircleRotor> {
    associatedtype Self = Dipole;
    static func try_from(anti_circle_rotor: AntiCircleRotor) -> Option<Self> {
        float disallowed_scalar = anti_circle_rotor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(anti_circle_rotor.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            anti_circle_rotor.group1, 
            /* e15, e25, e35, 0 */
            float4(anti_circle_rotor.group2.xyz, 0.0)
        );
    }
}
extension AntiCircleRotor: TryInto<Dipole> {
    public func try_into() -> Optional<Dipole> {
        return Dipole.try_from(this);
    }
}

extension DipoleInversion: TryFrom<AntiCircleRotor> {
    associatedtype Self = DipoleInversion;
    static func try_from(anti_circle_rotor: AntiCircleRotor) -> Option<Self> {
        float disallowed_scalar = anti_circle_rotor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(anti_circle_rotor.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            anti_circle_rotor.group1, 
            /* e15, e25, e35, e1234 */
            float4(anti_circle_rotor.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension AntiCircleRotor: TryInto<DipoleInversion> {
    public func try_into() -> Optional<DipoleInversion> {
        return DipoleInversion.try_from(this);
    }
}

extension FlatPoint: TryFrom<AntiCircleRotor> {
    associatedtype Self = FlatPoint;
    static func try_from(anti_circle_rotor: AntiCircleRotor) -> Option<Self> {
        float disallowed_e41 = anti_circle_rotor.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = anti_circle_rotor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_circle_rotor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_circle_rotor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_scalar = anti_circle_rotor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(anti_circle_rotor.e15, anti_circle_rotor.e25, anti_circle_rotor.e35, anti_circle_rotor.e45)
        );
    }
}
extension AntiCircleRotor: TryInto<FlatPoint> {
    public func try_into() -> Optional<FlatPoint> {
        return FlatPoint.try_from(this);
    }
}

extension Flector: TryFrom<AntiCircleRotor> {
    associatedtype Self = Flector;
    static func try_from(anti_circle_rotor: AntiCircleRotor) -> Option<Self> {
        float disallowed_e41 = anti_circle_rotor.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = anti_circle_rotor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_circle_rotor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_circle_rotor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_scalar = anti_circle_rotor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(anti_circle_rotor.e15, anti_circle_rotor.e25, anti_circle_rotor.e35, anti_circle_rotor.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension AntiCircleRotor: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension Scalar: TryFrom<AntiCircleRotor> {
    associatedtype Self = Scalar;
    static func try_from(anti_circle_rotor: AntiCircleRotor) -> Option<Self> {
        float disallowed_e41 = anti_circle_rotor.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = anti_circle_rotor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_circle_rotor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_circle_rotor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e45 = anti_circle_rotor.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e15 = anti_circle_rotor.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = anti_circle_rotor.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = anti_circle_rotor.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(anti_circle_rotor.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
    }
}

extension AntiFlatPoint: TryFrom<AntiDipoleInversion> {
    associatedtype Self = AntiFlatPoint;
    static func try_from(anti_dipole_inversion: AntiDipoleInversion) -> Option<Self> {
        float disallowed_e423 = anti_dipole_inversion.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e5 = anti_dipole_inversion.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(anti_dipole_inversion.e235, anti_dipole_inversion.e315, anti_dipole_inversion.e125, anti_dipole_inversion.e321)
        );
    }
}
extension AntiDipoleInversion: TryInto<AntiFlatPoint> {
    public func try_into() -> Optional<AntiFlatPoint> {
        return AntiFlatPoint.try_from(this);
    }
}

extension AntiFlector: TryFrom<AntiDipoleInversion> {
    associatedtype Self = AntiFlector;
    static func try_from(anti_dipole_inversion: AntiDipoleInversion) -> Option<Self> {
        float disallowed_e423 = anti_dipole_inversion.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(anti_dipole_inversion.e235, anti_dipole_inversion.e315, anti_dipole_inversion.e125, anti_dipole_inversion.e321), 
            /* e1, e2, e3, e5 */
            anti_dipole_inversion.group3
        );
    }
}
extension AntiDipoleInversion: TryInto<AntiFlector> {
    public func try_into() -> Optional<AntiFlector> {
        return AntiFlector.try_from(this);
    }
}

extension AntiPlane: TryFrom<AntiDipoleInversion> {
    associatedtype Self = AntiPlane;
    static func try_from(anti_dipole_inversion: AntiDipoleInversion) -> Option<Self> {
        float disallowed_e423 = anti_dipole_inversion.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e235 = anti_dipole_inversion.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = anti_dipole_inversion.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = anti_dipole_inversion.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            anti_dipole_inversion.group3
        );
    }
}
extension AntiDipoleInversion: TryInto<AntiPlane> {
    public func try_into() -> Optional<AntiPlane> {
        return AntiPlane.try_from(this);
    }
}

extension Circle: TryFrom<AntiDipoleInversion> {
    associatedtype Self = Circle;
    static func try_from(anti_dipole_inversion: AntiDipoleInversion) -> Option<Self> {
        float disallowed_e4 = anti_dipole_inversion.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e5 = anti_dipole_inversion.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(anti_dipole_inversion.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            anti_dipole_inversion.group1, 
            /* e235, e315, e125, 0 */
            float4(anti_dipole_inversion.group2.xyz, 0.0)
        );
    }
}
extension AntiDipoleInversion: TryInto<Circle> {
    public func try_into() -> Optional<Circle> {
        return Circle.try_from(this);
    }
}

extension CircleRotor: TryFrom<AntiDipoleInversion> {
    associatedtype Self = CircleRotor;
    static func try_from(anti_dipole_inversion: AntiDipoleInversion) -> Option<Self> {
        float disallowed_e4 = anti_dipole_inversion.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e5 = anti_dipole_inversion.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(anti_dipole_inversion.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            anti_dipole_inversion.group1, 
            /* e235, e315, e125, e12345 */
            float4(anti_dipole_inversion.group2.xyz, 0.0)
        );
    }
}
extension AntiDipoleInversion: TryInto<CircleRotor> {
    public func try_into() -> Optional<CircleRotor> {
        return CircleRotor.try_from(this);
    }
}

extension DualNum: TryFrom<AntiDipoleInversion> {
    associatedtype Self = DualNum;
    static func try_from(anti_dipole_inversion: AntiDipoleInversion) -> Option<Self> {
        float disallowed_e423 = anti_dipole_inversion.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e235 = anti_dipole_inversion.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = anti_dipole_inversion.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = anti_dipole_inversion.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(anti_dipole_inversion.e5, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

extension Line: TryFrom<AntiDipoleInversion> {
    associatedtype Self = Line;
    static func try_from(anti_dipole_inversion: AntiDipoleInversion) -> Option<Self> {
        float disallowed_e423 = anti_dipole_inversion.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e5 = anti_dipole_inversion.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(anti_dipole_inversion.group1.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(anti_dipole_inversion.group2.xyz, 0.0)
        );
    }
}
extension AntiDipoleInversion: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

extension Motor: TryFrom<AntiDipoleInversion> {
    associatedtype Self = Motor;
    static func try_from(anti_dipole_inversion: AntiDipoleInversion) -> Option<Self> {
        float disallowed_e423 = anti_dipole_inversion.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(anti_dipole_inversion.group1.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(anti_dipole_inversion.e235, anti_dipole_inversion.e315, anti_dipole_inversion.e125, anti_dipole_inversion.e5)
        );
    }
}
extension AntiDipoleInversion: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension RoundPoint: TryFrom<AntiDipoleInversion> {
    associatedtype Self = RoundPoint;
    static func try_from(anti_dipole_inversion: AntiDipoleInversion) -> Option<Self> {
        float disallowed_e423 = anti_dipole_inversion.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e235 = anti_dipole_inversion.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = anti_dipole_inversion.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = anti_dipole_inversion.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_dipole_inversion.e1, anti_dipole_inversion.e2, anti_dipole_inversion.e3, anti_dipole_inversion.e4), 
            /* e5, 0, 0, 0 */
            float4(anti_dipole_inversion.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: TryInto<RoundPoint> {
    public func try_into() -> Optional<RoundPoint> {
        return RoundPoint.try_from(this);
    }
}

extension AntiCircleRotor: TryFrom<AntiDualNum> {
    associatedtype Self = AntiCircleRotor;
    static func try_from(anti_dual_num: AntiDualNum) -> Option<Self> {
        float disallowed_e3215 = anti_dual_num.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), anti_dual_num.scalar)
        );
    }
}
extension AntiDualNum: TryInto<AntiCircleRotor> {
    public func try_into() -> Optional<AntiCircleRotor> {
        return AntiCircleRotor.try_from(this);
    }
}

extension DipoleInversion: TryFrom<AntiDualNum> {
    associatedtype Self = DipoleInversion;
    static func try_from(anti_dual_num: AntiDualNum) -> Option<Self> {
        float disallowed_scalar = anti_dual_num.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_dual_num.e3215)
        );
    }
}
extension AntiDualNum: TryInto<DipoleInversion> {
    public func try_into() -> Optional<DipoleInversion> {
        return DipoleInversion.try_from(this);
    }
}

extension Flector: TryFrom<AntiDualNum> {
    associatedtype Self = Flector;
    static func try_from(anti_dual_num: AntiDualNum) -> Option<Self> {
        float disallowed_scalar = anti_dual_num.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_dual_num.e3215)
        );
    }
}
extension AntiDualNum: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension Plane: TryFrom<AntiDualNum> {
    associatedtype Self = Plane;
    static func try_from(anti_dual_num: AntiDualNum) -> Option<Self> {
        float disallowed_scalar = anti_dual_num.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_dual_num.e3215)
        );
    }
}
extension AntiDualNum: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

extension Scalar: TryFrom<AntiDualNum> {
    associatedtype Self = Scalar;
    static func try_from(anti_dual_num: AntiDualNum) -> Option<Self> {
        float disallowed_e3215 = anti_dual_num.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(anti_dual_num.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
    }
}

extension Sphere: TryFrom<AntiDualNum> {
    associatedtype Self = Sphere;
    static func try_from(anti_dual_num: AntiDualNum) -> Option<Self> {
        float disallowed_scalar = anti_dual_num.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_dual_num.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: TryInto<Sphere> {
    public func try_into() -> Optional<Sphere> {
        return Sphere.try_from(this);
    }
}

extension Line: TryFrom<AntiFlatPoint> {
    associatedtype Self = Line;
    static func try_from(anti_flat_point: AntiFlatPoint) -> Option<Self> {
        float disallowed_e321 = anti_flat_point.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(anti_flat_point.group0.xyz, 0.0)
        );
    }
}
extension AntiFlatPoint: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

extension Motor: TryFrom<AntiFlatPoint> {
    associatedtype Self = Motor;
    static func try_from(anti_flat_point: AntiFlatPoint) -> Option<Self> {
        float disallowed_e321 = anti_flat_point.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(anti_flat_point.group0.xyz, 0.0)
        );
    }
}
extension AntiFlatPoint: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension AntiFlatPoint: TryFrom<AntiFlector> {
    associatedtype Self = AntiFlatPoint;
    static func try_from(anti_flector: AntiFlector) -> Option<Self> {
        float disallowed_e1 = anti_flector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e5 = anti_flector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            anti_flector.group0
        );
    }
}
extension AntiFlector: TryInto<AntiFlatPoint> {
    public func try_into() -> Optional<AntiFlatPoint> {
        return AntiFlatPoint.try_from(this);
    }
}

extension AntiPlane: TryFrom<AntiFlector> {
    associatedtype Self = AntiPlane;
    static func try_from(anti_flector: AntiFlector) -> Option<Self> {
        float disallowed_e235 = anti_flector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = anti_flector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = anti_flector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e321 = anti_flector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            anti_flector.group1
        );
    }
}
extension AntiFlector: TryInto<AntiPlane> {
    public func try_into() -> Optional<AntiPlane> {
        return AntiPlane.try_from(this);
    }
}

extension Circle: TryFrom<AntiFlector> {
    associatedtype Self = Circle;
    static func try_from(anti_flector: AntiFlector) -> Option<Self> {
        float disallowed_e1 = anti_flector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e5 = anti_flector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_flector.e321), 
            /* e235, e315, e125, 0 */
            float4(anti_flector.group0.xyz, 0.0)
        );
    }
}
extension AntiFlector: TryInto<Circle> {
    public func try_into() -> Optional<Circle> {
        return Circle.try_from(this);
    }
}

extension CircleRotor: TryFrom<AntiFlector> {
    associatedtype Self = CircleRotor;
    static func try_from(anti_flector: AntiFlector) -> Option<Self> {
        float disallowed_e1 = anti_flector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e5 = anti_flector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_flector.e321), 
            /* e235, e315, e125, e12345 */
            float4(anti_flector.group0.xyz, 0.0)
        );
    }
}
extension AntiFlector: TryInto<CircleRotor> {
    public func try_into() -> Optional<CircleRotor> {
        return CircleRotor.try_from(this);
    }
}

extension DualNum: TryFrom<AntiFlector> {
    associatedtype Self = DualNum;
    static func try_from(anti_flector: AntiFlector) -> Option<Self> {
        float disallowed_e235 = anti_flector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = anti_flector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = anti_flector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e321 = anti_flector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e1 = anti_flector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(anti_flector.e5, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlector: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

extension Line: TryFrom<AntiFlector> {
    associatedtype Self = Line;
    static func try_from(anti_flector: AntiFlector) -> Option<Self> {
        float disallowed_e321 = anti_flector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e1 = anti_flector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e5 = anti_flector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(anti_flector.group0.xyz, 0.0)
        );
    }
}
extension AntiFlector: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

extension Motor: TryFrom<AntiFlector> {
    associatedtype Self = Motor;
    static func try_from(anti_flector: AntiFlector) -> Option<Self> {
        float disallowed_e321 = anti_flector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e1 = anti_flector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(anti_flector.e235, anti_flector.e315, anti_flector.e125, anti_flector.e5)
        );
    }
}
extension AntiFlector: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension RoundPoint: TryFrom<AntiFlector> {
    associatedtype Self = RoundPoint;
    static func try_from(anti_flector: AntiFlector) -> Option<Self> {
        float disallowed_e235 = anti_flector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = anti_flector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = anti_flector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e321 = anti_flector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_flector.group1.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(anti_flector.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: TryInto<RoundPoint> {
    public func try_into() -> Optional<RoundPoint> {
        return RoundPoint.try_from(this);
    }
}

extension FlatPoint: TryFrom<AntiLine> {
    associatedtype Self = FlatPoint;
    static func try_from(anti_line: AntiLine) -> Option<Self> {
        float disallowed_e23 = anti_line.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_line.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_line.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(anti_line.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: TryInto<FlatPoint> {
    public func try_into() -> Optional<FlatPoint> {
        return FlatPoint.try_from(this);
    }
}

extension Flector: TryFrom<AntiLine> {
    associatedtype Self = Flector;
    static func try_from(anti_line: AntiLine) -> Option<Self> {
        float disallowed_e23 = anti_line.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_line.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_line.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(anti_line.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension AntiLine: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension AntiCircleRotor: TryFrom<AntiMotor> {
    associatedtype Self = AntiCircleRotor;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_e3215 = anti_motor.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35, scalar */
            float4(anti_motor.e15, anti_motor.e25, anti_motor.e35, anti_motor.scalar)
        );
    }
}
extension AntiMotor: TryInto<AntiCircleRotor> {
    public func try_into() -> Optional<AntiCircleRotor> {
        return AntiCircleRotor.try_from(this);
    }
}

extension AntiDualNum: TryFrom<AntiMotor> {
    associatedtype Self = AntiDualNum;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_e23 = anti_motor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e15 = anti_motor.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = anti_motor.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = anti_motor.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(anti_motor.e3215, anti_motor.scalar), 0.0, 0.0)
        );
    }
}
extension AntiMotor: TryInto<AntiDualNum> {
    public func try_into() -> Optional<AntiDualNum> {
        return AntiDualNum.try_from(this);
    }
}

extension AntiLine: TryFrom<AntiMotor> {
    associatedtype Self = AntiLine;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_scalar = anti_motor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e3215 = anti_motor.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(anti_motor.group1.xyz, 0.0)
        );
    }
}
extension AntiMotor: TryInto<AntiLine> {
    public func try_into() -> Optional<AntiLine> {
        return AntiLine.try_from(this);
    }
}

extension Dipole: TryFrom<AntiMotor> {
    associatedtype Self = Dipole;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_scalar = anti_motor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e3215 = anti_motor.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(anti_motor.group1.xyz, 0.0)
        );
    }
}
extension AntiMotor: TryInto<Dipole> {
    public func try_into() -> Optional<Dipole> {
        return Dipole.try_from(this);
    }
}

extension DipoleInversion: TryFrom<AntiMotor> {
    associatedtype Self = DipoleInversion;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_scalar = anti_motor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(anti_motor.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_motor.e3215)
        );
    }
}
extension AntiMotor: TryInto<DipoleInversion> {
    public func try_into() -> Optional<DipoleInversion> {
        return DipoleInversion.try_from(this);
    }
}

extension FlatPoint: TryFrom<AntiMotor> {
    associatedtype Self = FlatPoint;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_e23 = anti_motor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_scalar = anti_motor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e3215 = anti_motor.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(anti_motor.group1.xyz, 0.0)
        );
    }
}
extension AntiMotor: TryInto<FlatPoint> {
    public func try_into() -> Optional<FlatPoint> {
        return FlatPoint.try_from(this);
    }
}

extension Flector: TryFrom<AntiMotor> {
    associatedtype Self = Flector;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_e23 = anti_motor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_scalar = anti_motor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(anti_motor.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_motor.e3215)
        );
    }
}
extension AntiMotor: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension Plane: TryFrom<AntiMotor> {
    associatedtype Self = Plane;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_e23 = anti_motor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_scalar = anti_motor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e15 = anti_motor.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = anti_motor.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = anti_motor.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_motor.e3215)
        );
    }
}
extension AntiMotor: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

extension Scalar: TryFrom<AntiMotor> {
    associatedtype Self = Scalar;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_e23 = anti_motor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e15 = anti_motor.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = anti_motor.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = anti_motor.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e3215 = anti_motor.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(anti_motor.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
    }
}

extension Sphere: TryFrom<AntiMotor> {
    associatedtype Self = Sphere;
    static func try_from(anti_motor: AntiMotor) -> Option<Self> {
        float disallowed_e23 = anti_motor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_scalar = anti_motor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e15 = anti_motor.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = anti_motor.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = anti_motor.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_motor.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: TryInto<Sphere> {
    public func try_into() -> Optional<Sphere> {
        return Sphere.try_from(this);
    }
}

extension DualNum: TryFrom<AntiPlane> {
    associatedtype Self = DualNum;
    static func try_from(anti_plane: AntiPlane) -> Option<Self> {
        float disallowed_e1 = anti_plane.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_plane.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_plane.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(anti_plane.e5, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiPlane: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

extension Motor: TryFrom<AntiPlane> {
    associatedtype Self = Motor;
    static func try_from(anti_plane: AntiPlane) -> Option<Self> {
        float disallowed_e1 = anti_plane.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = anti_plane.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = anti_plane.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), anti_plane.e5)
        );
    }
}
extension AntiPlane: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension AntiFlatPoint: TryFrom<Circle> {
    associatedtype Self = AntiFlatPoint;
    static func try_from(circle: Circle) -> Option<Self> {
        float disallowed_e423 = circle.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = circle.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = circle.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = circle.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(circle.group2.xyz, circle.e321)
        );
    }
}
extension Circle: TryInto<AntiFlatPoint> {
    public func try_into() -> Optional<AntiFlatPoint> {
        return AntiFlatPoint.try_from(this);
    }
}

extension AntiFlector: TryFrom<Circle> {
    associatedtype Self = AntiFlector;
    static func try_from(circle: Circle) -> Option<Self> {
        float disallowed_e423 = circle.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = circle.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = circle.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = circle.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(circle.group2.xyz, circle.e321), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension Circle: TryInto<AntiFlector> {
    public func try_into() -> Optional<AntiFlector> {
        return AntiFlector.try_from(this);
    }
}

extension Line: TryFrom<Circle> {
    associatedtype Self = Line;
    static func try_from(circle: Circle) -> Option<Self> {
        float disallowed_e423 = circle.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e321 = circle.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(circle.group1.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(circle.group2.xyz, 0.0)
        );
    }
}
extension Circle: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

extension Motor: TryFrom<Circle> {
    associatedtype Self = Motor;
    static func try_from(circle: Circle) -> Option<Self> {
        float disallowed_e423 = circle.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e321 = circle.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(circle.group1.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(circle.group2.xyz, 0.0)
        );
    }
}
extension Circle: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension AntiDipoleInversion: TryFrom<CircleRotor> {
    associatedtype Self = AntiDipoleInversion;
    static func try_from(circle_rotor: CircleRotor) -> Option<Self> {
        float disallowed_e12345 = circle_rotor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(circle_rotor.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            circle_rotor.group1, 
            /* e235, e315, e125, e4 */
            float4(circle_rotor.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension CircleRotor: TryInto<AntiDipoleInversion> {
    public func try_into() -> Optional<AntiDipoleInversion> {
        return AntiDipoleInversion.try_from(this);
    }
}

extension AntiFlatPoint: TryFrom<CircleRotor> {
    associatedtype Self = AntiFlatPoint;
    static func try_from(circle_rotor: CircleRotor) -> Option<Self> {
        float disallowed_e423 = circle_rotor.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = circle_rotor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = circle_rotor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = circle_rotor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e12345 = circle_rotor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(circle_rotor.e235, circle_rotor.e315, circle_rotor.e125, circle_rotor.e321)
        );
    }
}
extension CircleRotor: TryInto<AntiFlatPoint> {
    public func try_into() -> Optional<AntiFlatPoint> {
        return AntiFlatPoint.try_from(this);
    }
}

extension AntiFlector: TryFrom<CircleRotor> {
    associatedtype Self = AntiFlector;
    static func try_from(circle_rotor: CircleRotor) -> Option<Self> {
        float disallowed_e423 = circle_rotor.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = circle_rotor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = circle_rotor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = circle_rotor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e12345 = circle_rotor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(circle_rotor.e235, circle_rotor.e315, circle_rotor.e125, circle_rotor.e321), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension CircleRotor: TryInto<AntiFlector> {
    public func try_into() -> Optional<AntiFlector> {
        return AntiFlector.try_from(this);
    }
}

extension AntiScalar: TryFrom<CircleRotor> {
    associatedtype Self = AntiScalar;
    static func try_from(circle_rotor: CircleRotor) -> Option<Self> {
        float disallowed_e423 = circle_rotor.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = circle_rotor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = circle_rotor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = circle_rotor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = circle_rotor.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e235 = circle_rotor.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = circle_rotor.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = circle_rotor.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(circle_rotor.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

extension Circle: TryFrom<CircleRotor> {
    associatedtype Self = Circle;
    static func try_from(circle_rotor: CircleRotor) -> Option<Self> {
        float disallowed_e12345 = circle_rotor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(circle_rotor.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            circle_rotor.group1, 
            /* e235, e315, e125, 0 */
            float4(circle_rotor.group2.xyz, 0.0)
        );
    }
}
extension CircleRotor: TryInto<Circle> {
    public func try_into() -> Optional<Circle> {
        return Circle.try_from(this);
    }
}

extension DualNum: TryFrom<CircleRotor> {
    associatedtype Self = DualNum;
    static func try_from(circle_rotor: CircleRotor) -> Option<Self> {
        float disallowed_e423 = circle_rotor.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = circle_rotor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = circle_rotor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = circle_rotor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = circle_rotor.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e235 = circle_rotor.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = circle_rotor.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = circle_rotor.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(0.0, circle_rotor.e12345), 0.0, 0.0)
        );
    }
}
extension CircleRotor: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

extension Line: TryFrom<CircleRotor> {
    associatedtype Self = Line;
    static func try_from(circle_rotor: CircleRotor) -> Option<Self> {
        float disallowed_e423 = circle_rotor.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e321 = circle_rotor.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e12345 = circle_rotor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(circle_rotor.group1.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(circle_rotor.group2.xyz, 0.0)
        );
    }
}
extension CircleRotor: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

extension Motor: TryFrom<CircleRotor> {
    associatedtype Self = Motor;
    static func try_from(circle_rotor: CircleRotor) -> Option<Self> {
        float disallowed_e423 = circle_rotor.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e321 = circle_rotor.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(circle_rotor.e415, circle_rotor.e425, circle_rotor.e435, circle_rotor.e12345), 
            /* e235, e315, e125, e5 */
            float4(circle_rotor.group2.xyz, 0.0)
        );
    }
}
extension CircleRotor: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension AntiLine: TryFrom<Dipole> {
    associatedtype Self = AntiLine;
    static func try_from(dipole: Dipole) -> Option<Self> {
        float disallowed_e41 = dipole.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e45 = dipole.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(dipole.group1.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(dipole.group2.xyz, 0.0)
        );
    }
}
extension Dipole: TryInto<AntiLine> {
    public func try_into() -> Optional<AntiLine> {
        return AntiLine.try_from(this);
    }
}

extension AntiMotor: TryFrom<Dipole> {
    associatedtype Self = AntiMotor;
    static func try_from(dipole: Dipole) -> Option<Self> {
        float disallowed_e41 = dipole.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e45 = dipole.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(dipole.group1.xyz, 0.0), 
            /* e15, e25, e35, e3215 */
            float4(dipole.group2.xyz, 0.0)
        );
    }
}
extension Dipole: TryInto<AntiMotor> {
    public func try_into() -> Optional<AntiMotor> {
        return AntiMotor.try_from(this);
    }
}

extension FlatPoint: TryFrom<Dipole> {
    associatedtype Self = FlatPoint;
    static func try_from(dipole: Dipole) -> Option<Self> {
        float disallowed_e41 = dipole.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = dipole.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = dipole.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = dipole.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(dipole.group2.xyz, dipole.e45)
        );
    }
}
extension Dipole: TryInto<FlatPoint> {
    public func try_into() -> Optional<FlatPoint> {
        return FlatPoint.try_from(this);
    }
}

extension Flector: TryFrom<Dipole> {
    associatedtype Self = Flector;
    static func try_from(dipole: Dipole) -> Option<Self> {
        float disallowed_e41 = dipole.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = dipole.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = dipole.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = dipole.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(dipole.group2.xyz, dipole.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension Dipole: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension AntiCircleRotor: TryFrom<DipoleInversion> {
    associatedtype Self = AntiCircleRotor;
    static func try_from(dipole_inversion: DipoleInversion) -> Option<Self> {
        float disallowed_e1234 = dipole_inversion.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = dipole_inversion.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(dipole_inversion.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            dipole_inversion.group1, 
            /* e15, e25, e35, scalar */
            float4(dipole_inversion.group2.xyz, 0.0)
        );
    }
}
extension DipoleInversion: TryInto<AntiCircleRotor> {
    public func try_into() -> Optional<AntiCircleRotor> {
        return AntiCircleRotor.try_from(this);
    }
}

extension AntiDualNum: TryFrom<DipoleInversion> {
    associatedtype Self = AntiDualNum;
    static func try_from(dipole_inversion: DipoleInversion) -> Option<Self> {
        float disallowed_e41 = dipole_inversion.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e15 = dipole_inversion.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = dipole_inversion.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = dipole_inversion.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(dipole_inversion.e3215, 0.0), 0.0, 0.0)
        );
    }
}
extension DipoleInversion: TryInto<AntiDualNum> {
    public func try_into() -> Optional<AntiDualNum> {
        return AntiDualNum.try_from(this);
    }
}

extension AntiLine: TryFrom<DipoleInversion> {
    associatedtype Self = AntiLine;
    static func try_from(dipole_inversion: DipoleInversion) -> Option<Self> {
        float disallowed_e41 = dipole_inversion.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = dipole_inversion.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(dipole_inversion.group1.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(dipole_inversion.group2.xyz, 0.0)
        );
    }
}
extension DipoleInversion: TryInto<AntiLine> {
    public func try_into() -> Optional<AntiLine> {
        return AntiLine.try_from(this);
    }
}

extension AntiMotor: TryFrom<DipoleInversion> {
    associatedtype Self = AntiMotor;
    static func try_from(dipole_inversion: DipoleInversion) -> Option<Self> {
        float disallowed_e41 = dipole_inversion.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(dipole_inversion.group1.xyz, 0.0), 
            /* e15, e25, e35, e3215 */
            float4(dipole_inversion.e15, dipole_inversion.e25, dipole_inversion.e35, dipole_inversion.e3215)
        );
    }
}
extension DipoleInversion: TryInto<AntiMotor> {
    public func try_into() -> Optional<AntiMotor> {
        return AntiMotor.try_from(this);
    }
}

extension Dipole: TryFrom<DipoleInversion> {
    associatedtype Self = Dipole;
    static func try_from(dipole_inversion: DipoleInversion) -> Option<Self> {
        float disallowed_e1234 = dipole_inversion.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = dipole_inversion.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(dipole_inversion.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            dipole_inversion.group1, 
            /* e15, e25, e35, 0 */
            float4(dipole_inversion.group2.xyz, 0.0)
        );
    }
}
extension DipoleInversion: TryInto<Dipole> {
    public func try_into() -> Optional<Dipole> {
        return Dipole.try_from(this);
    }
}

extension FlatPoint: TryFrom<DipoleInversion> {
    associatedtype Self = FlatPoint;
    static func try_from(dipole_inversion: DipoleInversion) -> Option<Self> {
        float disallowed_e41 = dipole_inversion.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = dipole_inversion.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(dipole_inversion.e15, dipole_inversion.e25, dipole_inversion.e35, dipole_inversion.e45)
        );
    }
}
extension DipoleInversion: TryInto<FlatPoint> {
    public func try_into() -> Optional<FlatPoint> {
        return FlatPoint.try_from(this);
    }
}

extension Flector: TryFrom<DipoleInversion> {
    associatedtype Self = Flector;
    static func try_from(dipole_inversion: DipoleInversion) -> Option<Self> {
        float disallowed_e41 = dipole_inversion.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(dipole_inversion.e15, dipole_inversion.e25, dipole_inversion.e35, dipole_inversion.e45), 
            /* e4235, e4315, e4125, e3215 */
            dipole_inversion.group3
        );
    }
}
extension DipoleInversion: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension Plane: TryFrom<DipoleInversion> {
    associatedtype Self = Plane;
    static func try_from(dipole_inversion: DipoleInversion) -> Option<Self> {
        float disallowed_e41 = dipole_inversion.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e15 = dipole_inversion.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = dipole_inversion.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = dipole_inversion.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            dipole_inversion.group3
        );
    }
}
extension DipoleInversion: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

extension Sphere: TryFrom<DipoleInversion> {
    associatedtype Self = Sphere;
    static func try_from(dipole_inversion: DipoleInversion) -> Option<Self> {
        float disallowed_e41 = dipole_inversion.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e15 = dipole_inversion.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = dipole_inversion.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = dipole_inversion.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            dipole_inversion.group3, 
            /* e1234, 0, 0, 0 */
            float4(dipole_inversion.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: TryInto<Sphere> {
    public func try_into() -> Optional<Sphere> {
        return Sphere.try_from(this);
    }
}

extension AntiDipoleInversion: TryFrom<DualNum> {
    associatedtype Self = AntiDipoleInversion;
    static func try_from(dual_num: DualNum) -> Option<Self> {
        float disallowed_e12345 = dual_num.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), dual_num.e5)
        );
    }
}
extension DualNum: TryInto<AntiDipoleInversion> {
    public func try_into() -> Optional<AntiDipoleInversion> {
        return AntiDipoleInversion.try_from(this);
    }
}

extension AntiFlector: TryFrom<DualNum> {
    associatedtype Self = AntiFlector;
    static func try_from(dual_num: DualNum) -> Option<Self> {
        float disallowed_e12345 = dual_num.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), dual_num.e5)
        );
    }
}
extension DualNum: TryInto<AntiFlector> {
    public func try_into() -> Optional<AntiFlector> {
        return AntiFlector.try_from(this);
    }
}

extension AntiPlane: TryFrom<DualNum> {
    associatedtype Self = AntiPlane;
    static func try_from(dual_num: DualNum) -> Option<Self> {
        float disallowed_e12345 = dual_num.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), dual_num.e5)
        );
    }
}
extension DualNum: TryInto<AntiPlane> {
    public func try_into() -> Optional<AntiPlane> {
        return AntiPlane.try_from(this);
    }
}

extension AntiScalar: TryFrom<DualNum> {
    associatedtype Self = AntiScalar;
    static func try_from(dual_num: DualNum) -> Option<Self> {
        float disallowed_e5 = dual_num.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(dual_num.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

extension CircleRotor: TryFrom<DualNum> {
    associatedtype Self = CircleRotor;
    static func try_from(dual_num: DualNum) -> Option<Self> {
        float disallowed_e5 = dual_num.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), dual_num.e12345)
        );
    }
}
extension DualNum: TryInto<CircleRotor> {
    public func try_into() -> Optional<CircleRotor> {
        return CircleRotor.try_from(this);
    }
}

extension RoundPoint: TryFrom<DualNum> {
    associatedtype Self = RoundPoint;
    static func try_from(dual_num: DualNum) -> Option<Self> {
        float disallowed_e12345 = dual_num.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(dual_num.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: TryInto<RoundPoint> {
    public func try_into() -> Optional<RoundPoint> {
        return RoundPoint.try_from(this);
    }
}

extension AntiLine: TryFrom<FlatPoint> {
    associatedtype Self = AntiLine;
    static func try_from(flat_point: FlatPoint) -> Option<Self> {
        float disallowed_e45 = flat_point.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e15, e25, e35, 0 */
            float4(flat_point.group0.xyz, 0.0)
        );
    }
}
extension FlatPoint: TryInto<AntiLine> {
    public func try_into() -> Optional<AntiLine> {
        return AntiLine.try_from(this);
    }
}

extension AntiMotor: TryFrom<FlatPoint> {
    associatedtype Self = AntiMotor;
    static func try_from(flat_point: FlatPoint) -> Option<Self> {
        float disallowed_e45 = flat_point.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(flat_point.group0.xyz, 0.0)
        );
    }
}
extension FlatPoint: TryInto<AntiMotor> {
    public func try_into() -> Optional<AntiMotor> {
        return AntiMotor.try_from(this);
    }
}

extension AntiCircleRotor: TryFrom<Flector> {
    associatedtype Self = AntiCircleRotor;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e4235 = flector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = flector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), flector.e45), 
            /* e15, e25, e35, scalar */
            float4(flector.group0.xyz, 0.0)
        );
    }
}
extension Flector: TryInto<AntiCircleRotor> {
    public func try_into() -> Optional<AntiCircleRotor> {
        return AntiCircleRotor.try_from(this);
    }
}

extension AntiDualNum: TryFrom<Flector> {
    associatedtype Self = AntiDualNum;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e15 = flector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = flector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = flector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = flector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e4235 = flector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(flector.e3215, 0.0), 0.0, 0.0)
        );
    }
}
extension Flector: TryInto<AntiDualNum> {
    public func try_into() -> Optional<AntiDualNum> {
        return AntiDualNum.try_from(this);
    }
}

extension AntiLine: TryFrom<Flector> {
    associatedtype Self = AntiLine;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e45 = flector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e4235 = flector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = flector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e15, e25, e35, 0 */
            float4(flector.group0.xyz, 0.0)
        );
    }
}
extension Flector: TryInto<AntiLine> {
    public func try_into() -> Optional<AntiLine> {
        return AntiLine.try_from(this);
    }
}

extension AntiMotor: TryFrom<Flector> {
    associatedtype Self = AntiMotor;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e45 = flector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e4235 = flector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(flector.e15, flector.e25, flector.e35, flector.e3215)
        );
    }
}
extension Flector: TryInto<AntiMotor> {
    public func try_into() -> Optional<AntiMotor> {
        return AntiMotor.try_from(this);
    }
}

extension Dipole: TryFrom<Flector> {
    associatedtype Self = Dipole;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e4235 = flector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = flector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), flector.e45), 
            /* e15, e25, e35, 0 */
            float4(flector.group0.xyz, 0.0)
        );
    }
}
extension Flector: TryInto<Dipole> {
    public func try_into() -> Optional<Dipole> {
        return Dipole.try_from(this);
    }
}

extension FlatPoint: TryFrom<Flector> {
    associatedtype Self = FlatPoint;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e4235 = flector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = flector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            flector.group0
        );
    }
}
extension Flector: TryInto<FlatPoint> {
    public func try_into() -> Optional<FlatPoint> {
        return FlatPoint.try_from(this);
    }
}

extension Plane: TryFrom<Flector> {
    associatedtype Self = Plane;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e15 = flector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = flector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = flector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = flector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            flector.group1
        );
    }
}
extension Flector: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

extension Sphere: TryFrom<Flector> {
    associatedtype Self = Sphere;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e15 = flector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = flector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = flector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = flector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            flector.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: TryInto<Sphere> {
    public func try_into() -> Optional<Sphere> {
        return Sphere.try_from(this);
    }
}

extension AntiFlatPoint: TryFrom<Line> {
    associatedtype Self = AntiFlatPoint;
    static func try_from(line: Line) -> Option<Self> {
        float disallowed_e415 = line.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = line.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = line.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(line.group1.xyz, 0.0)
        );
    }
}
extension Line: TryInto<AntiFlatPoint> {
    public func try_into() -> Optional<AntiFlatPoint> {
        return AntiFlatPoint.try_from(this);
    }
}

extension AntiFlector: TryFrom<Line> {
    associatedtype Self = AntiFlector;
    static func try_from(line: Line) -> Option<Self> {
        float disallowed_e415 = line.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = line.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = line.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(line.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension Line: TryInto<AntiFlector> {
    public func try_into() -> Optional<AntiFlector> {
        return AntiFlector.try_from(this);
    }
}

extension AntiDipoleInversion: TryFrom<Motor> {
    associatedtype Self = AntiDipoleInversion;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e12345 = motor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(motor.group0.xyz, 0.0), 
            /* e235, e315, e125, e4 */
            float4(motor.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), motor.e5)
        );
    }
}
extension Motor: TryInto<AntiDipoleInversion> {
    public func try_into() -> Optional<AntiDipoleInversion> {
        return AntiDipoleInversion.try_from(this);
    }
}

extension AntiFlatPoint: TryFrom<Motor> {
    associatedtype Self = AntiFlatPoint;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e415 = motor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e12345 = motor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e5 = motor.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(motor.group1.xyz, 0.0)
        );
    }
}
extension Motor: TryInto<AntiFlatPoint> {
    public func try_into() -> Optional<AntiFlatPoint> {
        return AntiFlatPoint.try_from(this);
    }
}

extension AntiFlector: TryFrom<Motor> {
    associatedtype Self = AntiFlector;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e415 = motor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e12345 = motor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(motor.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), motor.e5)
        );
    }
}
extension Motor: TryInto<AntiFlector> {
    public func try_into() -> Optional<AntiFlector> {
        return AntiFlector.try_from(this);
    }
}

extension AntiPlane: TryFrom<Motor> {
    associatedtype Self = AntiPlane;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e415 = motor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e12345 = motor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e235 = motor.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = motor.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = motor.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), motor.e5)
        );
    }
}
extension Motor: TryInto<AntiPlane> {
    public func try_into() -> Optional<AntiPlane> {
        return AntiPlane.try_from(this);
    }
}

extension AntiScalar: TryFrom<Motor> {
    associatedtype Self = AntiScalar;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e415 = motor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e235 = motor.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = motor.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = motor.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e5 = motor.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(motor.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

extension Circle: TryFrom<Motor> {
    associatedtype Self = Circle;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e12345 = motor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e5 = motor.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(motor.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(motor.group1.xyz, 0.0)
        );
    }
}
extension Motor: TryInto<Circle> {
    public func try_into() -> Optional<Circle> {
        return Circle.try_from(this);
    }
}

extension CircleRotor: TryFrom<Motor> {
    associatedtype Self = CircleRotor;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e5 = motor.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(motor.group0.xyz, 0.0), 
            /* e235, e315, e125, e12345 */
            float4(motor.e235, motor.e315, motor.e125, motor.e12345)
        );
    }
}
extension Motor: TryInto<CircleRotor> {
    public func try_into() -> Optional<CircleRotor> {
        return CircleRotor.try_from(this);
    }
}

extension DualNum: TryFrom<Motor> {
    associatedtype Self = DualNum;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e415 = motor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e235 = motor.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = motor.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = motor.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(motor.e5, motor.e12345), 0.0, 0.0)
        );
    }
}
extension Motor: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

extension Line: TryFrom<Motor> {
    associatedtype Self = Line;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e12345 = motor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e5 = motor.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(motor.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(motor.group1.xyz, 0.0)
        );
    }
}
extension Motor: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

extension RoundPoint: TryFrom<Motor> {
    associatedtype Self = RoundPoint;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e415 = motor.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e12345 = motor.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e235 = motor.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = motor.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = motor.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(motor.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: TryInto<RoundPoint> {
    public func try_into() -> Optional<RoundPoint> {
        return RoundPoint.try_from(this);
    }
}

extension AntiCircleRotor: TryFrom<MultiVector> {
    associatedtype Self = AntiCircleRotor;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(multi_vector.group4.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(multi_vector.group5.xyz, multi_vector.e45), 
            /* e15, e25, e35, scalar */
            float4(multi_vector.e15, multi_vector.e25, multi_vector.e35, multi_vector.scalar)
        );
    }
}
extension MultiVector: TryInto<AntiCircleRotor> {
    public func try_into() -> Optional<AntiCircleRotor> {
        return AntiCircleRotor.try_from(this);
    }
}

extension AntiDipoleInversion: TryFrom<MultiVector> {
    associatedtype Self = AntiDipoleInversion;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(multi_vector.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125, e4 */
            float4(multi_vector.group8.xyz, multi_vector.e4), 
            /* e1, e2, e3, e5 */
            float4(multi_vector.e1, multi_vector.e2, multi_vector.e3, multi_vector.e5)
        );
    }
}
extension MultiVector: TryInto<AntiDipoleInversion> {
    public func try_into() -> Optional<AntiDipoleInversion> {
        return AntiDipoleInversion.try_from(this);
    }
}

extension AntiDualNum: TryFrom<MultiVector> {
    associatedtype Self = AntiDualNum;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(multi_vector.e3215, multi_vector.scalar), 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<AntiDualNum> {
    public func try_into() -> Optional<AntiDualNum> {
        return AntiDualNum.try_from(this);
    }
}

extension AntiFlatPoint: TryFrom<MultiVector> {
    associatedtype Self = AntiFlatPoint;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(multi_vector.group8.xyz, multi_vector.e321)
        );
    }
}
extension MultiVector: TryInto<AntiFlatPoint> {
    public func try_into() -> Optional<AntiFlatPoint> {
        return AntiFlatPoint.try_from(this);
    }
}

extension AntiFlector: TryFrom<MultiVector> {
    associatedtype Self = AntiFlector;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(multi_vector.group8.xyz, multi_vector.e321), 
            /* e1, e2, e3, e5 */
            float4(multi_vector.e1, multi_vector.e2, multi_vector.e3, multi_vector.e5)
        );
    }
}
extension MultiVector: TryInto<AntiFlector> {
    public func try_into() -> Optional<AntiFlector> {
        return AntiFlector.try_from(this);
    }
}

extension AntiLine: TryFrom<MultiVector> {
    associatedtype Self = AntiLine;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(multi_vector.group5.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(multi_vector.group3.xyz, 0.0)
        );
    }
}
extension MultiVector: TryInto<AntiLine> {
    public func try_into() -> Optional<AntiLine> {
        return AntiLine.try_from(this);
    }
}

extension AntiMotor: TryFrom<MultiVector> {
    associatedtype Self = AntiMotor;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(multi_vector.group5.xyz, multi_vector.scalar), 
            /* e15, e25, e35, e3215 */
            float4(multi_vector.e15, multi_vector.e25, multi_vector.e35, multi_vector.e3215)
        );
    }
}
extension MultiVector: TryInto<AntiMotor> {
    public func try_into() -> Optional<AntiMotor> {
        return AntiMotor.try_from(this);
    }
}

extension AntiPlane: TryFrom<MultiVector> {
    associatedtype Self = AntiPlane;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(multi_vector.e1, multi_vector.e2, multi_vector.e3, multi_vector.e5)
        );
    }
}
extension MultiVector: TryInto<AntiPlane> {
    public func try_into() -> Optional<AntiPlane> {
        return AntiPlane.try_from(this);
    }
}

extension AntiScalar: TryFrom<MultiVector> {
    associatedtype Self = AntiScalar;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(multi_vector.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

extension Circle: TryFrom<MultiVector> {
    associatedtype Self = Circle;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(multi_vector.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125, 0 */
            float4(multi_vector.group8.xyz, 0.0)
        );
    }
}
extension MultiVector: TryInto<Circle> {
    public func try_into() -> Optional<Circle> {
        return Circle.try_from(this);
    }
}

extension CircleRotor: TryFrom<MultiVector> {
    associatedtype Self = CircleRotor;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(multi_vector.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125, e12345 */
            float4(multi_vector.group8.xyz, multi_vector.e12345)
        );
    }
}
extension MultiVector: TryInto<CircleRotor> {
    public func try_into() -> Optional<CircleRotor> {
        return CircleRotor.try_from(this);
    }
}

extension Dipole: TryFrom<MultiVector> {
    associatedtype Self = Dipole;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(multi_vector.group4.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(multi_vector.group5.xyz, multi_vector.e45), 
            /* e15, e25, e35, 0 */
            float4(multi_vector.group3.xyz, 0.0)
        );
    }
}
extension MultiVector: TryInto<Dipole> {
    public func try_into() -> Optional<Dipole> {
        return Dipole.try_from(this);
    }
}

extension DipoleInversion: TryFrom<MultiVector> {
    associatedtype Self = DipoleInversion;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(multi_vector.group4.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(multi_vector.group5.xyz, multi_vector.e45), 
            /* e15, e25, e35, e1234 */
            float4(multi_vector.e15, multi_vector.e25, multi_vector.e35, multi_vector.e1234), 
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9
        );
    }
}
extension MultiVector: TryInto<DipoleInversion> {
    public func try_into() -> Optional<DipoleInversion> {
        return DipoleInversion.try_from(this);
    }
}

extension DualNum: TryFrom<MultiVector> {
    associatedtype Self = DualNum;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(multi_vector.e5, multi_vector.e12345), 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

extension FlatPoint: TryFrom<MultiVector> {
    associatedtype Self = FlatPoint;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            multi_vector.group3
        );
    }
}
extension MultiVector: TryInto<FlatPoint> {
    public func try_into() -> Optional<FlatPoint> {
        return FlatPoint.try_from(this);
    }
}

extension Flector: TryFrom<MultiVector> {
    associatedtype Self = Flector;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            multi_vector.group3, 
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9
        );
    }
}
extension MultiVector: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension Line: TryFrom<MultiVector> {
    associatedtype Self = Line;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(multi_vector.group6.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(multi_vector.group8.xyz, 0.0)
        );
    }
}
extension MultiVector: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

extension Motor: TryFrom<MultiVector> {
    associatedtype Self = Motor;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(multi_vector.e415, multi_vector.e425, multi_vector.e435, multi_vector.e12345), 
            /* e235, e315, e125, e5 */
            float4(multi_vector.group8.xyz, multi_vector.e5)
        );
    }
}
extension MultiVector: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension Plane: TryFrom<MultiVector> {
    associatedtype Self = Plane;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9
        );
    }
}
extension MultiVector: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

extension RoundPoint: TryFrom<MultiVector> {
    associatedtype Self = RoundPoint;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            multi_vector.group1, 
            /* e5, 0, 0, 0 */
            float4(multi_vector.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<RoundPoint> {
    public func try_into() -> Optional<RoundPoint> {
        return RoundPoint.try_from(this);
    }
}

extension Scalar: TryFrom<MultiVector> {
    associatedtype Self = Scalar;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(multi_vector.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
    }
}

extension Sphere: TryFrom<MultiVector> {
    associatedtype Self = Sphere;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9, 
            /* e1234, 0, 0, 0 */
            float4(multi_vector.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<Sphere> {
    public func try_into() -> Optional<Sphere> {
        return Sphere.try_from(this);
    }
}

extension VersorEven: TryFrom<MultiVector> {
    associatedtype Self = VersorEven;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(multi_vector.group7.xyz, multi_vector.e12345), 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125, e5 */
            float4(multi_vector.group8.xyz, multi_vector.e5), 
            /* e1, e2, e3, e4 */
            multi_vector.group1
        );
    }
}
extension MultiVector: TryInto<VersorEven> {
    public func try_into() -> Optional<VersorEven> {
        return VersorEven.try_from(this);
    }
}

extension VersorOdd: TryFrom<MultiVector> {
    associatedtype Self = VersorOdd;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_e12345 = multi_vector.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(multi_vector.group4.xyz, multi_vector.scalar), 
            /* e23, e31, e12, e45 */
            float4(multi_vector.group5.xyz, multi_vector.e45), 
            /* e15, e25, e35, e1234 */
            float4(multi_vector.e15, multi_vector.e25, multi_vector.e35, multi_vector.e1234), 
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9
        );
    }
}
extension MultiVector: TryInto<VersorOdd> {
    public func try_into() -> Optional<VersorOdd> {
        return VersorOdd.try_from(this);
    }
}

extension AntiDualNum: TryFrom<Plane> {
    associatedtype Self = AntiDualNum;
    static func try_from(plane: Plane) -> Option<Self> {
        float disallowed_e4235 = plane.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = plane.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = plane.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(plane.e3215, 0.0), 0.0, 0.0)
        );
    }
}
extension Plane: TryInto<AntiDualNum> {
    public func try_into() -> Optional<AntiDualNum> {
        return AntiDualNum.try_from(this);
    }
}

extension AntiMotor: TryFrom<Plane> {
    associatedtype Self = AntiMotor;
    static func try_from(plane: Plane) -> Option<Self> {
        float disallowed_e4235 = plane.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = plane.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = plane.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(float3(0.0), plane.e3215)
        );
    }
}
extension Plane: TryInto<AntiMotor> {
    public func try_into() -> Optional<AntiMotor> {
        return AntiMotor.try_from(this);
    }
}

extension AntiFlector: TryFrom<RoundPoint> {
    associatedtype Self = AntiFlector;
    static func try_from(round_point: RoundPoint) -> Option<Self> {
        float disallowed_e4 = round_point.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(round_point.e1, round_point.e2, round_point.e3, round_point.e5)
        );
    }
}
extension RoundPoint: TryInto<AntiFlector> {
    public func try_into() -> Optional<AntiFlector> {
        return AntiFlector.try_from(this);
    }
}

extension AntiPlane: TryFrom<RoundPoint> {
    associatedtype Self = AntiPlane;
    static func try_from(round_point: RoundPoint) -> Option<Self> {
        float disallowed_e4 = round_point.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(round_point.e1, round_point.e2, round_point.e3, round_point.e5)
        );
    }
}
extension RoundPoint: TryInto<AntiPlane> {
    public func try_into() -> Optional<AntiPlane> {
        return AntiPlane.try_from(this);
    }
}

extension DualNum: TryFrom<RoundPoint> {
    associatedtype Self = DualNum;
    static func try_from(round_point: RoundPoint) -> Option<Self> {
        float disallowed_e1 = round_point.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = round_point.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = round_point.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = round_point.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(round_point.e5, 0.0), 0.0, 0.0)
        );
    }
}
extension RoundPoint: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

extension Motor: TryFrom<RoundPoint> {
    associatedtype Self = Motor;
    static func try_from(round_point: RoundPoint) -> Option<Self> {
        float disallowed_e1 = round_point.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = round_point.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = round_point.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = round_point.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), round_point.e5)
        );
    }
}
extension RoundPoint: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension AntiDualNum: TryFrom<Sphere> {
    associatedtype Self = AntiDualNum;
    static func try_from(sphere: Sphere) -> Option<Self> {
        float disallowed_e4235 = sphere.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = sphere.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = sphere.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e1234 = sphere.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(sphere.e3215, 0.0), 0.0, 0.0)
        );
    }
}
extension Sphere: TryInto<AntiDualNum> {
    public func try_into() -> Optional<AntiDualNum> {
        return AntiDualNum.try_from(this);
    }
}

extension AntiMotor: TryFrom<Sphere> {
    associatedtype Self = AntiMotor;
    static func try_from(sphere: Sphere) -> Option<Self> {
        float disallowed_e4235 = sphere.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = sphere.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = sphere.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e1234 = sphere.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(float3(0.0), sphere.e3215)
        );
    }
}
extension Sphere: TryInto<AntiMotor> {
    public func try_into() -> Optional<AntiMotor> {
        return AntiMotor.try_from(this);
    }
}

extension Flector: TryFrom<Sphere> {
    associatedtype Self = Flector;
    static func try_from(sphere: Sphere) -> Option<Self> {
        float disallowed_e1234 = sphere.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            sphere.group0
        );
    }
}
extension Sphere: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension Plane: TryFrom<Sphere> {
    associatedtype Self = Plane;
    static func try_from(sphere: Sphere) -> Option<Self> {
        float disallowed_e1234 = sphere.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            sphere.group0
        );
    }
}
extension Sphere: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

extension AntiDipoleInversion: TryFrom<VersorEven> {
    associatedtype Self = AntiDipoleInversion;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e12345 = versor_even.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(versor_even.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            versor_even.group1, 
            /* e235, e315, e125, e4 */
            float4(versor_even.e235, versor_even.e315, versor_even.e125, versor_even.e4), 
            /* e1, e2, e3, e5 */
            float4(versor_even.e1, versor_even.e2, versor_even.e3, versor_even.e5)
        );
    }
}
extension VersorEven: TryInto<AntiDipoleInversion> {
    public func try_into() -> Optional<AntiDipoleInversion> {
        return AntiDipoleInversion.try_from(this);
    }
}

extension AntiFlatPoint: TryFrom<VersorEven> {
    associatedtype Self = AntiFlatPoint;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e423 = versor_even.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e5 = versor_even.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(versor_even.e235, versor_even.e315, versor_even.e125, versor_even.e321)
        );
    }
}
extension VersorEven: TryInto<AntiFlatPoint> {
    public func try_into() -> Optional<AntiFlatPoint> {
        return AntiFlatPoint.try_from(this);
    }
}

extension AntiFlector: TryFrom<VersorEven> {
    associatedtype Self = AntiFlector;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e423 = versor_even.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(versor_even.e235, versor_even.e315, versor_even.e125, versor_even.e321), 
            /* e1, e2, e3, e5 */
            float4(versor_even.e1, versor_even.e2, versor_even.e3, versor_even.e5)
        );
    }
}
extension VersorEven: TryInto<AntiFlector> {
    public func try_into() -> Optional<AntiFlector> {
        return AntiFlector.try_from(this);
    }
}

extension AntiPlane: TryFrom<VersorEven> {
    associatedtype Self = AntiPlane;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e423 = versor_even.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e235 = versor_even.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = versor_even.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = versor_even.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(versor_even.e1, versor_even.e2, versor_even.e3, versor_even.e5)
        );
    }
}
extension VersorEven: TryInto<AntiPlane> {
    public func try_into() -> Optional<AntiPlane> {
        return AntiPlane.try_from(this);
    }
}

extension AntiScalar: TryFrom<VersorEven> {
    associatedtype Self = AntiScalar;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e423 = versor_even.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e235 = versor_even.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = versor_even.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = versor_even.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e5 = versor_even.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(versor_even.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

extension Circle: TryFrom<VersorEven> {
    associatedtype Self = Circle;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e12345 = versor_even.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e5 = versor_even.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(versor_even.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            versor_even.group1, 
            /* e235, e315, e125, 0 */
            float4(versor_even.group2.xyz, 0.0)
        );
    }
}
extension VersorEven: TryInto<Circle> {
    public func try_into() -> Optional<Circle> {
        return Circle.try_from(this);
    }
}

extension CircleRotor: TryFrom<VersorEven> {
    associatedtype Self = CircleRotor;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e5 = versor_even.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(versor_even.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            versor_even.group1, 
            /* e235, e315, e125, e12345 */
            float4(versor_even.e235, versor_even.e315, versor_even.e125, versor_even.e12345)
        );
    }
}
extension VersorEven: TryInto<CircleRotor> {
    public func try_into() -> Optional<CircleRotor> {
        return CircleRotor.try_from(this);
    }
}

extension DualNum: TryFrom<VersorEven> {
    associatedtype Self = DualNum;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e423 = versor_even.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e235 = versor_even.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = versor_even.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = versor_even.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(versor_even.e5, versor_even.e12345), 0.0, 0.0)
        );
    }
}
extension VersorEven: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

extension Line: TryFrom<VersorEven> {
    associatedtype Self = Line;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e423 = versor_even.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e5 = versor_even.e5;
        if disallowed_e5 != 0.0 {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(versor_even.group1.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(versor_even.group2.xyz, 0.0)
        );
    }
}
extension VersorEven: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

extension Motor: TryFrom<VersorEven> {
    associatedtype Self = Motor;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e423 = versor_even.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(versor_even.e415, versor_even.e425, versor_even.e435, versor_even.e12345), 
            /* e235, e315, e125, e5 */
            versor_even.group2
        );
    }
}
extension VersorEven: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension RoundPoint: TryFrom<VersorEven> {
    associatedtype Self = RoundPoint;
    static func try_from(versor_even: VersorEven) -> Option<Self> {
        float disallowed_e423 = versor_even.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if disallowed_e12345 != 0.0 {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if disallowed_e415 != 0.0 {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if disallowed_e425 != 0.0 {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if disallowed_e435 != 0.0 {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        float disallowed_e235 = versor_even.e235;
        if disallowed_e235 != 0.0 {
            return none;
        }
        float disallowed_e315 = versor_even.e315;
        if disallowed_e315 != 0.0 {
            return none;
        }
        float disallowed_e125 = versor_even.e125;
        if disallowed_e125 != 0.0 {
            return none;
        }
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            versor_even.group3, 
            /* e5, 0, 0, 0 */
            float4(versor_even.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: TryInto<RoundPoint> {
    public func try_into() -> Optional<RoundPoint> {
        return RoundPoint.try_from(this);
    }
}

extension AntiCircleRotor: TryFrom<VersorOdd> {
    associatedtype Self = AntiCircleRotor;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_e1234 = versor_odd.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(versor_odd.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            versor_odd.group1, 
            /* e15, e25, e35, scalar */
            float4(versor_odd.e15, versor_odd.e25, versor_odd.e35, versor_odd.scalar)
        );
    }
}
extension VersorOdd: TryInto<AntiCircleRotor> {
    public func try_into() -> Optional<AntiCircleRotor> {
        return AntiCircleRotor.try_from(this);
    }
}

extension AntiDualNum: TryFrom<VersorOdd> {
    associatedtype Self = AntiDualNum;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_e41 = versor_odd.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e15 = versor_odd.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = versor_odd.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = versor_odd.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(versor_odd.e3215, versor_odd.scalar), 0.0, 0.0)
        );
    }
}
extension VersorOdd: TryInto<AntiDualNum> {
    public func try_into() -> Optional<AntiDualNum> {
        return AntiDualNum.try_from(this);
    }
}

extension AntiLine: TryFrom<VersorOdd> {
    associatedtype Self = AntiLine;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_e41 = versor_odd.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(versor_odd.group1.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(versor_odd.group2.xyz, 0.0)
        );
    }
}
extension VersorOdd: TryInto<AntiLine> {
    public func try_into() -> Optional<AntiLine> {
        return AntiLine.try_from(this);
    }
}

extension AntiMotor: TryFrom<VersorOdd> {
    associatedtype Self = AntiMotor;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_e41 = versor_odd.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(versor_odd.e23, versor_odd.e31, versor_odd.e12, versor_odd.scalar), 
            /* e15, e25, e35, e3215 */
            float4(versor_odd.e15, versor_odd.e25, versor_odd.e35, versor_odd.e3215)
        );
    }
}
extension VersorOdd: TryInto<AntiMotor> {
    public func try_into() -> Optional<AntiMotor> {
        return AntiMotor.try_from(this);
    }
}

extension Dipole: TryFrom<VersorOdd> {
    associatedtype Self = Dipole;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_scalar = versor_odd.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(versor_odd.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            versor_odd.group1, 
            /* e15, e25, e35, 0 */
            float4(versor_odd.group2.xyz, 0.0)
        );
    }
}
extension VersorOdd: TryInto<Dipole> {
    public func try_into() -> Optional<Dipole> {
        return Dipole.try_from(this);
    }
}

extension DipoleInversion: TryFrom<VersorOdd> {
    associatedtype Self = DipoleInversion;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_scalar = versor_odd.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(versor_odd.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            versor_odd.group1, 
            /* e15, e25, e35, e1234 */
            versor_odd.group2, 
            /* e4235, e4315, e4125, e3215 */
            versor_odd.group3
        );
    }
}
extension VersorOdd: TryInto<DipoleInversion> {
    public func try_into() -> Optional<DipoleInversion> {
        return DipoleInversion.try_from(this);
    }
}

extension FlatPoint: TryFrom<VersorOdd> {
    associatedtype Self = FlatPoint;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_e41 = versor_odd.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(versor_odd.e15, versor_odd.e25, versor_odd.e35, versor_odd.e45)
        );
    }
}
extension VersorOdd: TryInto<FlatPoint> {
    public func try_into() -> Optional<FlatPoint> {
        return FlatPoint.try_from(this);
    }
}

extension Flector: TryFrom<VersorOdd> {
    associatedtype Self = Flector;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_e41 = versor_odd.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(versor_odd.e15, versor_odd.e25, versor_odd.e35, versor_odd.e45), 
            /* e4235, e4315, e4125, e3215 */
            versor_odd.group3
        );
    }
}
extension VersorOdd: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension Plane: TryFrom<VersorOdd> {
    associatedtype Self = Plane;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_e41 = versor_odd.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e15 = versor_odd.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = versor_odd.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = versor_odd.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            versor_odd.group3
        );
    }
}
extension VersorOdd: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

extension Scalar: TryFrom<VersorOdd> {
    associatedtype Self = Scalar;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_e41 = versor_odd.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e15 = versor_odd.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = versor_odd.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = versor_odd.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if disallowed_e4235 != 0.0 {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if disallowed_e4315 != 0.0 {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if disallowed_e4125 != 0.0 {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if disallowed_e3215 != 0.0 {
            return none;
        }
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(versor_odd.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
    }
}

extension Sphere: TryFrom<VersorOdd> {
    associatedtype Self = Sphere;
    static func try_from(versor_odd: VersorOdd) -> Option<Self> {
        float disallowed_e41 = versor_odd.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if disallowed_e45 != 0.0 {
            return none;
        }
        float disallowed_e15 = versor_odd.e15;
        if disallowed_e15 != 0.0 {
            return none;
        }
        float disallowed_e25 = versor_odd.e25;
        if disallowed_e25 != 0.0 {
            return none;
        }
        float disallowed_e35 = versor_odd.e35;
        if disallowed_e35 != 0.0 {
            return none;
        }
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            versor_odd.group3, 
            /* e1234, 0, 0, 0 */
            float4(versor_odd.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: TryInto<Sphere> {
    public func try_into() -> Optional<Sphere> {
        return Sphere.try_from(this);
    }
}
