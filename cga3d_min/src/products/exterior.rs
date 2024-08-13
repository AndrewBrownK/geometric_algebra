// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::*;

/// Exterior Product
///
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products
pub trait Wedge<T> {
    type Output;
    fn wedge(self, other: T) -> Self::Output;
}

/// Geometric Anti-Product
///
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products
pub trait AntiWedge<T> {
    type Output;
    fn anti_wedge(self, other: T) -> Self::Output;
}

impl AntiWedge<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn anti_wedge(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiWedge<Circle> for AntiScalar {
    type Output = Circle;

    fn anti_wedge(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x3::from(self.group0()) * other.group2(),
            },
        }
    }
}

impl AntiWedge<Dipole> for AntiScalar {
    type Output = Dipole;

    fn anti_wedge(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x4::from(self.group0()) * other.group2(),
            },
        }
    }
}

impl AntiWedge<DualNum> for AntiScalar {
    type Output = DualNum;

    fn anti_wedge(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<FlatPoint> for AntiScalar {
    type Output = FlatPoint;

    fn anti_wedge(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Flector> for AntiScalar {
    type Output = Flector;

    fn anti_wedge(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x4::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Line> for AntiScalar {
    type Output = Line;

    fn anti_wedge(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Motor> for AntiScalar {
    type Output = Motor;

    fn anti_wedge(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl AntiWedge<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x2::from(self.group0()) * other.group2(),
                g3: Simd32x3::from(self.group0()) * other.group3(),
                g4: Simd32x3::from(self.group0()) * other.group4(),
                g5: Simd32x4::from(self.group0()) * other.group5(),
                g6: Simd32x4::from(self.group0()) * other.group6(),
                g7: Simd32x3::from(self.group0()) * other.group7(),
                g8: Simd32x3::from(self.group0()) * other.group8(),
                g9: Simd32x3::from(self.group0()) * other.group9(),
                g10: Simd32x2::from(self.group0()) * other.group10(),
            },
        }
    }
}

impl AntiWedge<Plane> for AntiScalar {
    type Output = Plane;

    fn anti_wedge(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for AntiScalar {
    type Output = RoundPoint;

    fn anti_wedge(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Scalar> for AntiScalar {
    type Output = Scalar;

    fn anti_wedge(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiWedge<Sphere> for AntiScalar {
    type Output = Sphere;

    fn anti_wedge(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Circle {
    type Output = Circle;

    fn anti_wedge(self, other: AntiScalar) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Circle {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Circle) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group2()[2], -other.group2()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group2()[2], 0.0, other.group2()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], 0.0])
                    + Simd32x3::from(self.group0()[3]) * other.group1()
                    + self.group1() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl AntiWedge<Dipole> for Circle {
    type Output = Scalar;

    fn anti_wedge(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group2()[0]
                    - self.group0()[1] * other.group2()[1]
                    - self.group0()[2] * other.group2()[2]
                    - self.group0()[3] * other.group2()[3]
                    - self.group1()[0] * other.group1()[0]
                    - self.group1()[1] * other.group1()[1]
                    - self.group1()[2] * other.group1()[2]
                    - self.group2()[0] * other.group0()[0]
                    - self.group2()[1] * other.group0()[1]
                    - self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<DualNum> for Circle {
    type Output = Circle;

    fn anti_wedge(self, other: DualNum) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
                g2: self.group2() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<FlatPoint> for Circle {
    type Output = Scalar;

    fn anti_wedge(self, other: FlatPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<Flector> for Circle {
    type Output = MultiVector;

    fn anti_wedge(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[3])
                    - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g5: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group1()[3], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[3], -other.group1()[2]])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group1()[2], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], 0.0, -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Line> for Circle {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Line) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    + Simd32x3::from(self.group0()[3]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Motor> for Circle {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0() * Simd32x4::from(other.group0()[3]),
                g7: self.group1() * Simd32x3::from(other.group0()[3]),
                g8: self.group2() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group5()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group5()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group5()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group5()[3], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group4()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([-other.group3()[0], 0.0])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([-other.group3()[1], 0.0])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([-other.group3()[2], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group8()[2], -other.group8()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group8()[2], 0.0, other.group8()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group8()[1], -other.group8()[0], 0.0])
                    + Simd32x3::from(self.group0()[3]) * other.group7()
                    + self.group1() * Simd32x3::from(other.group6()[3])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group7()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group7()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group7()[2], 0.0])
                    - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group6()[0], other.group8()[0]])
                    - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group6()[1], other.group8()[1]])
                    - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group6()[2], other.group8()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group7()[2]]),
                g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group9()[2], -other.group9()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group9()[2], 0.0, other.group9()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group9()[1], -other.group9()[0], 0.0])
                    + self.group1() * Simd32x3::from(other.group10()[0]),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group10()[1]) - Simd32x3::from(self.group0()[3]) * other.group9()
                    + self.group2() * Simd32x3::from(other.group10()[0]),
                g5: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group10()[1], 0.0, 0.0, -other.group9()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group10()[1], 0.0, -other.group9()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group10()[1], -other.group9()[2]])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group9()[2], other.group9()[1], 0.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group9()[2], 0.0, -other.group9()[0], 0.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group9()[1], other.group9()[0], 0.0, 0.0]),
                g6: self.group0() * Simd32x4::from(other.group0()[1]),
                g7: self.group1() * Simd32x3::from(other.group0()[1]),
                g8: self.group2() * Simd32x3::from(other.group0()[1]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for Circle {
    type Output = Dipole;

    fn anti_wedge(self, other: Plane) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3])
                    - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl AntiWedge<Sphere> for Circle {
    type Output = Dipole;

    fn anti_wedge(self, other: Sphere) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0])
                    + self.group1() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[1]) - Simd32x3::from(self.group0()[3]) * other.group0()
                    + self.group2() * Simd32x3::from(other.group1()[0]),
                g2: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[1], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group1()[1], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[1], -other.group0()[2]])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Dipole {
    type Output = Dipole;

    fn anti_wedge(self, other: AntiScalar) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Dipole {
    type Output = Scalar;

    fn anti_wedge(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group2()[0]
                    - self.group0()[1] * other.group2()[1]
                    - self.group0()[2] * other.group2()[2]
                    - self.group1()[0] * other.group1()[0]
                    - self.group1()[1] * other.group1()[1]
                    - self.group1()[2] * other.group1()[2]
                    - self.group2()[0] * other.group0()[0]
                    - self.group2()[1] * other.group0()[1]
                    - self.group2()[2] * other.group0()[2]
                    - self.group2()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<DualNum> for Dipole {
    type Output = Dipole;

    fn anti_wedge(self, other: DualNum) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
                g2: self.group2() * Simd32x4::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<Flector> for Dipole {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Flector) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group1()[3])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + Simd32x2::from(self.group2()[3]) * Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl AntiWedge<Line> for Dipole {
    type Output = Scalar;

    fn anti_wedge(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group1()[0]
                    - self.group0()[1] * other.group1()[1]
                    - self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group0()[0]
                    - self.group1()[1] * other.group0()[1]
                    - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Motor> for Dipole {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0() * Simd32x3::from(other.group0()[3]),
                g4: self.group1() * Simd32x3::from(other.group0()[3]),
                g5: self.group2() * Simd32x4::from(other.group0()[3]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group8()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group8()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group8()[2], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group7()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group7()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group7()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([-other.group6()[0], 0.0])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([-other.group6()[1], 0.0])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([-other.group6()[2], 0.0])
                    + Simd32x2::from(self.group2()[3]) * Simd32x2::from([-other.group6()[3], 0.0]),
                g1: self.group0() * Simd32x3::from(other.group10()[1])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group9()[2], other.group9()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group9()[2], 0.0, -other.group9()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group9()[1], other.group9()[0], 0.0])
                    - Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) * Simd32x3::from(other.group10()[0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group9()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group9()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group9()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group9()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group9()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, other.group9()[2]])
                    + Simd32x2::from(self.group2()[3]) * other.group10() * Simd32x2::from([-1.0, 1.0]),
                g3: self.group0() * Simd32x3::from(other.group0()[1]),
                g4: self.group1() * Simd32x3::from(other.group0()[1]),
                g5: self.group2() * Simd32x4::from(other.group0()[1]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for Dipole {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Plane) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group2()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<Sphere> for Dipole {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Sphere) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group1()[1])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0])
                    - Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) * Simd32x3::from(other.group1()[0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group2()[3]) * other.group1() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for DualNum {
    type Output = DualNum;

    fn anti_wedge(self, other: AntiScalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for DualNum {
    type Output = Circle;

    fn anti_wedge(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
                g1: Simd32x3::from(self.group0()[1]) * other.group1(),
                g2: Simd32x3::from(self.group0()[1]) * other.group2(),
            },
        }
    }
}

impl AntiWedge<Dipole> for DualNum {
    type Output = Dipole;

    fn anti_wedge(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
                g1: Simd32x3::from(self.group0()[1]) * other.group1(),
                g2: Simd32x4::from(self.group0()[1]) * other.group2(),
            },
        }
    }
}

