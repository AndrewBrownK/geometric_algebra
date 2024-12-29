implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.constraint_valid();
    }
}
public extension AntiScalar {
    public AntiScalar constraint_valid() {
        return this;
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Horizon operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.constraint_valid();
    }
}
public extension Horizon {
    public Horizon constraint_valid() {
        return this;
    }
}
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.constraint_valid();
    }
}
public extension Origin {
    public Origin constraint_valid() {
        return this;
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.constraint_valid();
    }
}
public extension Plane {
    public Plane constraint_valid() {
        return this;
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.constraint_valid();
    }
}
public extension Point {
    public Point constraint_valid() {
        return this;
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.constraint_valid();
    }
}
public extension Scalar {
    public Scalar constraint_valid() {
        return this;
    }
}
