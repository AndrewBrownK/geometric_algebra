//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::aspect_duals::*;
use crate::products::exterior::AntiWedge;
use crate::*;

/// Bulk Contraction (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait BulkContraction<T> {
    type Output;
    fn bulk_contraction(self, other: T) -> Self::Output;
}

/// Weight Contraction (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait WeightContraction<T> {
    type Output;
    fn weight_contraction(self, other: T) -> Self::Output;
}

impl BulkContraction<MultiVector> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: Radial) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for Dipole {
    type Output = Radial;

    fn bulk_contraction(self, other: Radial) -> Radial {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for Horizon {
    type Output = Circle;

    fn bulk_contraction(self, other: Radial) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for Line {
    type Output = Dipole;

    fn bulk_contraction(self, other: Radial) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for LineAtInfinity {
    type Output = Dipole;

    fn bulk_contraction(self, other: Radial) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for LineAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: Radial) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Radial) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for Origin {
    type Output = Radial;

    fn bulk_contraction(self, other: Radial) -> Radial {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for Plane {
    type Output = Circle;

    fn bulk_contraction(self, other: Radial) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for PlaneAtOrigin {
    type Output = Circle;

    fn bulk_contraction(self, other: Radial) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Point {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for Point {
    type Output = Radial;

    fn bulk_contraction(self, other: Radial) -> Radial {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for PointAtInfinity {
    type Output = Radial;

    fn bulk_contraction(self, other: Radial) -> Radial {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Radial {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for Radial {
    type Output = Scalar;

    fn bulk_contraction(self, other: Radial) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Radial> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: Radial) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl WeightContraction<MultiVector> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Point {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Radial {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}
