using traits::DotProduct;
using traits::FlatBulk;
using traits::Wedge;
extension Flector {
    func div(_rhs: bulk_norm_squared) -> 
Scalar {
        this.bulk_norm_squared()
    }
}
extension Flector: BulkNormSquared {
    func bulk_norm_squared() -> Scalar {
Motor flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Line {
    func div(_rhs: bulk_norm_squared) -> 
Scalar {
        this.bulk_norm_squared()
    }
}
extension Line: BulkNormSquared {
    func bulk_norm_squared() -> Scalar {
Plane flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Motor {
    func div(_rhs: bulk_norm_squared) -> 
Scalar {
        this.bulk_norm_squared()
    }
}
extension Motor: BulkNormSquared {
    func bulk_norm_squared() -> Scalar {
Flector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension MultiVector {
    func div(_rhs: bulk_norm_squared) -> 
Scalar {
        this.bulk_norm_squared()
    }
}
extension MultiVector: BulkNormSquared {
    func bulk_norm_squared() -> Scalar {
MultiVector flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Point {
    func div(_rhs: bulk_norm_squared) -> 
Scalar {
        this.bulk_norm_squared()
    }
}
extension Point: BulkNormSquared {
    func bulk_norm_squared() -> Scalar {
Line flat_bulk_thing = this.flat_bulk().wedge(Origin.from_groups(
            /* e4 */
            1.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
