//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::characteristics::{Attitude, Sqrt};
use crate::involutions::AntiDual;
use crate::norms::*;
use crate::products::exterior::*;
use crate::products::geometric::*;
use crate::products::projections::*;
use crate::unitize::Unitize;
use crate::*;

/// Euclidean distance between objects
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Euclidean_distance
/// distance(a,b) = bulk_norm(attitude(a wedge b)) + weight_norm(a wedge attitude(b))
/// where attitude(c) = c anti_wedge complement(e4) where e4 is the projective dimension
pub trait Distance<T> {
    type Output;
    fn distance(self, other: T) -> Self::Output;
}

/// The cosine of the angle between two objects.
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait CosineAngle<T> {
    type Output;
    fn cosine_angle(self, other: T) -> Self::Output;
}

/// The sine of the angle between two objects.
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait SineAngle<T> {
    type Output;
    fn sine_angle(self, other: T) -> Self::Output;
}

impl CosineAngle<AntiCircleOnOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiFlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiFlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for AntiLineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for AntiLineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for AntiLineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for AntiLineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for AntiPlane {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for AntiPlane {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for AntiPlane {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for AntiPlane {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for AntiPlane {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for AntiPlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for AntiPlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiFlatPointAtOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiFlatPointAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiFlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<PlaneAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: PlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SphereAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: SphereAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SphereOnOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: SphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: PlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SphereOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: SphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for RoundPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for RoundPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for RoundPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiFlatPointAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<PlaneAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: PlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SphereAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: SphereAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SphereOnOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: SphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiDipoleOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiFlatPointAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SphereAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: SphereAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SphereOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: SphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiCircleOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiLineAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlane> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlane) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiPlaneAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<AntiSphereOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAligningOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleOrthogonalOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAligningOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleOrthogonalOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPointAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<PlaneAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: PlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SphereAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: SphereAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SphereOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: SphereOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl SineAngle<AntiCircleOnOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for AntiCircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiFlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiFlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for AntiLineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for AntiLineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for AntiLineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for AntiLineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for AntiPlane {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for AntiPlane {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for AntiPlane {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for AntiPlane {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for AntiPlane {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for AntiPlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for AntiPlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for AntiSphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiFlatPointAtOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for CircleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiFlatPointAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for CircleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for CircleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiFlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for DipoleAligningOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for DipoleAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for DipoleOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: PlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Sphere) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SphereAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: SphereAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SphereOnOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: SphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: PlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Sphere) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SphereOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: SphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for RoundPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for RoundPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiFlatPointAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: PlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Sphere) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SphereAtOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: SphereAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SphereOnOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: SphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiDipoleOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiDipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiFlatPointAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiFlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Sphere) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SphereAtOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: SphereAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SphereOnOrigin> for SphereAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: SphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiCircleOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiCircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiLineAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiLineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlane> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiPlaneAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiPlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<AntiSphereOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: AntiSphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAligningOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleOrthogonalOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAligningOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAligningOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleOrthogonalOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleOrthogonalOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: PlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Sphere) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SphereAtOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: SphereAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SphereOnOrigin> for SphereOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: SphereOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}
