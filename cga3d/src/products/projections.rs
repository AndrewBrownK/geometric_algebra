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

impl AntiProjectOrthogonallyOnto<Circle> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for Circle {
    type Output = CircleBulk;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for Circle {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Circle {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Circle {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for Circle {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Circle {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Circle {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: Origin) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for CircleBulk {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for CircleBulk {
    type Output = CircleBulk;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for CircleBulk {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for CircleBulk {
    type Output = CircleBulk;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for CircleBulk {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for CircleBulk {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for CircleBulk {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for CircleBulk {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for CircleBulk {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for CircleBulk {
    type Output = CircleBulk;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for CircleCarrierAspect {
    type Output = CircleBulk;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for CircleCarrierAspect {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for CircleCarrierAspect {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for CircleWeight {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for CircleWeight {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for CircleWeight {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for CircleWeight {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for CircleWeight {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for CircleWeight {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for CircleWeight {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for CircleWeight {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for CircleWeight {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Dipole {
    type Output = DipoleBulk;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Dipole {
    type Output = DipoleWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Dipole {
    type Output = FlatPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Dipole {
    type Output = DipoleWeight;

    fn anti_project_orthogonally_onto(self, other: Origin) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Dipole {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Dipole {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Dipole {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for DipoleBulk {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for DipoleBulk {
    type Output = FlatPoint;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for DipoleBulk {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for DipoleBulk {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for DipoleBulk {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> DipoleBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for DipoleCarrierAspect {
    type Output = FlatPoint;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for DipoleWeight {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for DipoleWeight {
    type Output = FlatPoint;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for DipoleWeight {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for DipoleWeight {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for DipoleWeight {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_project_orthogonally_onto(self, other: Origin) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for FlatPoint {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_project_orthogonally_onto(self, other: Origin) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = FlatPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Flector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for Flector {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Flector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for Flector {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Flector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Flector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Flector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<PlaneAtOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Flector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<SphereWeight> for Flector {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for FlectorAtInfinity {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for FlectorAtInfinity {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for FlectorAtInfinity {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for FlectorAtInfinity {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for FlectorAtInfinity {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for FlectorAtInfinity {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<SphereWeight> for FlectorAtInfinity {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Horizon {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for Horizon {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for Horizon {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Horizon {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Horizon {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Horizon {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Horizon {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Horizon {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Horizon {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Horizon {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: Origin) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Horizon {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Horizon {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Horizon {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Horizon {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Horizon {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Horizon {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<SphereWeight> for Horizon {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Horizon {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Horizon {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Infinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Infinity {
    type Output = Origin;

    fn anti_project_orthogonally_onto(self, other: Origin) -> Origin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Infinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Infinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Infinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Line {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Line {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for Line {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Line {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Line {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Line {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Line {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Line {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Line {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Line {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: Origin) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Line {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Line {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Line {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Line {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Line {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for LineAtInfinity {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for LineAtInfinity {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_project_orthogonally_onto(self, other: Origin) -> CircleWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for LineAtOrigin {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for LineAtOrigin {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for Motor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Motor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<PlaneAtOrigin> for Motor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: PlaneAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Motor {
    type Output = Motor;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Motor {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Motor {
    type Output = Motor;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> Motor {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Motor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Horizon) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: LineAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Plane) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: PlaneAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: SphereWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Origin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for Origin {
    type Output = Infinity;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Origin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Origin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Origin {
    type Output = RoundPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Origin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Origin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for Plane {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for Plane {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Plane {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Plane {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Plane {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Plane {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Plane {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: Origin) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Plane {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Plane {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Plane {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<SphereWeight> for Plane {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for Rotor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Rotor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Rotor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<PlaneAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: PlaneAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Rotor {
    type Output = Motor;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Motor {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Rotor {
    type Output = Motor;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> Motor {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Rotor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for RoundPoint {
    type Output = Infinity;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for RoundPoint {
    type Output = Origin;

    fn anti_project_orthogonally_onto(self, other: Origin) -> Origin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for RoundPoint {
    type Output = RoundPointBulk;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for RoundPointAtInfinity {
    type Output = Origin;

    fn anti_project_orthogonally_onto(self, other: Origin) -> Origin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_project_orthogonally_onto(self, other: Origin) -> Origin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointBulk {
    type Output = RoundPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointBulk {
    type Output = RoundPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for Sphere {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Sphere {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Sphere {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for Sphere {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Horizon> for Sphere {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for Sphere {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Sphere {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtInfinity> for Sphere {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: LineAtInfinity) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Sphere {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: Origin) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Sphere {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<PlaneAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<SphereWeight> for Sphere {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for SphereWeight {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for SphereWeight {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for SphereWeight {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtInfinity> for SphereWeight {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Horizon> for SphereWeight {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Infinity> for SphereWeight {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: Infinity) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for SphereWeight {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtInfinity> for SphereWeight {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: LineAtInfinity) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for SphereWeight {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for SphereWeight {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for SphereWeight {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for SphereWeight {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for SphereWeight {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Transflector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for Transflector {
    type Output = Horizon;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Transflector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for Transflector {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Transflector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Transflector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Transflector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<PlaneAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn anti_project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Transflector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<SphereWeight> for Transflector {
    type Output = SphereWeight;

    fn anti_project_orthogonally_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleBulk> for Translator {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: CircleBulk) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<CircleWeight> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: CircleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleBulk> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<DipoleWeight> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPointAtOrigin> for Translator {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: FlatPointAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<LineAtOrigin> for Translator {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: LineAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Origin> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Translator {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<PlaneAtOrigin> for Translator {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: PlaneAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtInfinity> for Translator {
    type Output = Translator;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtInfinity) -> Translator {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointBulk> for Translator {
    type Output = Translator;

    fn anti_project_orthogonally_onto(self, other: RoundPointBulk) -> Translator {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Translator {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for Circle {
    type Output = CircleBulk;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for Circle {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Circle {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Circle {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: Origin) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for CircleBulk {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for CircleBulk {
    type Output = CircleBulk;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for CircleBulk {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for CircleBulk {
    type Output = CircleBulk;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for CircleBulk {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for CircleBulk {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for CircleBulk {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for CircleBulk {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for CircleBulk {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for CircleBulk {
    type Output = CircleBulk;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for CircleCarrierAspect {
    type Output = CircleBulk;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> CircleBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for CircleCarrierAspect {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for CircleCarrierAspect {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for CircleWeight {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for CircleWeight {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for CircleWeight {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for CircleWeight {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for CircleWeight {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for CircleWeight {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for CircleWeight {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for CircleWeight {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for CircleWeight {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Dipole {
    type Output = DipoleBulk;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Dipole {
    type Output = DipoleWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Dipole {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Dipole {
    type Output = DipoleWeight;

    fn anti_project_via_horizon_onto(self, other: Origin) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for DipoleBulk {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for DipoleBulk {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for DipoleBulk {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for DipoleBulk {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for DipoleBulk {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> DipoleBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for DipoleCarrierAspect {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for DipoleWeight {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for DipoleWeight {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for DipoleWeight {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for DipoleWeight {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for DipoleWeight {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_project_via_horizon_onto(self, other: Origin) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_project_via_horizon_onto(self, other: Origin) -> DipoleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for Flector {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for Flector {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeight> for Flector {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for FlectorAtInfinity {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for FlectorAtInfinity {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for FlectorAtInfinity {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for FlectorAtInfinity {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for FlectorAtInfinity {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for FlectorAtInfinity {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeight> for FlectorAtInfinity {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for Horizon {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Horizon {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Horizon {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: Origin) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeight> for Horizon {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Infinity {
    type Output = Origin;

    fn anti_project_via_horizon_onto(self, other: Origin) -> Origin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Line {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for Line {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Line {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Line {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Line {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: Origin) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for LineAtInfinity {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for LineAtInfinity {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_project_via_horizon_onto(self, other: Origin) -> CircleWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> LineAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for LineAtOrigin {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for LineAtOrigin {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> LineAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Motor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Motor {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Motor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> Motor {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Plane) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: SphereWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Origin {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Origin {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for Plane {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for Plane {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Plane {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Plane {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: Origin) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeight> for Plane {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Rotor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Motor {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Rotor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> Motor {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for RoundPoint {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for RoundPoint {
    type Output = Origin;

    fn anti_project_via_horizon_onto(self, other: Origin) -> Origin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for RoundPoint {
    type Output = RoundPointBulk;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for RoundPointAtInfinity {
    type Output = Origin;

    fn anti_project_via_horizon_onto(self, other: Origin) -> Origin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_project_via_horizon_onto(self, other: Origin) -> Origin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointBulk {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointBulk {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for Sphere {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Sphere {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Sphere {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Sphere {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Sphere {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: Origin) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeight> for Sphere {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for SphereWeight {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for SphereWeight {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for SphereWeight {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for SphereWeight {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for SphereWeight {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for SphereWeight {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for SphereWeight {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for SphereWeight {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for SphereWeight {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for SphereWeight {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for SphereWeight {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for SphereWeight {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for SphereWeight {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for SphereWeight {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Transflector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for Transflector {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> Horizon {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Transflector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for Transflector {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Transflector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Transflector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Transflector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Transflector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<SphereWeight> for Transflector {
    type Output = SphereWeight;

    fn anti_project_via_horizon_onto(self, other: SphereWeight) -> SphereWeight {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleBulk> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: CircleBulk) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<CircleWeight> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: CircleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleBulk> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleBulk) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<DipoleWeight> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: DipoleWeight) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtOrigin> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<LineAtOrigin> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: LineAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Origin> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Origin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<PlaneAtOrigin> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: PlaneAtOrigin) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Translator {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Translator {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointBulk> for Translator {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: RoundPointBulk) -> Translator {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for Circle {
    type Output = CircleBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> CircleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for Circle {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for Circle {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Circle {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Circle {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: SphereWeight) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for CircleBulk {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for CircleBulk {
    type Output = CircleBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> CircleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for CircleBulk {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for CircleBulk {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for CircleBulk {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for CircleCarrierAspect {
    type Output = CircleBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> CircleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for CircleCarrierAspect {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for CircleCarrierAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for CircleCarrierAspect {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for CircleWeight {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for CircleWeight {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for CircleWeight {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for CircleWeight {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for CircleWeight {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for CircleWeight {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for Dipole {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleWeight) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulk> for Dipole {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for Dipole {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Dipole {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: SphereWeight) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for DipoleBulk {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for DipoleBulk {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for DipoleBulk {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for DipoleBulk {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for DipoleBulk {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for DipoleBulk {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulk> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for DipoleCarrierAspect {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for DipoleWeight {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for DipoleWeight {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for DipoleWeight {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for DipoleWeight {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for DipoleWeight {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for DipoleWeight {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for DipoleWeight {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for DipoleWeight {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: SphereWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: SphereWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Flector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Flector {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for Flector {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Flector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Flector {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for Flector {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Flector {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Flector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: SphereWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlectorAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for FlectorAtInfinity {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlectorAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for FlectorAtInfinity {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlectorAtInfinity {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlectorAtInfinity {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: SphereWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Horizon {
    type Output = SphereWeight;

    fn project_orthogonally_onto(self, other: SphereWeight) -> SphereWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Infinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for Infinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: CircleWeight) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Infinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for Infinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPoint) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Line) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Origin> for Infinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Plane) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Infinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Infinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: SphereWeight) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Line {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for Line {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Line {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: SphereWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for LineAtInfinity {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for LineAtInfinity {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: SphereWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for LineAtOrigin {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for LineAtOrigin {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Motor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Motor {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for Motor {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Motor {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Motor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Motor {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: SphereWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: CircleBulk) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: CircleWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: DipoleBulk) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Horizon) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Infinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Origin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Origin) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Plane) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPointBulk) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: SphereWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for Origin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for Origin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for Origin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for Origin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for Origin {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Origin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Plane {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Plane {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Plane {
    type Output = SphereWeight;

    fn project_orthogonally_onto(self, other: SphereWeight) -> SphereWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Rotor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Rotor {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Rotor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Rotor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Rotor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for RoundPoint {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulk> for RoundPoint {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: DipoleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for RoundPoint {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for RoundPoint {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Origin> for RoundPoint {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulk> for RoundPoint {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: SphereWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulk> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: DipoleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Origin> for RoundPointAtInfinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulk> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: SphereWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: CircleWeight) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: SphereWeight) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: DipoleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointBulk {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointBulk {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleBulk> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: CircleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleBulk> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: DipoleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Infinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for Sphere {
    type Output = Horizon;

    fn project_orthogonally_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Sphere {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Sphere {
    type Output = SphereWeight;

    fn project_orthogonally_onto(self, other: SphereWeight) -> SphereWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Horizon> for SphereWeight {
    type Output = Horizon;

    fn project_orthogonally_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for SphereWeight {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for SphereWeight {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Transflector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for Transflector {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Transflector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<DipoleWeight> for Transflector {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Transflector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Transflector {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Transflector {
    type Output = Transflector;

    fn project_orthogonally_onto(self, other: Plane) -> Transflector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Transflector {
    type Output = Transflector;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Transflector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: SphereWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Translator {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleCarrierAspect> for Translator {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<CircleWeight> for Translator {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Translator {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Translator {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Translator {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Translator {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<SphereWeight> for Translator {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: SphereWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for Circle {
    type Output = CircleBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> CircleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for Circle {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for Circle {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Circle {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Circle {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: SphereWeight) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for CircleBulk {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for CircleBulk {
    type Output = CircleBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> CircleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for CircleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for CircleBulk {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for CircleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for CircleBulk {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for CircleBulk {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for CircleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for CircleCarrierAspect {
    type Output = CircleBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> CircleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for CircleCarrierAspect {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for CircleCarrierAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for CircleCarrierAspect {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for CircleWeight {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for CircleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for CircleWeight {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for CircleWeight {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for CircleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for CircleWeight {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for CircleWeight {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for CircleWeight {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for CircleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for Dipole {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleWeight) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleBulk> for Dipole {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for Dipole {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for Dipole {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Dipole {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: SphereWeight) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for DipoleBulk {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for DipoleBulk {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for DipoleBulk {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for DipoleBulk {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for DipoleBulk {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for DipoleBulk {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleBulk> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: DipoleBulk) -> DipoleBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for DipoleCarrierAspect {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for DipoleCarrierAspect {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for DipoleWeight {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for DipoleWeight {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for DipoleWeight {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for DipoleWeight {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for DipoleWeight {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for DipoleWeight {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for DipoleWeight {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for DipoleWeight {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: SphereWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: SphereWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Flector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Flector {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for Flector {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for Flector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Flector {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for Flector {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for Flector {
    type Output = FlatPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Flector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: SphereWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for FlectorAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for FlectorAtInfinity {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlectorAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for FlectorAtInfinity {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlectorAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for FlectorAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: SphereWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Horizon {
    type Output = SphereWeight;

    fn project_via_origin_onto(self, other: SphereWeight) -> SphereWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for Infinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: CircleWeight) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for Infinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: DipoleWeight) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPoint) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Line) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Origin> for Infinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Plane) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Infinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: SphereWeight) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Line {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Line {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for Line {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Line {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Line {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: SphereWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for LineAtInfinity {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for LineAtInfinity {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: SphereWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for LineAtOrigin {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for LineAtOrigin {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Motor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Motor {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for Motor {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Motor {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Motor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Motor {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: SphereWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: CircleBulk) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: CircleWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: DipoleBulk) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: DipoleWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlatPoint) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Horizon) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Infinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Infinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Line) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Origin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Origin) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Plane) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPoint) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointBulk) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: SphereWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for Origin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for Origin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for Origin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Infinity> for Origin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Origin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Origin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Origin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for Origin {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Origin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Plane {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Plane {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Plane {
    type Output = SphereWeight;

    fn project_via_origin_onto(self, other: SphereWeight) -> SphereWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Rotor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Rotor {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Rotor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Rotor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Rotor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for RoundPoint {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleBulk> for RoundPoint {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: DipoleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPoint {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Infinity> for RoundPoint {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Origin> for RoundPoint {
    type Output = Origin;

    fn project_via_origin_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointBulk> for RoundPoint {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: SphereWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleBulk> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: DipoleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Origin> for RoundPointAtInfinity {
    type Output = Origin;

    fn project_via_origin_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointBulk> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: SphereWeight) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: CircleWeight) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: DipoleWeight) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn project_via_origin_onto(self, other: SphereWeight) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: DipoleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointBulk {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointBulk {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointBulk {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleBulk> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: CircleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleBulk> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: DipoleBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Origin;

    fn project_via_origin_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Infinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: RoundPointBulk) -> RoundPointBulk {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for Sphere {
    type Output = Horizon;

    fn project_via_origin_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Sphere {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Sphere {
    type Output = SphereWeight;

    fn project_via_origin_onto(self, other: SphereWeight) -> SphereWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for SphereWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Horizon> for SphereWeight {
    type Output = Horizon;

    fn project_via_origin_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for SphereWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for SphereWeight {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for SphereWeight {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for SphereWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Transflector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for Transflector {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for Transflector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<DipoleWeight> for Transflector {
    type Output = DipoleWeight;

    fn project_via_origin_onto(self, other: DipoleWeight) -> DipoleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Transflector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<LineAtOrigin> for Transflector {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Transflector {
    type Output = Transflector;

    fn project_via_origin_onto(self, other: Plane) -> Transflector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Transflector {
    type Output = Transflector;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> Transflector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: SphereWeight) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Translator {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleCarrierAspect> for Translator {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<CircleWeight> for Translator {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: CircleWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlectorAtInfinity) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Translator {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Translator {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<PlaneAtOrigin> for Translator {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Translator {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<SphereWeight> for Translator {
    type Output = CircleWeight;

    fn project_via_origin_onto(self, other: SphereWeight) -> CircleWeight {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}
