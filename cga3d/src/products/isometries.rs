//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::AntiReversal;
use crate::products::geometric::*;
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

impl Sandwich<AntiCircleOnOrigin> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for AntiCircleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for AntiCircleOnOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for AntiCircleOnOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for AntiCircleOnOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for AntiCircleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for AntiCircleOnOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for AntiCircleOnOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for AntiCircleOnOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for AntiCircleOnOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for AntiCircleOnOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for AntiCircleOnOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for AntiCircleOnOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for AntiCircleOnOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for AntiCircleOnOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for AntiCircleOnOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for AntiCircleOnOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for AntiCircleOnOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for AntiCircleOnOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for AntiCircleOnOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for AntiCircleOnOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for AntiCircleOnOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for AntiCircleOnOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for AntiCircleOnOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for AntiCircleOnOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for AntiCircleOnOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for AntiCircleOnOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for AntiCircleOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for AntiCircleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for AntiCircleOnOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for AntiCircleOnOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for AntiCircleOnOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for AntiCircleOnOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for AntiCircleOnOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for AntiCircleOnOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for AntiCircleOnOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for AntiCircleOnOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for AntiCircleOnOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for AntiCircleOnOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for AntiDipoleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for AntiDipoleOnOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for AntiDipoleOnOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for AntiDipoleOnOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for AntiDipoleOnOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for AntiDipoleOnOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for AntiDipoleOnOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for AntiDipoleOnOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for AntiDipoleOnOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for AntiDipoleOnOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for AntiDipoleOnOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for AntiDipoleOnOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for AntiDipoleOnOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for AntiDipoleOnOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for AntiDipoleOnOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for AntiDipoleOnOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for AntiDipoleOnOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for AntiDipoleOnOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for AntiDipoleOnOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for AntiDipoleOnOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for AntiDipoleOnOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullDipoleAtOrigin> for AntiDipoleOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullSphereAtOrigin> for AntiDipoleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for AntiDipoleOnOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for AntiDipoleOnOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for AntiDipoleOnOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for AntiDipoleOnOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for AntiDipoleOnOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for AntiDipoleOnOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for AntiDipoleOnOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for AntiDipoleOnOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiLineAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiPlane> for AntiFlatPointAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiSphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for AntiFlatPointAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for AntiFlatPointAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for AntiFlatPointAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for AntiFlatPointAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for AntiFlatPointAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for AntiFlatPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for AntiFlatPointAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for AntiFlatPointAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Flector> for AntiFlatPointAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for AntiFlatPointAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for AntiFlatPointAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Infinity> for AntiFlatPointAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for AntiFlatPointAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for AntiFlatPointAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for AntiFlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Motor> for AntiFlatPointAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for AntiFlatPointAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullDipoleAtOrigin> for AntiFlatPointAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullSphereAtOrigin> for AntiFlatPointAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for AntiFlatPointAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for AntiFlatPointAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for AntiFlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for AntiFlatPointAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for AntiFlatPointAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Sphere> for AntiFlatPointAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for AntiFlatPointAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<SphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for AntiFlatPointAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for AntiFlatPointAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for AntiLineAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for AntiLineAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for AntiLineAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for AntiLineAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for AntiLineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for AntiLineAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for AntiLineAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for AntiLineAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for AntiLineAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for AntiLineAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for AntiLineAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for AntiLineAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for AntiLineAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for AntiLineAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for AntiLineAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for AntiLineAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for AntiLineAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for AntiLineAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for AntiLineAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for AntiLineAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for AntiLineAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for AntiLineAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for AntiLineAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for AntiLineAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for AntiLineAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for AntiLineAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for AntiLineAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for AntiLineAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for AntiLineAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for AntiLineAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for AntiLineAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for AntiLineAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for AntiLineAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for AntiLineAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for AntiLineAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for AntiLineAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for AntiLineAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for AntiLineAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for AntiLineAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for AntiLineAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for AntiLineAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for AntiPlane {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for AntiPlane {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for AntiPlane {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for AntiPlane {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for AntiPlane {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for AntiPlane {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for AntiPlane {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for AntiPlane {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for AntiPlane {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for AntiPlane {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for AntiPlane {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for AntiPlane {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for AntiPlane {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for AntiPlane {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for AntiPlane {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for AntiPlane {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for AntiPlane {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for AntiPlane {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for AntiPlane {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for AntiPlane {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for AntiPlane {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for AntiPlane {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for AntiPlane {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for AntiPlane {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for AntiPlane {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for AntiPlane {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for AntiPlane {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for AntiPlane {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for AntiPlane {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for AntiPlane {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for AntiPlane {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for AntiPlane {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for AntiPlane {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for AntiPlane {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for AntiPlane {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for AntiPlane {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for AntiPlane {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for AntiPlane {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for AntiPlane {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for AntiPlane {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for AntiPlane {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for AntiPlane {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for AntiPlane {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for AntiPlane {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for AntiPlane {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for AntiPlaneAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for AntiPlaneAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for AntiPlaneAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for AntiPlaneAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for AntiPlaneAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for AntiPlaneAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for AntiPlaneAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for AntiPlaneAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for AntiPlaneAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for AntiPlaneAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for AntiPlaneAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for AntiPlaneAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for AntiPlaneAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for AntiPlaneAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for AntiPlaneAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for AntiPlaneAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for AntiPlaneAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for AntiPlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for AntiPlaneAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for AntiPlaneAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for AntiPlaneAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for AntiPlaneAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for AntiPlaneAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for AntiPlaneAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for AntiPlaneAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for AntiPlaneAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for AntiPlaneAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for AntiPlaneAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for AntiPlaneAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for AntiPlaneAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for AntiPlaneAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for AntiPlaneAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for AntiPlaneAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for AntiPlaneAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for AntiPlaneAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for AntiPlaneAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for AntiPlaneAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for AntiScalar {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiDipoleOnOrigin> for AntiScalar {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for AntiScalar {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiLineAtOrigin> for AntiScalar {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiPlane> for AntiScalar {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiPlaneAtOrigin> for AntiScalar {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiSphereOnOrigin> for AntiScalar {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Circle> for AntiScalar {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleAligningOrigin> for AntiScalar {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleAtInfinity> for AntiScalar {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleAtOrigin> for AntiScalar {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleOnOrigin> for AntiScalar {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleOrthogonalOrigin> for AntiScalar {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Dipole> for AntiScalar {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleAligningOrigin> for AntiScalar {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleAtInfinity> for AntiScalar {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleAtOrigin> for AntiScalar {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleOnOrigin> for AntiScalar {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for AntiScalar {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for AntiScalar {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullDipoleAtOrigin> for AntiScalar {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullSphereAtOrigin> for AntiScalar {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
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

impl Sandwich<RoundPointAtOrigin> for AntiScalar {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Sphere> for AntiScalar {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<SphereAtOrigin> for AntiScalar {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<SphereOnOrigin> for AntiScalar {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for AntiSphereOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for AntiSphereOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for AntiSphereOnOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for AntiSphereOnOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for AntiSphereOnOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for AntiSphereOnOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for AntiSphereOnOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for AntiSphereOnOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for AntiSphereOnOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for AntiSphereOnOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for AntiSphereOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for AntiSphereOnOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for AntiSphereOnOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for AntiSphereOnOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for AntiSphereOnOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for AntiSphereOnOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for AntiSphereOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for AntiSphereOnOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for AntiSphereOnOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for AntiSphereOnOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for AntiSphereOnOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for AntiSphereOnOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for AntiSphereOnOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for AntiSphereOnOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for AntiSphereOnOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for AntiSphereOnOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for AntiSphereOnOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for AntiSphereOnOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for AntiSphereOnOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for AntiSphereOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for AntiSphereOnOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for AntiSphereOnOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for AntiSphereOnOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for AntiSphereOnOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for AntiSphereOnOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for AntiSphereOnOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for AntiSphereOnOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for AntiSphereOnOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for AntiSphereOnOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for AntiSphereOnOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for Circle {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Circle {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Circle {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Circle {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Circle {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Circle {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Circle {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Circle {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Circle {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Circle {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Circle {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Circle {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Circle {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Circle {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Circle {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Circle {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Circle {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Circle {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Circle {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Circle {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Circle {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Circle {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Circle {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Circle {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Circle {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Circle {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for CircleAligningOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for CircleAligningOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for CircleAligningOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for CircleAligningOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for CircleAligningOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for CircleAligningOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for CircleAligningOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for CircleAligningOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for CircleAligningOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for CircleAligningOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for CircleAligningOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for CircleAligningOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for CircleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for CircleAligningOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for CircleAligningOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for CircleAligningOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for CircleAligningOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for CircleAligningOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for CircleAligningOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for CircleAligningOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for CircleAligningOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for CircleAligningOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for CircleAligningOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for CircleAligningOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for CircleAligningOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for CircleAligningOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for CircleAligningOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for CircleAligningOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for CircleAligningOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for CircleAligningOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for CircleAligningOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for CircleAligningOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for CircleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for CircleAligningOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for CircleAligningOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for CircleAligningOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for CircleAligningOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for CircleAligningOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for CircleAligningOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for CircleAligningOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for CircleAligningOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for CircleAtInfinity {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for CircleAtInfinity {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for CircleAtInfinity {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for CircleAtInfinity {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for CircleAtInfinity {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for CircleAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for CircleAtInfinity {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for CircleAtInfinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for CircleAtInfinity {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for CircleAtInfinity {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for CircleAtInfinity {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for CircleAtInfinity {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for CircleAtInfinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for CircleAtInfinity {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for CircleAtInfinity {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for CircleAtInfinity {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for CircleAtInfinity {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for CircleAtInfinity {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for CircleAtInfinity {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for CircleAtInfinity {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for CircleAtInfinity {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for CircleAtInfinity {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for CircleAtInfinity {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for CircleAtInfinity {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for CircleAtInfinity {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for CircleAtInfinity {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for CircleAtInfinity {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for CircleAtInfinity {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for CircleAtInfinity {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for CircleAtInfinity {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for CircleAtInfinity {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for CircleAtInfinity {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for CircleAtInfinity {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for CircleAtInfinity {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for CircleAtInfinity {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for CircleAtInfinity {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for CircleAtInfinity {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for CircleAtInfinity {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for CircleAtInfinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for CircleAtInfinity {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for CircleAtInfinity {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for CircleAtInfinity {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for CircleAtInfinity {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for CircleAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for CircleAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for CircleAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for CircleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for CircleAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for CircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for CircleAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for CircleAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for CircleAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for CircleAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for CircleAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for CircleAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for CircleAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for CircleAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for CircleAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for CircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for CircleAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for CircleAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for CircleAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for CircleAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for CircleAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for CircleAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for CircleAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for CircleAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for CircleAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for CircleAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for CircleAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for CircleAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for CircleAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for CircleAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for CircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for CircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for CircleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for CircleAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for CircleAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for CircleAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for CircleAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for CircleAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for CircleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for CircleAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for CircleAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for CircleAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for CircleAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for CircleAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for CircleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for CircleOnOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for CircleOnOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for CircleOnOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for CircleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for CircleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for CircleOnOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for CircleOnOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for CircleOnOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for CircleOnOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for CircleOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for CircleOnOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for CircleOnOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for CircleOnOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for CircleOnOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for CircleOnOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for CircleOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for CircleOnOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for CircleOnOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for CircleOnOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for CircleOnOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for CircleOnOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for CircleOnOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for CircleOnOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for CircleOnOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for CircleOnOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for CircleOnOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for CircleOnOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for CircleOnOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for CircleOnOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for CircleOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for CircleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for CircleOnOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for CircleOnOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for CircleOnOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for CircleOnOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for CircleOnOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for CircleOnOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for CircleOnOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for CircleOnOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for CircleOnOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for CircleOnOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for CircleOnOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for CircleOrthogonalOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for CircleOrthogonalOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for CircleOrthogonalOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for CircleOrthogonalOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for CircleOrthogonalOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for CircleOrthogonalOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for CircleOrthogonalOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for CircleOrthogonalOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for CircleOrthogonalOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for CircleOrthogonalOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for CircleOrthogonalOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for CircleOrthogonalOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for CircleOrthogonalOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for CircleOrthogonalOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for CircleOrthogonalOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for CircleOrthogonalOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for CircleOrthogonalOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for CircleOrthogonalOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for CircleOrthogonalOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for CircleOrthogonalOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for CircleOrthogonalOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for CircleOrthogonalOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for CircleOrthogonalOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for CircleOrthogonalOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for CircleOrthogonalOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for CircleOrthogonalOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for CircleOrthogonalOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for Dipole {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Dipole {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Dipole {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Dipole {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Dipole {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Dipole {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Dipole {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Dipole {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Dipole {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Dipole {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Dipole {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Dipole {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Dipole {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Dipole {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Dipole {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Dipole {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Dipole {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Dipole {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Dipole {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Dipole {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Dipole {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Dipole {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Dipole {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Dipole {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Dipole {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Dipole {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for DipoleAligningOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for DipoleAligningOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for DipoleAligningOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for DipoleAligningOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for DipoleAligningOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for DipoleAligningOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DipoleAligningOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for DipoleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for DipoleAligningOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for DipoleAligningOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for DipoleAligningOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DipoleAligningOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for DipoleAligningOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for DipoleAligningOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for DipoleAligningOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DipoleAligningOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for DipoleAligningOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DipoleAligningOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for DipoleAligningOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for DipoleAligningOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for DipoleAligningOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DipoleAligningOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for DipoleAligningOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for DipoleAligningOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DipoleAligningOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for DipoleAligningOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for DipoleAligningOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for DipoleAligningOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for DipoleAligningOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for DipoleAligningOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for DipoleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for DipoleAligningOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DipoleAligningOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for DipoleAligningOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DipoleAligningOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for DipoleAligningOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for DipoleAligningOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for DipoleAligningOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DipoleAligningOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for DipoleAtInfinity {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for DipoleAtInfinity {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for DipoleAtInfinity {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for DipoleAtInfinity {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for DipoleAtInfinity {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for DipoleAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for DipoleAtInfinity {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DipoleAtInfinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for DipoleAtInfinity {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for DipoleAtInfinity {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for DipoleAtInfinity {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for DipoleAtInfinity {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DipoleAtInfinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for DipoleAtInfinity {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for DipoleAtInfinity {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for DipoleAtInfinity {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DipoleAtInfinity {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for DipoleAtInfinity {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for DipoleAtInfinity {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DipoleAtInfinity {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for DipoleAtInfinity {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for DipoleAtInfinity {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for DipoleAtInfinity {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DipoleAtInfinity {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for DipoleAtInfinity {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for DipoleAtInfinity {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DipoleAtInfinity {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for DipoleAtInfinity {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for DipoleAtInfinity {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for DipoleAtInfinity {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for DipoleAtInfinity {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for DipoleAtInfinity {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for DipoleAtInfinity {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for DipoleAtInfinity {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DipoleAtInfinity {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for DipoleAtInfinity {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DipoleAtInfinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for DipoleAtInfinity {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for DipoleAtInfinity {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for DipoleAtInfinity {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DipoleAtInfinity {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for DipoleAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for DipoleAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for DipoleAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for DipoleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for DipoleAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for DipoleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for DipoleAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DipoleAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for DipoleAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for DipoleAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for DipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for DipoleAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DipoleAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for DipoleAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for DipoleAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for DipoleAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DipoleAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for DipoleAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for DipoleAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DipoleAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for DipoleAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for DipoleAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for DipoleAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DipoleAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for DipoleAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for DipoleAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DipoleAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for DipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for DipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for DipoleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for DipoleAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for DipoleAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for DipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for DipoleAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DipoleAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for DipoleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DipoleAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for DipoleAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for DipoleAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for DipoleAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DipoleAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for DipoleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for DipoleOnOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for DipoleOnOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for DipoleOnOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for DipoleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for DipoleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DipoleOnOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for DipoleOnOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for DipoleOnOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for DipoleOnOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for DipoleOnOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DipoleOnOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DipoleOnOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for DipoleOnOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for DipoleOnOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DipoleOnOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for DipoleOnOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for DipoleOnOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for DipoleOnOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DipoleOnOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for DipoleOnOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for DipoleOnOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DipoleOnOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for DipoleOnOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullDipoleAtOrigin> for DipoleOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullSphereAtOrigin> for DipoleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for DipoleOnOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for DipoleOnOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for DipoleOnOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for DipoleOnOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DipoleOnOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for DipoleOnOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DipoleOnOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for DipoleOnOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for DipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for DipoleOnOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DipoleOnOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for DipoleOrthogonalOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DipoleOrthogonalOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DipoleOrthogonalOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DipoleOrthogonalOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for DipoleOrthogonalOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DipoleOrthogonalOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for DipoleOrthogonalOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for DipoleOrthogonalOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for DipoleOrthogonalOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DipoleOrthogonalOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for DipoleOrthogonalOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DipoleOrthogonalOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for DipoleOrthogonalOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for DipoleOrthogonalOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for DipoleOrthogonalOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for DipoleOrthogonalOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DipoleOrthogonalOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for DipoleOrthogonalOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for DipoleOrthogonalOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DipoleOrthogonalOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for DualNum {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for DualNum {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for DualNum {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for DualNum {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for DualNum {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for DualNum {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for DualNum {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DualNum {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for DualNum {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for DualNum {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for DualNum {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for DualNum {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for DualNum {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DualNum {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for DualNum {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for DualNum {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for DualNum {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for DualNum {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for DualNum {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DualNum {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for DualNum {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for DualNum {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DualNum {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for DualNum {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for DualNum {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for DualNum {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DualNum {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for DualNum {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for DualNum {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DualNum {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DualNum {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for DualNum {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for DualNum {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for DualNum {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for DualNum {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for DualNum {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for DualNum {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for DualNum {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DualNum {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for DualNum {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DualNum {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for DualNum {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for DualNum {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for DualNum {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DualNum {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for FlatPoint {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for FlatPoint {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for FlatPoint {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for FlatPoint {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for FlatPoint {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for FlatPoint {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for FlatPoint {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for FlatPoint {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for FlatPoint {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for FlatPoint {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for FlatPoint {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for FlatPoint {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for FlatPoint {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for FlatPoint {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for FlatPoint {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for FlatPoint {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for FlatPoint {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for FlatPoint {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for FlatPoint {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for FlatPoint {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for FlatPoint {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for FlatPoint {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for FlatPoint {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for FlatPoint {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for FlatPoint {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for FlatPointAtInfinity {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for FlatPointAtInfinity {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for FlatPointAtInfinity {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for FlatPointAtInfinity {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for FlatPointAtInfinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for FlatPointAtInfinity {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for FlatPointAtInfinity {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for FlatPointAtInfinity {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for FlatPointAtInfinity {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for FlatPointAtInfinity {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for FlatPointAtInfinity {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for FlatPointAtInfinity {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for FlatPointAtInfinity {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for FlatPointAtInfinity {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for FlatPointAtInfinity {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for FlatPointAtInfinity {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for FlatPointAtInfinity {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<Sphere> for FlatPointAtInfinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for FlatPointAtInfinity {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for FlatPointAtInfinity {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for FlatPointAtInfinity {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for FlatPointAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for FlatPointAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiLineAtOrigin> for FlatPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiPlane> for FlatPointAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for FlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiSphereOnOrigin> for FlatPointAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for FlatPointAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for FlatPointAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for FlatPointAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for FlatPointAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleOnOrigin> for FlatPointAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for FlatPointAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for FlatPointAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for FlatPointAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for FlatPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleOnOrigin> for FlatPointAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for FlatPointAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<NullCircleAtOrigin> for FlatPointAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullDipoleAtOrigin> for FlatPointAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullSphereAtOrigin> for FlatPointAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
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

impl Sandwich<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Sphere> for FlatPointAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for FlatPointAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<SphereOnOrigin> for FlatPointAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<AntiCircleOnOrigin> for Flector {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Flector {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Flector {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Flector {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Flector {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Flector {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Flector {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Flector {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Flector {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Flector {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Flector {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Flector {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Flector {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Flector {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Flector {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Flector {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Flector {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Flector {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Flector {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Flector {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Flector {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Flector {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Flector {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Flector {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Flector {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Flector {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for FlectorAtInfinity {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for FlectorAtInfinity {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for FlectorAtInfinity {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for FlectorAtInfinity {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for FlectorAtInfinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for FlectorAtInfinity {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for FlectorAtInfinity {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for FlectorAtInfinity {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for FlectorAtInfinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for FlectorAtInfinity {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for FlectorAtInfinity {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for FlectorAtInfinity {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for FlectorAtInfinity {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for FlectorAtInfinity {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for FlectorAtInfinity {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for FlectorAtInfinity {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<Sphere> for FlectorAtInfinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for FlectorAtInfinity {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for FlectorAtInfinity {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for FlectorAtInfinity {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for Horizon {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Horizon {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Horizon {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Horizon {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Horizon {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Horizon {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Horizon {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Horizon {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Horizon {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Horizon {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Horizon {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Horizon {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Horizon {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Horizon {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Horizon {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Horizon {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Horizon {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<Sphere> for Horizon {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Horizon {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Horizon {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for Infinity {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Infinity {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Infinity {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Infinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Infinity {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Infinity {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Infinity {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Infinity {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Infinity {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Infinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Infinity {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Infinity {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Infinity {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Infinity {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Infinity {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Infinity {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Infinity {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<Sphere> for Infinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Infinity {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Infinity {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for Line {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Line {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Line {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Line {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Line {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Line {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Line {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Line {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Line {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Line {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Line {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Line {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Line {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Line {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Line {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Line {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Line {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Line {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Line {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Line {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Line {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Line {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Line {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Line {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Line {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Line {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for LineAtInfinity {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for LineAtInfinity {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for LineAtInfinity {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for LineAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for LineAtInfinity {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for LineAtInfinity {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for LineAtInfinity {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for LineAtInfinity {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for LineAtInfinity {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for LineAtInfinity {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for LineAtInfinity {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for LineAtInfinity {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for LineAtInfinity {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for LineAtInfinity {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for LineAtInfinity {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for LineAtInfinity {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for LineAtInfinity {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for LineAtInfinity {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for LineAtInfinity {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for LineAtInfinity {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for LineAtInfinity {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for LineAtInfinity {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for LineAtInfinity {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for LineAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for LineAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for LineAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for LineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for LineAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for LineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for LineAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for LineAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for LineAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for LineAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for LineAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for LineAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for LineAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for LineAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for LineAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for LineAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for LineAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for LineAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for LineAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for LineAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for LineAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for LineAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for LineAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for LineAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for LineAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for LineAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for Motor {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Motor {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Motor {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Motor {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Motor {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Motor {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Motor {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Motor {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Motor {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Motor {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Motor {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Motor {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Motor {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Motor {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Motor {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Motor {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Motor {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Motor {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Motor {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Motor {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Motor {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Motor {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Motor {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Motor {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Motor {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Motor {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for MultiVector {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for MultiVector {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for MultiVector {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for MultiVector {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for MultiVector {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for MultiVector {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for MultiVector {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for MultiVector {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for MultiVector {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for MultiVector {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for MultiVector {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for MultiVector {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for MultiVector {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for MultiVector {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for MultiVector {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for MultiVector {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for MultiVector {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for MultiVector {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for MultiVector {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for MultiVector {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for MultiVector {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for MultiVector {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for MultiVector {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for MultiVector {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for MultiVector {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for MultiVector {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for NullCircleAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for NullCircleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for NullCircleAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for NullCircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for NullCircleAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for NullCircleAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for NullCircleAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for NullCircleAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for NullCircleAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for NullCircleAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for NullCircleAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for NullCircleAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for NullCircleAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for NullCircleAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for NullCircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for NullCircleAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for NullCircleAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for NullCircleAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for NullCircleAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for NullCircleAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for NullCircleAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for NullCircleAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for NullCircleAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for NullCircleAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for NullCircleAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for NullCircleAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for NullCircleAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for NullCircleAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for NullCircleAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for NullCircleAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for NullCircleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for NullCircleAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for NullCircleAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for NullCircleAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for NullCircleAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for NullCircleAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for NullDipoleAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for NullDipoleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for NullDipoleAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for NullDipoleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for NullDipoleAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for NullDipoleAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for NullDipoleAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for NullDipoleAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for NullDipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for NullDipoleAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for NullDipoleAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for NullDipoleAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for NullDipoleAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for NullDipoleAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for NullDipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for NullDipoleAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for NullDipoleAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for NullDipoleAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for NullDipoleAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for NullDipoleAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for NullDipoleAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for NullDipoleAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for NullDipoleAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for NullDipoleAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for NullDipoleAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for NullDipoleAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for NullDipoleAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for NullDipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for NullDipoleAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for NullDipoleAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for NullDipoleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for NullDipoleAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for NullDipoleAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for NullDipoleAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for NullDipoleAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for NullDipoleAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for NullSphereAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for NullSphereAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for NullSphereAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for NullSphereAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for NullSphereAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for NullSphereAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for NullSphereAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for NullSphereAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for NullSphereAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for NullSphereAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for NullSphereAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for NullSphereAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for NullSphereAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for NullSphereAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for NullSphereAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for NullSphereAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for NullSphereAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for NullSphereAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for NullSphereAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for NullSphereAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for NullSphereAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for NullSphereAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for NullSphereAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for NullSphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for NullSphereAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for NullSphereAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for NullSphereAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for NullSphereAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Origin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Origin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Origin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Origin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Origin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Origin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Origin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Origin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Origin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Origin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Origin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<SphereAtOrigin> for Origin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for Plane {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Plane {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Plane {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Plane {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Plane {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Plane {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Plane {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Plane {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Plane {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Plane {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Plane {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Plane {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Plane {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Plane {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Plane {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Plane {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Plane {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Plane {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Plane {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Plane {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Plane {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Plane {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Plane {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Plane {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Plane {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Plane {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for PlaneAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for PlaneAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for PlaneAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for PlaneAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for PlaneAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for PlaneAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for PlaneAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for PlaneAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for PlaneAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for PlaneAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for PlaneAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for PlaneAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for PlaneAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for PlaneAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for PlaneAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for PlaneAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for PlaneAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for PlaneAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for PlaneAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for PlaneAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for Rotor {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Rotor {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Rotor {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Rotor {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Rotor {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Rotor {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Rotor {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Rotor {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Rotor {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Rotor {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Rotor {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Rotor {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Rotor {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Rotor {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Rotor {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Rotor {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Rotor {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Rotor {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Rotor {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Rotor {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Rotor {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Rotor {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Rotor {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Rotor {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Rotor {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Rotor {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for RoundPoint {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for RoundPoint {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for RoundPoint {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for RoundPoint {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for RoundPoint {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for RoundPoint {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for RoundPoint {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for RoundPoint {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for RoundPoint {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for RoundPoint {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for RoundPoint {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for RoundPoint {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for RoundPoint {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for RoundPoint {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for RoundPoint {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for RoundPoint {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for RoundPoint {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for RoundPoint {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for RoundPoint {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for RoundPoint {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for RoundPoint {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for RoundPoint {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for RoundPoint {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for RoundPoint {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for RoundPoint {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for RoundPointAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for RoundPointAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for RoundPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for RoundPointAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for RoundPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for RoundPointAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for RoundPointAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for RoundPointAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for RoundPointAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for RoundPointAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for RoundPointAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for RoundPointAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for RoundPointAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for RoundPointAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for RoundPointAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for RoundPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for RoundPointAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for RoundPointAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for RoundPointAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for RoundPointAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for RoundPointAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for RoundPointAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for RoundPointAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for Scalar {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Scalar {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Scalar {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiLineAtOrigin> for Scalar {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiPlane> for Scalar {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Scalar {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<AntiSphereOnOrigin> for Scalar {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Circle> for Scalar {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleAligningOrigin> for Scalar {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleAtInfinity> for Scalar {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleAtOrigin> for Scalar {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleOnOrigin> for Scalar {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Scalar {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Dipole> for Scalar {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleAligningOrigin> for Scalar {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleAtInfinity> for Scalar {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleAtOrigin> for Scalar {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleOnOrigin> for Scalar {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Scalar {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Scalar {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullDipoleAtOrigin> for Scalar {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullSphereAtOrigin> for Scalar {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
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

impl Sandwich<RoundPointAtOrigin> for Scalar {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Sphere> for Scalar {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<SphereAtOrigin> for Scalar {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<SphereOnOrigin> for Scalar {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for Sphere {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Sphere {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Sphere {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Sphere {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Sphere {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Sphere {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Sphere {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Sphere {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Sphere {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Sphere {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Sphere {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Sphere {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Sphere {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Sphere {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Sphere {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Sphere {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Sphere {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Sphere {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Sphere {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Sphere {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Sphere {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Sphere {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Sphere {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Sphere {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Sphere {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Sphere {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for SphereAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for SphereAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for SphereAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for SphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for SphereAtOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for SphereAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for SphereAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for SphereAtOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for SphereAtOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for SphereAtOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for SphereAtOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for SphereAtOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for SphereAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for SphereAtOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for SphereAtOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for SphereAtOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for SphereAtOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for SphereAtOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for SphereAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for SphereAtOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for SphereAtOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for SphereAtOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for SphereAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for SphereAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for SphereAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for SphereAtOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for SphereAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for SphereAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for SphereAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for SphereAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for SphereAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for SphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for SphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for SphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for SphereAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for SphereAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for SphereAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for SphereAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for SphereAtOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for SphereAtOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for SphereAtOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for SphereAtOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for SphereAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for SphereOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for SphereOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for SphereOnOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for SphereOnOrigin {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for SphereOnOrigin {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for SphereOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for SphereOnOrigin {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for SphereOnOrigin {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for SphereOnOrigin {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for SphereOnOrigin {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for SphereOnOrigin {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for SphereOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for SphereOnOrigin {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for SphereOnOrigin {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for SphereOnOrigin {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for SphereOnOrigin {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for SphereOnOrigin {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for SphereOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for SphereOnOrigin {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtInfinity> for SphereOnOrigin {
    type Output = FlatPointAtInfinity;

