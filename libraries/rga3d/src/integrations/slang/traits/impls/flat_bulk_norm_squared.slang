using traits::DotProduct;
using traits::FlatBulk;
using traits::Wedge;
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Flector: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
Motor flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Line: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
Plane flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Motor: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
Flector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension MultiVector: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
MultiVector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Point: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
Line flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
