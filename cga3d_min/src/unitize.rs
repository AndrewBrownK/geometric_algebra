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

impl Unitize for Circle {
    type Output = Circle;

    fn unitize(self) -> Circle {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Dipole {
    type Output = Dipole;

    fn unitize(self) -> Dipole {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for MultiVector {
    type Output = MultiVector;

    fn unitize(self) -> MultiVector {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for RoundPoint {
    type Output = RoundPoint;

    fn unitize(self) -> RoundPoint {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}

impl Unitize for Sphere {
    type Output = Sphere;

    fn unitize(self) -> Sphere {
        self.wedge_dot(Scalar {
            groups: ScalarGroups {
                g0: 1.0 / self.round_weight_norm().group0(),
            },
        })
    }
}
