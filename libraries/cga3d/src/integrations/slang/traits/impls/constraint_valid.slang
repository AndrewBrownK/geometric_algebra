implementing cga3d;
using traits;
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.constraint_valid();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint constraint_valid() {
        return this;
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public AntiPlane operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.constraint_valid();
    }
}
public extension AntiPlane {
    public AntiPlane constraint_valid() {
        return this;
    }
}
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
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.constraint_valid();
    }
}
public extension FlatPoint {
    public FlatPoint constraint_valid() {
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
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.constraint_valid();
    }
}
public extension RoundPoint {
    public RoundPoint constraint_valid() {
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
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(ConstraintValidPrefixOrPostfix rhs)  {
        return this.constraint_valid();
    }
}
public extension ConstraintValidPrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.constraint_valid();
    }
}
public extension Sphere {
    public Sphere constraint_valid() {
        return this;
    }
}
