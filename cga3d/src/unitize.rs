//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::norms::WeightNorm;
use crate::products::geometric::*;
use crate::*;

/// Unitization
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Unitization
pub trait Unitize {
    type Output;
    fn unitize(self) -> Self::Output;
}

impl Unitize for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn unitize(self) -> AntiCircleOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn unitize(self) -> AntiDipoleOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn unitize(self) -> AntiFlatPointAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn unitize(self) -> AntiLineAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for AntiPlane {
    type Output = AntiPlane;

    fn unitize(self) -> AntiPlane {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn unitize(self) -> AntiPlaneAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for AntiScalar {
    type Output = AntiScalar;

    fn unitize(self) -> AntiScalar {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn unitize(self) -> AntiSphereOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Circle {
    type Output = Circle;

    fn unitize(self) -> Circle {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn unitize(self) -> CircleAligningOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn unitize(self) -> CircleAtInfinity {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn unitize(self) -> CircleAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn unitize(self) -> CircleOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn unitize(self) -> CircleOrthogonalOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Dipole {
    type Output = Dipole;

    fn unitize(self) -> Dipole {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn unitize(self) -> DipoleAligningOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn unitize(self) -> DipoleAtInfinity {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn unitize(self) -> DipoleAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn unitize(self) -> DipoleOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn unitize(self) -> DipoleOrthogonalOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DualNum {
    type Output = DualNum;

    fn unitize(self) -> DualNum {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for FlatPoint {
    type Output = FlatPoint;

    fn unitize(self) -> FlatPoint {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn unitize(self) -> FlatPointAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Flector {
    type Output = Flector;

    fn unitize(self) -> Flector {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Line {
    type Output = Line;

    fn unitize(self) -> Line {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for LineAtOrigin {
    type Output = LineAtOrigin;

    fn unitize(self) -> LineAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Motor {
    type Output = Motor;

    fn unitize(self) -> Motor {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for MultiVector {
    type Output = MultiVector;

    fn unitize(self) -> MultiVector {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Plane {
    type Output = Plane;

    fn unitize(self) -> Plane {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn unitize(self) -> PlaneAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Rotor {
    type Output = Rotor;

    fn unitize(self) -> Rotor {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for RoundPoint {
    type Output = RoundPoint;

    fn unitize(self) -> RoundPoint {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn unitize(self) -> RoundPointAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Scalar {
    type Output = Scalar;

    fn unitize(self) -> Scalar {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Sphere {
    type Output = Sphere;

    fn unitize(self) -> Sphere {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn unitize(self) -> SphereAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn unitize(self) -> SphereOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Transflector {
    type Output = Transflector;

    fn unitize(self) -> Transflector {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Translator {
    type Output = Translator;

    fn unitize(self) -> Translator {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}