impl AntiWedge<DualNum> for DualNum {
    type Output = DualNum;

    fn anti_wedge(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<FlatPoint> for DualNum {
    type Output = FlatPoint;

    fn anti_wedge(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Flector> for DualNum {
    type Output = Flector;

    fn anti_wedge(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
                g1: Simd32x4::from(self.group0()[1]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Line> for DualNum {
    type Output = Line;

    fn anti_wedge(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
                g1: Simd32x3::from(self.group0()[1]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Motor> for DualNum {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()[3]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(self.group0()[1]) * other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<MultiVector> for DualNum {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0(),
                g1: Simd32x3::from(self.group0()[1]) * other.group1(),
                g2: Simd32x2::from(self.group0()[1]) * other.group2(),
                g3: Simd32x3::from(self.group0()[1]) * other.group3(),
                g4: Simd32x3::from(self.group0()[1]) * other.group4(),
                g5: Simd32x4::from(self.group0()[1]) * other.group5(),
                g6: Simd32x4::from(self.group0()[1]) * other.group6(),
                g7: Simd32x3::from(self.group0()[1]) * other.group7(),
                g8: Simd32x3::from(self.group0()[1]) * other.group8(),
                g9: Simd32x3::from(self.group0()[1]) * other.group9(),
                g10: Simd32x2::from(self.group0()[1]) * other.group10(),
            },
        }
    }
}

impl AntiWedge<Plane> for DualNum {
    type Output = Plane;

    fn anti_wedge(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for DualNum {
    type Output = RoundPoint;

    fn anti_wedge(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
                g1: Simd32x2::from(self.group0()[1]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Scalar> for DualNum {
    type Output = Scalar;

    fn anti_wedge(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Sphere> for DualNum {
    type Output = Sphere;

    fn anti_wedge(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
                g1: Simd32x2::from(self.group0()[1]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for FlatPoint {
    type Output = FlatPoint;

    fn anti_wedge(self, other: AntiScalar) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for FlatPoint {
    type Output = Scalar;

    fn anti_wedge(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<DualNum> for FlatPoint {
    type Output = FlatPoint;

    fn anti_wedge(self, other: DualNum) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<Flector> for FlatPoint {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Flector) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl AntiWedge<Motor> for FlatPoint {
    type Output = FlatPoint;

    fn anti_wedge(self, other: Motor) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group6()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group6()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group6()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group6()[3], 0.0]),
                g1: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group10()[0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group9()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group9()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group9()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group10() * Simd32x2::from([-1.0, 1.0]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0() * Simd32x4::from(other.group0()[1]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for FlatPoint {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Plane) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<Sphere> for FlatPoint {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Sphere) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group1() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Flector {
    type Output = Flector;

    fn anti_wedge(self, other: AntiScalar) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Flector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g4: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g5: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group2()[2], -other.group2()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group2()[2], 0.0, other.group2()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Dipole> for Flector {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Dipole) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0])
                    - Simd32x3::from(self.group1()[3]) * other.group0(),
                g1: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], -other.group2()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], -other.group2()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], -other.group2()[2]])
                    + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group2()[3]]),
            },
        }
    }
}

impl AntiWedge<DualNum> for Flector {
    type Output = Flector;

    fn anti_wedge(self, other: DualNum) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
                g1: self.group1() * Simd32x4::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<FlatPoint> for Flector {
    type Output = RoundPoint;

    fn anti_wedge(self, other: FlatPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<Flector> for Flector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group1()[3]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group0()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g8: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group1()[3])
                    - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Line> for Flector {
    type Output = FlatPoint;

    fn anti_wedge(self, other: Line) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Motor> for Flector {
    type Output = Flector;

    fn anti_wedge(self, other: Motor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: self.group1() * Simd32x4::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group6()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group6()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group6()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group6()[3], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], 0.0])
                    + Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group2()[0], 0.0]),
                g1: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group10()[0])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group4()[2], other.group4()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group4()[2], 0.0, -other.group4()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], 0.0])
                    - Simd32x3::from(self.group1()[3]) * other.group3(),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group9()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group9()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group9()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group10() * Simd32x2::from([-1.0, 1.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], -other.group5()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], -other.group5()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], -other.group5()[2]])
                    + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group5()[3]]),
                g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0]),
                g4: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group6()[3])
                    + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]),
                g5: self.group0() * Simd32x4::from(other.group0()[1])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group8()[2], -other.group8()[1], -other.group7()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group8()[2], 0.0, other.group8()[0], -other.group7()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group8()[1], -other.group8()[0], 0.0, -other.group7()[2]])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], 0.0]),
                g6: Simd32x4::from(0.0) - self.group1() * Simd32x4::from(other.group10()[0]),
                g7: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group9()[2], -other.group9()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group9()[2], 0.0, other.group9()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group9()[1], -other.group9()[0], 0.0]),
                g8: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group10()[1]) - Simd32x3::from(self.group1()[3]) * other.group9(),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[1]),
                g10: Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group0()[1]]),
            },
        }
    }
}

impl AntiWedge<Plane> for Flector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g8: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3])
                    - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for Flector {
    type Output = Scalar;

    fn anti_wedge(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] + self.group1()[3] * other.group1()[0],
            },
        }
    }
}

impl AntiWedge<Sphere> for Flector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group1() * Simd32x2::from([-1.0, 1.0]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0) - self.group1() * Simd32x4::from(other.group1()[0]),
                g7: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g8: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group1()[1]) - Simd32x3::from(self.group1()[3]) * other.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Line {
    type Output = Line;

    fn anti_wedge(self, other: AntiScalar) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Line {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Circle) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl AntiWedge<Dipole> for Line {
    type Output = Scalar;

    fn anti_wedge(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group1()[0]
                    - self.group0()[1] * other.group1()[1]
                    - self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group0()[0]
                    - self.group1()[1] * other.group0()[1]
                    - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<DualNum> for Line {
    type Output = Line;

    fn anti_wedge(self, other: DualNum) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<Flector> for Line {
    type Output = FlatPoint;

    fn anti_wedge(self, other: Flector) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[3], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[3], -other.group1()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group1()[2], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], 0.0, -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], 0.0, 0.0]),
            },
        }
    }
}

impl AntiWedge<Line> for Line {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Line) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Motor> for Line {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[3]),
                g8: self.group1() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group4()[2], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group3()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group3()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group3()[2], 0.0]),
                g1: self.group0() * Simd32x3::from(other.group6()[3])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0]),
                g2: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group6()[0], other.group8()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group6()[1], other.group8()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group6()[2], other.group8()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group7()[2]]),
                g3: self.group0() * Simd32x3::from(other.group10()[0]),
                g4: self.group1() * Simd32x3::from(other.group10()[0]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group10()[1], 0.0, 0.0, -other.group9()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group10()[1], 0.0, -other.group9()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group10()[1], -other.group9()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group9()[2], other.group9()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group9()[2], 0.0, -other.group9()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group9()[1], other.group9()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[1]),
                g8: self.group1() * Simd32x3::from(other.group0()[1]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for Line {
    type Output = FlatPoint;

    fn anti_wedge(self, other: Plane) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl AntiWedge<Sphere> for Line {
    type Output = Dipole;

    fn anti_wedge(self, other: Sphere) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group1()[0]),
                g1: self.group1() * Simd32x3::from(other.group1()[0]),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[1], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[1], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[1], -other.group0()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Motor {
    type Output = Motor;

    fn anti_wedge(self, other: AntiScalar) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g2: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(self.group0()[3]) * other.group0(),
                g7: Simd32x3::from(self.group0()[3]) * other.group1(),
                g8: Simd32x3::from(self.group0()[3]) * other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Dipole> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(self.group0()[3]) * other.group0(),
                g4: Simd32x3::from(self.group0()[3]) * other.group1(),
                g5: Simd32x4::from(self.group0()[3]) * other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<DualNum> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]),
                g8: self.group1() * Simd32x3::from(other.group0()[1]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<FlatPoint> for Motor {
    type Output = FlatPoint;

    fn anti_wedge(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Flector> for Motor {
    type Output = Flector;

    fn anti_wedge(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[3], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[3], -other.group1()[2]])
                    + Simd32x4::from(self.group0()[3]) * other.group0()
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group1()[2], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], 0.0, -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], 0.0, 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Line> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[3]) * other.group0(),
                g8: Simd32x3::from(self.group0()[3]) * other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Motor> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(self.group0()[3]) * other.group1() + self.group1() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group4()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * other.group0()
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group3()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group3()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group3()[2], 0.0]),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group6()[3])
                    + Simd32x3::from(self.group0()[3]) * other.group1()
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0]),
                g2: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group6()[0], other.group8()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group6()[1], other.group8()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group6()[2], other.group8()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group2()
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group7()[2]]),
                g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group10()[0]) + Simd32x3::from(self.group0()[3]) * other.group3(),
                g4: Simd32x3::from(self.group0()[3]) * other.group4() + self.group1() * Simd32x3::from(other.group10()[0]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group10()[1], 0.0, 0.0, -other.group9()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group10()[1], 0.0, -other.group9()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group10()[1], -other.group9()[2]])
                    + Simd32x4::from(self.group0()[3]) * other.group5()
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group9()[2], other.group9()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group9()[2], 0.0, -other.group9()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group9()[1], other.group9()[0], 0.0, 0.0]),
                g6: Simd32x4::from(self.group0()[3]) * other.group6(),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]) + Simd32x3::from(self.group0()[3]) * other.group7(),
                g8: Simd32x3::from(self.group0()[3]) * other.group8() + self.group1() * Simd32x3::from(other.group0()[1]),
                g9: Simd32x3::from(self.group0()[3]) * other.group9(),
                g10: Simd32x2::from(self.group0()[3]) * other.group10(),
            },
        }
    }
}

