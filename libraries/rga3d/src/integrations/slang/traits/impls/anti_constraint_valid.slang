implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension AntiScalar {
    public AntiScalar anti_constraint_valid() {
        return this;
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Horizon operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension Horizon {
    public Horizon anti_constraint_valid() {
        return this;
    }
}
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension Origin {
    public Origin anti_constraint_valid() {
        return this;
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension Plane {
    public Plane anti_constraint_valid() {
        return this;
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension Point {
    public Point anti_constraint_valid() {
        return this;
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension Scalar {
    public Scalar anti_constraint_valid() {
        return this;
    }
}
