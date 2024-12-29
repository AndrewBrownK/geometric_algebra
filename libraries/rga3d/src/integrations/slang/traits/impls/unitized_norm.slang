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
public extension Flector: UnitizedNorm {
    public typedef float UnitizedNormOutput;
    public func unitized_norm() -> float {
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
public extension Line: UnitizedNorm {
    public typedef float UnitizedNormOutput;
    public func unitized_norm() -> float {
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
public extension Motor: UnitizedNorm {
    public typedef float UnitizedNormOutput;
    public func unitized_norm() -> float {
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
public extension MultiVector: UnitizedNorm {
    public typedef float UnitizedNormOutput;
    public func unitized_norm() -> float {
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
public extension Point: UnitizedNorm {
    public typedef float UnitizedNormOutput;
    public func unitized_norm() -> float {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
