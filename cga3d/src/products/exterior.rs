//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::*;

/// Exterior Product
/// Synonyms included: Wedge, Join
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products
pub trait Wedge<T> {
    type Output;
    fn wedge(self, other: T) -> Self::Output;
}

/// Exterior Product
/// Synonyms included: Wedge, Join
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products
pub trait Join<T> {
    type Output;
    fn join(self, other: T) -> Self::Output;
}

/// Geometric Anti-Product
/// Synonyms included: AntiWedge, Meet
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products
pub trait AntiWedge<T> {
    type Output;
    fn anti_wedge(self, other: T) -> Self::Output;
}

/// Geometric Anti-Product
/// Synonyms included: AntiWedge, Meet
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products
pub trait Meet<T> {
    type Output;
    fn meet(self, other: T) -> Self::Output;
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

impl AntiWedge<Horizon> for AntiScalar {
    type Output = Horizon;

    fn anti_wedge(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0(),
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

impl AntiWedge<LineAtInfinity> for AntiScalar {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn anti_wedge(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Magnitude> for AntiScalar {
    type Output = Magnitude;

    fn anti_wedge(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from(self.group0()) * other.group0(),
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

impl AntiWedge<Origin> for AntiScalar {
    type Output = Origin;

    fn anti_wedge(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0(),
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

impl AntiWedge<PlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn anti_wedge(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Point> for AntiScalar {
    type Output = Point;

    fn anti_wedge(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for AntiScalar {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Radial> for AntiScalar {
    type Output = Radial;

    fn anti_wedge(self, other: Radial) -> Radial {
        Radial {
            groups: RadialGroups {
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
    type Output = Radial;

    fn anti_wedge(self, other: Circle) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl AntiWedge<Horizon> for Circle {
    type Output = Dipole;

    fn anti_wedge(self, other: Horizon) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()),
                g2: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]])
                    * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]),
            },
        }
    }
}

impl AntiWedge<Line> for Circle {
    type Output = Radial;

    fn anti_wedge(self, other: Line) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl AntiWedge<LineAtInfinity> for Circle {
    type Output = Radial;

    fn anti_wedge(self, other: LineAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Circle {
    type Output = Radial;

    fn anti_wedge(self, other: LineAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Circle {
    type Output = Circle;

    fn anti_wedge(self, other: Magnitude) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
                g2: self.group2() * Simd32x3::from(other.group0()[1]),
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

impl AntiWedge<Origin> for Circle {
    type Output = Scalar;

    fn anti_wedge(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
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

impl AntiWedge<PlaneAtOrigin> for Circle {
    type Output = Dipole;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0(),
                g2: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl AntiWedge<Point> for Circle {
    type Output = Scalar;

    fn anti_wedge(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for Circle {
    type Output = Scalar;

    fn anti_wedge(self, other: PointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl AntiWedge<Horizon> for Dipole {
    type Output = Radial;

    fn anti_wedge(self, other: Horizon) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: Simd32x2::from(self.group2()[3]) * Simd32x2::from([0.0, other.group0()]),
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

impl AntiWedge<LineAtInfinity> for Dipole {
    type Output = Scalar;

    fn anti_wedge(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Dipole {
    type Output = Scalar;

    fn anti_wedge(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Magnitude> for Dipole {
    type Output = Dipole;

    fn anti_wedge(self, other: Magnitude) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
                g2: self.group2() * Simd32x4::from(other.group0()[1]),
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
    type Output = Radial;

    fn anti_wedge(self, other: Plane) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl AntiWedge<PlaneAtOrigin> for Dipole {
    type Output = Radial;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Sphere> for Dipole {
    type Output = Radial;

    fn anti_wedge(self, other: Sphere) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl AntiWedge<AntiScalar> for Horizon {
    type Output = Horizon;

    fn anti_wedge(self, other: AntiScalar) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiWedge<Circle> for Horizon {
    type Output = Dipole;

    fn anti_wedge(self, other: Circle) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Dipole> for Horizon {
    type Output = Radial;

    fn anti_wedge(self, other: Dipole) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group2()[3]]),
            },
        }
    }
}

impl AntiWedge<Line> for Horizon {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: Line) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Horizon {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: LineAtOrigin) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Horizon {
    type Output = Horizon;

    fn anti_wedge(self, other: Magnitude) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl AntiWedge<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group2()[0], 0.0]),
                g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group3(),
                g2: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group5()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(self.group0()) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]),
                g5: Simd32x4::from(self.group0()) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], 0.0]),
                g6: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group10()[0]]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group9(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[1]]),
            },
        }
    }
}

impl AntiWedge<Origin> for Horizon {
    type Output = Radial;

    fn anti_wedge(self, other: Origin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group0()]),
            },
        }
    }
}

impl AntiWedge<Plane> for Horizon {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: Plane) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: PlaneAtOrigin) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Point> for Horizon {
    type Output = Radial;

    fn anti_wedge(self, other: Point) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<Radial> for Horizon {
    type Output = Scalar;

    fn anti_wedge(self, other: Radial) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group1()[0],
            },
        }
    }
}

impl AntiWedge<Sphere> for Horizon {
    type Output = Circle;

    fn anti_wedge(self, other: Sphere) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
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
    type Output = Radial;

    fn anti_wedge(self, other: Circle) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl AntiWedge<Horizon> for Line {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Line> for Line {
    type Output = Radial;

    fn anti_wedge(self, other: Line) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl AntiWedge<LineAtInfinity> for Line {
    type Output = Radial;

    fn anti_wedge(self, other: LineAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Line {
    type Output = Radial;

    fn anti_wedge(self, other: LineAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Line {
    type Output = Line;

    fn anti_wedge(self, other: Magnitude) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
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
    type Output = Point;

    fn anti_wedge(self, other: Plane) -> Point {
        Point {
            groups: PointGroups {
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

impl AntiWedge<PlaneAtOrigin> for Line {
    type Output = Point;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
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

impl AntiWedge<AntiScalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: AntiScalar) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for LineAtInfinity {
    type Output = Radial;

    fn anti_wedge(self, other: Circle) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl AntiWedge<Dipole> for LineAtInfinity {
    type Output = Scalar;

    fn anti_wedge(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Line> for LineAtInfinity {
    type Output = Radial;

    fn anti_wedge(self, other: Line) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for LineAtInfinity {
    type Output = Radial;

    fn anti_wedge(self, other: LineAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Magnitude> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group3()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group3()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group3()[2], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group7()[2]]),
                g3: Simd32x3::from(0.0),
                g4: self.group0() * Simd32x3::from(other.group10()[0]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group9()[2], other.group9()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group9()[2], 0.0, -other.group9()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group9()[1], other.group9()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x3::from(other.group0()[1]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: Plane) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: PlaneAtOrigin) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
            },
        }
    }
}

impl AntiWedge<Sphere> for LineAtInfinity {
    type Output = Dipole;

    fn anti_wedge(self, other: Sphere) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group0() * Simd32x3::from(other.group1()[0]),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_wedge(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for LineAtOrigin {
    type Output = Radial;

    fn anti_wedge(self, other: Circle) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
                g1: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]),
            },
        }
    }
}

impl AntiWedge<Dipole> for LineAtOrigin {
    type Output = Scalar;

    fn anti_wedge(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiWedge<Horizon> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Line> for LineAtOrigin {
    type Output = Radial;

    fn anti_wedge(self, other: Line) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl AntiWedge<LineAtInfinity> for LineAtOrigin {
    type Output = Radial;

    fn anti_wedge(self, other: LineAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Magnitude> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_wedge(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group4()[2], 0.0]),
                g1: self.group0() * Simd32x3::from(other.group6()[3]),
                g2: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group6()[0], other.group8()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group6()[1], other.group8()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group6()[2], other.group8()[2]]),
                g3: self.group0() * Simd32x3::from(other.group10()[0]),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group10()[1], 0.0, 0.0, -other.group9()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group10()[1], 0.0, -other.group9()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group10()[1], -other.group9()[2]]),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[1]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for LineAtOrigin {
    type Output = Point;

    fn anti_wedge(self, other: Plane) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for LineAtOrigin {
    type Output = Origin;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Sphere> for LineAtOrigin {
    type Output = Dipole;

    fn anti_wedge(self, other: Sphere) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[1], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[1], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[1], -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Magnitude {
    type Output = Magnitude;

    fn anti_wedge(self, other: AntiScalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Magnitude {
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

impl AntiWedge<Dipole> for Magnitude {
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

impl AntiWedge<Horizon> for Magnitude {
    type Output = Horizon;

    fn anti_wedge(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Line> for Magnitude {
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

impl AntiWedge<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn anti_wedge(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn anti_wedge(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Magnitude {
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

impl AntiWedge<Origin> for Magnitude {
    type Output = Origin;

    fn anti_wedge(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Plane> for Magnitude {
    type Output = Plane;

    fn anti_wedge(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn anti_wedge(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Point> for Magnitude {
    type Output = Point;

    fn anti_wedge(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for Magnitude {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Radial> for Magnitude {
    type Output = Radial;

    fn anti_wedge(self, other: Radial) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
                g1: Simd32x2::from(self.group0()[1]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Scalar> for Magnitude {
    type Output = Scalar;

    fn anti_wedge(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Sphere> for Magnitude {
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

impl AntiWedge<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Horizon) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group0(), 0.0]),
                g1: self.group3() * Simd32x3::from(other.group0()),
                g2: Simd32x2::from(self.group5()[3]) * Simd32x2::from([0.0, other.group0()]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group0()),
                g5: Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group7()[0]])
                    * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]),
                g6: Simd32x4::from(self.group10()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g7: Simd32x3::from(0.0),
                g8: self.group9() * Simd32x3::from(other.group0()),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()]),
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

impl AntiWedge<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group7()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(self.group10()[0]) * other.group0(),
                g5: Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[1]) * other.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group6()[3]) * other.group0(),
                g2: Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(self.group10()[0]) * other.group0(),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[1]) * other.group0(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Magnitude) -> MultiVector {
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

impl AntiWedge<Origin> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Origin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[3]) * Simd32x2::from([-other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: self.group10() * Simd32x2::from(other.group0()),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
                g3: Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g4: Simd32x3::from(0.0) - Simd32x3::from(self.group6()[3]) * other.group0(),
                g5: Simd32x4::from(self.group7()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group7()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group7()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group8()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group8()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(self.group10()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g7: Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g8: Simd32x3::from(0.0) - Simd32x3::from(self.group10()[1]) * other.group0(),
                g9: Simd32x3::from(self.group0()[1]) * other.group0(),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Point> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Point) -> MultiVector {
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

impl AntiWedge<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: PointAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group10()[0]) * other.group0(),
                g2: Simd32x2::from(self.group9()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Radial> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Radial) -> MultiVector {
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

impl AntiWedge<AntiScalar> for Origin {
    type Output = Origin;

    fn anti_wedge(self, other: AntiScalar) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiWedge<Circle> for Origin {
    type Output = Scalar;

    fn anti_wedge(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<Horizon> for Origin {
    type Output = Radial;

    fn anti_wedge(self, other: Horizon) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Origin {
    type Output = Origin;

    fn anti_wedge(self, other: Magnitude) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl AntiWedge<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([-other.group6()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()) * other.group10() * Simd32x2::from([-1.0, 1.0]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for Origin {
    type Output = Radial;

    fn anti_wedge(self, other: Plane) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<Sphere> for Origin {
    type Output = Radial;

    fn anti_wedge(self, other: Sphere) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * other.group1() * Simd32x2::from([-1.0, 1.0]),
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
    type Output = Radial;

    fn anti_wedge(self, other: Dipole) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl AntiWedge<Horizon> for Plane {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Line> for Plane {
    type Output = Point;

    fn anti_wedge(self, other: Line) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<LineAtInfinity> for Plane {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Plane {
    type Output = Point;

    fn anti_wedge(self, other: LineAtOrigin) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Plane {
    type Output = Plane;

    fn anti_wedge(self, other: Magnitude) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
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

impl AntiWedge<Origin> for Plane {
    type Output = Radial;

    fn anti_wedge(self, other: Origin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()]),
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

impl AntiWedge<PlaneAtOrigin> for Plane {
    type Output = Line;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Point> for Plane {
    type Output = Radial;

    fn anti_wedge(self, other: Point) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for Plane {
    type Output = Radial;

    fn anti_wedge(self, other: PointAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Radial> for Plane {
    type Output = Scalar;

    fn anti_wedge(self, other: Radial) -> Scalar {
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

impl AntiWedge<AntiScalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_wedge(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for PlaneAtOrigin {
    type Output = Dipole;

    fn anti_wedge(self, other: Circle) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group2()[2], -other.group2()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], 0.0, other.group2()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], 0.0, -other.group1()[2]]),
            },
        }
    }
}

impl AntiWedge<Dipole> for PlaneAtOrigin {
    type Output = Radial;

    fn anti_wedge(self, other: Dipole) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], -other.group2()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], -other.group2()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], -other.group2()[2]]),
            },
        }
    }
}

impl AntiWedge<Horizon> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Line> for PlaneAtOrigin {
    type Output = Point;

    fn anti_wedge(self, other: Line) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<LineAtInfinity> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for PlaneAtOrigin {
    type Output = Origin;

    fn anti_wedge(self, other: LineAtOrigin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Magnitude> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_wedge(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group4()[2], other.group4()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], 0.0, -other.group4()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], 0.0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], -other.group5()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], -other.group5()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], -other.group5()[2]]),
                g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0]),
                g4: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group6()[3]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group8()[2], -other.group8()[1], -other.group7()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group8()[2], 0.0, other.group8()[0], -other.group7()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group8()[1], -other.group8()[0], 0.0, -other.group7()[2]]),
                g6: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([-other.group10()[0], -other.group10()[0], -other.group10()[0], 0.0]),
                g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group9()[2], -other.group9()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group9()[2], 0.0, other.group9()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group9()[1], -other.group9()[0], 0.0]),
                g8: self.group0() * Simd32x3::from(other.group10()[1]),
                g9: self.group0() * Simd32x3::from(other.group0()[1]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for PlaneAtOrigin {
    type Output = Line;

    fn anti_wedge(self, other: Plane) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: self.group0() * Simd32x3::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn anti_wedge(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl AntiWedge<Point> for PlaneAtOrigin {
    type Output = Radial;

    fn anti_wedge(self, other: Point) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for PlaneAtOrigin {
    type Output = Radial;

    fn anti_wedge(self, other: PointAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Radial> for PlaneAtOrigin {
    type Output = Scalar;

    fn anti_wedge(self, other: Radial) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Sphere> for PlaneAtOrigin {
    type Output = Circle;

    fn anti_wedge(self, other: Sphere) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([-other.group1()[0], -other.group1()[0], -other.group1()[0], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: self.group0() * Simd32x3::from(other.group1()[1]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Point {
    type Output = Point;

    fn anti_wedge(self, other: AntiScalar) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Point {
    type Output = Scalar;

    fn anti_wedge(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<Horizon> for Point {
    type Output = Radial;

    fn anti_wedge(self, other: Horizon) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Point {
    type Output = Point;

    fn anti_wedge(self, other: Magnitude) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Point {
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

impl AntiWedge<Plane> for Point {
    type Output = Radial;

    fn anti_wedge(self, other: Plane) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for Point {
    type Output = Radial;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Sphere> for Point {
    type Output = Radial;

    fn anti_wedge(self, other: Sphere) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group1() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: AntiScalar) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for PointAtInfinity {
    type Output = Scalar;

    fn anti_wedge(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Magnitude> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group6()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group6()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group6()[2], 0.0]),
                g1: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group10()[0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group9()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group9()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group9()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Plane> for PointAtInfinity {
    type Output = Radial;

    fn anti_wedge(self, other: Plane) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for PointAtInfinity {
    type Output = Radial;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Sphere> for PointAtInfinity {
    type Output = Radial;

    fn anti_wedge(self, other: Sphere) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Radial {
    type Output = Radial;

    fn anti_wedge(self, other: AntiScalar) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Horizon> for Radial {
    type Output = Scalar;

    fn anti_wedge(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Radial {
    type Output = Radial;

    fn anti_wedge(self, other: Magnitude) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x2::from(other.group0()[1]),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Radial {
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

impl AntiWedge<Plane> for Radial {
    type Output = Scalar;

    fn anti_wedge(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for Radial {
    type Output = Scalar;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Sphere> for Radial {
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

impl AntiWedge<Magnitude> for Scalar {
    type Output = Scalar;

    fn anti_wedge(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[1],
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
    type Output = Radial;

    fn anti_wedge(self, other: Dipole) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl AntiWedge<Horizon> for Sphere {
    type Output = Circle;

    fn anti_wedge(self, other: Horizon) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: self.group0() * Simd32x3::from(other.group0()),
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

impl AntiWedge<LineAtInfinity> for Sphere {
    type Output = Dipole;

    fn anti_wedge(self, other: LineAtInfinity) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(self.group1()[0]) * other.group0(),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Sphere {
    type Output = Dipole;

    fn anti_wedge(self, other: LineAtOrigin) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Sphere {
    type Output = Sphere;

    fn anti_wedge(self, other: Magnitude) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x2::from(other.group0()[1]),
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

impl AntiWedge<Origin> for Sphere {
    type Output = Radial;

    fn anti_wedge(self, other: Origin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1() * Simd32x2::from(other.group0()),
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

impl AntiWedge<PlaneAtOrigin> for Sphere {
    type Output = Circle;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[1]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Point> for Sphere {
    type Output = Radial;

    fn anti_wedge(self, other: Point) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for Sphere {
    type Output = Radial;

    fn anti_wedge(self, other: PointAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Radial> for Sphere {
    type Output = Scalar;

    fn anti_wedge(self, other: Radial) -> Scalar {
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

impl Join<Magnitude> for AntiScalar {
    type Output = AntiScalar;

    fn join(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Join<MultiVector> for AntiScalar {
    type Output = AntiScalar;

    fn join(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Join<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn join(self, other: Scalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Join<Dipole> for Circle {
    type Output = AntiScalar;

    fn join(self, other: Dipole) -> AntiScalar {
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

impl Join<Magnitude> for Circle {
    type Output = Circle;

    fn join(self, other: Magnitude) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
                g2: self.group2() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Circle {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Origin> for Circle {
    type Output = AntiScalar;

    fn join(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Join<Point> for Circle {
    type Output = AntiScalar;

    fn join(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Join<PointAtInfinity> for Circle {
    type Output = AntiScalar;

    fn join(self, other: PointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<Radial> for Circle {
    type Output = Sphere;

    fn join(self, other: Radial) -> Sphere {
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

impl Join<Scalar> for Circle {
    type Output = Circle;

    fn join(self, other: Scalar) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Join<Circle> for Dipole {
    type Output = AntiScalar;

    fn join(self, other: Circle) -> AntiScalar {
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

impl Join<Dipole> for Dipole {
    type Output = Sphere;

    fn join(self, other: Dipole) -> Sphere {
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

impl Join<Line> for Dipole {
    type Output = AntiScalar;

    fn join(self, other: Line) -> AntiScalar {
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

impl Join<LineAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn join(self, other: LineAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<LineAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn join(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<Magnitude> for Dipole {
    type Output = Dipole;

    fn join(self, other: Magnitude) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
                g2: self.group2() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Dipole {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Origin> for Dipole {
    type Output = PlaneAtOrigin;

    fn join(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group1() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Join<Point> for Dipole {
    type Output = Plane;

    fn join(self, other: Point) -> Plane {
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

impl Join<PointAtInfinity> for Dipole {
    type Output = Plane;

    fn join(self, other: PointAtInfinity) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Join<Radial> for Dipole {
    type Output = Circle;

    fn join(self, other: Radial) -> Circle {
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

impl Join<Scalar> for Dipole {
    type Output = Dipole;

    fn join(self, other: Scalar) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Join<Magnitude> for Horizon {
    type Output = Horizon;

    fn join(self, other: Magnitude) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Join<MultiVector> for Horizon {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group2()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[0]]),
            },
        }
    }
}

impl Join<Radial> for Horizon {
    type Output = AntiScalar;

    fn join(self, other: Radial) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group1()[0],
            },
        }
    }
}

impl Join<Scalar> for Horizon {
    type Output = Horizon;

    fn join(self, other: Scalar) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Join<Dipole> for Line {
    type Output = AntiScalar;

    fn join(self, other: Dipole) -> AntiScalar {
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

impl Join<Magnitude> for Line {
    type Output = Line;

    fn join(self, other: Magnitude) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Line {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Radial> for Line {
    type Output = Plane;

    fn join(self, other: Radial) -> Plane {
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

impl Join<Scalar> for Line {
    type Output = Line;

    fn join(self, other: Scalar) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Join<Dipole> for LineAtInfinity {
    type Output = AntiScalar;

    fn join(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<Magnitude> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn join(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x3::from(other.group0()[0]),
                g9: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group2()[0]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]]),
            },
        }
    }
}

impl Join<Radial> for LineAtInfinity {
    type Output = Plane;

    fn join(self, other: Radial) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[0], 0.0, 0.0, other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, -other.group1()[0], 0.0, other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, -other.group1()[0], other.group0()[2]]),
            },
        }
    }
}

impl Join<Scalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn join(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Join<Dipole> for LineAtOrigin {
    type Output = AntiScalar;

    fn join(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Join<Magnitude> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn join(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[0]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Join<Radial> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn join(self, other: Radial) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Join<Scalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn join(self, other: Scalar) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Join<AntiScalar> for Magnitude {
    type Output = AntiScalar;

    fn join(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Join<Circle> for Magnitude {
    type Output = Circle;

    fn join(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * other.group1(),
                g2: Simd32x3::from(self.group0()[0]) * other.group2(),
            },
        }
    }
}

impl Join<Dipole> for Magnitude {
    type Output = Dipole;

    fn join(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * other.group1(),
                g2: Simd32x4::from(self.group0()[0]) * other.group2(),
            },
        }
    }
}

impl Join<Horizon> for Magnitude {
    type Output = Horizon;

    fn join(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Join<Line> for Magnitude {
    type Output = Line;

    fn join(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Join<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn join(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Join<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn join(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Join<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn join(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]),
            },
        }
    }
}

impl Join<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Origin> for Magnitude {
    type Output = Origin;

    fn join(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Join<Plane> for Magnitude {
    type Output = Plane;

    fn join(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Join<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn join(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Join<Point> for Magnitude {
    type Output = Point;

    fn join(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Join<PointAtInfinity> for Magnitude {
    type Output = PointAtInfinity;

    fn join(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Join<Radial> for Magnitude {
    type Output = Radial;

    fn join(self, other: Radial) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Join<Scalar> for Magnitude {
    type Output = Magnitude;

    fn join(self, other: Scalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Join<Sphere> for Magnitude {
    type Output = Sphere;

    fn join(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Join<AntiScalar> for MultiVector {
    type Output = AntiScalar;

    fn join(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Join<Circle> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Circle) -> MultiVector {
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

impl Join<Dipole> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Dipole) -> MultiVector {
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

impl Join<Horizon> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Horizon) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl Join<Line> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Line) -> MultiVector {
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

impl Join<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[0]) * other.group0(),
                g9: Simd32x3::from(self.group2()[0]) * other.group0(),
                g10: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Join<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[0]) * other.group0(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Join<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Magnitude) -> MultiVector {
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

impl Join<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Origin> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Origin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[3]) * Simd32x2::from([0.0, -other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - self.group1() * Simd32x3::from(other.group0()),
                g8: Simd32x3::from(0.0),
                g9: self.group4() * Simd32x3::from(other.group0()),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Join<Plane> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Plane) -> MultiVector {
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

impl Join<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[0]) * other.group0(),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Join<Point> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Point) -> MultiVector {
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

impl Join<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: PointAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group2()[0]) * other.group0(),
                g8: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g9: Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g10: Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Join<Radial> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Radial) -> MultiVector {
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

impl Join<Scalar> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Scalar) -> MultiVector {
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

impl Join<Sphere> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Sphere) -> MultiVector {
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

impl Join<Circle> for Origin {
    type Output = AntiScalar;

    fn join(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl Join<Dipole> for Origin {
    type Output = PlaneAtOrigin;

    fn join(self, other: Dipole) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Join<Magnitude> for Origin {
    type Output = Origin;

    fn join(self, other: Magnitude) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Join<MultiVector> for Origin {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group6()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group1(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()) * other.group4(),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Join<Radial> for Origin {
    type Output = LineAtOrigin;

    fn join(self, other: Radial) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<Scalar> for Origin {
    type Output = Origin;

    fn join(self, other: Scalar) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Join<Magnitude> for Plane {
    type Output = Plane;

    fn join(self, other: Magnitude) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Plane {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Radial> for Plane {
    type Output = AntiScalar;

    fn join(self, other: Radial) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl Join<Scalar> for Plane {
    type Output = Plane;

    fn join(self, other: Scalar) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Join<Magnitude> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn join(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0() * Simd32x3::from(other.group0()[0]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Join<Radial> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn join(self, other: Radial) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<Scalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn join(self, other: Scalar) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Join<Circle> for Point {
    type Output = AntiScalar;

    fn join(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Join<Dipole> for Point {
    type Output = Plane;

    fn join(self, other: Dipole) -> Plane {
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

impl Join<Magnitude> for Point {
    type Output = Point;

    fn join(self, other: Magnitude) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Point {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Radial> for Point {
    type Output = Line;

    fn join(self, other: Radial) -> Line {
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

impl Join<Scalar> for Point {
    type Output = Point;

    fn join(self, other: Scalar) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Join<Circle> for PointAtInfinity {
    type Output = AntiScalar;

    fn join(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<Dipole> for PointAtInfinity {
    type Output = Plane;

    fn join(self, other: Dipole) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]),
            },
        }
    }
}

impl Join<Magnitude> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn join(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group6()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group6()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group6()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group2()[0]),
                g8: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group3()[2], -other.group3()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], 0.0, other.group3()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], 0.0]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]]),
            },
        }
    }
}

impl Join<Radial> for PointAtInfinity {
    type Output = Line;

    fn join(self, other: Radial) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Join<Scalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn join(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Join<Circle> for Radial {
    type Output = Sphere;

    fn join(self, other: Circle) -> Sphere {
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

impl Join<Dipole> for Radial {
    type Output = Circle;

    fn join(self, other: Dipole) -> Circle {
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

impl Join<Horizon> for Radial {
    type Output = AntiScalar;

    fn join(self, other: Horizon) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl Join<Line> for Radial {
    type Output = Plane;

    fn join(self, other: Line) -> Plane {
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

impl Join<LineAtInfinity> for Radial {
    type Output = Plane;

    fn join(self, other: LineAtInfinity) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Join<LineAtOrigin> for Radial {
    type Output = PlaneAtOrigin;

    fn join(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Join<Magnitude> for Radial {
    type Output = Radial;

    fn join(self, other: Magnitude) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x2::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Radial {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Origin> for Radial {
    type Output = LineAtOrigin;

    fn join(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Join<Plane> for Radial {
    type Output = AntiScalar;

    fn join(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl Join<PlaneAtOrigin> for Radial {
    type Output = AntiScalar;

    fn join(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<Point> for Radial {
    type Output = Line;

    fn join(self, other: Point) -> Line {
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

impl Join<PointAtInfinity> for Radial {
    type Output = Line;

    fn join(self, other: PointAtInfinity) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
            },
        }
    }
}

impl Join<Radial> for Radial {
    type Output = Dipole;

    fn join(self, other: Radial) -> Dipole {
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

impl Join<Scalar> for Radial {
    type Output = Radial;

    fn join(self, other: Scalar) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Join<Sphere> for Radial {
    type Output = AntiScalar;

    fn join(self, other: Sphere) -> AntiScalar {
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

impl Join<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn join(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Join<Circle> for Scalar {
    type Output = Circle;

    fn join(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x3::from(self.group0()) * other.group2(),
            },
        }
    }
}

impl Join<Dipole> for Scalar {
    type Output = Dipole;

    fn join(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x4::from(self.group0()) * other.group2(),
            },
        }
    }
}

impl Join<Horizon> for Scalar {
    type Output = Horizon;

    fn join(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Join<Line> for Scalar {
    type Output = Line;

    fn join(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Join<LineAtInfinity> for Scalar {
    type Output = LineAtInfinity;

    fn join(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<LineAtOrigin> for Scalar {
    type Output = LineAtOrigin;

    fn join(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<Magnitude> for Scalar {
    type Output = Magnitude;

    fn join(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<MultiVector> for Scalar {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Origin> for Scalar {
    type Output = Origin;

    fn join(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Join<Plane> for Scalar {
    type Output = Plane;

    fn join(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<PlaneAtOrigin> for Scalar {
    type Output = PlaneAtOrigin;

    fn join(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<Point> for Scalar {
    type Output = Point;

    fn join(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<PointAtInfinity> for Scalar {
    type Output = PointAtInfinity;

    fn join(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<Radial> for Scalar {
    type Output = Radial;

    fn join(self, other: Radial) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Join<Scalar> for Scalar {
    type Output = Scalar;

    fn join(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Join<Sphere> for Scalar {
    type Output = Sphere;

    fn join(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Join<Magnitude> for Sphere {
    type Output = Sphere;

    fn join(self, other: Magnitude) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x2::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Sphere {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<Radial> for Sphere {
    type Output = AntiScalar;

    fn join(self, other: Radial) -> AntiScalar {
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

impl Join<Scalar> for Sphere {
    type Output = Sphere;

    fn join(self, other: Scalar) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Meet<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn meet(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Meet<Circle> for AntiScalar {
    type Output = Circle;

    fn meet(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x3::from(self.group0()) * other.group2(),
            },
        }
    }
}

impl Meet<Dipole> for AntiScalar {
    type Output = Dipole;

    fn meet(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
                g2: Simd32x4::from(self.group0()) * other.group2(),
            },
        }
    }
}

impl Meet<Horizon> for AntiScalar {
    type Output = Horizon;

    fn meet(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Meet<Line> for AntiScalar {
    type Output = Line;

    fn meet(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Meet<LineAtInfinity> for AntiScalar {
    type Output = LineAtInfinity;

    fn meet(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<LineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn meet(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<Magnitude> for AntiScalar {
    type Output = Magnitude;

    fn meet(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Origin> for AntiScalar {
    type Output = Origin;

    fn meet(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Meet<Plane> for AntiScalar {
    type Output = Plane;

    fn meet(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn meet(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<Point> for AntiScalar {
    type Output = Point;

    fn meet(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<PointAtInfinity> for AntiScalar {
    type Output = PointAtInfinity;

    fn meet(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<Radial> for AntiScalar {
    type Output = Radial;

    fn meet(self, other: Radial) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Meet<Scalar> for AntiScalar {
    type Output = Scalar;

    fn meet(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Meet<Sphere> for AntiScalar {
    type Output = Sphere;

    fn meet(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Meet<AntiScalar> for Circle {
    type Output = Circle;

    fn meet(self, other: AntiScalar) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Circle {
    type Output = Radial;

    fn meet(self, other: Circle) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl Meet<Dipole> for Circle {
    type Output = Scalar;

    fn meet(self, other: Dipole) -> Scalar {
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

impl Meet<Horizon> for Circle {
    type Output = Dipole;

    fn meet(self, other: Horizon) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()),
                g2: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]])
                    * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]),
            },
        }
    }
}

impl Meet<Line> for Circle {
    type Output = Radial;

    fn meet(self, other: Line) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl Meet<LineAtInfinity> for Circle {
    type Output = Radial;

    fn meet(self, other: LineAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Circle {
    type Output = Radial;

    fn meet(self, other: LineAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Magnitude> for Circle {
    type Output = Circle;

    fn meet(self, other: Magnitude) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
                g2: self.group2() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for Circle {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Origin> for Circle {
    type Output = Scalar;

    fn meet(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Plane> for Circle {
    type Output = Dipole;

    fn meet(self, other: Plane) -> Dipole {
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

impl Meet<PlaneAtOrigin> for Circle {
    type Output = Dipole;

    fn meet(self, other: PlaneAtOrigin) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0(),
                g2: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl Meet<Point> for Circle {
    type Output = Scalar;

    fn meet(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Meet<PointAtInfinity> for Circle {
    type Output = Scalar;

    fn meet(self, other: PointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Sphere> for Circle {
    type Output = Dipole;

    fn meet(self, other: Sphere) -> Dipole {
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

impl Meet<AntiScalar> for Dipole {
    type Output = Dipole;

    fn meet(self, other: AntiScalar) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
                g2: self.group2() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Dipole {
    type Output = Scalar;

    fn meet(self, other: Circle) -> Scalar {
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

impl Meet<Horizon> for Dipole {
    type Output = Radial;

    fn meet(self, other: Horizon) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: Simd32x2::from(self.group2()[3]) * Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl Meet<Line> for Dipole {
    type Output = Scalar;

    fn meet(self, other: Line) -> Scalar {
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

impl Meet<LineAtInfinity> for Dipole {
    type Output = Scalar;

    fn meet(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<LineAtOrigin> for Dipole {
    type Output = Scalar;

    fn meet(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Magnitude> for Dipole {
    type Output = Dipole;

    fn meet(self, other: Magnitude) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
                g2: self.group2() * Simd32x4::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for Dipole {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Plane> for Dipole {
    type Output = Radial;

    fn meet(self, other: Plane) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl Meet<PlaneAtOrigin> for Dipole {
    type Output = Radial;

    fn meet(self, other: PlaneAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl Meet<Sphere> for Dipole {
    type Output = Radial;

    fn meet(self, other: Sphere) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl Meet<AntiScalar> for Horizon {
    type Output = Horizon;

    fn meet(self, other: AntiScalar) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Meet<Circle> for Horizon {
    type Output = Dipole;

    fn meet(self, other: Circle) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            },
        }
    }
}

impl Meet<Dipole> for Horizon {
    type Output = Radial;

    fn meet(self, other: Dipole) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group2()[3]]),
            },
        }
    }
}

impl Meet<Line> for Horizon {
    type Output = PointAtInfinity;

    fn meet(self, other: Line) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Horizon {
    type Output = PointAtInfinity;

    fn meet(self, other: LineAtOrigin) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<Magnitude> for Horizon {
    type Output = Horizon;

    fn meet(self, other: Magnitude) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl Meet<MultiVector> for Horizon {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group2()[0], 0.0]),
                g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group3(),
                g2: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group5()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(self.group0()) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]),
                g5: Simd32x4::from(self.group0()) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], 0.0]),
                g6: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group10()[0]]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group9(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[1]]),
            },
        }
    }
}

impl Meet<Origin> for Horizon {
    type Output = Radial;

    fn meet(self, other: Origin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group0()]),
            },
        }
    }
}

impl Meet<Plane> for Horizon {
    type Output = LineAtInfinity;

    fn meet(self, other: Plane) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn meet(self, other: PlaneAtOrigin) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<Point> for Horizon {
    type Output = Radial;

    fn meet(self, other: Point) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group0()[3]]),
            },
        }
    }
}

impl Meet<Radial> for Horizon {
    type Output = Scalar;

    fn meet(self, other: Radial) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group1()[0],
            },
        }
    }
}

impl Meet<Sphere> for Horizon {
    type Output = Circle;

    fn meet(self, other: Sphere) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<AntiScalar> for Line {
    type Output = Line;

    fn meet(self, other: AntiScalar) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Line {
    type Output = Radial;

    fn meet(self, other: Circle) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl Meet<Dipole> for Line {
    type Output = Scalar;

    fn meet(self, other: Dipole) -> Scalar {
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

impl Meet<Horizon> for Line {
    type Output = PointAtInfinity;

    fn meet(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Line> for Line {
    type Output = Radial;

    fn meet(self, other: Line) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl Meet<LineAtInfinity> for Line {
    type Output = Radial;

    fn meet(self, other: LineAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Line {
    type Output = Radial;

    fn meet(self, other: LineAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Magnitude> for Line {
    type Output = Line;

    fn meet(self, other: Magnitude) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for Line {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Plane> for Line {
    type Output = Point;

    fn meet(self, other: Plane) -> Point {
        Point {
            groups: PointGroups {
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

impl Meet<PlaneAtOrigin> for Line {
    type Output = Point;

    fn meet(self, other: PlaneAtOrigin) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl Meet<Sphere> for Line {
    type Output = Dipole;

    fn meet(self, other: Sphere) -> Dipole {
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

impl Meet<AntiScalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn meet(self, other: AntiScalar) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for LineAtInfinity {
    type Output = Radial;

    fn meet(self, other: Circle) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl Meet<Dipole> for LineAtInfinity {
    type Output = Scalar;

    fn meet(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Line> for LineAtInfinity {
    type Output = Radial;

    fn meet(self, other: Line) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<LineAtOrigin> for LineAtInfinity {
    type Output = Radial;

    fn meet(self, other: LineAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Magnitude> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn meet(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group3()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group3()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group3()[2], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group7()[2]]),
                g3: Simd32x3::from(0.0),
                g4: self.group0() * Simd32x3::from(other.group10()[0]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group9()[2], other.group9()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group9()[2], 0.0, -other.group9()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group9()[1], other.group9()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x3::from(other.group0()[1]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Plane> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn meet(self, other: Plane) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn meet(self, other: PlaneAtOrigin) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
            },
        }
    }
}

impl Meet<Sphere> for LineAtInfinity {
    type Output = Dipole;

    fn meet(self, other: Sphere) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group0() * Simd32x3::from(other.group1()[0]),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl Meet<AntiScalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn meet(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for LineAtOrigin {
    type Output = Radial;

    fn meet(self, other: Circle) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
                g1: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]),
            },
        }
    }
}

impl Meet<Dipole> for LineAtOrigin {
    type Output = Scalar;

    fn meet(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Meet<Horizon> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn meet(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Line> for LineAtOrigin {
    type Output = Radial;

    fn meet(self, other: Line) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
            },
        }
    }
}

impl Meet<LineAtInfinity> for LineAtOrigin {
    type Output = Radial;

    fn meet(self, other: LineAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Magnitude> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn meet(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group4()[2], 0.0]),
                g1: self.group0() * Simd32x3::from(other.group6()[3]),
                g2: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group6()[0], other.group8()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group6()[1], other.group8()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group6()[2], other.group8()[2]]),
                g3: self.group0() * Simd32x3::from(other.group10()[0]),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group10()[1], 0.0, 0.0, -other.group9()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group10()[1], 0.0, -other.group9()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group10()[1], -other.group9()[2]]),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[1]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Plane> for LineAtOrigin {
    type Output = Point;

    fn meet(self, other: Plane) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for LineAtOrigin {
    type Output = Origin;

    fn meet(self, other: PlaneAtOrigin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Sphere> for LineAtOrigin {
    type Output = Dipole;

    fn meet(self, other: Sphere) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[1], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[1], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[1], -other.group0()[2]]),
            },
        }
    }
}

impl Meet<AntiScalar> for Magnitude {
    type Output = Magnitude;

    fn meet(self, other: AntiScalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Magnitude {
    type Output = Circle;

    fn meet(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
                g1: Simd32x3::from(self.group0()[1]) * other.group1(),
                g2: Simd32x3::from(self.group0()[1]) * other.group2(),
            },
        }
    }
}

impl Meet<Dipole> for Magnitude {
    type Output = Dipole;

    fn meet(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
                g1: Simd32x3::from(self.group0()[1]) * other.group1(),
                g2: Simd32x4::from(self.group0()[1]) * other.group2(),
            },
        }
    }
}

impl Meet<Horizon> for Magnitude {
    type Output = Horizon;

    fn meet(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl Meet<Line> for Magnitude {
    type Output = Line;

    fn meet(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
                g1: Simd32x3::from(self.group0()[1]) * other.group1(),
            },
        }
    }
}

impl Meet<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn meet(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn meet(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl Meet<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn meet(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl Meet<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Origin> for Magnitude {
    type Output = Origin;

    fn meet(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl Meet<Plane> for Magnitude {
    type Output = Plane;

    fn meet(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn meet(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl Meet<Point> for Magnitude {
    type Output = Point;

    fn meet(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl Meet<PointAtInfinity> for Magnitude {
    type Output = PointAtInfinity;

    fn meet(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
            },
        }
    }
}

impl Meet<Radial> for Magnitude {
    type Output = Radial;

    fn meet(self, other: Radial) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
                g1: Simd32x2::from(self.group0()[1]) * other.group1(),
            },
        }
    }
}

impl Meet<Scalar> for Magnitude {
    type Output = Scalar;

    fn meet(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl Meet<Sphere> for Magnitude {
    type Output = Sphere;

    fn meet(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(self.group0()[1]) * other.group0(),
                g1: Simd32x2::from(self.group0()[1]) * other.group1(),
            },
        }
    }
}

impl Meet<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: AntiScalar) -> MultiVector {
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

impl Meet<Circle> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Circle) -> MultiVector {
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

impl Meet<Dipole> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Dipole) -> MultiVector {
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

impl Meet<Horizon> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Horizon) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group0(), 0.0]),
                g1: self.group3() * Simd32x3::from(other.group0()),
                g2: Simd32x2::from(self.group5()[3]) * Simd32x2::from([0.0, other.group0()]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group0()),
                g5: Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group7()[0]])
                    * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]),
                g6: Simd32x4::from(self.group10()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g7: Simd32x3::from(0.0),
                g8: self.group9() * Simd32x3::from(other.group0()),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl Meet<Line> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Line) -> MultiVector {
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

impl Meet<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group7()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(self.group10()[0]) * other.group0(),
                g5: Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[1]) * other.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group6()[3]) * other.group0(),
                g2: Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(self.group10()[0]) * other.group0(),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[1]) * other.group0(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Magnitude) -> MultiVector {
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

impl Meet<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Origin> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Origin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[3]) * Simd32x2::from([-other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: self.group10() * Simd32x2::from(other.group0()),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Plane> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Plane) -> MultiVector {
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

impl Meet<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group5()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group5()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group5()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
                g3: Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g4: Simd32x3::from(0.0) - Simd32x3::from(self.group6()[3]) * other.group0(),
                g5: Simd32x4::from(self.group7()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group7()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group7()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group8()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group8()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(self.group10()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g7: Simd32x3::from(self.group9()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group9()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group9()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g8: Simd32x3::from(0.0) - Simd32x3::from(self.group10()[1]) * other.group0(),
                g9: Simd32x3::from(self.group0()[1]) * other.group0(),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Point> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Point) -> MultiVector {
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

impl Meet<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: PointAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group10()[0]) * other.group0(),
                g2: Simd32x2::from(self.group9()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group9()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group9()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Radial> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Radial) -> MultiVector {
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

impl Meet<Scalar> for MultiVector {
    type Output = Scalar;

    fn meet(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl Meet<Sphere> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Sphere) -> MultiVector {
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

impl Meet<AntiScalar> for Origin {
    type Output = Origin;

    fn meet(self, other: AntiScalar) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Meet<Circle> for Origin {
    type Output = Scalar;

    fn meet(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl Meet<Horizon> for Origin {
    type Output = Radial;

    fn meet(self, other: Horizon) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl Meet<Magnitude> for Origin {
    type Output = Origin;

    fn meet(self, other: Magnitude) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl Meet<MultiVector> for Origin {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([-other.group6()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()) * other.group10() * Simd32x2::from([-1.0, 1.0]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Plane> for Origin {
    type Output = Radial;

    fn meet(self, other: Plane) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Meet<Sphere> for Origin {
    type Output = Radial;

    fn meet(self, other: Sphere) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * other.group1() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl Meet<AntiScalar> for Plane {
    type Output = Plane;

    fn meet(self, other: AntiScalar) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Plane {
    type Output = Dipole;

    fn meet(self, other: Circle) -> Dipole {
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

impl Meet<Dipole> for Plane {
    type Output = Radial;

    fn meet(self, other: Dipole) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl Meet<Horizon> for Plane {
    type Output = LineAtInfinity;

    fn meet(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Line> for Plane {
    type Output = Point;

    fn meet(self, other: Line) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Meet<LineAtInfinity> for Plane {
    type Output = PointAtInfinity;

    fn meet(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Plane {
    type Output = Point;

    fn meet(self, other: LineAtOrigin) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Magnitude> for Plane {
    type Output = Plane;

    fn meet(self, other: Magnitude) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for Plane {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Origin> for Plane {
    type Output = Radial;

    fn meet(self, other: Origin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()]),
            },
        }
    }
}

impl Meet<Plane> for Plane {
    type Output = Line;

    fn meet(self, other: Plane) -> Line {
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

impl Meet<PlaneAtOrigin> for Plane {
    type Output = Line;

    fn meet(self, other: PlaneAtOrigin) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<Point> for Plane {
    type Output = Radial;

    fn meet(self, other: Point) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()[3]]),
            },
        }
    }
}

impl Meet<PointAtInfinity> for Plane {
    type Output = Radial;

    fn meet(self, other: PointAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Radial> for Plane {
    type Output = Scalar;

    fn meet(self, other: Radial) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl Meet<Sphere> for Plane {
    type Output = Circle;

    fn meet(self, other: Sphere) -> Circle {
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

impl Meet<AntiScalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn meet(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for PlaneAtOrigin {
    type Output = Dipole;

    fn meet(self, other: Circle) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g1: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group2()[2], -other.group2()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], 0.0, other.group2()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], 0.0, -other.group1()[2]]),
            },
        }
    }
}

impl Meet<Dipole> for PlaneAtOrigin {
    type Output = Radial;

    fn meet(self, other: Dipole) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], -other.group2()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], -other.group2()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], -other.group2()[2]]),
            },
        }
    }
}

impl Meet<Horizon> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn meet(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Line> for PlaneAtOrigin {
    type Output = Point;

    fn meet(self, other: Line) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<LineAtInfinity> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn meet(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Meet<LineAtOrigin> for PlaneAtOrigin {
    type Output = Origin;

    fn meet(self, other: LineAtOrigin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Magnitude> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn meet(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group4()[2], other.group4()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], 0.0, -other.group4()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], 0.0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], -other.group5()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], -other.group5()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], -other.group5()[2]]),
                g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0]),
                g4: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group6()[3]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group8()[2], -other.group8()[1], -other.group7()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group8()[2], 0.0, other.group8()[0], -other.group7()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group8()[1], -other.group8()[0], 0.0, -other.group7()[2]]),
                g6: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([-other.group10()[0], -other.group10()[0], -other.group10()[0], 0.0]),
                g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group9()[2], -other.group9()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group9()[2], 0.0, other.group9()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group9()[1], -other.group9()[0], 0.0]),
                g8: self.group0() * Simd32x3::from(other.group10()[1]),
                g9: self.group0() * Simd32x3::from(other.group0()[1]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Plane> for PlaneAtOrigin {
    type Output = Line;

    fn meet(self, other: Plane) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g1: self.group0() * Simd32x3::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn meet(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Meet<Point> for PlaneAtOrigin {
    type Output = Radial;

    fn meet(self, other: Point) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<PointAtInfinity> for PlaneAtOrigin {
    type Output = Radial;

    fn meet(self, other: PointAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Radial> for PlaneAtOrigin {
    type Output = Scalar;

    fn meet(self, other: Radial) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Sphere> for PlaneAtOrigin {
    type Output = Circle;

    fn meet(self, other: Sphere) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([-other.group1()[0], -other.group1()[0], -other.group1()[0], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: self.group0() * Simd32x3::from(other.group1()[1]),
            },
        }
    }
}

impl Meet<AntiScalar> for Point {
    type Output = Point;

    fn meet(self, other: AntiScalar) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Point {
    type Output = Scalar;

    fn meet(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Meet<Horizon> for Point {
    type Output = Radial;

    fn meet(self, other: Horizon) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl Meet<Magnitude> for Point {
    type Output = Point;

    fn meet(self, other: Magnitude) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for Point {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Plane> for Point {
    type Output = Radial;

    fn meet(self, other: Plane) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for Point {
    type Output = Radial;

    fn meet(self, other: PlaneAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl Meet<Sphere> for Point {
    type Output = Radial;

    fn meet(self, other: Sphere) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group1() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl Meet<AntiScalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn meet(self, other: AntiScalar) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for PointAtInfinity {
    type Output = Scalar;

    fn meet(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Magnitude> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn meet(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group6()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group6()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group6()[2], 0.0]),
                g1: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group10()[0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group9()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group9()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group9()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Plane> for PointAtInfinity {
    type Output = Radial;

    fn meet(self, other: Plane) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for PointAtInfinity {
    type Output = Radial;

    fn meet(self, other: PlaneAtOrigin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl Meet<Sphere> for PointAtInfinity {
    type Output = Radial;

    fn meet(self, other: Sphere) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl Meet<AntiScalar> for Radial {
    type Output = Radial;

    fn meet(self, other: AntiScalar) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Meet<Horizon> for Radial {
    type Output = Scalar;

    fn meet(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl Meet<Magnitude> for Radial {
    type Output = Radial;

    fn meet(self, other: Magnitude) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x2::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for Radial {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Plane> for Radial {
    type Output = Scalar;

    fn meet(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for Radial {
    type Output = Scalar;

    fn meet(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Sphere> for Radial {
    type Output = Scalar;

    fn meet(self, other: Sphere) -> Scalar {
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

impl Meet<AntiScalar> for Scalar {
    type Output = Scalar;

    fn meet(self, other: AntiScalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Meet<Magnitude> for Scalar {
    type Output = Scalar;

    fn meet(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl Meet<MultiVector> for Scalar {
    type Output = Scalar;

    fn meet(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl Meet<AntiScalar> for Sphere {
    type Output = Sphere;

    fn meet(self, other: AntiScalar) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Sphere {
    type Output = Dipole;

    fn meet(self, other: Circle) -> Dipole {
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

impl Meet<Dipole> for Sphere {
    type Output = Radial;

    fn meet(self, other: Dipole) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl Meet<Horizon> for Sphere {
    type Output = Circle;

    fn meet(self, other: Horizon) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Line> for Sphere {
    type Output = Dipole;

    fn meet(self, other: Line) -> Dipole {
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

impl Meet<LineAtInfinity> for Sphere {
    type Output = Dipole;

    fn meet(self, other: LineAtInfinity) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(self.group1()[0]) * other.group0(),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Sphere {
    type Output = Dipole;

    fn meet(self, other: LineAtOrigin) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Meet<Magnitude> for Sphere {
    type Output = Sphere;

    fn meet(self, other: Magnitude) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x2::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<MultiVector> for Sphere {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Origin> for Sphere {
    type Output = Radial;

    fn meet(self, other: Origin) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Meet<Plane> for Sphere {
    type Output = Circle;

    fn meet(self, other: Plane) -> Circle {
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

impl Meet<PlaneAtOrigin> for Sphere {
    type Output = Circle;

    fn meet(self, other: PlaneAtOrigin) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[1]) * other.group0(),
            },
        }
    }
}

impl Meet<Point> for Sphere {
    type Output = Radial;

    fn meet(self, other: Point) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]])
                    + self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<PointAtInfinity> for Sphere {
    type Output = Radial;

    fn meet(self, other: PointAtInfinity) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Radial> for Sphere {
    type Output = Scalar;

    fn meet(self, other: Radial) -> Scalar {
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

impl Meet<Sphere> for Sphere {
    type Output = Circle;

    fn meet(self, other: Sphere) -> Circle {
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

impl Wedge<Magnitude> for AntiScalar {
    type Output = AntiScalar;

    fn wedge(self, other: Magnitude) -> AntiScalar {
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

impl Wedge<Magnitude> for Circle {
    type Output = Circle;

    fn wedge(self, other: Magnitude) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
                g2: self.group2() * Simd32x3::from(other.group0()[0]),
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

impl Wedge<Origin> for Circle {
    type Output = AntiScalar;

    fn wedge(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Wedge<Point> for Circle {
    type Output = AntiScalar;

    fn wedge(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Wedge<PointAtInfinity> for Circle {
    type Output = AntiScalar;

    fn wedge(self, other: PointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<Radial> for Circle {
    type Output = Sphere;

    fn wedge(self, other: Radial) -> Sphere {
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

impl Wedge<LineAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn wedge(self, other: LineAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<LineAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn wedge(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<Magnitude> for Dipole {
    type Output = Dipole;

    fn wedge(self, other: Magnitude) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
                g2: self.group2() * Simd32x4::from(other.group0()[0]),
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

impl Wedge<Origin> for Dipole {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group1() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Wedge<Point> for Dipole {
    type Output = Plane;

    fn wedge(self, other: Point) -> Plane {
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

impl Wedge<PointAtInfinity> for Dipole {
    type Output = Plane;

    fn wedge(self, other: PointAtInfinity) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Radial> for Dipole {
    type Output = Circle;

    fn wedge(self, other: Radial) -> Circle {
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

impl Wedge<Magnitude> for Horizon {
    type Output = Horizon;

    fn wedge(self, other: Magnitude) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Wedge<MultiVector> for Horizon {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group2()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[0]]),
            },
        }
    }
}

impl Wedge<Radial> for Horizon {
    type Output = AntiScalar;

    fn wedge(self, other: Radial) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group1()[0],
            },
        }
    }
}

impl Wedge<Scalar> for Horizon {
    type Output = Horizon;

    fn wedge(self, other: Scalar) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0(),
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

impl Wedge<Magnitude> for Line {
    type Output = Line;

    fn wedge(self, other: Magnitude) -> Line {
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

impl Wedge<Radial> for Line {
    type Output = Plane;

    fn wedge(self, other: Radial) -> Plane {
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

impl Wedge<Dipole> for LineAtInfinity {
    type Output = AntiScalar;

    fn wedge(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<Magnitude> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn wedge(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x3::from(other.group0()[0]),
                g9: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group2()[0]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]]),
            },
        }
    }
}

impl Wedge<Radial> for LineAtInfinity {
    type Output = Plane;

    fn wedge(self, other: Radial) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[0], 0.0, 0.0, other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, -other.group1()[0], 0.0, other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, -other.group1()[0], other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Scalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn wedge(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Wedge<Dipole> for LineAtOrigin {
    type Output = AntiScalar;

    fn wedge(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Wedge<Magnitude> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn wedge(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[0]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Wedge<Radial> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: Radial) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Scalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn wedge(self, other: Scalar) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Wedge<AntiScalar> for Magnitude {
    type Output = AntiScalar;

    fn wedge(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Wedge<Circle> for Magnitude {
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

impl Wedge<Dipole> for Magnitude {
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

impl Wedge<Horizon> for Magnitude {
    type Output = Horizon;

    fn wedge(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Wedge<Line> for Magnitude {
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

impl Wedge<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn wedge(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Wedge<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn wedge(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Wedge<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn wedge(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]),
            },
        }
    }
}

impl Wedge<MultiVector> for Magnitude {
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

impl Wedge<Origin> for Magnitude {
    type Output = Origin;

    fn wedge(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Wedge<Plane> for Magnitude {
    type Output = Plane;

    fn wedge(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Wedge<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Wedge<Point> for Magnitude {
    type Output = Point;

    fn wedge(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Wedge<PointAtInfinity> for Magnitude {
    type Output = PointAtInfinity;

    fn wedge(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Wedge<Radial> for Magnitude {
    type Output = Radial;

    fn wedge(self, other: Radial) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: Simd32x3::from(self.group0()[0]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * other.group1(),
            },
        }
    }
}

impl Wedge<Scalar> for Magnitude {
    type Output = Magnitude;

    fn wedge(self, other: Scalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Wedge<Sphere> for Magnitude {
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

impl Wedge<Horizon> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Horizon) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()]),
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

impl Wedge<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[0]) * other.group0(),
                g9: Simd32x3::from(self.group2()[0]) * other.group0(),
                g10: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Wedge<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[0]) * other.group0(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Wedge<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Magnitude) -> MultiVector {
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

impl Wedge<Origin> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Origin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[3]) * Simd32x2::from([0.0, -other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - self.group1() * Simd32x3::from(other.group0()),
                g8: Simd32x3::from(0.0),
                g9: self.group4() * Simd32x3::from(other.group0()),
                g10: Simd32x2::from(0.0),
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

impl Wedge<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[0]) * other.group0(),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Wedge<Point> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Point) -> MultiVector {
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

impl Wedge<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: PointAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group6()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group2()[0]) * other.group0(),
                g8: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g9: Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g10: Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Radial> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Radial) -> MultiVector {
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

impl Wedge<Circle> for Origin {
    type Output = AntiScalar;

    fn wedge(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl Wedge<Dipole> for Origin {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: Dipole) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Wedge<Magnitude> for Origin {
    type Output = Origin;

    fn wedge(self, other: Magnitude) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Wedge<MultiVector> for Origin {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group6()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group1(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()) * other.group4(),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Wedge<Radial> for Origin {
    type Output = LineAtOrigin;

    fn wedge(self, other: Radial) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<Scalar> for Origin {
    type Output = Origin;

    fn wedge(self, other: Scalar) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Wedge<Magnitude> for Plane {
    type Output = Plane;

    fn wedge(self, other: Magnitude) -> Plane {
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

impl Wedge<Radial> for Plane {
    type Output = AntiScalar;

    fn wedge(self, other: Radial) -> AntiScalar {
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

impl Wedge<Magnitude> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0() * Simd32x3::from(other.group0()[0]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Wedge<Radial> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn wedge(self, other: Radial) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<Scalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: Scalar) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Wedge<Circle> for Point {
    type Output = AntiScalar;

    fn wedge(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Wedge<Dipole> for Point {
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

impl Wedge<Magnitude> for Point {
    type Output = Point;

    fn wedge(self, other: Magnitude) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for Point {
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

impl Wedge<Radial> for Point {
    type Output = Line;

    fn wedge(self, other: Radial) -> Line {
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

impl Wedge<Scalar> for Point {
    type Output = Point;

    fn wedge(self, other: Scalar) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Wedge<Circle> for PointAtInfinity {
    type Output = AntiScalar;

    fn wedge(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<Dipole> for PointAtInfinity {
    type Output = Plane;

    fn wedge(self, other: Dipole) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]),
            },
        }
    }
}

impl Wedge<Magnitude> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn wedge(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group6()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group6()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group6()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group2()[0]),
                g8: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group3()[2], -other.group3()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], 0.0, other.group3()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], 0.0]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]]),
            },
        }
    }
}

impl Wedge<Radial> for PointAtInfinity {
    type Output = Line;

    fn wedge(self, other: Radial) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Scalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn wedge(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Wedge<Circle> for Radial {
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

impl Wedge<Dipole> for Radial {
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

impl Wedge<Horizon> for Radial {
    type Output = AntiScalar;

    fn wedge(self, other: Horizon) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl Wedge<Line> for Radial {
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

impl Wedge<LineAtInfinity> for Radial {
    type Output = Plane;

    fn wedge(self, other: LineAtInfinity) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Wedge<LineAtOrigin> for Radial {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Magnitude> for Radial {
    type Output = Radial;

    fn wedge(self, other: Magnitude) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x2::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for Radial {
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

impl Wedge<Origin> for Radial {
    type Output = LineAtOrigin;

    fn wedge(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Wedge<Plane> for Radial {
    type Output = AntiScalar;

    fn wedge(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl Wedge<PlaneAtOrigin> for Radial {
    type Output = AntiScalar;

    fn wedge(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<Point> for Radial {
    type Output = Line;

    fn wedge(self, other: Point) -> Line {
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

impl Wedge<PointAtInfinity> for Radial {
    type Output = Line;

    fn wedge(self, other: PointAtInfinity) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Radial> for Radial {
    type Output = Dipole;

    fn wedge(self, other: Radial) -> Dipole {
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

impl Wedge<Scalar> for Radial {
    type Output = Radial;

    fn wedge(self, other: Scalar) -> Radial {
        Radial {
            groups: RadialGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Wedge<Sphere> for Radial {
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

impl Wedge<Horizon> for Scalar {
    type Output = Horizon;

    fn wedge(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0(),
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

impl Wedge<LineAtInfinity> for Scalar {
    type Output = LineAtInfinity;

    fn wedge(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<LineAtOrigin> for Scalar {
    type Output = LineAtOrigin;

    fn wedge(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<Magnitude> for Scalar {
    type Output = Magnitude;

    fn wedge(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from(self.group0()) * other.group0(),
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

impl Wedge<Origin> for Scalar {
    type Output = Origin;

    fn wedge(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0(),
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

impl Wedge<PlaneAtOrigin> for Scalar {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<Point> for Scalar {
    type Output = Point;

    fn wedge(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<PointAtInfinity> for Scalar {
    type Output = PointAtInfinity;

    fn wedge(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<Radial> for Scalar {
    type Output = Radial;

    fn wedge(self, other: Radial) -> Radial {
        Radial {
            groups: RadialGroups {
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

impl Wedge<Magnitude> for Sphere {
    type Output = Sphere;

    fn wedge(self, other: Magnitude) -> Sphere {
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

impl Wedge<Radial> for Sphere {
    type Output = AntiScalar;

    fn wedge(self, other: Radial) -> AntiScalar {
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
