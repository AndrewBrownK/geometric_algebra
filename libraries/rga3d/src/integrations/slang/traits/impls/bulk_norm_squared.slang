using traits::DotProduct;
using traits::FlatBulk;
using traits::Wedge;
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: BulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.bulk_norm_squared();
    }
}
extension Flector: BulkNormSquared {
    associatedtype Output = Scalar;
    func bulk_norm_squared() -> Output {
        Motor flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: BulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.bulk_norm_squared();
    }
}
extension Line: BulkNormSquared {
    associatedtype Output = Scalar;
    func bulk_norm_squared() -> Output {
        Plane flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: BulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.bulk_norm_squared();
    }
}
extension Motor: BulkNormSquared {
    associatedtype Output = Scalar;
    func bulk_norm_squared() -> Output {
        Flector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: BulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.bulk_norm_squared();
    }
}
extension MultiVector: BulkNormSquared {
    associatedtype Output = Scalar;
    func bulk_norm_squared() -> Output {
        MultiVector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: BulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.bulk_norm_squared();
    }
}
extension Point: BulkNormSquared {
    associatedtype Output = Scalar;
    func bulk_norm_squared() -> Output {
        Line flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
