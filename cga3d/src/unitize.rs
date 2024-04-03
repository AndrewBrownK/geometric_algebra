//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::norms::WeightNorm;
use crate::products::geometric::GeometricProduct;
use crate::*;

/// Unitization
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Unitization
pub trait Unitize {
    type Output;
    fn unitize(self) -> Self::Output;
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

impl Unitize for CircleBulk {
    type Output = CircleBulk;

    fn unitize(self) -> CircleBulk {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn unitize(self) -> CircleCarrierAspect {
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

impl Unitize for DipoleBulk {
    type Output = DipoleBulk;

    fn unitize(self) -> DipoleBulk {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn unitize(self) -> DipoleCarrierAspect {
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

impl Unitize for Magnitude {
    type Output = Magnitude;

    fn unitize(self) -> Magnitude {
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

impl Unitize for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn unitize(self) -> RoundPointAtInfinity {
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

impl Unitize for RoundPointBulk {
    type Output = RoundPointBulk;

    fn unitize(self) -> RoundPointBulk {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn unitize(self) -> RoundPointCarrierAspect {
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
