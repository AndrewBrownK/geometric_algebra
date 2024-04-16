//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
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

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for Circle {
    type Output = CircleBulkAspect;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> CircleBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for Circle {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Circle {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Circle {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: Origin) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for CircleBulkAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> CircleBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for CircleBulkAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for CircleBulkAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> CircleBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for CircleBulkAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for CircleBulkAspect {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for CircleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for CircleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for CircleBulkAspect {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for CircleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for CircleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for CircleBulkAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for CircleBulkAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for CircleBulkAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> CircleBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for CircleBulkAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for CircleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for CircleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for CircleCarrierAspect {
    type Output = CircleBulkAspect;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> CircleBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for CircleCarrierAspect {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for CircleCarrierAspect {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for CircleWeightAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for CircleWeightAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for CircleWeightAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for CircleWeightAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for CircleWeightAspect {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for CircleWeightAspect {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for CircleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for CircleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for CircleWeightAspect {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for CircleWeightAspect {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for CircleWeightAspect {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for CircleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for CircleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for CircleWeightAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for CircleWeightAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for CircleWeightAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for CircleWeightAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for CircleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for CircleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Dipole {
    type Output = DipoleBulkAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Dipole {
    type Output = DipoleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Dipole {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Dipole {
    type Output = DipoleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: Origin) -> DipoleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for DipoleBulkAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for DipoleBulkAspect {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for DipoleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for DipoleBulkAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for DipoleBulkAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for DipoleBulkAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> DipoleBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for DipoleBulkAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for DipoleBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = DipoleBulkAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for DipoleCarrierAspect {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for DipoleWeightAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for DipoleWeightAspect {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for DipoleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for DipoleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for DipoleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for DipoleWeightAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for DipoleWeightAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for DipoleWeightAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> DipoleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for DipoleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for DipoleWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: Origin) -> DipoleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for FlatPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for FlatPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: Origin) -> DipoleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for Flector {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for Flector {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeightAspect> for Flector {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for FlectorAtInfinity {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for FlectorAtInfinity {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for FlectorAtInfinity {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for FlectorAtInfinity {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for FlectorAtInfinity {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for FlectorAtInfinity {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeightAspect> for FlectorAtInfinity {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for Horizon {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Horizon {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Horizon {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: Origin) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeightAspect> for Horizon {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Infinity {
    type Output = Origin;

    fn anti_project_via_horizon_onto(self, other: Origin) -> Origin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Line {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for Line {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Line {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Line {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Line {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: Origin) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for LineAtInfinity {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for LineAtInfinity {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for LineAtInfinity {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for LineAtInfinity {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for LineAtInfinity {
    type Output = CircleWeightAspect;

    fn anti_project_via_horizon_onto(self, other: Origin) -> CircleWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for LineAtOrigin {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for LineAtOrigin {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Motor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Motor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Plane) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: SphereWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Origin {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Origin {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for Plane {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for Plane {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Plane {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Plane {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: Origin) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeightAspect> for Plane {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Rotor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Rotor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for RoundPoint {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for RoundPoint {
    type Output = Origin;

    fn anti_project_via_horizon_onto(self, other: Origin) -> Origin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for RoundPoint {
    type Output = RoundPointBulkAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for RoundPointAtInfinity {
    type Output = Origin;

    fn anti_project_via_horizon_onto(self, other: Origin) -> Origin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = RoundPointBulkAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_project_via_horizon_onto(self, other: Origin) -> Origin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for Sphere {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Sphere {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Sphere {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Sphere {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Sphere {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: Origin) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeightAspect> for Sphere {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for SphereWeightAspect {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for SphereWeightAspect {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for SphereWeightAspect {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for SphereWeightAspect {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for SphereWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for SphereWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for SphereWeightAspect {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for SphereWeightAspect {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for SphereWeightAspect {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for SphereWeightAspect {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for SphereWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for SphereWeightAspect {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for SphereWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for SphereWeightAspect {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for SphereWeightAspect {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for SphereWeightAspect {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for SphereWeightAspect {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for SphereWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for SphereWeightAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Transflector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for Transflector {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Transflector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for Transflector {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Transflector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Transflector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Transflector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Transflector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeightAspect> for Transflector {
    type Output = SphereWeightAspect;

    fn anti_project_via_horizon_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulkAspect> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: CircleBulkAspect) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeightAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulkAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleBulkAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeightAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeightAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Translator {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Translator {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulkAspect> for Translator {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulkAspect) -> Translator {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for Circle {
    type Output = CircleBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> CircleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for Circle {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for Circle {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Circle {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for CircleBulkAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> CircleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for CircleBulkAspect {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for CircleBulkAspect {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for CircleBulkAspect {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for CircleBulkAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for CircleCarrierAspect {
    type Output = CircleBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> CircleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for CircleCarrierAspect {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for CircleCarrierAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for CircleCarrierAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for CircleWeightAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for CircleWeightAspect {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for CircleWeightAspect {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for CircleWeightAspect {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for CircleWeightAspect {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for CircleWeightAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for CircleWeightAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for Dipole {
    type Output = DipoleBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulkAspect> for Dipole {
    type Output = DipoleBulkAspect;

    fn project_orthogonally_onto(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for Dipole {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Dipole {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for DipoleBulkAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn project_orthogonally_onto(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for DipoleBulkAspect {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> DipoleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for DipoleCarrierAspect {
    type Output = DipoleBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = DipoleBulkAspect;

    fn project_orthogonally_onto(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for DipoleCarrierAspect {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for DipoleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for DipoleWeightAspect {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for DipoleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for DipoleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for DipoleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for FlatPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for FlatPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for FlatPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Flector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Flector {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for Flector {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Flector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Flector {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for Flector {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Flector {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlectorAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for FlectorAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlectorAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for FlectorAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlectorAtInfinity {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlectorAtInfinity {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Horizon {
    type Output = SphereWeightAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Infinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for Infinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Infinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for Infinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPoint) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Line) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Origin> for Infinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Plane) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Infinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Infinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Line {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for Line {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Line {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for LineAtInfinity {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for LineAtInfinity {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for LineAtInfinity {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for LineAtInfinity {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for LineAtOrigin {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for LineAtOrigin {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Motor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Motor {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for Motor {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Motor {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Motor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Motor {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: DipoleBulkAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Horizon) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Infinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Origin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Origin) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Plane) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPointBulkAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for Origin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for Origin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for Origin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for Origin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for Origin {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Plane {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Plane {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Plane {
    type Output = SphereWeightAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Rotor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Rotor {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Rotor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Rotor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Rotor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for RoundPoint {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulkAspect> for RoundPoint {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for RoundPoint {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for RoundPoint {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Origin> for RoundPoint {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulkAspect> for RoundPoint {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for RoundPointAtInfinity {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulkAspect> for RoundPointAtInfinity {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Origin> for RoundPointAtInfinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPointBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulkAspect> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPointBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleBulkAspect> for RoundPointCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulkAspect> for RoundPointCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn project_orthogonally_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for Sphere {
    type Output = Horizon;

    fn project_orthogonally_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Sphere {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Sphere {
    type Output = SphereWeightAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for SphereWeightAspect {
    type Output = Horizon;

    fn project_orthogonally_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for SphereWeightAspect {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for SphereWeightAspect {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Transflector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for Transflector {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Transflector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeightAspect> for Transflector {
    type Output = DipoleWeightAspect;

    fn project_orthogonally_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Transflector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Transflector {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Transflector {
    type Output = Transflector;

    fn project_orthogonally_onto(self, other: Plane) -> Transflector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Transflector {
    type Output = Transflector;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Transflector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Translator {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Translator {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<CircleWeightAspect> for Translator {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Translator {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Translator {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Translator {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Translator {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<SphereWeightAspect> for Translator {
    type Output = CircleWeightAspect;

    fn project_orthogonally_onto(self, other: SphereWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for Circle {
    type Output = CircleBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> CircleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for Circle {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Circle {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Circle {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for CircleBulkAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> CircleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for CircleBulkAspect {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for CircleBulkAspect {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for CircleBulkAspect {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for CircleBulkAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for CircleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for CircleCarrierAspect {
    type Output = CircleBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> CircleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for CircleCarrierAspect {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for CircleCarrierAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for CircleCarrierAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for CircleWeightAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for CircleWeightAspect {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for CircleWeightAspect {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for CircleWeightAspect {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for CircleWeightAspect {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for CircleWeightAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for CircleWeightAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for CircleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for Dipole {
    type Output = DipoleBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleBulkAspect> for Dipole {
    type Output = DipoleBulkAspect;

    fn project_via_origin_onto(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for Dipole {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for Dipole {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for DipoleBulkAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn project_via_origin_onto(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for DipoleBulkAspect {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> DipoleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for DipoleBulkAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for DipoleBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for DipoleCarrierAspect {
    type Output = DipoleBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = DipoleBulkAspect;

    fn project_via_origin_onto(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for DipoleCarrierAspect {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for DipoleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for DipoleWeightAspect {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for DipoleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for DipoleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for DipoleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for DipoleWeightAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for DipoleWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for FlatPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for FlatPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for FlatPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Flector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Flector {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for Flector {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Flector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Flector {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for Flector {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for Flector {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for FlectorAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for FlectorAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlectorAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for FlectorAtInfinity {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlectorAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for FlectorAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Horizon {
    type Output = SphereWeightAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for Infinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for Infinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPoint) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Line) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Origin> for Infinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Plane) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Infinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Line {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Line {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for Line {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Line {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Line {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for LineAtInfinity {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for LineAtInfinity {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for LineAtInfinity {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for LineAtInfinity {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for LineAtOrigin {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for LineAtOrigin {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Motor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Motor {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for Motor {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Motor {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Motor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Motor {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: DipoleBulkAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlatPoint) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Horizon) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Infinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Line) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Origin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Origin) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Plane) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPoint) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointBulkAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for Origin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for Origin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Origin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for Origin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Origin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Origin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Origin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for Origin {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Plane {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Plane {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Plane {
    type Output = SphereWeightAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Rotor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Rotor {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Rotor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Rotor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Rotor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for RoundPoint {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleBulkAspect> for RoundPoint {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPoint {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for RoundPoint {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Origin> for RoundPoint {
    type Output = Origin;

    fn project_via_origin_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointBulkAspect> for RoundPoint {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for RoundPointAtInfinity {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleBulkAspect> for RoundPointAtInfinity {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Origin> for RoundPointAtInfinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPointBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleBulkAspect> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPointBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleBulkAspect> for RoundPointCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleBulkAspect> for RoundPointCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Origin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn project_via_origin_onto(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Sphere {
    type Output = Horizon;

    fn project_via_origin_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Sphere {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Sphere {
    type Output = SphereWeightAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> SphereWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for SphereWeightAspect {
    type Output = Horizon;

    fn project_via_origin_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for SphereWeightAspect {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for SphereWeightAspect {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for SphereWeightAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Transflector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for Transflector {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Transflector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<DipoleWeightAspect> for Transflector {
    type Output = DipoleWeightAspect;

    fn project_via_origin_onto(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Transflector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Transflector {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Transflector {
    type Output = Transflector;

    fn project_via_origin_onto(self, other: Plane) -> Transflector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Transflector {
    type Output = Transflector;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Transflector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Translator {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Translator {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<CircleWeightAspect> for Translator {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: CircleWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Translator {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Translator {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Translator {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Translator {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<SphereWeightAspect> for Translator {
    type Output = CircleWeightAspect;

    fn project_via_origin_onto(self, other: SphereWeightAspect) -> CircleWeightAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}
