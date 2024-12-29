implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConstraintValidPrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
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
    public Horizon operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConstraintValidPrefixOrPostfix& rhs) -> Horizon& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
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
    public Origin operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConstraintValidPrefixOrPostfix& rhs) -> Origin& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
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
    public Plane operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConstraintValidPrefixOrPostfix& rhs) -> Plane& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
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
    public Point operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConstraintValidPrefixOrPostfix& rhs) -> Point& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
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
    public Scalar operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConstraintValidPrefixOrPostfix& rhs) -> Scalar& {
        this = this.constraint_valid();
        return *this;
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.constraint_valid();
    }
}
public extension Scalar: ConstraintValid {
    public typedef Scalar ConstraintValidOutput;
    public func constraint_valid() -> Scalar {
        return this;
    }
}
