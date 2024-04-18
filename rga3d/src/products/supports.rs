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

impl AntiSupport for DualNum {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Flector {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_support(self) -> MultiVectorAtInfinity {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Horizon {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Line {
    type Output = Plane;

    fn anti_support(self) -> Plane {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for LineAtInfinity {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Motor {
    type Output = Plane;

    fn anti_support(self) -> Plane {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for MultiVector {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_support(self) -> MultiVectorAtInfinity {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Plane {
    type Output = Plane;

    fn anti_support(self) -> Plane {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Point {
    type Output = Plane;

    fn anti_support(self) -> Plane {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for PointAtInfinity {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Scalar {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Transflector {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl AntiSupport for Translator {
    type Output = Horizon;

    fn anti_support(self) -> Horizon {
        self.wedge(Origin::one().right_complement().anti_wedge(self.dual()))
    }
}

impl Support for Flector {
    type Output = Flector;

    fn support(self) -> Flector {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for Line {
    type Output = Point;

    fn support(self) -> Point {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for LineAtOrigin {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for Motor {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for MultiVector {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn support(self) -> MultiVectorAtOrigin {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for Origin {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for Plane {
    type Output = Point;

    fn support(self) -> Point {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for PlaneAtOrigin {
    type Output = Origin;

    fn support(self) -> Origin {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for Point {
    type Output = Point;

    fn support(self) -> Point {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for Rotor {
    type Output = MultiVectorAtOrigin;

    fn support(self) -> MultiVectorAtOrigin {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}

impl Support for Transflector {
    type Output = Point;

    fn support(self) -> Point {
        self.anti_wedge(Origin::one().wedge(self.anti_dual()))
    }
}
