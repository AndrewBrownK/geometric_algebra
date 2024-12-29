implementing rga3d;
using traits;
public extension Flector {
    // Fancy postfix trick
    public Scalar operator /(BulkNormSquaredPrefixOrPostfix rhs)  {
        return this.bulk_norm_squared();
    }
}
public extension BulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Flector rhs) {
        return rhs.bulk_norm_squared();
    }
}
public extension Flector: BulkNormSquared {
    public typedef Scalar BulkNormSquaredOutput;
    public func bulk_norm_squared() -> Scalar {
        Motor flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
public extension Line {
    // Fancy postfix trick
    public Scalar operator /(BulkNormSquaredPrefixOrPostfix rhs)  {
        return this.bulk_norm_squared();
    }
}
public extension BulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Line rhs) {
        return rhs.bulk_norm_squared();
    }
}
public extension Line: BulkNormSquared {
    public typedef Scalar BulkNormSquaredOutput;
    public func bulk_norm_squared() -> Scalar {
        Plane flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
public extension Motor {
    // Fancy postfix trick
    public Scalar operator /(BulkNormSquaredPrefixOrPostfix rhs)  {
        return this.bulk_norm_squared();
    }
}
public extension BulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Motor rhs) {
        return rhs.bulk_norm_squared();
    }
}
public extension Motor: BulkNormSquared {
    public typedef Scalar BulkNormSquaredOutput;
    public func bulk_norm_squared() -> Scalar {
        Flector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public Scalar operator /(BulkNormSquaredPrefixOrPostfix rhs)  {
        return this.bulk_norm_squared();
    }
}
public extension BulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
        return rhs.bulk_norm_squared();
    }
}
public extension MultiVector: BulkNormSquared {
    public typedef Scalar BulkNormSquaredOutput;
    public func bulk_norm_squared() -> Scalar {
        MultiVector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
public extension Point {
    // Fancy postfix trick
    public Scalar operator /(BulkNormSquaredPrefixOrPostfix rhs)  {
        return this.bulk_norm_squared();
    }
}
public extension BulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Point rhs) {
        return rhs.bulk_norm_squared();
    }
}
public extension Point: BulkNormSquared {
    public typedef Scalar BulkNormSquaredOutput;
    public func bulk_norm_squared() -> Scalar {
        Line flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