impl AntiWedge<Plane> for Motor {
    type Output = Flector;

    fn anti_wedge(self, other: Plane) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for Motor {
    type Output = RoundPoint;

    fn anti_wedge(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
                g1: Simd32x2::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Scalar> for Motor {
    type Output = Scalar;

    fn anti_wedge(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Sphere> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]),
                g4: self.group1() * Simd32x3::from(other.group1()[0]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[1], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[1], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[1], -other.group0()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[3]) * other.group0(),
                g10: Simd32x2::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x2::from(other.group0()),
                g3: self.group3() * Simd32x3::from(other.group0()),
                g4: self.group4() * Simd32x3::from(other.group0()),
                g5: self.group5() * Simd32x4::from(other.group0()),
                g6: self.group6() * Simd32x4::from(other.group0()),
                g7: self.group7() * Simd32x3::from(other.group0()),
                g8: self.group8() * Simd32x3::from(other.group0()),
                g9: self.group9() * Simd32x3::from(other.group0()),
                g10: self.group10() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group2()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group2()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group2()[2], 0.0])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group5()[3]) * Simd32x2::from([-other.group0()[3], 0.0]),
                g1: Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group2()[2], -other.group2()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group2()[2], 0.0, other.group2()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], 0.0])
                    + Simd32x3::from(self.group6()[3]) * other.group1()
                    + self.group7() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group8()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group8()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group8()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    - Simd32x2::from(self.group7()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group7()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group7()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
                g3: Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0])
                    + Simd32x3::from(self.group10()[0]) * other.group1(),
                g4: Simd32x3::from(0.0) - self.group9() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group10()[0]) * other.group2()
                    + Simd32x3::from(self.group10()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g5: Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, other.group2()[2], -other.group2()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([-other.group2()[2], 0.0, other.group2()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
                g6: Simd32x4::from(self.group0()[1]) * other.group0(),
                g7: Simd32x3::from(self.group0()[1]) * other.group1(),
                g8: Simd32x3::from(self.group0()[1]) * other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group2()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group2()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group2()[2], 0.0])
                    + Simd32x2::from(self.group6()[3]) * Simd32x2::from([-other.group2()[3], 0.0])
                    + Simd32x2::from(self.group7()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0])
                    + Simd32x3::from(self.group10()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])
                    - Simd32x3::from(self.group10()[1]) * other.group0(),
                g2: Simd32x2::from(self.group9()[0]) * Simd32x2::from([other.group0()[0], -other.group2()[0]])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([other.group0()[1], -other.group2()[1]])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([other.group0()[2], -other.group2()[2]])
                    + self.group10() * Simd32x2::from(other.group2()[3]),
                g3: Simd32x3::from(self.group0()[1]) * other.group0(),
                g4: Simd32x3::from(self.group0()[1]) * other.group1(),
                g5: Simd32x4::from(self.group0()[1]) * other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<DualNum> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0(),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
                g2: self.group2() * Simd32x2::from(other.group0()[1]),
                g3: self.group3() * Simd32x3::from(other.group0()[1]),
                g4: self.group4() * Simd32x3::from(other.group0()[1]),
                g5: self.group5() * Simd32x4::from(other.group0()[1]),
                g6: self.group6() * Simd32x4::from(other.group0()[1]),
                g7: self.group7() * Simd32x3::from(other.group0()[1]),
                g8: self.group8() * Simd32x3::from(other.group0()[1]),
                g9: self.group9() * Simd32x3::from(other.group0()[1]),
                g10: self.group10() * Simd32x2::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group6()[3]) * Simd32x2::from([-other.group0()[3], 0.0]),
                g1: Simd32x3::from(self.group10()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x2::from(self.group9()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + self.group10() * Simd32x2::from(other.group0()[3]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[1]) * other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group1()[3], 0.0])
                    + Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group6()[3]) * Simd32x2::from([-other.group0()[3], 0.0]),
                g1: self.group3() * Simd32x3::from(other.group1()[3])
                    + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0])
                    + Simd32x3::from(self.group10()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + Simd32x2::from(self.group5()[3]) * Simd32x2::from([0.0, other.group1()[3]])
                    + Simd32x2::from(self.group9()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + self.group10() * Simd32x2::from(other.group0()[3]),
                g3: Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g4: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group1()[3])
                    - Simd32x3::from(self.group6()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g5: Simd32x4::from(self.group0()[1]) * other.group0()
                    + Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group1()[3], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group7()[1]) * Simd32x4::from([0.0, other.group1()[3], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group7()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[3], -other.group1()[2]])
                    + Simd32x4::from(self.group8()[0]) * Simd32x4::from([0.0, -other.group1()[2], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group1()[2], 0.0, -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group8()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], 0.0, 0.0]),
                g6: Simd32x4::from(self.group10()[0]) * other.group1(),
                g7: Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g8: self.group9() * Simd32x3::from(other.group1()[3])
                    - Simd32x3::from(self.group10()[1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g9: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl AntiWedge<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    + Simd32x3::from(self.group6()[3]) * other.group0(),
                g2: Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group7()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(self.group10()[0]) * other.group0(),
                g4: Simd32x3::from(self.group10()[0]) * other.group1(),
                g5: Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[1]) * other.group0(),
                g8: Simd32x3::from(self.group0()[1]) * other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: self.group1() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    + Simd32x3::from(self.group6()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: self.group2() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group7()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: self.group3() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group10()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g4: self.group4() * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group10()[0]) * other.group1(),
                g5: self.group5() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: self.group6() * Simd32x4::from(other.group0()[3]),
                g7: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    + self.group7() * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(self.group0()[1]) * other.group1() + self.group8() * Simd32x3::from(other.group0()[3]),
                g9: self.group9() * Simd32x3::from(other.group0()[3]),
                g10: self.group10() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[1]) * other.group0()
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group9()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group9()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group9()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group10()[1], 0.0])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group10()[0], 0.0])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group8()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group8()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group8()[2], 0.0])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group7()[0], 0.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group7()[1], 0.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group7()[2], 0.0])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([-other.group6()[0], 0.0])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([-other.group6()[1], 0.0])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([-other.group6()[2], 0.0])
                    + Simd32x2::from(self.group5()[3]) * Simd32x2::from([-other.group6()[3], 0.0])
                    + Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group5()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group5()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group5()[2], 0.0])
                    + Simd32x2::from(self.group6()[3]) * Simd32x2::from([-other.group5()[3], 0.0])
                    + Simd32x2::from(self.group7()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([-other.group4()[2], 0.0])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([-other.group3()[0], 0.0])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([-other.group3()[1], 0.0])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([-other.group3()[2], 0.0])
                    + Simd32x2::from(self.group9()[0]) * Simd32x2::from([other.group1()[0], 0.0])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([other.group1()[1], 0.0])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([other.group1()[2], 0.0])
                    + Simd32x2::from(self.group10()[0]) * Simd32x2::from([other.group2()[1], 0.0])
                    + Simd32x2::from(self.group10()[1]) * Simd32x2::from([other.group2()[0], 0.0]),
                g1: Simd32x3::from(self.group0()[1]) * other.group1()
                    + self.group1() * Simd32x3::from(other.group0()[1])
                    + self.group3() * Simd32x3::from(other.group10()[1])
                    + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group9()[2], other.group9()[1]])
                    + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group9()[2], 0.0, -other.group9()[0]])
                    + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group9()[1], other.group9()[0], 0.0])
                    - Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]) * Simd32x3::from(other.group10()[0])
                    + Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group8()[2], -other.group8()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group8()[2], 0.0, other.group8()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group8()[1], -other.group8()[0], 0.0])
                    + Simd32x3::from(self.group6()[3]) * other.group7()
                    + self.group7() * Simd32x3::from(other.group6()[3])
                    + Simd32x3::from(self.group8()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group8()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group8()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0])
                    + Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, -other.group4()[2], other.group4()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([other.group4()[2], 0.0, -other.group4()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], 0.0])
                    + Simd32x3::from(self.group10()[0]) * Simd32x3::from([other.group5()[0], other.group5()[1], other.group5()[2]])
                    - Simd32x3::from(self.group10()[1]) * other.group3(),
                g2: Simd32x2::from(self.group0()[1]) * other.group2()
                    + self.group2() * Simd32x2::from(other.group0()[1])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group9()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group9()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group9()[2], 0.0])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, other.group9()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, other.group9()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, other.group9()[2]])
                    + Simd32x2::from(self.group5()[3]) * other.group10() * Simd32x2::from([-1.0, 1.0])
                    + Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group7()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group7()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group7()[2], 0.0])
                    - Simd32x2::from(self.group7()[0]) * Simd32x2::from([other.group6()[0], other.group8()[0]])
                    - Simd32x2::from(self.group7()[1]) * Simd32x2::from([other.group6()[1], other.group8()[1]])
                    - Simd32x2::from(self.group7()[2]) * Simd32x2::from([other.group6()[2], other.group8()[2]])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group7()[2]])
                    + Simd32x2::from(self.group9()[0]) * Simd32x2::from([other.group3()[0], -other.group5()[0]])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([other.group3()[1], -other.group5()[1]])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([other.group3()[2], -other.group5()[2]])
                    + self.group10() * Simd32x2::from(other.group5()[3]),
                g3: Simd32x3::from(self.group0()[1]) * other.group3()
                    + self.group3() * Simd32x3::from(other.group0()[1])
                    + Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group9()[2], -other.group9()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group9()[2], 0.0, other.group9()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group9()[1], -other.group9()[0], 0.0])
                    + self.group7() * Simd32x3::from(other.group10()[0])
                    + Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0])
                    + Simd32x3::from(self.group10()[0]) * other.group7(),
                g4: Simd32x3::from(self.group0()[1]) * other.group4()
                    + self.group4() * Simd32x3::from(other.group0()[1])
                    + Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group10()[1])
                    - Simd32x3::from(self.group6()[3]) * other.group9()
                    + self.group8() * Simd32x3::from(other.group10()[0])
                    - self.group9() * Simd32x3::from(other.group6()[3])
                    + Simd32x3::from(self.group10()[0]) * other.group8()
                    + Simd32x3::from(self.group10()[1]) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]),
                g5: Simd32x4::from(self.group0()[1]) * other.group5()
                    + self.group5() * Simd32x4::from(other.group0()[1])
                    + Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group10()[1], 0.0, 0.0, -other.group9()[0]])
                    + Simd32x4::from(self.group7()[1]) * Simd32x4::from([0.0, other.group10()[1], 0.0, -other.group9()[1]])
                    + Simd32x4::from(self.group7()[2]) * Simd32x4::from([0.0, 0.0, other.group10()[1], -other.group9()[2]])
                    + Simd32x4::from(self.group8()[0]) * Simd32x4::from([0.0, -other.group9()[2], other.group9()[1], 0.0])
                    + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group9()[2], 0.0, -other.group9()[0], 0.0])
                    + Simd32x4::from(self.group8()[2]) * Simd32x4::from([-other.group9()[1], other.group9()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, other.group8()[2], -other.group8()[1], -other.group7()[0]])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([-other.group8()[2], 0.0, other.group8()[0], -other.group7()[1]])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group8()[1], -other.group8()[0], 0.0, -other.group7()[2]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], 0.0]),
                g6: Simd32x4::from(self.group0()[1]) * other.group6()
                    + self.group6() * Simd32x4::from(other.group0()[1])
                    + Simd32x4::from([self.group9()[0], self.group9()[1], self.group9()[2], self.group9()[0]])
                        * Simd32x4::from([-other.group10()[0], -other.group10()[0], -other.group10()[0], 0.0])
                    + Simd32x4::from(self.group10()[0]) * Simd32x4::from([other.group9()[0], other.group9()[1], other.group9()[2], other.group10()[1]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group10()[0]]),
                g7: Simd32x3::from(self.group0()[1]) * other.group7()
                    + self.group7() * Simd32x3::from(other.group0()[1])
                    + Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, other.group9()[2], -other.group9()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([-other.group9()[2], 0.0, other.group9()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([other.group9()[1], -other.group9()[0], 0.0]),
                g8: Simd32x3::from(self.group0()[1]) * other.group8() + self.group8() * Simd32x3::from(other.group0()[1]) + self.group9() * Simd32x3::from(other.group10()[1])
                    - Simd32x3::from(self.group10()[1]) * other.group9(),
                g9: Simd32x3::from(self.group0()[1]) * other.group9() + self.group9() * Simd32x3::from(other.group0()[1]),
                g10: Simd32x2::from(self.group0()[1]) * other.group10() + self.group10() * Simd32x2::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group0()[3], 0.0]),
                g1: self.group3() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group5()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g3: Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g4: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group0()[3])
                    - Simd32x3::from(self.group6()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g5: Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group7()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group7()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]])
                    + Simd32x4::from(self.group8()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group8()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(self.group10()[0]) * other.group0(),
                g7: Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g8: self.group9() * Simd32x3::from(other.group0()[3])
                    - Simd32x3::from(self.group10()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group9()[0]) * Simd32x2::from([other.group0()[0], 0.0])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([other.group0()[1], 0.0])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([other.group0()[2], 0.0])
                    + Simd32x2::from(self.group10()[0]) * Simd32x2::from([other.group1()[1], 0.0])
                    + Simd32x2::from(self.group10()[1]) * Simd32x2::from([other.group1()[0], 0.0]),
                g1: Simd32x3::from(self.group0()[1]) * other.group0(),
                g2: Simd32x2::from(self.group0()[1]) * other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Scalar> for MultiVector {
    type Output = Scalar;

    fn anti_wedge(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group1()[1], 0.0])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group1()[0], 0.0]),
                g1: self.group3() * Simd32x3::from(other.group1()[1])
                    + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0])
                    - Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]) * Simd32x3::from(other.group1()[0]),
                g2: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group5()[3]) * other.group1() * Simd32x2::from([-1.0, 1.0]),
                g3: Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0])
                    + self.group7() * Simd32x3::from(other.group1()[0]),
                g4: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group1()[1]) - Simd32x3::from(self.group6()[3]) * other.group0()
                    + self.group8() * Simd32x3::from(other.group1()[0]),
                g5: Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group1()[1], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group7()[1]) * Simd32x4::from([0.0, other.group1()[1], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group7()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[1], -other.group0()[2]])
                    + Simd32x4::from(self.group8()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group8()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from([self.group9()[0], self.group9()[1], self.group9()[2], self.group9()[0]])
                    * Simd32x4::from([-other.group1()[0], -other.group1()[0], -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group10()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[1]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]),
                g7: Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g8: self.group9() * Simd32x3::from(other.group1()[1]) - Simd32x3::from(self.group10()[1]) * other.group0(),
                g9: Simd32x3::from(self.group0()[1]) * other.group0(),
                g10: Simd32x2::from(self.group0()[1]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Plane {
    type Output = Plane;

    fn anti_wedge(self, other: AntiScalar) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Plane {
    type Output = Dipole;

    fn anti_wedge(self, other: Circle) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group2()[2], -other.group2()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], 0.0, other.group2()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Dipole> for Plane {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Dipole) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0])
                    - Simd32x3::from(self.group0()[3]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], -other.group2()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], -other.group2()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], -other.group2()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group2()[3]]),
            },
        }
    }
}

