using traits;
using traits;
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: NormPrefixOrPostfix) -> DualNum {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.norm();
    }
}
public extension Flector: Norm {
    public typedef DualNum NormOutput;
    public func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: NormPrefixOrPostfix) -> DualNum {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.norm();
    }
}
public extension Line: Norm {
    public typedef DualNum NormOutput;
    public func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: NormPrefixOrPostfix) -> DualNum {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.norm();
    }
}
public extension Motor: Norm {
    public typedef DualNum NormOutput;
    public func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: NormPrefixOrPostfix) -> DualNum {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.norm();
    }
}
public extension MultiVector: Norm {
    public typedef DualNum NormOutput;
    public func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: NormPrefixOrPostfix) -> DualNum {
        return this.norm();
    }
}
public extension NormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.norm();
    }
}
public extension Point: Norm {
    public typedef DualNum NormOutput;
    public func norm() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e1234)
        );
    }
}
