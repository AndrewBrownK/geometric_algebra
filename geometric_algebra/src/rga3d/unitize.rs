#![allow(clippy::assign_op_pattern)]
use crate::rga3d::norms::WeightNorm;
use crate::rga3d::products::geometric::GeometricProduct;
use crate::rga3d::*;

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

impl Unitize for Origin {
    type Output = Origin;

    fn unitize(self) -> Origin {
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

impl Unitize for Point {
    type Output = Point;

    fn unitize(self) -> Point {
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
