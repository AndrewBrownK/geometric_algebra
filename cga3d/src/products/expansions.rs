#![allow(clippy::assign_op_pattern)]
use crate::cga3d::aspect_duals::*;
use crate::cga3d::products::exterior::Wedge;
use crate::cga3d::*;

/// Bulk Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}

/// Weight Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait WeightExpansion<T> {
    type Output;
    fn weight_expansion(self, other: T) -> Self::Output;
}
