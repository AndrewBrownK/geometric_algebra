//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::AntiReversal;
use crate::products::geometric::GeometricAntiProduct;
use crate::unitize::Unitize;
use crate::*;

/// self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
///
/// Also called sandwich product
/// See article "Projective Geometric Algebra Done Right"
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projective_Geometric_Algebra_Done_Right
pub trait Sandwich<T> {
    type Output;
    fn sandwich(self, other: T) -> Self::Output;
}

/// Point Inversion
/// An improper isometry that performs an inversion through a point.
/// Points may pass as specialized as Flectors, so in other words, this is a specialized Flector sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Inversion
///
/// Be careful not to confuse with `Inverse`, which raises a number to the power of `-1.0`.
pub trait PointInversion<T> {
    type Output;
    fn point_inversion(self, other: T) -> Self::Output;
}

/// Reflection
/// An improper isometry that performs reflection across a plane.
/// Planes may pass as specialized Flectors, so in other words, this is a specialized Flector sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Reflection
pub trait Reflect<T> {
    type Output;
    fn reflect(self, other: T) -> Self::Output;
}

/// Transflection
/// An improper isometry that performs a reflection and translation.
/// Transflectors are specialized Flectors, so in other words, this is a specialized Flector sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Transflection
pub trait Transflect<T> {
    type Output;
    fn transflect(self, other: T) -> Self::Output;
}

/// Translate
/// A proper isometry that performs translation.
/// Translators are specialized Motors, so in other words, this is a specialized Motor sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Translation
pub trait Translate<T> {
    type Output;
    fn translate(self, other: T) -> Self::Output;
}

/// Rotate
/// A proper isometry that performs rotation.
/// Rotors are specialized Motors, so in other words, this is a specialized Motor sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Rotation
pub trait Rotate<T> {
    type Output;
    fn rotate(self, other: T) -> Self::Output;
}