impl AntiWedge<DualNum> for Plane {
    type Output = Plane;

    fn anti_wedge(self, other: DualNum) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<FlatPoint> for Plane {
    type Output = RoundPoint;

    fn anti_wedge(self, other: FlatPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<Flector> for Plane {
    type Output = MultiVector;

    fn anti_wedge(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[3])
                    - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Line> for Plane {
    type Output = FlatPoint;

    fn anti_wedge(self, other: Line) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Motor> for Plane {
    type Output = Flector;

    fn anti_wedge(self, other: Motor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: self.group0() * Simd32x4::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group2()[0], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group4()[2], other.group4()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], 0.0, -other.group4()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], 0.0])
                    - Simd32x3::from(self.group0()[3]) * other.group3(),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], -other.group5()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], -other.group5()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], -other.group5()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group5()[3]]),
                g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0]),
                g4: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group6()[3])
                    + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group8()[2], -other.group8()[1], -other.group7()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group8()[2], 0.0, other.group8()[0], -other.group7()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group8()[1], -other.group8()[0], 0.0, -other.group7()[2]])
                    + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], 0.0]),
                g6: Simd32x4::from(0.0) - self.group0() * Simd32x4::from(other.group10()[0]),
                g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group9()[2], -other.group9()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group9()[2], 0.0, other.group9()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group9()[1], -other.group9()[0], 0.0]),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group10()[1]) - Simd32x3::from(self.group0()[3]) * other.group9(),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]),
                g10: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[1]]),
            },
        }
    }
}

