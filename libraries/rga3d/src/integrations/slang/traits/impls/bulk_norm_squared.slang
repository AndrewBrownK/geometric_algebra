implementing rga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(Flector lhs, BulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.bulk_norm_squared();
}
public extension Flector {
    public Scalar bulk_norm_squared() {
        Motor flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, BulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.bulk_norm_squared();
}
public extension Line {
    public Scalar bulk_norm_squared() {
        Plane flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, BulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.bulk_norm_squared();
}
public extension Motor {
    public Scalar bulk_norm_squared() {
        Flector flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, BulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.bulk_norm_squared();
}
public extension MultiVector {
    public Scalar bulk_norm_squared() {
        MultiVector flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Point lhs, BulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormSquaredPrefixOrPostfix lhs, Point rhs) {
    return rhs.bulk_norm_squared();
}
public extension Point {
    public Scalar bulk_norm_squared() {
        Line flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
