//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::characteristics::Attitude;
use crate::norms::*;
use crate::products::contractions::WeightContraction;
use crate::products::exterior::Wedge;
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

impl CosineAngle<Line> for Line {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<LineAtOrigin> for Line {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Origin> for Line {
    type Output = f32;

    fn cosine_angle(self, other: Origin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Point> for Line {
    type Output = f32;

    fn cosine_angle(self, other: Point) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<LineAtOrigin> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Origin> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Origin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Point> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Point) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Origin> for Origin {
    type Output = f32;

    fn cosine_angle(self, other: Origin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Point> for Origin {
    type Output = f32;

    fn cosine_angle(self, other: Point) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<LineAtOrigin> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Origin> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: Origin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Plane> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: Plane) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<PlaneAtOrigin> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: PlaneAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Point> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: Point) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<LineAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Origin> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Origin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Plane> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Plane) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: PlaneAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Point> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Point) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Origin> for Point {
    type Output = f32;

    fn cosine_angle(self, other: Origin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Point> for Point {
    type Output = f32;

    fn cosine_angle(self, other: Point) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl Distance<MultiVector> for AntiScalar {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Flector {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for Flector {
    type Output = Magnitude;

    fn distance(self, other: Line) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<LineAtOrigin> for Flector {
    type Output = Magnitude;

    fn distance(self, other: LineAtOrigin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Magnitude> for Flector {
    type Output = Magnitude;

    fn distance(self, other: Magnitude) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for Flector {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Flector {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Flector {
    type Output = Magnitude;

    fn distance(self, other: Origin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Plane> for Flector {
    type Output = Magnitude;

    fn distance(self, other: Plane) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<PlaneAtOrigin> for Flector {
    type Output = Magnitude;

    fn distance(self, other: PlaneAtOrigin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Flector {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for Flector {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Translator> for Flector {
    type Output = Magnitude;

    fn distance(self, other: Translator) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Horizon {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Horizon {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Line {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for Line {
    type Output = Magnitude;

    fn distance(self, other: Line) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<LineAtOrigin> for Line {
    type Output = Magnitude;

    fn distance(self, other: LineAtOrigin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for Line {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Line {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Line {
    type Output = Magnitude;

    fn distance(self, other: Origin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Line {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for Line {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for LineAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for LineAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for LineAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for LineAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for LineAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for LineAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: Line) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for LineAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for LineAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for LineAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: Origin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Motor {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for Motor {
    type Output = Magnitude;

    fn distance(self, other: Line) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<LineAtOrigin> for Motor {
    type Output = Magnitude;

    fn distance(self, other: LineAtOrigin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for Motor {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Motor {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Motor {
    type Output = Magnitude;

    fn distance(self, other: Origin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Motor {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for Motor {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<AntiScalar> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: AntiScalar) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Line) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<LineAtOrigin> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: LineAtOrigin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Magnitude> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Magnitude) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Origin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Plane> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Plane) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<PlaneAtOrigin> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: PlaneAtOrigin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Translator> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Translator) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Origin {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for Origin {
    type Output = Magnitude;

    fn distance(self, other: Line) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Magnitude> for Origin {
    type Output = Magnitude;

    fn distance(self, other: Magnitude) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for Origin {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Origin {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Plane> for Origin {
    type Output = Magnitude;

    fn distance(self, other: Plane) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Origin {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Translator> for Origin {
    type Output = Magnitude;

    fn distance(self, other: Translator) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Plane {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Plane {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Plane {
    type Output = Magnitude;

    fn distance(self, other: Origin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Plane {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for PlaneAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for PlaneAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for PlaneAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Point {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for Point {
    type Output = Magnitude;

    fn distance(self, other: Line) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<LineAtOrigin> for Point {
    type Output = Magnitude;

    fn distance(self, other: LineAtOrigin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Magnitude> for Point {
    type Output = Magnitude;

    fn distance(self, other: Magnitude) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for Point {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Point {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Point {
    type Output = Magnitude;

    fn distance(self, other: Origin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Plane> for Point {
    type Output = Magnitude;

    fn distance(self, other: Plane) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<PlaneAtOrigin> for Point {
    type Output = Magnitude;

    fn distance(self, other: PlaneAtOrigin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Point {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for Point {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Translator> for Point {
    type Output = Magnitude;

    fn distance(self, other: Translator) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for PointAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for PointAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for PointAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for PointAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Rotor {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for Rotor {
    type Output = Magnitude;

    fn distance(self, other: Line) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for Rotor {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Rotor {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Rotor {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Translator {
    type Output = Magnitude;

    fn distance(self, other: Flector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for Translator {
    type Output = Magnitude;

    fn distance(self, other: Motor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Translator {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Translator {
    type Output = Magnitude;

    fn distance(self, other: Origin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Translator {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for Translator {
    type Output = Magnitude;

    fn distance(self, other: Rotor) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl SineAngle<Line> for Line {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Line {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Origin> for Line {
    type Output = f32;

    fn sine_angle(self, other: Origin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Point> for Line {
    type Output = f32;

    fn sine_angle(self, other: Point) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Origin> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Origin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Point> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Point) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Origin> for Origin {
    type Output = f32;

    fn sine_angle(self, other: Origin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Point> for Origin {
    type Output = f32;

    fn sine_angle(self, other: Point) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for Plane {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Plane {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Origin> for Plane {
    type Output = f32;

    fn sine_angle(self, other: Origin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Plane> for Plane {
    type Output = f32;

    fn sine_angle(self, other: Plane) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for Plane {
    type Output = f32;

    fn sine_angle(self, other: PlaneAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Point> for Plane {
    type Output = f32;

    fn sine_angle(self, other: Point) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Origin> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Origin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Plane> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Plane) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: PlaneAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Point> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Point) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Origin> for Point {
    type Output = f32;

    fn sine_angle(self, other: Origin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Point> for Point {
    type Output = f32;

    fn sine_angle(self, other: Point) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}
