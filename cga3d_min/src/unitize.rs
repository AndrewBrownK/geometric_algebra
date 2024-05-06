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

impl Unitize for AntiScalar {
    type Output = AntiScalar;

    fn unitize(self) -> AntiScalar {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Circle {
    type Output = Circle;

    fn unitize(self) -> Circle {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Dipole {
    type Output = Dipole;

    fn unitize(self) -> Dipole {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for DualNum {
    type Output = DualNum;

    fn unitize(self) -> DualNum {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for FlatPoint {
    type Output = FlatPoint;

    fn unitize(self) -> FlatPoint {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Flector {
    type Output = Flector;

    fn unitize(self) -> Flector {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Line {
    type Output = Line;

    fn unitize(self) -> Line {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Motor {
    type Output = Motor;

    fn unitize(self) -> Motor {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for MultiVector {
    type Output = MultiVector;

    fn unitize(self) -> MultiVector {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Plane {
    type Output = Plane;

    fn unitize(self) -> Plane {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Rotor {
    type Output = Rotor;

    fn unitize(self) -> Rotor {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for RoundPoint {
    type Output = RoundPoint;

    fn unitize(self) -> RoundPoint {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Scalar {
    type Output = Scalar;

    fn unitize(self) -> Scalar {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Sphere {
    type Output = Sphere;

    fn unitize(self) -> Sphere {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Transflector {
    type Output = Transflector;

    fn unitize(self) -> Transflector {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Translator {
    type Output = Translator;

    fn unitize(self) -> Translator {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.weight_norm().group0(),
            },
        })
    }
}
