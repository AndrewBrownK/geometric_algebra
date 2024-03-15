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

impl Distance<MultiVector> for Circle {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Radial> for Circle {
    type Output = Magnitude;

    fn distance(self, other: Radial) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for Dipole {
    type Output = Magnitude;

    fn distance(self, other: Dipole) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Dipole {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Radial> for Dipole {
    type Output = Magnitude;

    fn distance(self, other: Radial) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Horizon {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Line {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for LineAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for LineAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: Circle) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: Dipole) -> Magnitude {
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

impl Distance<Radial> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: Radial) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Sphere> for Magnitude {
    type Output = Magnitude;

    fn distance(self, other: Sphere) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Circle) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Dipole) -> Magnitude {
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

impl Distance<Radial> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Radial) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Sphere> for MultiVector {
    type Output = Magnitude;

    fn distance(self, other: Sphere) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Origin {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Plane {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for PlaneAtOrigin {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Point {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for PointAtInfinity {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for Radial {
    type Output = Magnitude;

    fn distance(self, other: Circle) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for Radial {
    type Output = Magnitude;

    fn distance(self, other: Dipole) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Magnitude> for Radial {
    type Output = Magnitude;

    fn distance(self, other: Magnitude) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Radial {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Radial> for Radial {
    type Output = Magnitude;

    fn distance(self, other: Radial) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: Circle) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: Dipole) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: Origin) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: Point) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Radial> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: Radial) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Sphere> for Scalar {
    type Output = Magnitude;

    fn distance(self, other: Sphere) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Sphere {
    type Output = Magnitude;

    fn distance(self, other: MultiVector) -> Magnitude {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}
