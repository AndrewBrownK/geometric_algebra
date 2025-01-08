implementing cga3d;
using traits;
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_constraint_valid() {
        return this;
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public AntiPlane operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension AntiPlane {
    public AntiPlane anti_constraint_valid() {
        return this;
    }
}
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
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension FlatPoint {
    public FlatPoint anti_constraint_valid() {
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
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension RoundPoint {
    public RoundPoint anti_constraint_valid() {
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
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(AntiConstraintValidPrefixOrPostfix rhs)  {
        return this.anti_constraint_valid();
    }
}
public extension AntiConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.anti_constraint_valid();
    }
}
public extension Sphere {
    public Sphere anti_constraint_valid() {
        return this;
    }
}
