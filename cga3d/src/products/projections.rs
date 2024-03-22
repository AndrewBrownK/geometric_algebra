//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::aspect_duals::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::products::exterior::AntiWedge;
use crate::products::exterior::Wedge;
use crate::*;

/// Orthogonal Projection
/// Typically involves bringing a lower dimensional object to a higher dimensional object
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait ProjectOrthogonallyOnto<T> {
    type Output;
    fn project_orthogonally_onto(self, other: T) -> Self::Output;
}

/// Orthogonal AntiProjection
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait AntiProjectOrthogonallyOnto<T> {
    type Output;
    fn anti_project_orthogonally_onto(self, other: T) -> Self::Output;
}

/// Central (to origin) Projection
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait ProjectViaOriginOnto<T> {
    type Output;
    fn project_via_origin_onto(self, other: T) -> Self::Output;
}

/// Outward (to horizon) AntiProjection
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
pub trait AntiProjectViaHorizonOnto<T> {
    type Output;
    fn anti_project_via_horizon_onto(self, other: T) -> Self::Output;
}

impl AntiProjectViaHorizonOnto<Circle> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Radial) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Origin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for Origin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Point {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Point {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for Point {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for PointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for PointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Radial {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for Radial {
    type Output = Radial;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Radial {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Radial> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Radial) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for LineAtInfinity {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for LineAtOrigin {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Radial> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Radial) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Origin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Origin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Origin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Plane {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Point {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Point {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Point {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Point {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for PointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for PointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for PointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Radial {
    type Output = Radial;

    fn project_orthogonally_onto(self, other: Circle) -> Radial {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Radial {
    type Output = Radial;

    fn project_orthogonally_onto(self, other: Dipole) -> Radial {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Radial {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Radial> for Radial {
    type Output = Radial;

    fn project_orthogonally_onto(self, other: Radial) -> Radial {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Radial {
    type Output = Radial;

    fn project_orthogonally_onto(self, other: Sphere) -> Radial {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Line {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Radial> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Radial) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Origin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Origin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Point {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Point {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Point {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for PointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for PointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Radial {
    type Output = Radial;

    fn project_via_origin_onto(self, other: Circle) -> Radial {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Radial {
    type Output = Radial;

    fn project_via_origin_onto(self, other: Dipole) -> Radial {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Radial {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Radial> for Radial {
    type Output = Radial;

    fn project_via_origin_onto(self, other: Radial) -> Radial {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}
