using traits;
using traits;
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.norm_squared();
    }
}
public extension Flector: NormSquared {
    public typedef DualNum NormSquaredOutput;
    public func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.norm_squared();
    }
}
public extension Line: NormSquared {
    public typedef DualNum NormSquaredOutput;
    public func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.norm_squared();
    }
}
public extension Motor: NormSquared {
    public typedef DualNum NormSquaredOutput;
    public func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.norm_squared();
    }
}
public extension MultiVector: NormSquared {
    public typedef DualNum NormSquaredOutput;
    public func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: NormSquaredPrefixOrPostfix) -> DualNum {
        return this.norm_squared();
    }
}
public extension NormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.norm_squared();
    }
}
public extension Point: NormSquared {
    public typedef DualNum NormSquaredOutput;
    public func norm_squared() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e1234)
        );
    }
}
