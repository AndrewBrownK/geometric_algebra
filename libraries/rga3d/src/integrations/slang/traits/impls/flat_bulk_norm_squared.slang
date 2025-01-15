implementing rga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(Flector lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Flector {
    public Scalar flat_bulk_norm_squared() {
        Motor flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Line {
    public Scalar flat_bulk_norm_squared() {
        Plane flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Motor {
    public Scalar flat_bulk_norm_squared() {
        Flector flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension MultiVector {
    public Scalar flat_bulk_norm_squared() {
        MultiVector flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Point lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Point rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Point {
    public Scalar flat_bulk_norm_squared() {
        Line flat_bulk_thing = this.flat_bulk().wedge(Origin(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