impl AntiWedge<Plane> for Plane {
    type Output = Line;

    fn anti_wedge(self, other: Plane) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3])
                    - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for Plane {
    type Output = Scalar;

    fn anti_wedge(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl AntiWedge<Sphere> for Plane {
    type Output = Circle;

    fn anti_wedge(self, other: Sphere) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0) - self.group0() * Simd32x4::from(other.group1()[0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[1]) - Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for RoundPoint {
    type Output = RoundPoint;

    fn anti_wedge(self, other: AntiScalar) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<DualNum> for RoundPoint {
    type Output = RoundPoint;

    fn anti_wedge(self, other: DualNum) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x2::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<Flector> for RoundPoint {
    type Output = Scalar;

    fn anti_wedge(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] + self.group1()[0] * other.group1()[3],
            },
        }
    }
}

impl AntiWedge<Motor> for RoundPoint {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Motor) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
                g1: self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group9()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group9()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group9()[2], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group10()[1], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group10()[0], 0.0]),
                g1: self.group0() * Simd32x3::from(other.group0()[1]),
                g2: self.group1() * Simd32x2::from(other.group0()[1]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for RoundPoint {
    type Output = Scalar;

    fn anti_wedge(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<Sphere> for RoundPoint {
    type Output = Scalar;

    fn anti_wedge(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    + self.group1()[0] * other.group1()[1]
                    + self.group1()[1] * other.group1()[0],
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Scalar {
    type Output = Scalar;

    fn anti_wedge(self, other: AntiScalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiWedge<DualNum> for Scalar {
    type Output = Scalar;

    fn anti_wedge(self, other: DualNum) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl AntiWedge<Motor> for Scalar {
    type Output = Scalar;

    fn anti_wedge(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<MultiVector> for Scalar {
    type Output = Scalar;

    fn anti_wedge(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Sphere {
    type Output = Sphere;

    fn anti_wedge(self, other: AntiScalar) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Sphere {
    type Output = Dipole;

    fn anti_wedge(self, other: Circle) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0])
                    + Simd32x3::from(self.group1()[0]) * other.group1(),
                g1: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group1()[0]) * other.group2()
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group2()[2], -other.group2()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], 0.0, other.group2()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Dipole> for Sphere {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Dipole) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])
                    - Simd32x3::from(self.group1()[1]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], -other.group2()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], -other.group2()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], -other.group2()[2]])
                    + self.group1() * Simd32x2::from(other.group2()[3]),
            },
        }
    }
}

impl AntiWedge<DualNum> for Sphere {
    type Output = Sphere;

    fn anti_wedge(self, other: DualNum) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x2::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<FlatPoint> for Sphere {
    type Output = RoundPoint;

    fn anti_wedge(self, other: FlatPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<Flector> for Sphere {
    type Output = MultiVector;

    fn anti_wedge(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + self.group1() * Simd32x2::from(other.group0()[3]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(self.group1()[0]) * other.group1(),
                g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g8: self.group0() * Simd32x3::from(other.group1()[3])
                    - Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Line> for Sphere {
    type Output = Dipole;

    fn anti_wedge(self, other: Line) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x3::from(self.group1()[0]) * other.group1(),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Motor> for Sphere {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g4: Simd32x3::from(self.group1()[0]) * other.group1(),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0() * Simd32x3::from(other.group0()[3]),
                g10: self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group2()[1], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group2()[0], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group4()[2], other.group4()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], 0.0, -other.group4()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], 0.0])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group5()[0], other.group5()[1], other.group5()[2]])
                    - Simd32x3::from(self.group1()[1]) * other.group3(),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], -other.group5()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], -other.group5()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], -other.group5()[2]])
                    + self.group1() * Simd32x2::from(other.group5()[3]),
                g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0])
                    + Simd32x3::from(self.group1()[0]) * other.group7(),
                g4: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group6()[3])
                    + Simd32x3::from(self.group1()[0]) * other.group8()
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group8()[2], -other.group8()[1], -other.group7()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group8()[2], 0.0, other.group8()[0], -other.group7()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group8()[1], -other.group8()[0], 0.0, -other.group7()[2]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], 0.0]),
                g6: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([-other.group10()[0], -other.group10()[0], -other.group10()[0], 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group9()[0], other.group9()[1], other.group9()[2], other.group10()[1]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group10()[0]]),
                g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group9()[2], -other.group9()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group9()[2], 0.0, other.group9()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group9()[1], -other.group9()[0], 0.0]),
                g8: self.group0() * Simd32x3::from(other.group10()[1]) - Simd32x3::from(self.group1()[1]) * other.group9(),
                g9: self.group0() * Simd32x3::from(other.group0()[1]),
                g10: self.group1() * Simd32x2::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<Plane> for Sphere {
    type Output = Circle;

    fn anti_wedge(self, other: Plane) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group1()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: self.group0() * Simd32x3::from(other.group0()[3])
                    - Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for Sphere {
    type Output = Scalar;

    fn anti_wedge(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    + self.group1()[0] * other.group1()[1]
                    + self.group1()[1] * other.group1()[0],
            },
        }
    }
}

impl AntiWedge<Sphere> for Sphere {
    type Output = Circle;

    fn anti_wedge(self, other: Sphere) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([-other.group1()[0], -other.group1()[0], -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[1]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: self.group0() * Simd32x3::from(other.group1()[1]) - Simd32x3::from(self.group1()[1]) * other.group0(),
            },
        }
    }
}

impl Wedge<DualNum> for AntiScalar {
    type Output = AntiScalar;

