// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::products::exterior::*;
use crate::*;

/// Support
/// The support is the point enclosed by the object closest to the origin.
pub trait Support {
    type Output;
    fn support(self) -> Self::Output;
}

/// AntiSupport
/// The anti-support is the anti-vector furthest from the origin that encloses the object.
pub trait AntiSupport {
    type Output;
    fn anti_support(self) -> Self::Output;
}

impl AntiSupport for AntiCircleOnOrigin {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for AntiDipoleOnOrigin {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for AntiFlatPointAtOrigin {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for AntiLineAtOrigin {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for AntiPlane {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for AntiPlaneAtOrigin {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for AntiSphereOnOrigin {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Circle {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for CircleAligningOrigin {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for CircleAtInfinity {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for CircleAtOrigin {
    type Output = SphereAtOrigin;

    fn anti_support(self) -> SphereAtOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for CircleOnOrigin {
    type Output = SphereOnOrigin;

    fn anti_support(self) -> SphereOnOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for CircleOrthogonalOrigin {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Dilator {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Dipole {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for DipoleAligningOrigin {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for DipoleAtInfinity {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for DipoleAtOrigin {
    type Output = SphereAtOrigin;

    fn anti_support(self) -> SphereAtOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for DipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn anti_support(self) -> SphereOnOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for DualNum {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for FlatPoint {
    type Output = Plane;

    fn anti_support(self) -> Plane {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Flector {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Motor {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for MultiVector {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for NullCircleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn anti_support(self) -> NullSphereAtOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for NullDipoleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn anti_support(self) -> NullSphereAtOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn anti_support(self) -> NullSphereAtOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Origin {
    type Output = NullSphereAtOrigin;

    fn anti_support(self) -> NullSphereAtOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Rotor {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for RoundPoint {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn anti_support(self) -> SphereAtOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Scalar {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Sphere {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn anti_support(self) -> SphereAtOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn anti_support(self) -> SphereOnOrigin {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Transflector {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Translator {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl Support for AntiPlane {
    type Output = AntiPlane;

    fn support(self) -> AntiPlane {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for AntiScalar {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Circle {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for CircleAligningOrigin {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for CircleAtInfinity {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for CircleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn support(self) -> RoundPointAtOrigin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for CircleOnOrigin {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for CircleOrthogonalOrigin {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Dilator {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Dipole {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for DipoleAligningOrigin {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for DipoleAtInfinity {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for DipoleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn support(self) -> RoundPointAtOrigin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for DipoleOnOrigin {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for DipoleOrthogonalOrigin {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for DualNum {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for FlatPoint {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for FlatPointAtInfinity {
    type Output = Infinity;

    fn support(self) -> Infinity {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for FlatPointAtOrigin {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Flector {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for FlectorAtInfinity {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Horizon {
    type Output = Infinity;

    fn support(self) -> Infinity {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Infinity {
    type Output = Infinity;

    fn support(self) -> Infinity {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Line {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for LineAtInfinity {
    type Output = Infinity;

    fn support(self) -> Infinity {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for LineAtOrigin {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Motor {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for MultiVector {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Plane {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for PlaneAtOrigin {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Rotor {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for RoundPoint {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn support(self) -> RoundPointAtOrigin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Sphere {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn support(self) -> RoundPointAtOrigin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for SphereOnOrigin {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Transflector {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Translator {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}
