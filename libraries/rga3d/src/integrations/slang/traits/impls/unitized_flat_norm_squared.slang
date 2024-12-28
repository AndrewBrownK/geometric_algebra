using traits;
using traits;
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Flector: UnitizedFlatNormSquared {
    public typedef float UnitizedFlatNormSquaredOutput;
    public func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Line: UnitizedFlatNormSquared {
    public typedef float UnitizedFlatNormSquaredOutput;
    public func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Motor: UnitizedFlatNormSquared {
    public typedef float UnitizedFlatNormSquaredOutput;
    public func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension MultiVector: UnitizedFlatNormSquared {
    public typedef float UnitizedFlatNormSquaredOutput;
    public func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Point: UnitizedFlatNormSquared {
    public typedef float UnitizedFlatNormSquaredOutput;
    public func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
