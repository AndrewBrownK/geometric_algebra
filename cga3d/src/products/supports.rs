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

impl AntiSupport for Circle {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for CircleBulk {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for CircleCarrierAspect {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for CircleWeight {
    type Output = SphereWeight;

    fn anti_support(self) -> SphereWeight {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Dipole {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for DipoleBulk {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for DipoleCarrierAspect {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for DipoleWeight {
    type Output = SphereWeight;

    fn anti_support(self) -> SphereWeight {
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

impl AntiSupport for Line {
    type Output = Plane;

    fn anti_support(self) -> Plane {
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

impl AntiSupport for Origin {
    type Output = SphereWeight;

    fn anti_support(self) -> SphereWeight {
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

impl AntiSupport for RoundPointAtInfinity {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for RoundPointAtOrigin {
    type Output = SpacialCurvature;

    fn anti_support(self) -> SpacialCurvature {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for RoundPointBulk {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for RoundPointOnOrigin {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Scalar {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for SpacialCurvature {
    type Output = SpacialCurvature;

    fn anti_support(self) -> SpacialCurvature {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Sphere {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(Origin::unit().complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for SphereWeight {
    type Output = SphereWeight;

    fn anti_support(self) -> SphereWeight {
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

impl Support for CircleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn support(self) -> RoundPointOnOrigin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for Dipole {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for DipoleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn support(self) -> RoundPointOnOrigin {
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

impl Support for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn support(self) -> RoundPointAtInfinity {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn support(self) -> RoundPointAtOrigin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn support(self) -> RoundPointOnOrigin {
        self.anti_wedge(Origin::unit().wedge(self.anti_dual()))
    }
}

impl Support for SpacialCurvature {
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