impl Sandwich<Circle> for AntiScalar {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleBulkAspect> for AntiScalar {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleCarrierAspect> for AntiScalar {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleWeightAspect> for AntiScalar {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Dipole> for AntiScalar {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleBulkAspect> for AntiScalar {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleCarrierAspect> for AntiScalar {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleWeightAspect> for AntiScalar {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPoint> for AntiScalar {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtInfinity> for AntiScalar {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtOrigin> for AntiScalar {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Flector> for AntiScalar {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for AntiScalar {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for AntiScalar {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Infinity> for AntiScalar {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for AntiScalar {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtInfinity> for AntiScalar {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Motor> for AntiScalar {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for AntiScalar {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for AntiScalar {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<PlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for AntiScalar {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPoint> for AntiScalar {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointAtInfinity> for AntiScalar {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointAtOrigin> for AntiScalar {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointBulkAspect> for AntiScalar {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointCarrierAspect> for AntiScalar {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Sphere> for AntiScalar {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<SphereWeightAspect> for AntiScalar {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Transflector> for AntiScalar {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Translator> for AntiScalar {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Circle> for Circle {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Circle {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Circle {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Circle {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Circle {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Circle {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Circle {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Circle {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Circle {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Circle {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Circle {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Circle {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Circle {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Circle {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Circle {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Circle {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Circle {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Circle {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Circle {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Circle {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Circle {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Circle {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Circle {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Circle {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Circle {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Circle {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Circle {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Circle {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Circle {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Circle {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Circle {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for CircleBulkAspect {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleCarrierAspect> for CircleBulkAspect {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for CircleBulkAspect {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Dipole> for CircleBulkAspect {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleCarrierAspect> for CircleBulkAspect {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for CircleBulkAspect {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPoint> for CircleBulkAspect {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for CircleBulkAspect {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtOrigin> for CircleBulkAspect {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Flector> for CircleBulkAspect {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for CircleBulkAspect {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for CircleBulkAspect {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Infinity> for CircleBulkAspect {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for CircleBulkAspect {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for CircleBulkAspect {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for CircleBulkAspect {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Motor> for CircleBulkAspect {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for CircleBulkAspect {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for CircleBulkAspect {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for CircleBulkAspect {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for CircleBulkAspect {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for CircleBulkAspect {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for CircleBulkAspect {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for CircleBulkAspect {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointBulkAspect> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointCarrierAspect> for CircleBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for CircleBulkAspect {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for CircleBulkAspect {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Transflector> for CircleBulkAspect {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for CircleBulkAspect {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for CircleCarrierAspect {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for CircleCarrierAspect {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Dipole> for CircleCarrierAspect {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for CircleCarrierAspect {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for CircleCarrierAspect {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPoint> for CircleCarrierAspect {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for CircleCarrierAspect {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for CircleCarrierAspect {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for CircleCarrierAspect {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for CircleCarrierAspect {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for CircleCarrierAspect {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for CircleCarrierAspect {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for CircleCarrierAspect {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for CircleCarrierAspect {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for CircleCarrierAspect {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for CircleCarrierAspect {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for CircleCarrierAspect {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for CircleCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for CircleCarrierAspect {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for CircleCarrierAspect {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Transflector> for CircleCarrierAspect {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for CircleCarrierAspect {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for CircleWeightAspect {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for CircleWeightAspect {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for CircleWeightAspect {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for CircleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for CircleWeightAspect {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for CircleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for CircleWeightAspect {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for CircleWeightAspect {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for CircleWeightAspect {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for CircleWeightAspect {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for CircleWeightAspect {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for CircleWeightAspect {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for CircleWeightAspect {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for CircleWeightAspect {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for CircleWeightAspect {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for CircleWeightAspect {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for CircleWeightAspect {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for CircleWeightAspect {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for CircleWeightAspect {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for CircleWeightAspect {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for CircleWeightAspect {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for CircleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for CircleWeightAspect {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for CircleWeightAspect {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for CircleWeightAspect {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Dipole {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Dipole {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Dipole {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Dipole {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Dipole {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Dipole {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Dipole {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Dipole {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Dipole {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Dipole {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Dipole {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Dipole {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Dipole {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Dipole {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Dipole {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Dipole {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Dipole {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Dipole {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Dipole {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Dipole {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Dipole {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Dipole {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Dipole {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Dipole {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Dipole {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Dipole {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Dipole {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Dipole {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Dipole {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Dipole {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Dipole {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DipoleBulkAspect {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for DipoleBulkAspect {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for DipoleBulkAspect {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for DipoleBulkAspect {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DipoleBulkAspect {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for DipoleBulkAspect {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DipoleBulkAspect {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for DipoleBulkAspect {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for DipoleBulkAspect {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DipoleBulkAspect {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for DipoleBulkAspect {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for DipoleBulkAspect {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for DipoleBulkAspect {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DipoleBulkAspect {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for DipoleBulkAspect {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for DipoleBulkAspect {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DipoleBulkAspect {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for DipoleBulkAspect {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for DipoleBulkAspect {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for DipoleBulkAspect {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for DipoleBulkAspect {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DipoleBulkAspect {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for DipoleBulkAspect {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for DipoleBulkAspect {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for DipoleBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DipoleBulkAspect {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for DipoleBulkAspect {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for DipoleBulkAspect {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DipoleBulkAspect {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DipoleCarrierAspect {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for DipoleCarrierAspect {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for DipoleCarrierAspect {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for DipoleCarrierAspect {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DipoleCarrierAspect {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for DipoleCarrierAspect {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DipoleCarrierAspect {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for DipoleCarrierAspect {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for DipoleCarrierAspect {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DipoleCarrierAspect {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for DipoleCarrierAspect {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for DipoleCarrierAspect {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DipoleCarrierAspect {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for DipoleCarrierAspect {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for DipoleCarrierAspect {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for DipoleCarrierAspect {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DipoleCarrierAspect {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for DipoleCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DipoleCarrierAspect {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for DipoleCarrierAspect {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for DipoleCarrierAspect {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DipoleCarrierAspect {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DipoleWeightAspect {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DipoleWeightAspect {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for DipoleWeightAspect {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DipoleWeightAspect {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for DipoleWeightAspect {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DipoleWeightAspect {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for DipoleWeightAspect {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for DipoleWeightAspect {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for DipoleWeightAspect {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DipoleWeightAspect {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for DipoleWeightAspect {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for DipoleWeightAspect {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DipoleWeightAspect {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for DipoleWeightAspect {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for DipoleWeightAspect {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for DipoleWeightAspect {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DipoleWeightAspect {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for DipoleWeightAspect {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for DipoleWeightAspect {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for DipoleWeightAspect {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for DipoleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DipoleWeightAspect {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for DipoleWeightAspect {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DipoleWeightAspect {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for FlatPoint {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for FlatPoint {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for FlatPoint {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for FlatPoint {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for FlatPoint {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for FlatPoint {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtInfinity> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for FlatPoint {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for FlatPoint {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for FlatPoint {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Infinity> for FlatPoint {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for FlatPoint {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for FlatPoint {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for FlatPoint {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for FlatPoint {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for FlatPoint {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for FlatPoint {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for FlatPoint {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for FlatPoint {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for FlatPoint {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for FlatPoint {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for FlatPoint {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for FlatPoint {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for FlatPoint {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for FlatPoint {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Circle> for FlatPointAtInfinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for FlatPointAtInfinity {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for FlatPointAtInfinity {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for FlatPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for FlatPointAtInfinity {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for FlatPointAtInfinity {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for FlatPointAtInfinity {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for FlatPointAtInfinity {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for FlatPointAtInfinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for FlatPointAtInfinity {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for FlatPointAtInfinity {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for FlatPointAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for FlatPointAtOrigin {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for FlatPointAtOrigin {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for FlatPointAtOrigin {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for FlatPointAtOrigin {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPoint> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtInfinity> for FlatPointAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Flector> for FlatPointAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for FlatPointAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Infinity> for FlatPointAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for FlatPointAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for FlatPointAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Motor> for FlatPointAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for FlatPointAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for FlatPointAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for FlatPointAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for FlatPointAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for FlatPointAtOrigin {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for FlatPointAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for FlatPointAtOrigin {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Transflector> for FlatPointAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for FlatPointAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Circle> for Flector {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Flector {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Flector {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Flector {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Flector {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Flector {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Flector {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Flector {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Flector {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Flector {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Flector {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Flector {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Flector {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Flector {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Flector {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Flector {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Flector {
    type Output = Line;

