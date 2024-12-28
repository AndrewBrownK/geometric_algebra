public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> AntiScalar {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.constraint_valid();
    }
}
public extension AntiScalar: ConstraintValid {
    public typedef AntiScalar ConstraintValidOutput;
    public func constraint_valid() -> AntiScalar {
        return this;
    }
}
public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Horizon {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.constraint_valid();
    }
}
public extension Horizon: ConstraintValid {
    public typedef Horizon ConstraintValidOutput;
    public func constraint_valid() -> Horizon {
        return this;
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Origin {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Origin& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.constraint_valid();
    }
}
public extension Origin: ConstraintValid {
    public typedef Origin ConstraintValidOutput;
    public func constraint_valid() -> Origin {
        return this;
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Plane {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Plane& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.constraint_valid();
    }
}
public extension Plane: ConstraintValid {
    public typedef Plane ConstraintValidOutput;
    public func constraint_valid() -> Plane {
        return this;
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Point {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Point& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.constraint_valid();
    }
}
public extension Point: ConstraintValid {
    public typedef Point ConstraintValidOutput;
    public func constraint_valid() -> Point {
        return this;
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Scalar {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.constraint_valid();
    }
}
public extension Scalar: ConstraintValid {
    public typedef Scalar ConstraintValidOutput;
    public func constraint_valid() -> Scalar {
        return this;
    }
}
