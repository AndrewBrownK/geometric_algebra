//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::characteristics::{Attitude, Sqrt};
use crate::involutions::AntiDual;
use crate::norms::*;
use crate::products::exterior::{AntiWedge, Wedge};
use crate::products::geometric::{GeometricAntiProduct, GeometricProduct};
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

impl CosineAngle<Circle> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleBulk> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl CosineAngle<RoundPointAtInfinity> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleBulk> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleBulk> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for CircleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleBulk> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for CircleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl CosineAngle<RoundPointAtInfinity> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for DipoleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for DipoleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for DipoleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for DipoleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for DipoleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for DipoleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for DipoleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for DipoleBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for DipoleCarrierAspect {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl CosineAngle<RoundPointOnOrigin> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl CosineAngle<RoundPointOnOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl CosineAngle<RoundPointAtInfinity> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl CosineAngle<RoundPointAtInfinity> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl CosineAngle<RoundPointAtInfinity> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SpacialCurvature> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: SpacialCurvature) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl CosineAngle<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for RoundPointAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for RoundPointAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for RoundPointAtInfinity {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for RoundPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for RoundPointAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for RoundPointBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPointBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for RoundPointBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for RoundPointBulk {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for RoundPointOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPointOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPointOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for RoundPointOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for RoundPointOnOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleBulk> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtInfinity> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SpacialCurvature> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: SpacialCurvature) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for SpacialCurvature {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleAtInfinity> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleBulk> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<CircleCarrierAspect> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: CircleCarrierAspect) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleAtInfinity> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleBulk> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<DipoleCarrierAspect> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl CosineAngle<RoundPointAtInfinity> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointAtOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointBulk> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointBulk) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<RoundPointOnOrigin> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<SpacialCurvature> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: SpacialCurvature) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
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

impl SineAngle<CircleAtInfinity> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: CircleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
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

impl SineAngle<DipoleAtInfinity> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<CircleAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: CircleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
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

impl SineAngle<DipoleAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for CircleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: CircleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for CircleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: CircleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for CircleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<DipoleAtInfinity> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for DipoleAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for DipoleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for DipoleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for DipoleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for DipoleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for DipoleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for DipoleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for DipoleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for DipoleBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for DipoleCarrierAspect {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<DipoleCarrierAspect> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointOnOrigin> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointOnOrigin> for FlatPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<CircleCarrierAspect> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
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

impl SineAngle<DipoleCarrierAspect> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<CircleCarrierAspect> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
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

impl SineAngle<DipoleCarrierAspect> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<CircleCarrierAspect> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
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

impl SineAngle<DipoleAtInfinity> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SpacialCurvature> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: SpacialCurvature) -> DualNum {
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

impl SineAngle<Circle> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
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

impl SineAngle<DipoleAtInfinity> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<RoundPoint> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPointAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for RoundPointAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for RoundPointAtInfinity {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointOnOrigin> for RoundPointAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPointBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for RoundPointBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for RoundPointBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for RoundPointBulk {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPointOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for RoundPointOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for RoundPointOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for RoundPointOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for RoundPointOnOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleAtInfinity> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: CircleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleAtInfinity> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SpacialCurvature> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: SpacialCurvature) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for SpacialCurvature {
    type Output = DualNum;

    fn sine_angle(self, other: Sphere) -> DualNum {
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

impl SineAngle<CircleAtInfinity> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: CircleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: CircleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: CircleCarrierAspect) -> DualNum {
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

impl SineAngle<DipoleAtInfinity> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleAtInfinity) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: DipoleCarrierAspect) -> DualNum {
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

impl SineAngle<RoundPointAtInfinity> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointAtInfinity) -> DualNum {
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

impl SineAngle<RoundPointBulk> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointBulk) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPointOnOrigin> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPointOnOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<SpacialCurvature> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: SpacialCurvature) -> DualNum {
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
