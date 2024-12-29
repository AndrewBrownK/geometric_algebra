implementing rga3d;
using traits;

public extension AntiScalar: TryFrom<DualNum> {
    public static func try_from(DualNum dual_num) -> Optional<This> {
        float disallowed_scalar = dual_num.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return AntiScalar.from_groups(
            /* e1234 */
            dual_num.e1234
        );
    }
}
public extension DualNum: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

public extension Scalar: TryFrom<DualNum> {
    public static func try_from(DualNum dual_num) -> Optional<This> {
        float disallowed_e1234 = dual_num.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Scalar.from_groups(
            /* scalar */
            dual_num.scalar
        );
    }
}
public extension DualNum: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
    }
}

public extension Horizon: TryFrom<Flector> {
    public static func try_from(Flector flector) -> Optional<This> {
        float disallowed_e1 = flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = flector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e423 = flector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = flector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = flector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        return Horizon.from_groups(
            /* e321 */
            flector.e321
        );
    }
}
public extension Flector: TryInto<Horizon> {
    public func try_into() -> Optional<Horizon> {
        return Horizon.try_from(this);
    }
}

public extension Origin: TryFrom<Flector> {
    public static func try_from(Flector flector) -> Optional<This> {
        float disallowed_e1 = flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e423 = flector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = flector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = flector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = flector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Origin.from_groups(
            /* e4 */
            flector.e4
        );
    }
}
public extension Flector: TryInto<Origin> {
    public func try_into() -> Optional<Origin> {
        return Origin.try_from(this);
    }
}

public extension Plane: TryFrom<Flector> {
    public static func try_from(Flector flector) -> Optional<This> {
        float disallowed_e1 = flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = flector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            flector.group1
        );
    }
}
public extension Flector: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

public extension Point: TryFrom<Flector> {
    public static func try_from(Flector flector) -> Optional<This> {
        float disallowed_e423 = flector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = flector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = flector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = flector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            flector.group0
        );
    }
}
public extension Flector: TryInto<Point> {
    public func try_into() -> Optional<Point> {
        return Point.try_from(this);
    }
}

public extension AntiScalar: TryFrom<Motor> {
    public static func try_from(Motor motor) -> Optional<This> {
        float disallowed_e41 = motor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = motor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = motor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_scalar = motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return AntiScalar.from_groups(
            /* e1234 */
            motor.e1234
        );
    }
}
public extension Motor: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

public extension DualNum: TryFrom<Motor> {
    public static func try_from(Motor motor) -> Optional<This> {
        float disallowed_e41 = motor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = motor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = motor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(motor.scalar, motor.e1234)
        );
    }
}
public extension Motor: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

public extension Line: TryFrom<Motor> {
    public static func try_from(Motor motor) -> Optional<This> {
        float disallowed_e1234 = motor.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_scalar = motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return Line.from_groups(
            /* e41, e42, e43 */
            motor.group0.xyz, 
            /* e23, e31, e12 */
            motor.group1.xyz
        );
    }
}
public extension Motor: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

public extension Scalar: TryFrom<Motor> {
    public static func try_from(Motor motor) -> Optional<This> {
        float disallowed_e41 = motor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = motor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = motor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e1234 = motor.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e23 = motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return Scalar.from_groups(
            /* scalar */
            motor.scalar
        );
    }
}
public extension Motor: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
    }
}

public extension AntiScalar: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return AntiScalar.from_groups(
            /* e1234 */
            multi_vector.e1234
        );
    }
}
public extension MultiVector: TryInto<AntiScalar> {
    public func try_into() -> Optional<AntiScalar> {
        return AntiScalar.try_from(this);
    }
}

public extension DualNum: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return DualNum.from_groups(
            /* scalar, e1234 */
            multi_vector.group0.xy
        );
    }
}
public extension MultiVector: TryInto<DualNum> {
    public func try_into() -> Optional<DualNum> {
        return DualNum.try_from(this);
    }
}

public extension Flector: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
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
public extension MultiVector: TryInto<Flector> {
    public func try_into() -> Optional<Flector> {
        return Flector.try_from(this);
    }
}

public extension Horizon: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        return Horizon.from_groups(
            /* e321 */
            multi_vector.e321
        );
    }
}
public extension MultiVector: TryInto<Horizon> {
    public func try_into() -> Optional<Horizon> {
        return Horizon.try_from(this);
    }
}

public extension Line: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Line.from_groups(
            /* e41, e42, e43 */
            multi_vector.group2.xyz, 
            /* e23, e31, e12 */
            multi_vector.group3.xyz
        );
    }
}
public extension MultiVector: TryInto<Line> {
    public func try_into() -> Optional<Line> {
        return Line.try_from(this);
    }
}

public extension Motor: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
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
public extension MultiVector: TryInto<Motor> {
    public func try_into() -> Optional<Motor> {
        return Motor.try_from(this);
    }
}

public extension Origin: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Origin.from_groups(
            /* e4 */
            multi_vector.e4
        );
    }
}
public extension MultiVector: TryInto<Origin> {
    public func try_into() -> Optional<Origin> {
        return Origin.try_from(this);
    }
}

public extension Plane: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            multi_vector.group4
        );
    }
}
public extension MultiVector: TryInto<Plane> {
    public func try_into() -> Optional<Plane> {
        return Plane.try_from(this);
    }
}

public extension Point: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            multi_vector.group1
        );
    }
}
public extension MultiVector: TryInto<Point> {
    public func try_into() -> Optional<Point> {
        return Point.try_from(this);
    }
}

public extension Scalar: TryFrom<MultiVector> {
    public static func try_from(MultiVector multi_vector) -> Optional<This> {
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Scalar.from_groups(
            /* scalar */
            multi_vector.scalar
        );
    }
}
public extension MultiVector: TryInto<Scalar> {
    public func try_into() -> Optional<Scalar> {
        return Scalar.try_from(this);
    }
}

public extension Horizon: TryFrom<Plane> {
    public static func try_from(Plane plane) -> Optional<This> {
        float disallowed_e423 = plane.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = plane.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = plane.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        return Horizon.from_groups(
            /* e321 */
            plane.e321
        );
    }
}
public extension Plane: TryInto<Horizon> {
    public func try_into() -> Optional<Horizon> {
        return Horizon.try_from(this);
    }
}

public extension Origin: TryFrom<Point> {
    public static func try_from(Point point) -> Optional<This> {
        float disallowed_e1 = point.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = point.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = point.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        return Origin.from_groups(
            /* e4 */
            point.e4
        );
    }
}
public extension Point: TryInto<Origin> {
    public func try_into() -> Optional<Origin> {
        return Origin.try_from(this);
    }
}