    fn wedge(self, other: DualNum) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Wedge<MultiVector> for AntiScalar {
    type Output = AntiScalar;

    fn wedge(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Wedge<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn wedge(self, other: Scalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Wedge<Dipole> for Circle {
    type Output = AntiScalar;

    fn wedge(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group2()[0]
                    - self.group0()[1] * other.group2()[1]
                    - self.group0()[2] * other.group2()[2]
                    - self.group0()[3] * other.group2()[3]
                    - self.group1()[0] * other.group1()[0]
                    - self.group1()[1] * other.group1()[1]
                    - self.group1()[2] * other.group1()[2]
                    - self.group2()[0] * other.group0()[0]
                    - self.group2()[1] * other.group0()[1]
                    - self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<DualNum> for Circle {
    type Output = Circle;

    fn wedge(self, other: DualNum) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
                g2: self.group2() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<FlatPoint> for Circle {
    type Output = AntiScalar;

    fn wedge(self, other: FlatPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Wedge<Flector> for Circle {
    type Output = AntiScalar;

    fn wedge(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Wedge<MultiVector> for Circle {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group5()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group5()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group5()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group5()[3]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group4()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group3()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group3()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group3()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0() * Simd32x4::from(other.group0()[0]),
                g7: self.group1() * Simd32x3::from(other.group0()[0]),
                g8: self.group2() * Simd32x3::from(other.group0()[0]),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group2()[1])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    - self.group2() * Simd32x3::from(other.group2()[0]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * other.group2() * Simd32x2::from([-1.0, 1.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, other.group1()[2]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for Circle {
    type Output = Sphere;

    fn wedge(self, other: RoundPoint) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[1])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0])
                    - self.group2() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * other.group1() * Simd32x2::from([-1.0, 1.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Scalar> for Circle {
    type Output = Circle;

    fn wedge(self, other: Scalar) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Wedge<Circle> for Dipole {
    type Output = AntiScalar;

    fn wedge(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group2()[0]
                    - self.group0()[1] * other.group2()[1]
                    - self.group0()[2] * other.group2()[2]
                    - self.group1()[0] * other.group1()[0]
                    - self.group1()[1] * other.group1()[1]
                    - self.group1()[2] * other.group1()[2]
                    - self.group2()[0] * other.group0()[0]
                    - self.group2()[1] * other.group0()[1]
                    - self.group2()[2] * other.group0()[2]
                    - self.group2()[3] * other.group0()[3],
            },
        }
    }
}

impl Wedge<Dipole> for Dipole {
    type Output = Sphere;

    fn wedge(self, other: Dipole) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group2()[2], other.group2()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], 0.0, -other.group2()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], 0.0])
                    + self.group1() * Simd32x3::from(other.group2()[3])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0])
                    + Simd32x3::from(self.group2()[3]) * other.group1(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl Wedge<DualNum> for Dipole {
    type Output = Dipole;

    fn wedge(self, other: DualNum) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
                g2: self.group2() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<FlatPoint> for Dipole {
    type Output = Plane;

    fn wedge(self, other: FlatPoint) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Flector> for Dipole {
    type Output = Plane;

    fn wedge(self, other: Flector) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Line> for Dipole {
    type Output = AntiScalar;

    fn wedge(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group1()[0]
                    - self.group0()[1] * other.group1()[1]
                    - self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group0()[0]
                    - self.group1()[1] * other.group0()[1]
                    - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<Motor> for Dipole {
    type Output = AntiScalar;

    fn wedge(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group1()[0]
                    - self.group0()[1] * other.group1()[1]
                    - self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group0()[0]
                    - self.group1()[1] * other.group0()[1]
                    - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<MultiVector> for Dipole {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group8()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group8()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group8()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group7()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group6()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group6()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group6()[2]])
                    + Simd32x2::from(self.group2()[3]) * Simd32x2::from([0.0, -other.group6()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0() * Simd32x3::from(other.group0()[0]),
                g4: self.group1() * Simd32x3::from(other.group0()[0]),
                g5: self.group2() * Simd32x4::from(other.group0()[0]),
                g6: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group1()[2], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], 0.0, -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group2()[0], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group2()[0], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group2()[0], -other.group1()[2]]),
                g7: self.group0() * Simd32x3::from(other.group2()[1]) + Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) * Simd32x3::from(other.group2()[0])
                    - Simd32x3::from(self.group2()[3]) * other.group1(),
                g8: self.group1() * Simd32x3::from(other.group2()[1])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group5()[2], other.group5()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group5()[2], 0.0, -other.group5()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group5()[1], other.group5()[0], 0.0])
                    + self.group1() * Simd32x3::from(other.group5()[3])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group3()[2], -other.group3()[1]])
                    + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group3()[2], 0.0, other.group3()[0]])
                    + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], 0.0])
                    + Simd32x3::from(self.group2()[3]) * other.group4(),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group4()[2], 0.0])
                    - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], other.group5()[0]])
                    - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], other.group5()[1]])
                    - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], other.group5()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group4()[2]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for Dipole {
    type Output = Circle;

    fn wedge(self, other: RoundPoint) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group1()[0], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[0], -other.group0()[2]]),
                g1: self.group0() * Simd32x3::from(other.group1()[1]) + Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) * Simd32x3::from(other.group1()[0])
                    - Simd32x3::from(self.group2()[3]) * other.group0(),
                g2: self.group1() * Simd32x3::from(other.group1()[1])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Scalar> for Dipole {
    type Output = Dipole;

    fn wedge(self, other: Scalar) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Wedge<AntiScalar> for DualNum {
    type Output = AntiScalar;

    fn wedge(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Wedge<Circle> for DualNum {
    type Output = Circle;

    fn wedge(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * other.group1(),
                g2: Simd32x3::from(self.group0()[0]) * other.group2(),
            },
        }
    }
}

impl Wedge<Dipole> for DualNum {
    type Output = Dipole;

    fn wedge(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * other.group1(),
                g2: Simd32x4::from(self.group0()[0]) * other.group2(),
            },
        }
    }
}

impl Wedge<DualNum> for DualNum {
    type Output = DualNum;

    fn wedge(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]),
            },
        }
    }
}

impl Wedge<FlatPoint> for DualNum {
    type Output = FlatPoint;

