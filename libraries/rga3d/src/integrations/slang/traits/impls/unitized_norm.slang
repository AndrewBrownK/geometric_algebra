implementing rga3d;
using traits;
public extension Flector {
    // Fancy postfix trick
    public float operator /(UnitizedNormPrefixOrPostfix rhs)  {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Flector rhs) {
        return rhs.unitized_norm();
    }
}
public extension Flector {
    public float unitized_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Line {
    // Fancy postfix trick
    public float operator /(UnitizedNormPrefixOrPostfix rhs)  {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Line rhs) {
        return rhs.unitized_norm();
    }
}
public extension Line {
    public float unitized_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Motor {
    // Fancy postfix trick
    public float operator /(UnitizedNormPrefixOrPostfix rhs)  {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Motor rhs) {
        return rhs.unitized_norm();
    }
}
public extension Motor {
    public float unitized_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public float operator /(UnitizedNormPrefixOrPostfix rhs)  {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(MultiVector rhs) {
        return rhs.unitized_norm();
    }
}
public extension MultiVector {
    public float unitized_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Point {
    // Fancy postfix trick
    public float operator /(UnitizedNormPrefixOrPostfix rhs)  {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Point rhs) {
        return rhs.unitized_norm();
    }
}
public extension Point {
    public float unitized_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
