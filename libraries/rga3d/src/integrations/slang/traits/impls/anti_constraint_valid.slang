implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiConstraintValidPrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
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
    public Horizon operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiConstraintValidPrefixOrPostfix& rhs) -> Horizon& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
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
    public Origin operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiConstraintValidPrefixOrPostfix& rhs) -> Origin& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
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
    public Plane operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiConstraintValidPrefixOrPostfix& rhs) -> Plane& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
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
    public Point operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiConstraintValidPrefixOrPostfix& rhs) -> Point& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
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
    public Scalar operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiConstraintValidPrefixOrPostfix& rhs) -> Scalar& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension Scalar: AntiConstraintValid {
    public typedef Scalar AntiConstraintValidOutput;
    public func anti_constraint_valid() -> Scalar {
        return this;
    }
}