    fn wedge(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Wedge<Flector> for DualNum {
    type Output = Flector;

    fn wedge(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
                g1: Simd32x4::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Wedge<Line> for DualNum {
    type Output = Line;

    fn wedge(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Wedge<Motor> for DualNum {
    type Output = Motor;

    fn wedge(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Wedge<MultiVector> for DualNum {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]),
                g1: Simd32x3::from(self.group0()[0]) * other.group1(),
                g2: Simd32x2::from(self.group0()[0]) * other.group2(),
                g3: Simd32x3::from(self.group0()[0]) * other.group3(),
                g4: Simd32x3::from(self.group0()[0]) * other.group4(),
                g5: Simd32x4::from(self.group0()[0]) * other.group5(),
                g6: Simd32x4::from(self.group0()[0]) * other.group6(),
                g7: Simd32x3::from(self.group0()[0]) * other.group7(),
                g8: Simd32x3::from(self.group0()[0]) * other.group8(),
                g9: Simd32x3::from(self.group0()[0]) * other.group9(),
                g10: Simd32x2::from(self.group0()[0]) * other.group10(),
            },
        }
    }
}

impl Wedge<Plane> for DualNum {
    type Output = Plane;

    fn wedge(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Wedge<RoundPoint> for DualNum {
    type Output = RoundPoint;

    fn wedge(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Wedge<Scalar> for DualNum {
    type Output = DualNum;

    fn wedge(self, other: Scalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Wedge<Sphere> for DualNum {
    type Output = Sphere;

    fn wedge(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Wedge<Circle> for FlatPoint {
    type Output = AntiScalar;

    fn wedge(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Wedge<Dipole> for FlatPoint {
    type Output = Plane;

    fn wedge(self, other: Dipole) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            },
        }
    }
}

impl Wedge<DualNum> for FlatPoint {
    type Output = FlatPoint;

    fn wedge(self, other: DualNum) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group6()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group6()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group6()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group6()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0() * Simd32x4::from(other.group0()[0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group2()[0]) - Simd32x3::from(self.group0()[3]) * other.group1(),
                g8: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group3()[2], -other.group3()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], 0.0, other.group3()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], 0.0])
                    + Simd32x3::from(self.group0()[3]) * other.group4(),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for FlatPoint {
    type Output = Line;

    fn wedge(self, other: RoundPoint) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]) - Simd32x3::from(self.group0()[3]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Scalar> for FlatPoint {
    type Output = FlatPoint;

    fn wedge(self, other: Scalar) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Wedge<Circle> for Flector {
    type Output = AntiScalar;

    fn wedge(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Wedge<Dipole> for Flector {
    type Output = Plane;

    fn wedge(self, other: Dipole) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            },
        }
    }
}

impl Wedge<DualNum> for Flector {
    type Output = Flector;

    fn wedge(self, other: DualNum) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
                g1: self.group1() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for Flector {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group6()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group6()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group6()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group6()[3]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group2()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0() * Simd32x4::from(other.group0()[0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group2()[0]) - Simd32x3::from(self.group0()[3]) * other.group1(),
                g8: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group3()[2], -other.group3()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], 0.0, other.group3()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], 0.0])
                    + Simd32x3::from(self.group0()[3]) * other.group4()
                    + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[0]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]])
                    + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group0()[0]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for Flector {
    type Output = Motor;

    fn wedge(self, other: RoundPoint) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], 0.0])
                    + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Scalar> for Flector {
    type Output = Flector;

    fn wedge(self, other: Scalar) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Wedge<Dipole> for Line {
    type Output = AntiScalar;

    fn wedge(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group1()[0]
                    - self.group0()[1] * other.group1()[1]
                    - self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group0()[0]
                    - self.group1()[1] * other.group0()[1]
                    - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<DualNum> for Line {
    type Output = Line;

    fn wedge(self, other: DualNum) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for Line {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group3()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group3()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group3()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[0]),
                g8: self.group1() * Simd32x3::from(other.group0()[0]),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    - self.group1() * Simd32x3::from(other.group2()[0]),
                g10: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group1()[2]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for Line {
    type Output = Plane;

    fn wedge(self, other: RoundPoint) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([-other.group1()[0], 0.0, 0.0, other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, -other.group1()[0], 0.0, other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, -other.group1()[0], other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Scalar> for Line {
    type Output = Line;

    fn wedge(self, other: Scalar) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Wedge<Dipole> for Motor {
    type Output = AntiScalar;

    fn wedge(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group1()[0]
                    - self.group0()[1] * other.group1()[1]
                    - self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group0()[0]
                    - self.group1()[1] * other.group0()[1]
                    - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<DualNum> for Motor {
    type Output = Motor;

    fn wedge(self, other: DualNum) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for Motor {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group3()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group3()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group3()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]),
                g8: self.group1() * Simd32x3::from(other.group0()[0]),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    - self.group1() * Simd32x3::from(other.group2()[0]),
                g10: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group1()[2]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for Motor {
    type Output = Plane;

    fn wedge(self, other: RoundPoint) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([-other.group1()[0], 0.0, 0.0, other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, -other.group1()[0], 0.0, other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, -other.group1()[0], other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Scalar> for Motor {
    type Output = Motor;

    fn wedge(self, other: Scalar) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Wedge<AntiScalar> for MultiVector {
    type Output = AntiScalar;

    fn wedge(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Wedge<Circle> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group2()[0]])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group2()[1]])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group2()[2]])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + Simd32x2::from(self.group5()[3]) * Simd32x2::from([0.0, -other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(self.group0()[0]) * other.group0(),
                g7: Simd32x3::from(self.group0()[0]) * other.group1(),
                g8: Simd32x3::from(self.group0()[0]) * other.group2(),
                g9: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    + Simd32x3::from(self.group2()[0]) * other.group2()
                    - Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], -other.group2()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], -other.group2()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], -other.group2()[2]])
                    + self.group2() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl Wedge<Dipole> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[0]) * Simd32x2::from([0.0, -other.group2()[0]])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([0.0, -other.group2()[1]])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([0.0, -other.group2()[2]])
                    + Simd32x2::from(self.group6()[3]) * Simd32x2::from([0.0, -other.group2()[3]])
                    + Simd32x2::from(self.group7()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(self.group0()[0]) * other.group0(),
                g4: Simd32x3::from(self.group0()[0]) * other.group1(),
                g5: Simd32x4::from(self.group0()[0]) * other.group2(),
                g6: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
                g7: Simd32x3::from(0.0) - self.group1() * Simd32x3::from(other.group2()[3])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])
                    + Simd32x3::from(self.group2()[1]) * other.group0(),
                g8: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group2()[2], other.group2()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group2()[2], 0.0, -other.group2()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], 0.0])
                    + Simd32x3::from(self.group2()[1]) * other.group1(),
                g9: Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group2()[2], other.group2()[1]])
                    + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group2()[2], 0.0, -other.group2()[0]])
                    + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], 0.0])
                    + self.group4() * Simd32x3::from(other.group2()[3])
                    + Simd32x3::from(self.group5()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group5()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0])
                    + Simd32x3::from(self.group5()[3]) * other.group1(),
                g10: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    - Simd32x2::from(self.group4()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group4()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group4()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl Wedge<DualNum> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
                g2: self.group2() * Simd32x2::from(other.group0()[0]),
                g3: self.group3() * Simd32x3::from(other.group0()[0]),
                g4: self.group4() * Simd32x3::from(other.group0()[0]),
                g5: self.group5() * Simd32x4::from(other.group0()[0]),
                g6: self.group6() * Simd32x4::from(other.group0()[0]),
                g7: self.group7() * Simd32x3::from(other.group0()[0]),
                g8: self.group8() * Simd32x3::from(other.group0()[0]),
                g9: self.group9() * Simd32x3::from(other.group0()[0]),
                g10: self.group10() * Simd32x2::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + Simd32x2::from(self.group6()[3]) * Simd32x2::from([0.0, -other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - self.group1() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g9: Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0])
                    + self.group4() * Simd32x3::from(other.group0()[3]),
                g10: Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Flector> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group1()[3]])
                    + Simd32x2::from(self.group6()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + Simd32x2::from(self.group6()[3]) * Simd32x2::from([0.0, -other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - self.group1() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])
                    + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0])
                    + self.group4() * Simd32x3::from(other.group0()[3]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[3]])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Line> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[0]) * other.group0(),
                g8: Simd32x3::from(self.group0()[0]) * other.group1(),
                g9: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0])
                    + Simd32x3::from(self.group2()[0]) * other.group1(),
                g10: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl Wedge<Motor> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group1()[2]])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(self.group0()[0]) * other.group1(),
                g9: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0])
                    + Simd32x3::from(self.group2()[0]) * other.group1(),
                g10: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl Wedge<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * other.group0()
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group9()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group9()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group9()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group10()[1]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group10()[0]])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group8()[0]])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group8()[1]])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group8()[2]])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group7()[2]])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, -other.group6()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, -other.group6()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, -other.group6()[2]])
                    + Simd32x2::from(self.group5()[3]) * Simd32x2::from([0.0, -other.group6()[3]])
                    + Simd32x2::from(self.group6()[0]) * Simd32x2::from([0.0, -other.group5()[0]])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([0.0, -other.group5()[1]])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([0.0, -other.group5()[2]])
                    + Simd32x2::from(self.group6()[3]) * Simd32x2::from([0.0, -other.group5()[3]])
                    + Simd32x2::from(self.group7()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([0.0, -other.group4()[2]])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group3()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group3()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group3()[2]])
                    + Simd32x2::from(self.group9()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + Simd32x2::from(self.group10()[0]) * Simd32x2::from([0.0, other.group2()[1]])
                    + Simd32x2::from(self.group10()[1]) * Simd32x2::from([0.0, other.group2()[0]]),
                g1: Simd32x3::from(self.group0()[0]) * other.group1() + self.group1() * Simd32x3::from(other.group0()[0]),
                g2: Simd32x2::from(self.group0()[0]) * other.group2() + self.group2() * Simd32x2::from(other.group0()[0]),
                g3: Simd32x3::from(self.group0()[0]) * other.group3() - self.group1() * Simd32x3::from(other.group2()[0])
                    + Simd32x3::from(self.group2()[0]) * other.group1()
                    + self.group3() * Simd32x3::from(other.group0()[0]),
                g4: Simd32x3::from(self.group0()[0]) * other.group4()
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0])
                    + self.group4() * Simd32x3::from(other.group0()[0]),
                g5: Simd32x4::from(self.group0()[0]) * other.group5()
                    + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]])
                        * Simd32x4::from([other.group2()[1], other.group2()[1], other.group2()[1], 0.0])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group2()[1]])
                    - Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[0]])
                    + self.group5() * Simd32x4::from(other.group0()[0]),
                g6: Simd32x4::from(self.group0()[0]) * other.group6()
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group3()[2], -other.group3()[1], -other.group4()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group3()[2], 0.0, other.group3()[0], -other.group4()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], 0.0, -other.group4()[2]])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], 0.0])
                    + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, -other.group1()[2], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group1()[2], 0.0, -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group2()[0], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, other.group2()[0], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, other.group2()[0], -other.group1()[2]])
                    + self.group6() * Simd32x4::from(other.group0()[0]),
                g7: Simd32x3::from(self.group0()[0]) * other.group7() - self.group1() * Simd32x3::from(other.group5()[3])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group5()[0], other.group5()[1], other.group5()[2]])
                    + Simd32x3::from(self.group2()[1]) * other.group3()
                    + self.group3() * Simd32x3::from(other.group2()[1])
                    + Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]) * Simd32x3::from(other.group2()[0])
                    - Simd32x3::from(self.group5()[3]) * other.group1()
                    + self.group7() * Simd32x3::from(other.group0()[0]),
                g8: Simd32x3::from(self.group0()[0]) * other.group8()
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group5()[2], other.group5()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group5()[2], 0.0, -other.group5()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group5()[1], other.group5()[0], 0.0])
                    + Simd32x3::from(self.group2()[1]) * other.group4()
                    + self.group4() * Simd32x3::from(other.group2()[1])
                    + Simd32x3::from(self.group5()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group5()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    + self.group8() * Simd32x3::from(other.group0()[0]),
                g9: Simd32x3::from(self.group0()[0]) * other.group9()
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group7()[2], -other.group7()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group7()[2], 0.0, other.group7()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group7()[1], -other.group7()[0], 0.0])
                    + Simd32x3::from(self.group2()[0]) * other.group8()
                    - Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]])
                    + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group5()[2], other.group5()[1]])
                    + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group5()[2], 0.0, -other.group5()[0]])
                    + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group5()[1], other.group5()[0], 0.0])
                    + self.group4() * Simd32x3::from(other.group5()[3])
                    + Simd32x3::from(self.group5()[0]) * Simd32x3::from([0.0, other.group3()[2], -other.group3()[1]])
                    + Simd32x3::from(self.group5()[1]) * Simd32x3::from([-other.group3()[2], 0.0, other.group3()[0]])
                    + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], 0.0])
                    + Simd32x3::from(self.group5()[3]) * other.group4()
                    + Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group2()[1])
                    + Simd32x3::from(self.group7()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group7()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group7()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    - self.group8() * Simd32x3::from(other.group2()[0])
                    + self.group9() * Simd32x3::from(other.group0()[0]),
                g10: Simd32x2::from(self.group0()[0]) * other.group10()
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group6()[0], -other.group8()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group6()[1], -other.group8()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group6()[2], -other.group8()[2]])
                    + self.group2() * Simd32x2::from(other.group6()[3])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group4()[2], 0.0])
                    - Simd32x2::from(self.group4()[0]) * Simd32x2::from([other.group3()[0], other.group5()[0]])
                    - Simd32x2::from(self.group4()[1]) * Simd32x2::from([other.group3()[1], other.group5()[1]])
                    - Simd32x2::from(self.group4()[2]) * Simd32x2::from([other.group3()[2], other.group5()[2]])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, -other.group4()[2]])
                    + Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group1()[2], 0.0])
                    + Simd32x2::from(self.group6()[3]) * other.group2() * Simd32x2::from([-1.0, 1.0])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + self.group10() * Simd32x2::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<Plane> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group9()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group10()[0]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group10()[1]) * Simd32x2::from([0.0, other.group1()[0]]),
                g1: Simd32x3::from(self.group0()[0]) * other.group0(),
                g2: Simd32x2::from(self.group0()[0]) * other.group1(),
                g3: Simd32x3::from(0.0) - self.group1() * Simd32x3::from(other.group1()[0]) + Simd32x3::from(self.group2()[0]) * other.group0(),
                g4: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g5: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]])
                    * Simd32x4::from([other.group1()[1], other.group1()[1], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]])
                    - Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]),
                g6: Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group1()[0], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, other.group1()[0], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[0], -other.group0()[2]]),
                g7: self.group3() * Simd32x3::from(other.group1()[1]) + Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]) * Simd32x3::from(other.group1()[0])
                    - Simd32x3::from(self.group5()[3]) * other.group0(),
                g8: self.group4() * Simd32x3::from(other.group1()[1])
                    + Simd32x3::from(self.group5()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group5()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g9: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group1()[1])
                    + Simd32x3::from(self.group7()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group7()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group7()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0])
                    - self.group8() * Simd32x3::from(other.group1()[0]),
                g10: Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group6()[3]) * other.group1() * Simd32x2::from([-1.0, 1.0])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Scalar> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x2::from(other.group0()),
                g3: self.group3() * Simd32x3::from(other.group0()),
                g4: self.group4() * Simd32x3::from(other.group0()),
                g5: self.group5() * Simd32x4::from(other.group0()),
                g6: self.group6() * Simd32x4::from(other.group0()),
                g7: self.group7() * Simd32x3::from(other.group0()),
                g8: self.group8() * Simd32x3::from(other.group0()),
                g9: self.group9() * Simd32x3::from(other.group0()),
                g10: self.group10() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Wedge<Sphere> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group1()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[0]) * other.group0(),
                g10: Simd32x2::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Wedge<DualNum> for Plane {
    type Output = Plane;

    fn wedge(self, other: DualNum) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for Plane {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group2()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]),
                g10: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for Plane {
    type Output = AntiScalar;

    fn wedge(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl Wedge<Scalar> for Plane {
    type Output = Plane;

    fn wedge(self, other: Scalar) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Wedge<Circle> for RoundPoint {
    type Output = Sphere;

    fn wedge(self, other: Circle) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0])
                    + Simd32x3::from(self.group1()[0]) * other.group2()
                    - Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], -other.group2()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], -other.group2()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], -other.group2()[2]])
                    + self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl Wedge<Dipole> for RoundPoint {
    type Output = Circle;

    fn wedge(self, other: Dipole) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
                g1: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group2()[3])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])
                    + Simd32x3::from(self.group1()[1]) * other.group0(),
                g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group2()[2], other.group2()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], 0.0, -other.group2()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], 0.0])
                    + Simd32x3::from(self.group1()[1]) * other.group1(),
            },
        }
    }
}

