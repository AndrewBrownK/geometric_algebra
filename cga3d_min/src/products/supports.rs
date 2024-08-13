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

impl AntiSupport for Circle {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for Dipole {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for DualNum {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for FlatPoint {
    type Output = Plane;

    fn anti_support(self) -> Plane {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for Flector {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for Line {
    type Output = Plane;

    fn anti_support(self) -> Plane {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for Motor {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for MultiVector {
    type Output = MultiVector;

    fn anti_support(self) -> MultiVector {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for Plane {
    type Output = Plane;

    fn anti_support(self) -> Plane {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for RoundPoint {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for Scalar {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl AntiSupport for Sphere {
    type Output = Sphere;

    fn anti_support(self) -> Sphere {
        self.wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .complement()
            .anti_wedge(self.dual()),
        )
    }
}

impl Support for AntiScalar {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for Circle {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for Dipole {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for DualNum {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for FlatPoint {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for Flector {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for Line {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for Motor {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for MultiVector {
    type Output = MultiVector;

    fn support(self) -> MultiVector {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for Plane {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for RoundPoint {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}

impl Support for Sphere {
    type Output = RoundPoint;

    fn support(self) -> RoundPoint {
        self.anti_wedge(
            RoundPoint {
                groups: RoundPointGroups {
                    g0: Simd32x3::from(0.0),
                    g1: Simd32x2::from([1.0, 0.0]),
                },
            }
            .wedge(self.anti_dual()),
        )
    }
}
