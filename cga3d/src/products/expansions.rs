//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::aspect_duals::*;
use crate::products::exterior::Wedge;
use crate::*;

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

impl BulkExpansion<Circle> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Dipole) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Radial> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Radial) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Origin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Origin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Point {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Point {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Point {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for PointAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for PointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Radial {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Radial {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Radial {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Radial> for Radial {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Radial) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl WeightExpansion<Circle> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Horizon {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Dipole) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Radial> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Radial) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Origin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Origin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Sphere) -> LineAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Point {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Point {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Point {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Point {
    type Output = Line;

    fn weight_expansion(self, other: Sphere) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for PointAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for PointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for PointAtInfinity {
    type Output = Line;

    fn weight_expansion(self, other: Sphere) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Radial {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Radial {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Radial {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Radial> for Radial {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Radial) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Radial {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}
