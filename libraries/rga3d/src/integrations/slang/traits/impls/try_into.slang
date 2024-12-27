
extension AntiScalar: TryFrom<DualNum> {
    associatedtype Self = AntiScalar;
    static func try_from(dual_num: DualNum) -> Option<Self> {
        float disallowed_scalar = dual_num.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(dual_num.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

extension Scalar: TryFrom<DualNum> {
    associatedtype Self = Scalar;
    static func try_from(dual_num: DualNum) -> Option<Self> {
        float disallowed_e1234 = dual_num.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(dual_num.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
    }
}

extension Horizon: TryFrom<Flector> {
    associatedtype Self = Horizon;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e1 = flector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = flector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = flector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = flector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        float disallowed_e423 = flector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = flector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = flector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(flector.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: TryInto<Horizon> {
    public func try_into() -> Optional<Horizon> {
        return Horizon.try_from(this);
    }
}

extension Origin: TryFrom<Flector> {
    associatedtype Self = Origin;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e1 = flector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = flector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = flector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e423 = flector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = flector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = flector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e321 = flector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(flector.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: TryInto<Origin> {
    public func try_into() -> Optional<Origin> {
        return Origin.try_from(this);
    }
}

extension Plane: TryFrom<Flector> {
    associatedtype Self = Plane;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e1 = flector.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = flector.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = flector.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        float disallowed_e4 = flector.e4;
        if disallowed_e4 != 0.0 {
            return none;
        }
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            flector.group1
        );
    }
}
extension Flector: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

extension Point: TryFrom<Flector> {
    associatedtype Self = Point;
    static func try_from(flector: Flector) -> Option<Self> {
        float disallowed_e423 = flector.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = flector.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = flector.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        float disallowed_e321 = flector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            flector.group0
        );
    }
}
extension Flector: TryInto<Point> {
    public func try_into() -> Optional<Point> {
        return Point.try_from(this);
    }
}

extension AntiScalar: TryFrom<Motor> {
    associatedtype Self = AntiScalar;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e41 = motor.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = motor.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = motor.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = motor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = motor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = motor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        float disallowed_scalar = motor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(motor.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

extension DualNum: TryFrom<Motor> {
    associatedtype Self = DualNum;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e41 = motor.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = motor.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = motor.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e23 = motor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = motor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = motor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(motor.scalar, motor.e1234), 0.0, 0.0)
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
        float disallowed_e1234 = motor.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_scalar = motor.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(motor.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(motor.group1.xyz, 0.0)
        );
    }
}
extension Motor: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

extension Scalar: TryFrom<Motor> {
    associatedtype Self = Scalar;
    static func try_from(motor: Motor) -> Option<Self> {
        float disallowed_e41 = motor.e41;
        if disallowed_e41 != 0.0 {
            return none;
        }
        float disallowed_e42 = motor.e42;
        if disallowed_e42 != 0.0 {
            return none;
        }
        float disallowed_e43 = motor.e43;
        if disallowed_e43 != 0.0 {
            return none;
        }
        float disallowed_e1234 = motor.e1234;
        if disallowed_e1234 != 0.0 {
            return none;
        }
        float disallowed_e23 = motor.e23;
        if disallowed_e23 != 0.0 {
            return none;
        }
        float disallowed_e31 = motor.e31;
        if disallowed_e31 != 0.0 {
            return none;
        }
        float disallowed_e12 = motor.e12;
        if disallowed_e12 != 0.0 {
            return none;
        }
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(motor.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
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
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(multi_vector.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

extension DualNum: TryFrom<MultiVector> {
    associatedtype Self = DualNum;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
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
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(multi_vector.group0.xy, 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

extension Flector: TryFrom<MultiVector> {
    associatedtype Self = Flector;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
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
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            multi_vector.group1, 
            /* e423, e431, e412, e321 */
            multi_vector.group4
        );
    }
}
extension MultiVector: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

extension Horizon: TryFrom<MultiVector> {
    associatedtype Self = Horizon;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
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
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(multi_vector.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<Horizon> {
    public func try_into() -> Optional<Horizon> {
        return Horizon.try_from(this);
    }
}

extension Line: TryFrom<MultiVector> {
    associatedtype Self = Line;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
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
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(multi_vector.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(multi_vector.group3.xyz, 0.0)
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
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(multi_vector.group2.xyz, multi_vector.e1234), 
            /* e23, e31, e12, scalar */
            float4(multi_vector.group3.xyz, multi_vector.scalar)
        );
    }
}
extension MultiVector: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

extension Origin: TryFrom<MultiVector> {
    associatedtype Self = Origin;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
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
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(multi_vector.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: TryInto<Origin> {
    public func try_into() -> Optional<Origin> {
        return Origin.try_from(this);
    }
}

extension Plane: TryFrom<MultiVector> {
    associatedtype Self = Plane;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
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
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            multi_vector.group4
        );
    }
}
extension MultiVector: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

extension Point: TryFrom<MultiVector> {
    associatedtype Self = Point;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_scalar = multi_vector.scalar;
        if disallowed_scalar != 0.0 {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
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
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
            return none;
        }
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            multi_vector.group1
        );
    }
}
extension MultiVector: TryInto<Point> {
    public func try_into() -> Optional<Point> {
        return Point.try_from(this);
    }
}

extension Scalar: TryFrom<MultiVector> {
    associatedtype Self = Scalar;
    static func try_from(multi_vector: MultiVector) -> Option<Self> {
        float disallowed_e1234 = multi_vector.e1234;
        if disallowed_e1234 != 0.0 {
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
        float disallowed_e321 = multi_vector.e321;
        if disallowed_e321 != 0.0 {
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

extension Horizon: TryFrom<Plane> {
    associatedtype Self = Horizon;
    static func try_from(plane: Plane) -> Option<Self> {
        float disallowed_e423 = plane.e423;
        if disallowed_e423 != 0.0 {
            return none;
        }
        float disallowed_e431 = plane.e431;
        if disallowed_e431 != 0.0 {
            return none;
        }
        float disallowed_e412 = plane.e412;
        if disallowed_e412 != 0.0 {
            return none;
        }
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(plane.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: TryInto<Horizon> {
    public func try_into() -> Optional<Horizon> {
        return Horizon.try_from(this);
    }
}

extension Origin: TryFrom<Point> {
    associatedtype Self = Origin;
    static func try_from(point: Point) -> Option<Self> {
        float disallowed_e1 = point.e1;
        if disallowed_e1 != 0.0 {
            return none;
        }
        float disallowed_e2 = point.e2;
        if disallowed_e2 != 0.0 {
            return none;
        }
        float disallowed_e3 = point.e3;
        if disallowed_e3 != 0.0 {
            return none;
        }
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(point.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Point: TryInto<Origin> {
    public func try_into() -> Optional<Origin> {
        return Origin.try_from(this);
    }
}
