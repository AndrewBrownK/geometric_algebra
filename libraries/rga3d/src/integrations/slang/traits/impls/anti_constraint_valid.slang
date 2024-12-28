public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> AntiScalar {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.anti_constraint_valid();
    }
}
public extension AntiScalar: AntiConstraintValid {
    public typedef AntiScalar AntiConstraintValidOutput;
    public func anti_constraint_valid() -> AntiScalar {
        return this;
    }
}
public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Horizon {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.anti_constraint_valid();
    }
}
public extension Horizon: AntiConstraintValid {
    public typedef Horizon AntiConstraintValidOutput;
    public func anti_constraint_valid() -> Horizon {
        return this;
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Origin {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Origin& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.anti_constraint_valid();
    }
}
public extension Origin: AntiConstraintValid {
    public typedef Origin AntiConstraintValidOutput;
    public func anti_constraint_valid() -> Origin {
        return this;
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Plane {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Plane& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.anti_constraint_valid();
    }
}
public extension Plane: AntiConstraintValid {
    public typedef Plane AntiConstraintValidOutput;
    public func anti_constraint_valid() -> Plane {
        return this;
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Point {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Point& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.anti_constraint_valid();
    }
}
public extension Point: AntiConstraintValid {
    public typedef Point AntiConstraintValidOutput;
    public func anti_constraint_valid() -> Point {
        return this;
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.anti_constraint_valid();
    }
}
public extension Scalar: AntiConstraintValid {
    public typedef Scalar AntiConstraintValidOutput;
    public func anti_constraint_valid() -> Scalar {
        return this;
    }
}
