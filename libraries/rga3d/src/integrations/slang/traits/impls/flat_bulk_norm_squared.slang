using traits;
using traits;
using traits;
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.flat_bulk_norm_squared();
    }
}
public extension Flector: FlatBulkNormSquared {
    public typedef Scalar FlatBulkNormSquaredOutput;
    public func flat_bulk_norm_squared() -> Scalar {
        Motor flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.flat_bulk_norm_squared();
    }
}
public extension Line: FlatBulkNormSquared {
    public typedef Scalar FlatBulkNormSquaredOutput;
    public func flat_bulk_norm_squared() -> Scalar {
        Plane flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.flat_bulk_norm_squared();
    }
}
public extension Motor: FlatBulkNormSquared {
    public typedef Scalar FlatBulkNormSquaredOutput;
    public func flat_bulk_norm_squared() -> Scalar {
        Flector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.flat_bulk_norm_squared();
    }
}
public extension MultiVector: FlatBulkNormSquared {
    public typedef Scalar FlatBulkNormSquaredOutput;
    public func flat_bulk_norm_squared() -> Scalar {
        MultiVector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.flat_bulk_norm_squared();
    }
}
public extension Point: FlatBulkNormSquared {
    public typedef Scalar FlatBulkNormSquaredOutput;
    public func flat_bulk_norm_squared() -> Scalar {
        Line flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
