//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspects::*;
use crate::characteristics::*;
use crate::products::dot::*;
use crate::*;

/// BulkNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait BulkNorm {
    type Output;
    fn bulk_norm(self) -> Self::Output;
}

/// BulkNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait BulkNormSquared {
    type Output;
    fn bulk_norm_squared(self) -> Self::Output;
}

/// CenterBulkNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait CenterBulkNorm {
    type Output;
    fn center_bulk_norm(self) -> Self::Output;
}

/// CenterBulkNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait CenterBulkNormSquared {
    type Output;
    fn center_bulk_norm_squared(self) -> Self::Output;
}

/// GeometricNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait GeometricNorm {
    type Output;
    fn geometric_norm(self) -> Self::Output;
}

/// UnitizedNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedNorm {
    type Output;
    fn unitized_norm(self) -> Self::Output;
}

/// UnitizedNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedNormSquared {
    type Output;
    fn unitized_norm_squared(self) -> Self::Output;
}

/// WeightNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait WeightNorm {
    type Output;
    fn weight_norm(self) -> Self::Output;
}

/// WeightNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait WeightNormSquared {
    type Output;
    fn weight_norm_squared(self) -> Self::Output;
}

impl BulkNormSquared for AntiCircleOnOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for AntiLineAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for AntiPlane {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for AntiPlaneAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for AntiScalar {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for AntiSphereOnOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Circle {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for CircleAligningOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for CircleAtInfinity {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for CircleAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for CircleOnOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Dipole {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for DipoleAligningOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for DipoleAtInfinity {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for DipoleAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for DipoleOnOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for DualNum {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for FlatPoint {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for FlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Flector {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Line {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for LineAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Motor {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for MultiVector {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Plane {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Rotor {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for RoundPoint {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Scalar {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Sphere {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for SphereAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for SphereOnOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Transflector {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Translator {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNorm for AntiCircleOnOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for AntiLineAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for AntiPlane {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for AntiPlaneAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for AntiScalar {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for AntiSphereOnOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Circle {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for CircleAligningOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for CircleAtInfinity {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for CircleAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for CircleOnOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Dipole {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for DipoleAligningOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for DipoleAtInfinity {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for DipoleAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for DipoleOnOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for DualNum {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for FlatPoint {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for FlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Flector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Line {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for LineAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Motor {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for MultiVector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Plane {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Rotor {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for RoundPoint {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Scalar {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Sphere {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for SphereAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for SphereOnOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Transflector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Translator {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl WeightNormSquared for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for AntiLineAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for AntiPlane {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for AntiPlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for AntiScalar {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for AntiSphereOnOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Circle {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for CircleAligningOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for CircleAtInfinity {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for CircleAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for CircleOnOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Dipole {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for DipoleAtInfinity {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for DipoleAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for DipoleOnOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for DualNum {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for FlatPoint {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Flector {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Line {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Motor {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for MultiVector {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Plane {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Rotor {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for RoundPoint {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Scalar {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Sphere {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for SphereAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for SphereOnOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Transflector {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Translator {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNorm for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for AntiLineAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for AntiPlane {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for AntiPlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for AntiScalar {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for AntiSphereOnOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Circle {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for CircleAligningOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for CircleAtInfinity {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for CircleAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for CircleOnOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Dipole {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for DipoleAtInfinity {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for DipoleAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for DipoleOnOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for DualNum {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for FlatPoint {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Flector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Line {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Motor {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for MultiVector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Plane {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Rotor {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for RoundPoint {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Scalar {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Sphere {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for SphereAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for SphereOnOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Transflector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl WeightNorm for Translator {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).anti_sqrt()
    }
}

impl GeometricNorm for AntiCircleOnOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for AntiDipoleOnOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for AntiFlatPointAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for AntiLineAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for AntiPlane {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for AntiPlaneAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for AntiScalar {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for AntiSphereOnOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Circle {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for CircleAligningOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for CircleAtInfinity {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for CircleAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for CircleOnOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for CircleOrthogonalOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Dipole {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for DipoleAligningOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for DipoleAtInfinity {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for DipoleAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for DipoleOnOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for DipoleOrthogonalOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for DualNum {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for FlatPoint {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for FlatPointAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Flector {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Line {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for LineAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Motor {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for MultiVector {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Plane {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for PlaneAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Rotor {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for RoundPoint {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for RoundPointAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Scalar {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Sphere {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for SphereAtOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for SphereOnOrigin {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Transflector {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Translator {
    type Output = DualNum;

    fn geometric_norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl UnitizedNormSquared for AntiCircleOnOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for AntiDipoleOnOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for AntiFlatPointAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for AntiLineAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for AntiPlane {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for AntiPlaneAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for AntiScalar {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for AntiSphereOnOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Circle {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for CircleAligningOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for CircleAtInfinity {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for CircleAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for CircleOnOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for CircleOrthogonalOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Dipole {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for DipoleAligningOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for DipoleAtInfinity {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for DipoleAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for DipoleOnOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for DipoleOrthogonalOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for DualNum {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for FlatPoint {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for FlatPointAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Flector {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Line {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for LineAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Motor {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for MultiVector {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Plane {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for PlaneAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Rotor {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for RoundPoint {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for RoundPointAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Scalar {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Sphere {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for SphereAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for SphereOnOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Transflector {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Translator {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNorm for AntiCircleOnOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for AntiDipoleOnOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for AntiFlatPointAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for AntiLineAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for AntiPlane {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for AntiPlaneAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for AntiScalar {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for AntiSphereOnOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Circle {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for CircleAligningOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for CircleAtInfinity {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for CircleAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for CircleOnOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for CircleOrthogonalOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Dipole {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for DipoleAligningOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for DipoleAtInfinity {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for DipoleAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for DipoleOnOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for DipoleOrthogonalOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for DualNum {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for FlatPoint {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for FlatPointAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Flector {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Line {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for LineAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Motor {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for MultiVector {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Plane {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for PlaneAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Rotor {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for RoundPoint {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for RoundPointAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Scalar {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Sphere {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for SphereAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for SphereOnOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Transflector {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Translator {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl CenterBulkNormSquared for Circle {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for CircleAligningOrigin {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for CircleOnOrigin {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for Dipole {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for DipoleAligningOrigin {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for DipoleOnOrigin {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for RoundPoint {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for Sphere {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for SphereOnOrigin {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: AntiPlaneAtOrigin = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNorm for Circle {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for CircleAligningOrigin {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for CircleOnOrigin {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for Dipole {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for DipoleAligningOrigin {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for DipoleOnOrigin {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for RoundPoint {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for Sphere {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for SphereOnOrigin {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}