    fn sandwich(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPointAtOrigin> for SphereOnOrigin {
    type Output = FlatPointAtOrigin;

    fn sandwich(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for SphereOnOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for SphereOnOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for SphereOnOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Infinity> for SphereOnOrigin {
    type Output = Infinity;

    fn sandwich(self, other: Infinity) -> Infinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for SphereOnOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for SphereOnOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for SphereOnOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for SphereOnOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for SphereOnOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<NullCircleAtOrigin> for SphereOnOrigin {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for SphereOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for SphereOnOrigin {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for SphereOnOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for SphereOnOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for SphereOnOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for SphereOnOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for SphereOnOrigin {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPointAtOrigin> for SphereOnOrigin {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for SphereOnOrigin {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for SphereOnOrigin {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for SphereOnOrigin {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for SphereOnOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiCircleOnOrigin> for Transflector {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Transflector {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Transflector {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Transflector {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Transflector {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Transflector {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Transflector {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Transflector {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Transflector {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Transflector {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Transflector {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Transflector {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Transflector {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Transflector {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Transflector {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Transflector {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Transflector {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Transflector {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Transflector {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Transflector {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Transflector {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Transflector {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Transflector {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Transflector {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Transflector {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Transflector {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Sandwich<AntiCircleOnOrigin> for Translator {
    type Output = AntiCircleOnOrigin;

    fn sandwich(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiDipoleOnOrigin> for Translator {
    type Output = AntiDipoleOnOrigin;

    fn sandwich(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiFlatPointAtOrigin> for Translator {
    type Output = AntiFlatPointAtOrigin;

    fn sandwich(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiLineAtOrigin> for Translator {
    type Output = AntiLineAtOrigin;

    fn sandwich(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlane> for Translator {
    type Output = AntiPlane;

    fn sandwich(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiPlaneAtOrigin> for Translator {
    type Output = AntiPlaneAtOrigin;

    fn sandwich(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<AntiSphereOnOrigin> for Translator {
    type Output = AntiSphereOnOrigin;

    fn sandwich(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Translator {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAligningOrigin> for Translator {
    type Output = CircleAligningOrigin;

    fn sandwich(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtInfinity> for Translator {
    type Output = CircleAtInfinity;

    fn sandwich(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleAtOrigin> for Translator {
    type Output = CircleAtOrigin;

    fn sandwich(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOnOrigin> for Translator {
    type Output = CircleOnOrigin;

    fn sandwich(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<CircleOrthogonalOrigin> for Translator {
    type Output = CircleOrthogonalOrigin;

    fn sandwich(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Translator {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAligningOrigin> for Translator {
    type Output = DipoleAligningOrigin;

    fn sandwich(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtInfinity> for Translator {
    type Output = DipoleAtInfinity;

    fn sandwich(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleAtOrigin> for Translator {
    type Output = DipoleAtOrigin;

    fn sandwich(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOnOrigin> for Translator {
    type Output = DipoleOnOrigin;

    fn sandwich(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<DipoleOrthogonalOrigin> for Translator {
    type Output = DipoleOrthogonalOrigin;

    fn sandwich(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Sandwich<NullCircleAtOrigin> for Translator {
    type Output = NullCircleAtOrigin;

    fn sandwich(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullDipoleAtOrigin> for Translator {
    type Output = NullDipoleAtOrigin;

    fn sandwich(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<NullSphereAtOrigin> for Translator {
    type Output = NullSphereAtOrigin;

    fn sandwich(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
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

impl Sandwich<RoundPointAtOrigin> for Translator {
    type Output = RoundPointAtOrigin;

    fn sandwich(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Translator {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereAtOrigin> for Translator {
    type Output = SphereAtOrigin;

    fn sandwich(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<SphereOnOrigin> for Translator {
    type Output = SphereOnOrigin;

    fn sandwich(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl PointInversion<AntiCircleOnOrigin> for FlatPoint {
    type Output = AntiCircleOnOrigin;

    fn point_inversion(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<AntiDipoleOnOrigin> for FlatPoint {
    type Output = AntiDipoleOnOrigin;

    fn point_inversion(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<AntiFlatPointAtOrigin> for FlatPoint {
    type Output = AntiFlatPointAtOrigin;

    fn point_inversion(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<AntiLineAtOrigin> for FlatPoint {
    type Output = AntiLineAtOrigin;

    fn point_inversion(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<AntiPlane> for FlatPoint {
    type Output = AntiPlane;

    fn point_inversion(self, other: AntiPlane) -> AntiPlane {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<AntiPlaneAtOrigin> for FlatPoint {
    type Output = AntiPlaneAtOrigin;

    fn point_inversion(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<AntiSphereOnOrigin> for FlatPoint {
    type Output = AntiSphereOnOrigin;

    fn point_inversion(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Circle> for FlatPoint {
    type Output = Circle;

    fn point_inversion(self, other: Circle) -> Circle {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<CircleAligningOrigin> for FlatPoint {
    type Output = CircleAligningOrigin;

    fn point_inversion(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<CircleAtInfinity> for FlatPoint {
    type Output = CircleAtInfinity;

    fn point_inversion(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<CircleAtOrigin> for FlatPoint {
    type Output = CircleAtOrigin;

    fn point_inversion(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<CircleOnOrigin> for FlatPoint {
    type Output = CircleOnOrigin;

    fn point_inversion(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<CircleOrthogonalOrigin> for FlatPoint {
    type Output = CircleOrthogonalOrigin;

    fn point_inversion(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Dipole> for FlatPoint {
    type Output = Dipole;

    fn point_inversion(self, other: Dipole) -> Dipole {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<DipoleAligningOrigin> for FlatPoint {
    type Output = DipoleAligningOrigin;

    fn point_inversion(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<DipoleAtInfinity> for FlatPoint {
    type Output = DipoleAtInfinity;

    fn point_inversion(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<DipoleAtOrigin> for FlatPoint {
    type Output = DipoleAtOrigin;

    fn point_inversion(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<DipoleOnOrigin> for FlatPoint {
    type Output = DipoleOnOrigin;

    fn point_inversion(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<DipoleOrthogonalOrigin> for FlatPoint {
    type Output = DipoleOrthogonalOrigin;

    fn point_inversion(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl PointInversion<NullCircleAtOrigin> for FlatPoint {
    type Output = NullCircleAtOrigin;

    fn point_inversion(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<NullDipoleAtOrigin> for FlatPoint {
    type Output = NullDipoleAtOrigin;

    fn point_inversion(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<NullSphereAtOrigin> for FlatPoint {
    type Output = NullSphereAtOrigin;

    fn point_inversion(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
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

impl PointInversion<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPointAtOrigin;

    fn point_inversion(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Sphere> for FlatPoint {
    type Output = Sphere;

    fn point_inversion(self, other: Sphere) -> Sphere {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<SphereAtOrigin> for FlatPoint {
    type Output = SphereAtOrigin;

    fn point_inversion(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<SphereOnOrigin> for FlatPoint {
    type Output = SphereOnOrigin;

    fn point_inversion(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Reflect<AntiCircleOnOrigin> for Plane {
    type Output = AntiCircleOnOrigin;

    fn reflect(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<AntiDipoleOnOrigin> for Plane {
    type Output = AntiDipoleOnOrigin;

    fn reflect(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<AntiFlatPointAtOrigin> for Plane {
    type Output = AntiFlatPointAtOrigin;

    fn reflect(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<AntiLineAtOrigin> for Plane {
    type Output = AntiLineAtOrigin;

    fn reflect(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<AntiPlane> for Plane {
    type Output = AntiPlane;

    fn reflect(self, other: AntiPlane) -> AntiPlane {
        self.unitize().sandwich(other)
    }
}

impl Reflect<AntiPlaneAtOrigin> for Plane {
    type Output = AntiPlaneAtOrigin;

    fn reflect(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<AntiSphereOnOrigin> for Plane {
    type Output = AntiSphereOnOrigin;

    fn reflect(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Circle> for Plane {
    type Output = Circle;

    fn reflect(self, other: Circle) -> Circle {
        self.unitize().sandwich(other)
    }
}

impl Reflect<CircleAligningOrigin> for Plane {
    type Output = CircleAligningOrigin;

    fn reflect(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<CircleAtInfinity> for Plane {
    type Output = CircleAtInfinity;

    fn reflect(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<CircleAtOrigin> for Plane {
    type Output = CircleAtOrigin;

    fn reflect(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<CircleOnOrigin> for Plane {
    type Output = CircleOnOrigin;

    fn reflect(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<CircleOrthogonalOrigin> for Plane {
    type Output = CircleOrthogonalOrigin;

    fn reflect(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Dipole> for Plane {
    type Output = Dipole;

    fn reflect(self, other: Dipole) -> Dipole {
        self.unitize().sandwich(other)
    }
}

impl Reflect<DipoleAligningOrigin> for Plane {
    type Output = DipoleAligningOrigin;

    fn reflect(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<DipoleAtInfinity> for Plane {
    type Output = DipoleAtInfinity;

    fn reflect(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<DipoleAtOrigin> for Plane {
    type Output = DipoleAtOrigin;

    fn reflect(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<DipoleOnOrigin> for Plane {
    type Output = DipoleOnOrigin;

    fn reflect(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<DipoleOrthogonalOrigin> for Plane {
    type Output = DipoleOrthogonalOrigin;

    fn reflect(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Reflect<NullCircleAtOrigin> for Plane {
    type Output = NullCircleAtOrigin;

    fn reflect(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<NullDipoleAtOrigin> for Plane {
    type Output = NullDipoleAtOrigin;

    fn reflect(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<NullSphereAtOrigin> for Plane {
    type Output = NullSphereAtOrigin;

    fn reflect(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
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

impl Reflect<RoundPointAtOrigin> for Plane {
    type Output = RoundPointAtOrigin;

    fn reflect(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Sphere> for Plane {
    type Output = Sphere;

    fn reflect(self, other: Sphere) -> Sphere {
        self.unitize().sandwich(other)
    }
}

impl Reflect<SphereAtOrigin> for Plane {
    type Output = SphereAtOrigin;

    fn reflect(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<SphereOnOrigin> for Plane {
    type Output = SphereOnOrigin;

    fn reflect(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Rotate<AntiCircleOnOrigin> for Rotor {
    type Output = AntiCircleOnOrigin;

    fn rotate(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.sandwich(other)
    }
}

impl Rotate<AntiDipoleOnOrigin> for Rotor {
    type Output = AntiDipoleOnOrigin;

    fn rotate(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.sandwich(other)
    }
}

impl Rotate<AntiFlatPointAtOrigin> for Rotor {
    type Output = AntiFlatPointAtOrigin;

    fn rotate(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<AntiLineAtOrigin> for Rotor {
    type Output = AntiLineAtOrigin;

    fn rotate(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<AntiPlane> for Rotor {
    type Output = AntiPlane;

    fn rotate(self, other: AntiPlane) -> AntiPlane {
        self.sandwich(other)
    }
}

impl Rotate<AntiPlaneAtOrigin> for Rotor {
    type Output = AntiPlaneAtOrigin;

    fn rotate(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<AntiSphereOnOrigin> for Rotor {
    type Output = AntiSphereOnOrigin;

    fn rotate(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.sandwich(other)
    }
}

impl Rotate<Circle> for Rotor {
    type Output = Circle;

    fn rotate(self, other: Circle) -> Circle {
        self.sandwich(other)
    }
}

impl Rotate<CircleAligningOrigin> for Rotor {
    type Output = CircleAligningOrigin;

    fn rotate(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.sandwich(other)
    }
}

impl Rotate<CircleAtInfinity> for Rotor {
    type Output = CircleAtInfinity;

    fn rotate(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<CircleAtOrigin> for Rotor {
    type Output = CircleAtOrigin;

    fn rotate(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<CircleOnOrigin> for Rotor {
    type Output = CircleOnOrigin;

    fn rotate(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.sandwich(other)
    }
}

impl Rotate<CircleOrthogonalOrigin> for Rotor {
    type Output = CircleOrthogonalOrigin;

    fn rotate(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.sandwich(other)
    }
}

impl Rotate<Dipole> for Rotor {
    type Output = Dipole;

    fn rotate(self, other: Dipole) -> Dipole {
        self.sandwich(other)
    }
}

impl Rotate<DipoleAligningOrigin> for Rotor {
    type Output = DipoleAligningOrigin;

    fn rotate(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.sandwich(other)
    }
}

impl Rotate<DipoleAtInfinity> for Rotor {
    type Output = DipoleAtInfinity;

    fn rotate(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<DipoleAtOrigin> for Rotor {
    type Output = DipoleAtOrigin;

    fn rotate(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<DipoleOnOrigin> for Rotor {
    type Output = DipoleOnOrigin;

    fn rotate(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.sandwich(other)
    }
}

impl Rotate<DipoleOrthogonalOrigin> for Rotor {
    type Output = DipoleOrthogonalOrigin;

    fn rotate(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Rotate<NullCircleAtOrigin> for Rotor {
    type Output = NullCircleAtOrigin;

    fn rotate(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<NullDipoleAtOrigin> for Rotor {
    type Output = NullDipoleAtOrigin;

    fn rotate(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<NullSphereAtOrigin> for Rotor {
    type Output = NullSphereAtOrigin;

    fn rotate(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
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

impl Rotate<RoundPointAtOrigin> for Rotor {
    type Output = RoundPointAtOrigin;

    fn rotate(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<Sphere> for Rotor {
    type Output = Sphere;

    fn rotate(self, other: Sphere) -> Sphere {
        self.sandwich(other)
    }
}

impl Rotate<SphereAtOrigin> for Rotor {
    type Output = SphereAtOrigin;

    fn rotate(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<SphereOnOrigin> for Rotor {
    type Output = SphereOnOrigin;

    fn rotate(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Transflect<AntiCircleOnOrigin> for Transflector {
    type Output = AntiCircleOnOrigin;

    fn transflect(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.sandwich(other)
    }
}

impl Transflect<AntiDipoleOnOrigin> for Transflector {
    type Output = AntiDipoleOnOrigin;

    fn transflect(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.sandwich(other)
    }
}

impl Transflect<AntiFlatPointAtOrigin> for Transflector {
    type Output = AntiFlatPointAtOrigin;

    fn transflect(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<AntiLineAtOrigin> for Transflector {
    type Output = AntiLineAtOrigin;

    fn transflect(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<AntiPlane> for Transflector {
    type Output = AntiPlane;

    fn transflect(self, other: AntiPlane) -> AntiPlane {
        self.sandwich(other)
    }
}

impl Transflect<AntiPlaneAtOrigin> for Transflector {
    type Output = AntiPlaneAtOrigin;

    fn transflect(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<AntiSphereOnOrigin> for Transflector {
    type Output = AntiSphereOnOrigin;

    fn transflect(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.sandwich(other)
    }
}

impl Transflect<Circle> for Transflector {
    type Output = Circle;

    fn transflect(self, other: Circle) -> Circle {
        self.sandwich(other)
    }
}

impl Transflect<CircleAligningOrigin> for Transflector {
    type Output = CircleAligningOrigin;

    fn transflect(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.sandwich(other)
    }
}

impl Transflect<CircleAtInfinity> for Transflector {
    type Output = CircleAtInfinity;

    fn transflect(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<CircleAtOrigin> for Transflector {
    type Output = CircleAtOrigin;

    fn transflect(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<CircleOnOrigin> for Transflector {
    type Output = CircleOnOrigin;

    fn transflect(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.sandwich(other)
    }
}

impl Transflect<CircleOrthogonalOrigin> for Transflector {
    type Output = CircleOrthogonalOrigin;

    fn transflect(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.sandwich(other)
    }
}

impl Transflect<Dipole> for Transflector {
    type Output = Dipole;

    fn transflect(self, other: Dipole) -> Dipole {
        self.sandwich(other)
    }
}

impl Transflect<DipoleAligningOrigin> for Transflector {
    type Output = DipoleAligningOrigin;

    fn transflect(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.sandwich(other)
    }
}

impl Transflect<DipoleAtInfinity> for Transflector {
    type Output = DipoleAtInfinity;

    fn transflect(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<DipoleAtOrigin> for Transflector {
    type Output = DipoleAtOrigin;

    fn transflect(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<DipoleOnOrigin> for Transflector {
    type Output = DipoleOnOrigin;

    fn transflect(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.sandwich(other)
    }
}

impl Transflect<DipoleOrthogonalOrigin> for Transflector {
    type Output = DipoleOrthogonalOrigin;

    fn transflect(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Transflect<NullCircleAtOrigin> for Transflector {
    type Output = NullCircleAtOrigin;

    fn transflect(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<NullDipoleAtOrigin> for Transflector {
    type Output = NullDipoleAtOrigin;

    fn transflect(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<NullSphereAtOrigin> for Transflector {
    type Output = NullSphereAtOrigin;

    fn transflect(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
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

impl Transflect<RoundPointAtOrigin> for Transflector {
    type Output = RoundPointAtOrigin;

    fn transflect(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<Sphere> for Transflector {
    type Output = Sphere;

    fn transflect(self, other: Sphere) -> Sphere {
        self.sandwich(other)
    }
}

impl Transflect<SphereAtOrigin> for Transflector {
    type Output = SphereAtOrigin;

    fn transflect(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<SphereOnOrigin> for Transflector {
    type Output = SphereOnOrigin;

    fn transflect(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl Translate<AntiCircleOnOrigin> for Translator {
    type Output = AntiCircleOnOrigin;

    fn translate(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.sandwich(other)
    }
}

impl Translate<AntiDipoleOnOrigin> for Translator {
    type Output = AntiDipoleOnOrigin;

    fn translate(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.sandwich(other)
    }
}

impl Translate<AntiFlatPointAtOrigin> for Translator {
    type Output = AntiFlatPointAtOrigin;

    fn translate(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<AntiLineAtOrigin> for Translator {
    type Output = AntiLineAtOrigin;

    fn translate(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<AntiPlane> for Translator {
    type Output = AntiPlane;

    fn translate(self, other: AntiPlane) -> AntiPlane {
        self.sandwich(other)
    }
}

impl Translate<AntiPlaneAtOrigin> for Translator {
    type Output = AntiPlaneAtOrigin;

    fn translate(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<AntiSphereOnOrigin> for Translator {
    type Output = AntiSphereOnOrigin;

    fn translate(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.sandwich(other)
    }
}

impl Translate<Circle> for Translator {
    type Output = Circle;

    fn translate(self, other: Circle) -> Circle {
        self.sandwich(other)
    }
}

impl Translate<CircleAligningOrigin> for Translator {
    type Output = CircleAligningOrigin;

    fn translate(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.sandwich(other)
    }
}

impl Translate<CircleAtInfinity> for Translator {
    type Output = CircleAtInfinity;

    fn translate(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<CircleAtOrigin> for Translator {
    type Output = CircleAtOrigin;

    fn translate(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<CircleOnOrigin> for Translator {
    type Output = CircleOnOrigin;

    fn translate(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.sandwich(other)
    }
}

impl Translate<CircleOrthogonalOrigin> for Translator {
    type Output = CircleOrthogonalOrigin;

    fn translate(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.sandwich(other)
    }
}

impl Translate<Dipole> for Translator {
    type Output = Dipole;

    fn translate(self, other: Dipole) -> Dipole {
        self.sandwich(other)
    }
}

impl Translate<DipoleAligningOrigin> for Translator {
    type Output = DipoleAligningOrigin;

    fn translate(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.sandwich(other)
    }
}

impl Translate<DipoleAtInfinity> for Translator {
    type Output = DipoleAtInfinity;

    fn translate(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<DipoleAtOrigin> for Translator {
    type Output = DipoleAtOrigin;

    fn translate(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<DipoleOnOrigin> for Translator {
    type Output = DipoleOnOrigin;

    fn translate(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.sandwich(other)
    }
}

impl Translate<DipoleOrthogonalOrigin> for Translator {
    type Output = DipoleOrthogonalOrigin;

    fn translate(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl Translate<NullCircleAtOrigin> for Translator {
    type Output = NullCircleAtOrigin;

    fn translate(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<NullDipoleAtOrigin> for Translator {
    type Output = NullDipoleAtOrigin;

    fn translate(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<NullSphereAtOrigin> for Translator {
    type Output = NullSphereAtOrigin;

    fn translate(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
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

impl Translate<RoundPointAtOrigin> for Translator {
    type Output = RoundPointAtOrigin;

    fn translate(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<Sphere> for Translator {
    type Output = Sphere;

    fn translate(self, other: Sphere) -> Sphere {
        self.sandwich(other)
    }
}

impl Translate<SphereAtOrigin> for Translator {
    type Output = SphereAtOrigin;

    fn translate(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<SphereOnOrigin> for Translator {
    type Output = SphereOnOrigin;

    fn translate(self, other: SphereOnOrigin) -> SphereOnOrigin {
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
