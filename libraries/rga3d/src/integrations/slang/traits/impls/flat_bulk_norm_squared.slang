implementing rga3d;
using traits;
public extension Flector {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Flector rhs) {
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
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Line rhs) {
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
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Motor rhs) {
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
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
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
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Point rhs) {
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