    fn sandwich(self, other: LineAtOrigin) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Flector {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Flector {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Flector {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Flector {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Flector {
    type Output = Plane;

    fn sandwich(self, other: PlaneAtOrigin) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Flector {
    type Output = Motor;

    fn sandwich(self, other: Rotor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Flector {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Flector {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Flector {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Flector {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Flector {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Flector {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Flector {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Flector {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Flector {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for FlectorAtInfinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for FlectorAtInfinity {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for FlectorAtInfinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for FlectorAtInfinity {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for FlectorAtInfinity {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for FlectorAtInfinity {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for FlectorAtInfinity {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for FlectorAtInfinity {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for FlectorAtInfinity {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = Plane;

    fn sandwich(self, other: PlaneAtOrigin) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for FlectorAtInfinity {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for FlectorAtInfinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for FlectorAtInfinity {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for FlectorAtInfinity {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Horizon {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Horizon {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Horizon {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Horizon {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Horizon {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Horizon {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Horizon {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Horizon {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Horizon {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Horizon {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Horizon {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Horizon {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Horizon {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Horizon {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Infinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Infinity {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Infinity {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Infinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Infinity {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Infinity {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Infinity {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Infinity {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Infinity {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Infinity {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Infinity {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Infinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Infinity {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Line {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Line {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Line {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Line {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Line {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Line {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Line {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Line {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Line {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Line {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Line {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Line {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for Line {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for Line {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Line {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Line {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Line {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Line {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Line {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Line {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Line {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Line {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Line {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Line {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Line {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Line {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Line {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Line {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Line {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Line {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Line {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Line {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for LineAtInfinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for LineAtInfinity {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for LineAtInfinity {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for LineAtInfinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for LineAtInfinity {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for LineAtInfinity {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for LineAtInfinity {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for LineAtInfinity {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for LineAtInfinity {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for LineAtInfinity {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for LineAtInfinity {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for LineAtInfinity {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for LineAtInfinity {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for LineAtInfinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for LineAtInfinity {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for LineAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for LineAtOrigin {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for LineAtOrigin {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for LineAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for LineAtOrigin {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for LineAtOrigin {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for LineAtOrigin {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for LineAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for LineAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for LineAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for LineAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for LineAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for LineAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for LineAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for LineAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for LineAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for LineAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for LineAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for LineAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPoint> for LineAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for LineAtOrigin {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for LineAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for LineAtOrigin {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for LineAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for LineAtOrigin {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for LineAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for LineAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Magnitude {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Magnitude {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Magnitude {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Magnitude {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Magnitude {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Magnitude {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Magnitude {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Magnitude {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Magnitude {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Magnitude {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Magnitude {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Magnitude {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Magnitude {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Magnitude {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Magnitude {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Magnitude {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Magnitude {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Magnitude {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Magnitude {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Magnitude {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Magnitude {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Magnitude {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Magnitude {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Magnitude {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Magnitude {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Magnitude {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Magnitude {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Magnitude {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Magnitude {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Motor {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Motor {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Motor {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Motor {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Motor {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Motor {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Motor {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Motor {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Motor {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Motor {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Motor {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Motor {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for Motor {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for Motor {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Motor {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Motor {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Motor {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Motor {
    type Output = Line;

    fn sandwich(self, other: LineAtOrigin) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Motor {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Motor {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Motor {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Motor {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Motor {
    type Output = Plane;

    fn sandwich(self, other: PlaneAtOrigin) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Motor {
    type Output = Motor;

    fn sandwich(self, other: Rotor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Motor {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Motor {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Motor {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Motor {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Motor {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Motor {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Motor {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Motor {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Motor {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for MultiVector {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for MultiVector {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for MultiVector {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for MultiVector {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for MultiVector {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for MultiVector {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for MultiVector {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for MultiVector {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for MultiVector {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for MultiVector {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for MultiVector {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for MultiVector {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for MultiVector {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for MultiVector {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for MultiVector {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for MultiVector {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for MultiVector {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for MultiVector {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for MultiVector {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for MultiVector {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for MultiVector {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for MultiVector {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for MultiVector {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for MultiVector {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for MultiVector {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for MultiVector {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for MultiVector {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for MultiVector {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for MultiVector {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for MultiVector {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for MultiVector {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for MultiVector {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Origin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Origin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Origin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Origin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Origin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Origin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Origin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Origin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Origin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Origin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Origin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Origin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Origin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Origin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Origin {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Origin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Origin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Origin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Plane {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Plane {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Plane {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Plane {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Plane {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Plane {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Plane {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Plane {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Plane {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Plane {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Plane {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Plane {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Plane {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Plane {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Plane {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Plane {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Plane {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Plane {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Plane {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Plane {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Plane {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Plane {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Plane {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Plane {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Plane {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Plane {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Plane {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Plane {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Plane {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Plane {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Plane {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Plane {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for PlaneAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for PlaneAtOrigin {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for PlaneAtOrigin {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for PlaneAtOrigin {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for PlaneAtOrigin {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for PlaneAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for PlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for PlaneAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for PlaneAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for PlaneAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for PlaneAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for PlaneAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for PlaneAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for PlaneAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for PlaneAtOrigin {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for PlaneAtOrigin {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for PlaneAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for PlaneAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Rotor {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Rotor {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Rotor {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Rotor {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Rotor {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Rotor {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Rotor {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Rotor {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Rotor {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Rotor {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Rotor {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Rotor {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for Rotor {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for Rotor {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Rotor {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Rotor {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Rotor {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Rotor {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Rotor {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Rotor {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Rotor {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Rotor {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPoint> for Rotor {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Rotor {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Rotor {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Rotor {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Rotor {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Rotor {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Rotor {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Rotor {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Rotor {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for RoundPoint {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for RoundPoint {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for RoundPoint {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for RoundPoint {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for RoundPoint {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for RoundPoint {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for RoundPoint {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for RoundPoint {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for RoundPoint {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for RoundPoint {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for RoundPoint {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for RoundPoint {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for RoundPoint {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for RoundPoint {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for RoundPoint {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for RoundPoint {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for RoundPoint {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for RoundPoint {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for RoundPoint {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for RoundPoint {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for RoundPoint {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for RoundPoint {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for RoundPoint {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for RoundPoint {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for RoundPoint {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for RoundPoint {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for RoundPoint {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for RoundPoint {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for RoundPointAtInfinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for RoundPointAtInfinity {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for RoundPointAtInfinity {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for RoundPointAtInfinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for RoundPointAtInfinity {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for RoundPointAtInfinity {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for RoundPointAtInfinity {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for RoundPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for RoundPointAtInfinity {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for RoundPointAtInfinity {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for RoundPointAtInfinity {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for RoundPointAtInfinity {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for RoundPointAtInfinity {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for RoundPointAtInfinity {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for RoundPointAtInfinity {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for RoundPointAtInfinity {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for RoundPointAtInfinity {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for RoundPointAtInfinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for RoundPointAtInfinity {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for RoundPointAtInfinity {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for RoundPointAtInfinity {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for RoundPointAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for RoundPointAtOrigin {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for RoundPointAtOrigin {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for RoundPointAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for RoundPointAtOrigin {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for RoundPointAtOrigin {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for RoundPointAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for RoundPointAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for RoundPointAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for RoundPointAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for RoundPointAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for RoundPointAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for RoundPointAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for RoundPointAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for RoundPointAtOrigin {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for RoundPointAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for RoundPointAtOrigin {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for RoundPointAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for RoundPointAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for RoundPointBulkAspect {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for RoundPointBulkAspect {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for RoundPointBulkAspect {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for RoundPointBulkAspect {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for RoundPointBulkAspect {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for RoundPointBulkAspect {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for RoundPointBulkAspect {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for RoundPointBulkAspect {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for RoundPointBulkAspect {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for RoundPointBulkAspect {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for RoundPointBulkAspect {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for RoundPointBulkAspect {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for RoundPointBulkAspect {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for RoundPointBulkAspect {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for RoundPointBulkAspect {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for RoundPointBulkAspect {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for RoundPointBulkAspect {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for RoundPointBulkAspect {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for RoundPointBulkAspect {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for RoundPointBulkAspect {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for RoundPointBulkAspect {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for RoundPointBulkAspect {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for RoundPointBulkAspect {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for RoundPointBulkAspect {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for RoundPointBulkAspect {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for RoundPointBulkAspect {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for RoundPointBulkAspect {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for RoundPointBulkAspect {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for RoundPointCarrierAspect {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for RoundPointCarrierAspect {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for RoundPointCarrierAspect {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for RoundPointCarrierAspect {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for RoundPointCarrierAspect {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for RoundPointCarrierAspect {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for RoundPointCarrierAspect {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for RoundPointCarrierAspect {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for RoundPointCarrierAspect {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for RoundPointCarrierAspect {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for RoundPointCarrierAspect {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for RoundPointCarrierAspect {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for RoundPointCarrierAspect {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for RoundPointCarrierAspect {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for RoundPointCarrierAspect {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for RoundPointCarrierAspect {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for RoundPointCarrierAspect {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for RoundPointCarrierAspect {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Scalar {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleBulkAspect> for Scalar {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleCarrierAspect> for Scalar {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Scalar {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Dipole> for Scalar {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleBulkAspect> for Scalar {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleCarrierAspect> for Scalar {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Scalar {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Scalar {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtOrigin> for Scalar {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Flector> for Scalar {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Scalar {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Scalar {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Infinity> for Scalar {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for Scalar {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Scalar {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for Scalar {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Motor> for Scalar {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Scalar {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Scalar {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Scalar {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<PlaneAtOrigin> for Scalar {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for Scalar {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Scalar {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointAtInfinity> for Scalar {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointAtOrigin> for Scalar {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Scalar {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<RoundPointCarrierAspect> for Scalar {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Scalar {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<SphereWeightAspect> for Scalar {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Transflector> for Scalar {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Scalar {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Sphere {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Sphere {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Sphere {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Sphere {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Sphere {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Sphere {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Sphere {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Sphere {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Sphere {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Sphere {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Sphere {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Sphere {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Sphere {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Sphere {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Sphere {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Sphere {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Sphere {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Sphere {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Sphere {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Sphere {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Sphere {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Sphere {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Sphere {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Sphere {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Sphere {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Sphere {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Sphere {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Sphere {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Sphere {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Sphere {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Sphere {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for SphereWeightAspect {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for SphereWeightAspect {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for SphereWeightAspect {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for SphereWeightAspect {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for SphereWeightAspect {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for SphereWeightAspect {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for SphereWeightAspect {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for SphereWeightAspect {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for SphereWeightAspect {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for SphereWeightAspect {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for SphereWeightAspect {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for SphereWeightAspect {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for SphereWeightAspect {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for SphereWeightAspect {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for SphereWeightAspect {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for SphereWeightAspect {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for SphereWeightAspect {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for SphereWeightAspect {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for SphereWeightAspect {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Transflector {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Transflector {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Transflector {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Transflector {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Transflector {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Transflector {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Transflector {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for Transflector {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for Transflector {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Transflector {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Transflector {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Transflector {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for Transflector {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Transflector {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Transflector {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Transflector {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Transflector {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Transflector {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Transflector {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Transflector {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Transflector {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Transflector {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Transflector {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Transflector {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Transflector {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Transflector {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Transflector {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Transflector {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Transflector {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Transflector {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Translator {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleBulkAspect> for Translator {
    type Output = CircleBulkAspect;

    fn sandwich(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleCarrierAspect> for Translator {
    type Output = CircleCarrierAspect;

    fn sandwich(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleWeightAspect> for Translator {
    type Output = CircleWeightAspect;

    fn sandwich(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Translator {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleBulkAspect> for Translator {
    type Output = DipoleBulkAspect;

    fn sandwich(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleCarrierAspect> for Translator {
    type Output = DipoleCarrierAspect;

    fn sandwich(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleWeightAspect> for Translator {
    type Output = DipoleWeightAspect;

    fn sandwich(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Translator {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtInfinity> for Translator {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtOrigin> for Translator {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Translator {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for Translator {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for Translator {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Infinity> for Translator {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for Translator {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for Translator {
    type Output = Line;

    fn sandwich(self, other: LineAtOrigin) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Translator {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Translator {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Translator {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Translator {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Translator {
    type Output = Plane;

    fn sandwich(self, other: PlaneAtOrigin) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Translator {
    type Output = Motor;

    fn sandwich(self, other: Rotor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Translator {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtInfinity> for Translator {
    type Output = RoundPointAtInfinity;

    fn sandwich(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for Translator {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointBulkAspect> for Translator {
    type Output = RoundPointBulkAspect;

    fn sandwich(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointCarrierAspect> for Translator {
    type Output = RoundPointCarrierAspect;

    fn sandwich(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Translator {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereWeightAspect> for Translator {
    type Output = SphereWeightAspect;

    fn sandwich(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Translator {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Translator> for Translator {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl PointInversion<Circle> for FlatPoint {
    type Output = Circle;

    fn point_inversion(self, other: Circle) -> Circle {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<CircleBulkAspect> for FlatPoint {
    type Output = CircleBulkAspect;

    fn point_inversion(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<CircleCarrierAspect> for FlatPoint {
    type Output = CircleCarrierAspect;

    fn point_inversion(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<CircleWeightAspect> for FlatPoint {
    type Output = CircleWeightAspect;

    fn point_inversion(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Dipole> for FlatPoint {
    type Output = Dipole;

    fn point_inversion(self, other: Dipole) -> Dipole {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<DipoleBulkAspect> for FlatPoint {
    type Output = DipoleBulkAspect;

    fn point_inversion(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<DipoleCarrierAspect> for FlatPoint {
    type Output = DipoleCarrierAspect;

    fn point_inversion(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<DipoleWeightAspect> for FlatPoint {
    type Output = DipoleWeightAspect;

    fn point_inversion(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn point_inversion(self, other: FlatPoint) -> FlatPoint {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<FlatPointAtInfinity> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn point_inversion(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn point_inversion(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Flector> for FlatPoint {
    type Output = Flector;

    fn point_inversion(self, other: Flector) -> Flector {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<FlectorAtInfinity> for FlatPoint {
    type Output = FlectorAtInfinity;

    fn point_inversion(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Horizon> for FlatPoint {
    type Output = Horizon;

    fn point_inversion(self, other: Horizon) -> Horizon {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Infinity> for FlatPoint {
    type Output = Infinity;

    fn point_inversion(self, other: Infinity) -> Infinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Line> for FlatPoint {
    type Output = Line;

    fn point_inversion(self, other: Line) -> Line {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<LineAtInfinity> for FlatPoint {
    type Output = LineAtInfinity;

    fn point_inversion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<LineAtOrigin> for FlatPoint {
    type Output = LineAtOrigin;

    fn point_inversion(self, other: LineAtOrigin) -> LineAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Motor> for FlatPoint {
    type Output = Motor;

    fn point_inversion(self, other: Motor) -> Motor {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn point_inversion(self, other: MultiVector) -> MultiVector {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Origin> for FlatPoint {
    type Output = Origin;

    fn point_inversion(self, other: Origin) -> Origin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Plane> for FlatPoint {
    type Output = Plane;

    fn point_inversion(self, other: Plane) -> Plane {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<PlaneAtOrigin> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn point_inversion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Rotor> for FlatPoint {
    type Output = Rotor;

    fn point_inversion(self, other: Rotor) -> Rotor {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn point_inversion(self, other: RoundPoint) -> RoundPoint {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<RoundPointAtInfinity> for FlatPoint {
    type Output = RoundPointAtInfinity;

    fn point_inversion(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPointAtOrigin;

    fn point_inversion(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<RoundPointBulkAspect> for FlatPoint {
    type Output = RoundPointBulkAspect;

    fn point_inversion(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<RoundPointCarrierAspect> for FlatPoint {
    type Output = RoundPointCarrierAspect;

    fn point_inversion(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Sphere> for FlatPoint {
    type Output = Sphere;

    fn point_inversion(self, other: Sphere) -> Sphere {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<SphereWeightAspect> for FlatPoint {
    type Output = SphereWeightAspect;

    fn point_inversion(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Transflector> for FlatPoint {
    type Output = Transflector;

    fn point_inversion(self, other: Transflector) -> Transflector {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Translator> for FlatPoint {
    type Output = Translator;

    fn point_inversion(self, other: Translator) -> Translator {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Circle> for Plane {
    type Output = Circle;

    fn reflect(self, other: Circle) -> Circle {
        self.unitize().sandwich(other)
    }
}

impl Reflect<CircleBulkAspect> for Plane {
    type Output = CircleBulkAspect;

    fn reflect(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.unitize().sandwich(other)
    }
}

impl Reflect<CircleCarrierAspect> for Plane {
    type Output = CircleCarrierAspect;

    fn reflect(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.unitize().sandwich(other)
    }
}

impl Reflect<CircleWeightAspect> for Plane {
    type Output = CircleWeightAspect;

    fn reflect(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Dipole> for Plane {
    type Output = Dipole;

    fn reflect(self, other: Dipole) -> Dipole {
        self.unitize().sandwich(other)
    }
}

impl Reflect<DipoleBulkAspect> for Plane {
    type Output = DipoleBulkAspect;

    fn reflect(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.unitize().sandwich(other)
    }
}

impl Reflect<DipoleCarrierAspect> for Plane {
    type Output = DipoleCarrierAspect;

    fn reflect(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.unitize().sandwich(other)
    }
}

impl Reflect<DipoleWeightAspect> for Plane {
    type Output = DipoleWeightAspect;

    fn reflect(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.unitize().sandwich(other)
    }
}

impl Reflect<FlatPoint> for Plane {
    type Output = FlatPoint;

    fn reflect(self, other: FlatPoint) -> FlatPoint {
        self.unitize().sandwich(other)
    }
}

impl Reflect<FlatPointAtInfinity> for Plane {
    type Output = FlatPointAtInfinity;

    fn reflect(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<FlatPointAtOrigin> for Plane {
    type Output = FlatPointAtOrigin;

    fn reflect(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Flector> for Plane {
    type Output = Flector;

    fn reflect(self, other: Flector) -> Flector {
        self.unitize().sandwich(other)
    }
}

impl Reflect<FlectorAtInfinity> for Plane {
    type Output = FlectorAtInfinity;

    fn reflect(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Horizon> for Plane {
    type Output = Horizon;

    fn reflect(self, other: Horizon) -> Horizon {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Infinity> for Plane {
    type Output = Infinity;

    fn reflect(self, other: Infinity) -> Infinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Line> for Plane {
    type Output = Line;

    fn reflect(self, other: Line) -> Line {
        self.unitize().sandwich(other)
    }
}

impl Reflect<LineAtInfinity> for Plane {
    type Output = LineAtInfinity;

    fn reflect(self, other: LineAtInfinity) -> LineAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<LineAtOrigin> for Plane {
    type Output = LineAtOrigin;

    fn reflect(self, other: LineAtOrigin) -> LineAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Motor> for Plane {
    type Output = Motor;

    fn reflect(self, other: Motor) -> Motor {
        self.unitize().sandwich(other)
    }
}

impl Reflect<MultiVector> for Plane {
    type Output = MultiVector;

    fn reflect(self, other: MultiVector) -> MultiVector {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Origin> for Plane {
    type Output = Origin;

    fn reflect(self, other: Origin) -> Origin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Plane> for Plane {
    type Output = Plane;

    fn reflect(self, other: Plane) -> Plane {
        self.unitize().sandwich(other)
    }
}

impl Reflect<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn reflect(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Rotor> for Plane {
    type Output = Rotor;

    fn reflect(self, other: Rotor) -> Rotor {
        self.unitize().sandwich(other)
    }
}

impl Reflect<RoundPoint> for Plane {
    type Output = RoundPoint;

    fn reflect(self, other: RoundPoint) -> RoundPoint {
        self.unitize().sandwich(other)
    }
}

impl Reflect<RoundPointAtInfinity> for Plane {
    type Output = RoundPointAtInfinity;

    fn reflect(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<RoundPointAtOrigin> for Plane {
    type Output = RoundPointAtOrigin;

    fn reflect(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<RoundPointBulkAspect> for Plane {
    type Output = RoundPointBulkAspect;

    fn reflect(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.unitize().sandwich(other)
    }
}

impl Reflect<RoundPointCarrierAspect> for Plane {
    type Output = RoundPointCarrierAspect;

    fn reflect(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Sphere> for Plane {
    type Output = Sphere;

    fn reflect(self, other: Sphere) -> Sphere {
        self.unitize().sandwich(other)
    }
}

impl Reflect<SphereWeightAspect> for Plane {
    type Output = SphereWeightAspect;

    fn reflect(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Transflector> for Plane {
    type Output = Transflector;

    fn reflect(self, other: Transflector) -> Transflector {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Translator> for Plane {
    type Output = Translator;

    fn reflect(self, other: Translator) -> Translator {
        self.unitize().sandwich(other)
    }
}

impl Rotate<Circle> for Rotor {
    type Output = Circle;

    fn rotate(self, other: Circle) -> Circle {
        self.sandwich(other)
    }
}

impl Rotate<CircleBulkAspect> for Rotor {
    type Output = CircleBulkAspect;

    fn rotate(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.sandwich(other)
    }
}

impl Rotate<CircleCarrierAspect> for Rotor {
    type Output = CircleCarrierAspect;

    fn rotate(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.sandwich(other)
    }
}

impl Rotate<CircleWeightAspect> for Rotor {
    type Output = CircleWeightAspect;

    fn rotate(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.sandwich(other)
    }
}

impl Rotate<Dipole> for Rotor {
    type Output = Dipole;

    fn rotate(self, other: Dipole) -> Dipole {
        self.sandwich(other)
    }
}

impl Rotate<DipoleBulkAspect> for Rotor {
    type Output = DipoleBulkAspect;

    fn rotate(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.sandwich(other)
    }
}

impl Rotate<DipoleCarrierAspect> for Rotor {
    type Output = DipoleCarrierAspect;

    fn rotate(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.sandwich(other)
    }
}

impl Rotate<DipoleWeightAspect> for Rotor {
    type Output = DipoleWeightAspect;

    fn rotate(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.sandwich(other)
    }
}

impl Rotate<FlatPoint> for Rotor {
    type Output = FlatPoint;

    fn rotate(self, other: FlatPoint) -> FlatPoint {
        self.sandwich(other)
    }
}

impl Rotate<FlatPointAtInfinity> for Rotor {
    type Output = FlatPointAtInfinity;

    fn rotate(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<FlatPointAtOrigin> for Rotor {
    type Output = FlatPointAtOrigin;

    fn rotate(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<Flector> for Rotor {
    type Output = Flector;

    fn rotate(self, other: Flector) -> Flector {
        self.sandwich(other)
    }
}

impl Rotate<FlectorAtInfinity> for Rotor {
    type Output = FlectorAtInfinity;

    fn rotate(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<Horizon> for Rotor {
    type Output = Horizon;

    fn rotate(self, other: Horizon) -> Horizon {
        self.sandwich(other)
    }
}

impl Rotate<Infinity> for Rotor {
    type Output = Infinity;

    fn rotate(self, other: Infinity) -> Infinity {
        self.sandwich(other)
    }
}

impl Rotate<Line> for Rotor {
    type Output = Line;

    fn rotate(self, other: Line) -> Line {
        self.sandwich(other)
    }
}

impl Rotate<LineAtInfinity> for Rotor {
    type Output = LineAtInfinity;

    fn rotate(self, other: LineAtInfinity) -> LineAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn rotate(self, other: LineAtOrigin) -> LineAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<Motor> for Rotor {
    type Output = Motor;

    fn rotate(self, other: Motor) -> Motor {
        self.sandwich(other)
    }
}

impl Rotate<MultiVector> for Rotor {
    type Output = MultiVector;

    fn rotate(self, other: MultiVector) -> MultiVector {
        self.sandwich(other)
    }
}

impl Rotate<Origin> for Rotor {
    type Output = Origin;

    fn rotate(self, other: Origin) -> Origin {
        self.sandwich(other)
    }
}

impl Rotate<Plane> for Rotor {
    type Output = Plane;

    fn rotate(self, other: Plane) -> Plane {
        self.sandwich(other)
    }
}

impl Rotate<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn rotate(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<Rotor> for Rotor {
    type Output = Rotor;

    fn rotate(self, other: Rotor) -> Rotor {
        self.sandwich(other)
    }
}

impl Rotate<RoundPoint> for Rotor {
    type Output = RoundPoint;

    fn rotate(self, other: RoundPoint) -> RoundPoint {
        self.sandwich(other)
    }
}

impl Rotate<RoundPointAtInfinity> for Rotor {
    type Output = RoundPointAtInfinity;

    fn rotate(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<RoundPointAtOrigin> for Rotor {
    type Output = RoundPointAtOrigin;

    fn rotate(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<RoundPointBulkAspect> for Rotor {
    type Output = RoundPointBulkAspect;

    fn rotate(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.sandwich(other)
    }
}

impl Rotate<RoundPointCarrierAspect> for Rotor {
    type Output = RoundPointCarrierAspect;

    fn rotate(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.sandwich(other)
    }
}

impl Rotate<Sphere> for Rotor {
    type Output = Sphere;

    fn rotate(self, other: Sphere) -> Sphere {
        self.sandwich(other)
    }
}

impl Rotate<SphereWeightAspect> for Rotor {
    type Output = SphereWeightAspect;

    fn rotate(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.sandwich(other)
    }
}

impl Rotate<Transflector> for Rotor {
    type Output = Transflector;

    fn rotate(self, other: Transflector) -> Transflector {
        self.sandwich(other)
    }
}

impl Rotate<Translator> for Rotor {
    type Output = Translator;

    fn rotate(self, other: Translator) -> Translator {
        self.sandwich(other)
    }
}

impl Transflect<Circle> for Transflector {
    type Output = Circle;

    fn transflect(self, other: Circle) -> Circle {
        self.sandwich(other)
    }
}

impl Transflect<CircleBulkAspect> for Transflector {
    type Output = CircleBulkAspect;

    fn transflect(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.sandwich(other)
    }
}

impl Transflect<CircleCarrierAspect> for Transflector {
    type Output = CircleCarrierAspect;

    fn transflect(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.sandwich(other)
    }
}

impl Transflect<CircleWeightAspect> for Transflector {
    type Output = CircleWeightAspect;

    fn transflect(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.sandwich(other)
    }
}

impl Transflect<Dipole> for Transflector {
    type Output = Dipole;

    fn transflect(self, other: Dipole) -> Dipole {
        self.sandwich(other)
    }
}

impl Transflect<DipoleBulkAspect> for Transflector {
    type Output = DipoleBulkAspect;

    fn transflect(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.sandwich(other)
    }
}

impl Transflect<DipoleCarrierAspect> for Transflector {
    type Output = DipoleCarrierAspect;

    fn transflect(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.sandwich(other)
    }
}

impl Transflect<DipoleWeightAspect> for Transflector {
    type Output = DipoleWeightAspect;

    fn transflect(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.sandwich(other)
    }
}

impl Transflect<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn transflect(self, other: FlatPoint) -> FlatPoint {
        self.sandwich(other)
    }
}

impl Transflect<FlatPointAtInfinity> for Transflector {
    type Output = FlatPointAtInfinity;

    fn transflect(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<FlatPointAtOrigin> for Transflector {
    type Output = FlatPointAtOrigin;

    fn transflect(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<Flector> for Transflector {
    type Output = Flector;

    fn transflect(self, other: Flector) -> Flector {
        self.sandwich(other)
    }
}

impl Transflect<FlectorAtInfinity> for Transflector {
    type Output = FlectorAtInfinity;

    fn transflect(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<Horizon> for Transflector {
    type Output = Horizon;

    fn transflect(self, other: Horizon) -> Horizon {
        self.sandwich(other)
    }
}

impl Transflect<Infinity> for Transflector {
    type Output = Infinity;

    fn transflect(self, other: Infinity) -> Infinity {
        self.sandwich(other)
    }
}

impl Transflect<Line> for Transflector {
    type Output = Line;

    fn transflect(self, other: Line) -> Line {
        self.sandwich(other)
    }
}

impl Transflect<LineAtInfinity> for Transflector {
    type Output = LineAtInfinity;

    fn transflect(self, other: LineAtInfinity) -> LineAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<LineAtOrigin> for Transflector {
    type Output = LineAtOrigin;

    fn transflect(self, other: LineAtOrigin) -> LineAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<Motor> for Transflector {
    type Output = Motor;

    fn transflect(self, other: Motor) -> Motor {
        self.sandwich(other)
    }
}

impl Transflect<MultiVector> for Transflector {
    type Output = MultiVector;

    fn transflect(self, other: MultiVector) -> MultiVector {
        self.sandwich(other)
    }
}

impl Transflect<Origin> for Transflector {
    type Output = Origin;

    fn transflect(self, other: Origin) -> Origin {
        self.sandwich(other)
    }
}

impl Transflect<Plane> for Transflector {
    type Output = Plane;

    fn transflect(self, other: Plane) -> Plane {
        self.sandwich(other)
    }
}

impl Transflect<PlaneAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn transflect(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<Rotor> for Transflector {
    type Output = Rotor;

    fn transflect(self, other: Rotor) -> Rotor {
        self.sandwich(other)
    }
}

impl Transflect<RoundPoint> for Transflector {
    type Output = RoundPoint;

    fn transflect(self, other: RoundPoint) -> RoundPoint {
        self.sandwich(other)
    }
}

impl Transflect<RoundPointAtInfinity> for Transflector {
    type Output = RoundPointAtInfinity;

    fn transflect(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<RoundPointAtOrigin> for Transflector {
    type Output = RoundPointAtOrigin;

    fn transflect(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<RoundPointBulkAspect> for Transflector {
    type Output = RoundPointBulkAspect;

    fn transflect(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.sandwich(other)
    }
}

impl Transflect<RoundPointCarrierAspect> for Transflector {
    type Output = RoundPointCarrierAspect;

    fn transflect(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.sandwich(other)
    }
}

impl Transflect<Sphere> for Transflector {
    type Output = Sphere;

    fn transflect(self, other: Sphere) -> Sphere {
        self.sandwich(other)
    }
}

impl Transflect<SphereWeightAspect> for Transflector {
    type Output = SphereWeightAspect;

    fn transflect(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.sandwich(other)
    }
}

impl Transflect<Transflector> for Transflector {
    type Output = Transflector;

    fn transflect(self, other: Transflector) -> Transflector {
        self.sandwich(other)
    }
}

impl Transflect<Translator> for Transflector {
    type Output = Translator;

    fn transflect(self, other: Translator) -> Translator {
        self.sandwich(other)
    }
}

impl Translate<Circle> for Translator {
    type Output = Circle;

    fn translate(self, other: Circle) -> Circle {
        self.sandwich(other)
    }
}

impl Translate<CircleBulkAspect> for Translator {
    type Output = CircleBulkAspect;

    fn translate(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.sandwich(other)
    }
}

impl Translate<CircleCarrierAspect> for Translator {
    type Output = CircleCarrierAspect;

    fn translate(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.sandwich(other)
    }
}

impl Translate<CircleWeightAspect> for Translator {
    type Output = CircleWeightAspect;

    fn translate(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.sandwich(other)
    }
}

impl Translate<Dipole> for Translator {
    type Output = Dipole;

    fn translate(self, other: Dipole) -> Dipole {
        self.sandwich(other)
    }
}

impl Translate<DipoleBulkAspect> for Translator {
    type Output = DipoleBulkAspect;

    fn translate(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.sandwich(other)
    }
}

impl Translate<DipoleCarrierAspect> for Translator {
    type Output = DipoleCarrierAspect;

    fn translate(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.sandwich(other)
    }
}

impl Translate<DipoleWeightAspect> for Translator {
    type Output = DipoleWeightAspect;

    fn translate(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
        self.sandwich(other)
    }
}

impl Translate<FlatPoint> for Translator {
    type Output = FlatPoint;

    fn translate(self, other: FlatPoint) -> FlatPoint {
        self.sandwich(other)
    }
}

impl Translate<FlatPointAtInfinity> for Translator {
    type Output = FlatPointAtInfinity;

    fn translate(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<FlatPointAtOrigin> for Translator {
    type Output = FlatPointAtOrigin;

    fn translate(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<Flector> for Translator {
    type Output = Flector;

    fn translate(self, other: Flector) -> Flector {
        self.sandwich(other)
    }
}

impl Translate<FlectorAtInfinity> for Translator {
    type Output = FlectorAtInfinity;

    fn translate(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<Horizon> for Translator {
    type Output = Horizon;

    fn translate(self, other: Horizon) -> Horizon {
        self.sandwich(other)
    }
}

impl Translate<Infinity> for Translator {
    type Output = Infinity;

    fn translate(self, other: Infinity) -> Infinity {
        self.sandwich(other)
    }
}

impl Translate<Line> for Translator {
    type Output = Line;

    fn translate(self, other: Line) -> Line {
        self.sandwich(other)
    }
}

impl Translate<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn translate(self, other: LineAtInfinity) -> LineAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<LineAtOrigin> for Translator {
    type Output = Line;

    fn translate(self, other: LineAtOrigin) -> Line {
        self.sandwich(other)
    }
}

impl Translate<Motor> for Translator {
    type Output = Motor;

    fn translate(self, other: Motor) -> Motor {
        self.sandwich(other)
    }
}

impl Translate<MultiVector> for Translator {
    type Output = MultiVector;

    fn translate(self, other: MultiVector) -> MultiVector {
        self.sandwich(other)
    }
}

impl Translate<Origin> for Translator {
    type Output = Origin;

    fn translate(self, other: Origin) -> Origin {
        self.sandwich(other)
    }
}

impl Translate<Plane> for Translator {
    type Output = Plane;

    fn translate(self, other: Plane) -> Plane {
        self.sandwich(other)
    }
}

impl Translate<PlaneAtOrigin> for Translator {
    type Output = Plane;

    fn translate(self, other: PlaneAtOrigin) -> Plane {
        self.sandwich(other)
    }
}

impl Translate<Rotor> for Translator {
    type Output = Motor;

    fn translate(self, other: Rotor) -> Motor {
        self.sandwich(other)
    }
}

impl Translate<RoundPoint> for Translator {
    type Output = RoundPoint;

    fn translate(self, other: RoundPoint) -> RoundPoint {
        self.sandwich(other)
    }
}

impl Translate<RoundPointAtInfinity> for Translator {
    type Output = RoundPointAtInfinity;

    fn translate(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<RoundPointAtOrigin> for Translator {
    type Output = RoundPointAtOrigin;

    fn translate(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<RoundPointBulkAspect> for Translator {
    type Output = RoundPointBulkAspect;

    fn translate(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.sandwich(other)
    }
}

impl Translate<RoundPointCarrierAspect> for Translator {
    type Output = RoundPointCarrierAspect;

    fn translate(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.sandwich(other)
    }
}

impl Translate<Sphere> for Translator {
    type Output = Sphere;

    fn translate(self, other: Sphere) -> Sphere {
        self.sandwich(other)
    }
}

impl Translate<SphereWeightAspect> for Translator {
    type Output = SphereWeightAspect;

    fn translate(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.sandwich(other)
    }
}

impl Translate<Transflector> for Translator {
    type Output = Transflector;

    fn translate(self, other: Transflector) -> Transflector {
        self.sandwich(other)
    }
}

impl Translate<Translator> for Translator {
    type Output = Translator;

    fn translate(self, other: Translator) -> Translator {
        self.sandwich(other)
    }
}
