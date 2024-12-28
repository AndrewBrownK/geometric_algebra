using traits;
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
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
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
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
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
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
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
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
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
public extension UnitizedNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.unitized_norm();
    }
}
public extension Point: UnitizedNorm {
    public typedef float UnitizedNormOutput;
    public func unitized_norm() -> float {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