impl Wedge<DualNum> for RoundPoint {
    type Output = RoundPoint;

    fn wedge(self, other: DualNum) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x2::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<FlatPoint> for RoundPoint {
    type Output = Line;

    fn wedge(self, other: FlatPoint) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Flector> for RoundPoint {
    type Output = Motor;

    fn wedge(self, other: Flector) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group0()[3], 0.0, 0.0, other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, -other.group0()[3], 0.0, other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, -other.group0()[3], other.group1()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Line> for RoundPoint {
    type Output = Plane;

    fn wedge(self, other: Line) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            },
        }
    }
}

impl Wedge<Motor> for RoundPoint {
    type Output = Plane;

    fn wedge(self, other: Motor) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            },
        }
    }
}

impl Wedge<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group9()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group9()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group9()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group10()[1]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group10()[0]]),
                g1: self.group0() * Simd32x3::from(other.group0()[0]),
                g2: self.group1() * Simd32x2::from(other.group0()[0]),
                g3: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group2()[0]) + Simd32x3::from(self.group1()[0]) * other.group1(),
                g4: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]),
                g5: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group2()[1], other.group2()[1], other.group2()[1], 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group2()[1]])
                    - Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[0]]),
                g6: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group3()[2], -other.group3()[1], -other.group4()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], 0.0, other.group3()[0], -other.group4()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], 0.0, -other.group4()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], 0.0]),
                g7: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group5()[3])
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group5()[0], other.group5()[1], other.group5()[2]])
                    + Simd32x3::from(self.group1()[1]) * other.group3(),
                g8: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group5()[2], other.group5()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group5()[2], 0.0, -other.group5()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group5()[1], other.group5()[0], 0.0])
                    + Simd32x3::from(self.group1()[1]) * other.group4(),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group7()[2], -other.group7()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group7()[2], 0.0, other.group7()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group7()[1], -other.group7()[0], 0.0])
                    + Simd32x3::from(self.group1()[0]) * other.group8()
                    - Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group6()[0], -other.group8()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group6()[1], -other.group8()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group6()[2], -other.group8()[2]])
                    + self.group1() * Simd32x2::from(other.group6()[3]),
            },
        }
    }
}

impl Wedge<Plane> for RoundPoint {
    type Output = AntiScalar;

    fn wedge(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl Wedge<RoundPoint> for RoundPoint {
    type Output = Dipole;

    fn wedge(self, other: RoundPoint) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group1()[0]) + Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g2: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group1()[1], other.group1()[1], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]])
                    - Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]),
            },
        }
    }
}

impl Wedge<Scalar> for RoundPoint {
    type Output = RoundPoint;

    fn wedge(self, other: Scalar) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Wedge<Sphere> for RoundPoint {
    type Output = AntiScalar;

    fn wedge(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    + self.group1()[0] * other.group1()[1]
                    + self.group1()[1] * other.group1()[0],
            },
        }
    }
}

impl Wedge<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn wedge(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Wedge<Circle> for Scalar {
    type Output = Circle;

    fn wedge(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x3::from(self.group0()) * other.group2(),
            },
        }
    }
}

impl Wedge<Dipole> for Scalar {
    type Output = Dipole;

    fn wedge(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x4::from(self.group0()) * other.group2(),
            },
        }
    }
}

impl Wedge<DualNum> for Scalar {
    type Output = DualNum;

    fn wedge(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn wedge(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<Flector> for Scalar {
    type Output = Flector;

    fn wedge(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x4::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Wedge<Line> for Scalar {
    type Output = Line;

    fn wedge(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Wedge<Motor> for Scalar {
    type Output = Motor;

    fn wedge(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Wedge<MultiVector> for Scalar {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x2::from(self.group0()) * other.group2(),
                g3: Simd32x3::from(self.group0()) * other.group3(),
                g4: Simd32x3::from(self.group0()) * other.group4(),
                g5: Simd32x4::from(self.group0()) * other.group5(),
                g6: Simd32x4::from(self.group0()) * other.group6(),
                g7: Simd32x3::from(self.group0()) * other.group7(),
                g8: Simd32x3::from(self.group0()) * other.group8(),
                g9: Simd32x3::from(self.group0()) * other.group9(),
                g10: Simd32x2::from(self.group0()) * other.group10(),
            },
        }
    }
}

impl Wedge<Plane> for Scalar {
    type Output = Plane;

    fn wedge(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<RoundPoint> for Scalar {
    type Output = RoundPoint;

    fn wedge(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Wedge<Scalar> for Scalar {
    type Output = Scalar;

    fn wedge(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Wedge<Sphere> for Scalar {
    type Output = Sphere;

    fn wedge(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Wedge<DualNum> for Sphere {
    type Output = Sphere;

    fn wedge(self, other: DualNum) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x2::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for Sphere {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]])
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group2()[1]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group2()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0() * Simd32x3::from(other.group0()[0]),
                g10: self.group1() * Simd32x2::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<RoundPoint> for Sphere {
    type Output = AntiScalar;

    fn wedge(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    + self.group1()[0] * other.group1()[1]
                    + self.group1()[1] * other.group1()[0],
            },
        }
    }
}

impl Wedge<Scalar> for Sphere {
    type Output = Sphere;

    fn wedge(self, other: Scalar) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}
