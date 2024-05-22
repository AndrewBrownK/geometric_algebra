//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::norms::RoundWeightNorm;
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
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn unitize(self) -> AntiDipoleOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn unitize(self) -> AntiSphereOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Circle {
    type Output = Circle;

    fn unitize(self) -> Circle {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn unitize(self) -> CircleAligningOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn unitize(self) -> CircleAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn unitize(self) -> CircleOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn unitize(self) -> CircleOrthogonalOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Dilator {
    type Output = Dilator;

    fn unitize(self) -> Dilator {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Dipole {
    type Output = Dipole;

    fn unitize(self) -> Dipole {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn unitize(self) -> DipoleAligningOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn unitize(self) -> DipoleAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn unitize(self) -> DipoleOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn unitize(self) -> DipoleOrthogonalOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for MultiVector {
    type Output = MultiVector;

    fn unitize(self) -> MultiVector {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn unitize(self) -> NullCircleAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn unitize(self) -> NullDipoleAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn unitize(self) -> NullSphereAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Origin {
    type Output = Origin;

    fn unitize(self) -> Origin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for RoundPoint {
    type Output = RoundPoint;

    fn unitize(self) -> RoundPoint {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn unitize(self) -> RoundPointAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Sphere {
    type Output = Sphere;

    fn unitize(self) -> Sphere {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn unitize(self) -> SphereAtOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn unitize(self) -> SphereOnOrigin {
        self.geometric_product(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}
