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

impl AntiWedge<Infinity> for AntiScalar {
    type Output = Infinity;

    fn anti_wedge(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl AntiWedge<Rotor> for AntiScalar {
    type Output = Rotor;

    fn anti_wedge(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
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

impl AntiWedge<Translator> for AntiScalar {
    type Output = Translator;

    fn anti_wedge(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
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

impl AntiWedge<LineAtInfinity> for Circle {
    type Output = RoundPoint;

    fn anti_wedge(self, other: LineAtInfinity) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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
    type Output = RoundPoint;

    fn anti_wedge(self, other: LineAtOrigin) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl AntiWedge<Rotor> for Circle {
    type Output = MultiVector;

    fn anti_wedge(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
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

impl AntiWedge<Translator> for Circle {
    type Output = MultiVector;

    fn anti_wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
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

impl AntiWedge<Horizon> for Dipole {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Horizon) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl AntiWedge<PlaneAtOrigin> for Dipole {
    type Output = RoundPoint;

    fn anti_wedge(self, other: PlaneAtOrigin) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl AntiWedge<Rotor> for Dipole {
    type Output = MultiVector;

    fn anti_wedge(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
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

impl AntiWedge<Translator> for Dipole {
    type Output = MultiVector;

    fn anti_wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
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

impl AntiWedge<Horizon> for Flector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Horizon) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Line> for Flector {
    type Output = Point;

    fn anti_wedge(self, other: Line) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<LineAtInfinity> for Flector {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Flector {
    type Output = Point;

    fn anti_wedge(self, other: LineAtOrigin) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Flector {
    type Output = Flector;

    fn anti_wedge(self, other: Magnitude) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
                g1: self.group1() * Simd32x4::from(other.group0()[1]),
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

impl AntiWedge<Origin> for Flector {
    type Output = Infinity;

    fn anti_wedge(self, other: Origin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group1()[3] * other.group0(),
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

impl AntiWedge<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_wedge(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g8: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * other.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Point> for Flector {
    type Output = Infinity;

    fn anti_wedge(self, other: Point) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] - self.group1()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for Flector {
    type Output = Infinity;

    fn anti_wedge(self, other: PointAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Rotor> for Flector {
    type Output = Flector;

    fn anti_wedge(self, other: Rotor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
                g1: self.group1() * Simd32x4::from(other.group0()[3]),
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

impl AntiWedge<Translator> for Flector {
    type Output = Flector;

    fn anti_wedge(self, other: Translator) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g1: self.group1() * Simd32x4::from(other.group0()[3]),
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
    type Output = RoundPoint;

    fn anti_wedge(self, other: Dipole) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group2()[3]]),
            },
        }
    }
}

impl AntiWedge<Flector> for Horizon {
    type Output = MultiVector;

    fn anti_wedge(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group0()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Motor> for Horizon {
    type Output = Flector;

    fn anti_wedge(self, other: Motor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
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
    type Output = Infinity;

    fn anti_wedge(self, other: Origin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0() * other.group0(),
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
    type Output = Infinity;

    fn anti_wedge(self, other: Point) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<Rotor> for Horizon {
    type Output = Flector;

    fn anti_wedge(self, other: Rotor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for Horizon {
    type Output = Scalar;

    fn anti_wedge(self, other: RoundPoint) -> Scalar {
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

impl AntiWedge<Translator> for Horizon {
    type Output = Horizon;

    fn anti_wedge(self, other: Translator) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Infinity {
    type Output = Infinity;

    fn anti_wedge(self, other: AntiScalar) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Infinity {
    type Output = Infinity;

    fn anti_wedge(self, other: Magnitude) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl AntiWedge<Motor> for Infinity {
    type Output = Infinity;

    fn anti_wedge(self, other: Motor) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<MultiVector> for Infinity {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group10()[0], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[1]]),
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

impl AntiWedge<Rotor> for Infinity {
    type Output = Infinity;

    fn anti_wedge(self, other: Rotor) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<Sphere> for Infinity {
    type Output = Scalar;

    fn anti_wedge(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group1()[0],
            },
        }
    }
}

impl AntiWedge<Translator> for Infinity {
    type Output = Infinity;

    fn anti_wedge(self, other: Translator) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[3],
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

impl AntiWedge<Flector> for Line {
    type Output = Point;

    fn anti_wedge(self, other: Flector) -> Point {
        Point {
            groups: PointGroups {
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
    type Output = Infinity;

    fn anti_wedge(self, other: Line) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl AntiWedge<LineAtInfinity> for Line {
    type Output = Infinity;

    fn anti_wedge(self, other: LineAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Line {
    type Output = Infinity;

    fn anti_wedge(self, other: LineAtOrigin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
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

impl AntiWedge<Rotor> for Line {
    type Output = MultiVector;

    fn anti_wedge(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
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

impl AntiWedge<Translator> for Line {
    type Output = MultiVector;

    fn anti_wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
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
    type Output = RoundPoint;

    fn anti_wedge(self, other: Circle) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl AntiWedge<Flector> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: Flector) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]),
            },
        }
    }
}

impl AntiWedge<Line> for LineAtInfinity {
    type Output = Infinity;

    fn anti_wedge(self, other: Line) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for LineAtInfinity {
    type Output = Infinity;

    fn anti_wedge(self, other: LineAtOrigin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl AntiWedge<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_wedge(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: Translator) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
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
    type Output = RoundPoint;

    fn anti_wedge(self, other: Circle) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl AntiWedge<Flector> for LineAtOrigin {
    type Output = Point;

    fn anti_wedge(self, other: Flector) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[3], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[3], -other.group1()[2]]),
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
    type Output = Infinity;

    fn anti_wedge(self, other: Line) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiWedge<LineAtInfinity> for LineAtOrigin {
    type Output = Infinity;

    fn anti_wedge(self, other: LineAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl AntiWedge<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Rotor> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_wedge(self, other: Rotor) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
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

impl AntiWedge<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Flector> for Magnitude {
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

impl AntiWedge<Infinity> for Magnitude {
    type Output = Infinity;

    fn anti_wedge(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl AntiWedge<Motor> for Magnitude {
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

impl AntiWedge<Rotor> for Magnitude {
    type Output = MultiVector;

    fn anti_wedge(self, other: Rotor) -> MultiVector {
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
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for Magnitude {
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

impl AntiWedge<Translator> for Magnitude {
    type Output = MultiVector;

    fn anti_wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()[3]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Horizon> for Motor {
    type Output = Flector;

    fn anti_wedge(self, other: Horizon) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl AntiWedge<Infinity> for Motor {
    type Output = Infinity;

    fn anti_wedge(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[3] * other.group0(),
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

impl AntiWedge<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[3]) * other.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[3]) * other.group0(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Magnitude) -> MultiVector {
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

impl AntiWedge<Origin> for Motor {
    type Output = Origin;

    fn anti_wedge(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[3] * other.group0(),
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

impl AntiWedge<PlaneAtOrigin> for Motor {
    type Output = Flector;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Point> for Motor {
    type Output = Point;

    fn anti_wedge(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for Motor {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Rotor> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: self.group1() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Translator> for Motor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    + self.group1() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Infinity> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Infinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group10()[0]) * Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()]),
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

impl AntiWedge<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: self.group1() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group6()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: self.group2() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: self.group3() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group10()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g4: self.group4() * Simd32x3::from(other.group0()[3]),
                g5: self.group5() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: self.group6() * Simd32x4::from(other.group0()[3]),
                g7: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    + self.group7() * Simd32x3::from(other.group0()[3]),
                g8: self.group8() * Simd32x3::from(other.group0()[3]),
                g9: self.group9() * Simd32x3::from(other.group0()[3]),
                g10: self.group10() * Simd32x2::from(other.group0()[3]),
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

impl AntiWedge<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: self.group1() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: self.group2() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group7()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: self.group3() * Simd32x3::from(other.group0()[3]),
                g4: self.group4() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group10()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g5: self.group5() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g6: self.group6() * Simd32x4::from(other.group0()[3]),
                g7: self.group7() * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    + self.group8() * Simd32x3::from(other.group0()[3]),
                g9: self.group9() * Simd32x3::from(other.group0()[3]),
                g10: self.group10() * Simd32x2::from(other.group0()[3]),
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

impl AntiWedge<Flector> for Origin {
    type Output = Infinity;

    fn anti_wedge(self, other: Flector) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group1()[3],
            },
        }
    }
}

impl AntiWedge<Horizon> for Origin {
    type Output = Infinity;

    fn anti_wedge(self, other: Horizon) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0(),
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

impl AntiWedge<Motor> for Origin {
    type Output = Origin;

    fn anti_wedge(self, other: Motor) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[3],
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
    type Output = Infinity;

    fn anti_wedge(self, other: Plane) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<Rotor> for Origin {
    type Output = Origin;

    fn anti_wedge(self, other: Rotor) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<Sphere> for Origin {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Sphere) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * other.group1() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl AntiWedge<Translator> for Origin {
    type Output = Origin;

    fn anti_wedge(self, other: Translator) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[3],
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

impl AntiWedge<Origin> for Plane {
    type Output = Infinity;

    fn anti_wedge(self, other: Origin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
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
    type Output = Infinity;

    fn anti_wedge(self, other: Point) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for Plane {
    type Output = Infinity;

    fn anti_wedge(self, other: PointAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Rotor> for Plane {
    type Output = Flector;

    fn anti_wedge(self, other: Rotor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
                g1: self.group0() * Simd32x4::from(other.group0()[3]),
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

impl AntiWedge<Translator> for Plane {
    type Output = Flector;

    fn anti_wedge(self, other: Translator) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g1: self.group0() * Simd32x4::from(other.group0()[3]),
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
    type Output = RoundPoint;

    fn anti_wedge(self, other: Dipole) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl AntiWedge<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_wedge(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g8: self.group0() * Simd32x3::from(other.group1()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Motor> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge(self, other: Motor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]),
                g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]),
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
    type Output = Infinity;

    fn anti_wedge(self, other: Point) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for PlaneAtOrigin {
    type Output = Infinity;

    fn anti_wedge(self, other: PointAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Rotor> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge(self, other: Rotor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]),
                g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for PlaneAtOrigin {
    type Output = Scalar;

    fn anti_wedge(self, other: RoundPoint) -> Scalar {
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

impl AntiWedge<Translator> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge(self, other: Translator) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]),
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

impl AntiWedge<Flector> for Point {
    type Output = Infinity;

    fn anti_wedge(self, other: Flector) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] + self.group0()[3] * other.group1()[3],
            },
        }
    }
}

impl AntiWedge<Horizon> for Point {
    type Output = Infinity;

    fn anti_wedge(self, other: Horizon) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[3] * other.group0(),
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

impl AntiWedge<Motor> for Point {
    type Output = Point;

    fn anti_wedge(self, other: Motor) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3]),
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
    type Output = Infinity;

    fn anti_wedge(self, other: Plane) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for Point {
    type Output = Infinity;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Rotor> for Point {
    type Output = Point;

    fn anti_wedge(self, other: Rotor) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<Sphere> for Point {
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

impl AntiWedge<Translator> for Point {
    type Output = Point;

    fn anti_wedge(self, other: Translator) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3]),
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

impl AntiWedge<Flector> for PointAtInfinity {
    type Output = Infinity;

    fn anti_wedge(self, other: Flector) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
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

impl AntiWedge<Motor> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: Motor) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
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
    type Output = Infinity;

    fn anti_wedge(self, other: Plane) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for PointAtInfinity {
    type Output = Infinity;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Rotor> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: Rotor) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<Sphere> for PointAtInfinity {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Sphere) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: Translator) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Rotor {
    type Output = Rotor;

    fn anti_wedge(self, other: AntiScalar) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Rotor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]),
                g2: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]),
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

impl AntiWedge<Dipole> for Rotor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], 0.0]),
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

impl AntiWedge<Flector> for Rotor {
    type Output = Flector;

    fn anti_wedge(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[3], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[3], -other.group1()[2]])
                    + Simd32x4::from(self.group0()[3]) * other.group0(),
                g1: Simd32x4::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Horizon> for Rotor {
    type Output = Flector;

    fn anti_wedge(self, other: Horizon) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl AntiWedge<Infinity> for Rotor {
    type Output = Infinity;

    fn anti_wedge(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Line> for Rotor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
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

impl AntiWedge<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_wedge(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[3]) * other.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn anti_wedge(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Rotor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Magnitude) -> MultiVector {
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
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Motor> for Rotor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(self.group0()[3]) * other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group4()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * other.group0(),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group6()[3]) + Simd32x3::from(self.group0()[3]) * other.group1(),
                g2: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group6()[0], other.group8()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group6()[1], other.group8()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group6()[2], other.group8()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group2(),
                g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group10()[0]) + Simd32x3::from(self.group0()[3]) * other.group3(),
                g4: Simd32x3::from(self.group0()[3]) * other.group4(),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group10()[1], 0.0, 0.0, -other.group9()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group10()[1], 0.0, -other.group9()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group10()[1], -other.group9()[2]])
                    + Simd32x4::from(self.group0()[3]) * other.group5(),
                g6: Simd32x4::from(self.group0()[3]) * other.group6(),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]) + Simd32x3::from(self.group0()[3]) * other.group7(),
                g8: Simd32x3::from(self.group0()[3]) * other.group8(),
                g9: Simd32x3::from(self.group0()[3]) * other.group9(),
                g10: Simd32x2::from(self.group0()[3]) * other.group10(),
            },
        }
    }
}

impl AntiWedge<Origin> for Rotor {
    type Output = Origin;

    fn anti_wedge(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Plane> for Rotor {
    type Output = Flector;

    fn anti_wedge(self, other: Plane) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]),
                g1: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for Rotor {
    type Output = Flector;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Point> for Rotor {
    type Output = Point;

    fn anti_wedge(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for Rotor {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Rotor> for Rotor {
    type Output = Rotor;

    fn anti_wedge(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0])
                    + Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for Rotor {
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

impl AntiWedge<Scalar> for Rotor {
    type Output = Scalar;

    fn anti_wedge(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Sphere> for Rotor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[1], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[1], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[1], -other.group0()[2]]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[3]) * other.group0(),
                g10: Simd32x2::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Translator> for Rotor {
    type Output = MultiVector;

    fn anti_wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl AntiWedge<Horizon> for RoundPoint {
    type Output = Scalar;

    fn anti_wedge(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Magnitude> for RoundPoint {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Magnitude) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x2::from(other.group0()[1]),
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

impl AntiWedge<PlaneAtOrigin> for RoundPoint {
    type Output = Scalar;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiWedge<Rotor> for RoundPoint {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Rotor) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
                g1: self.group1() * Simd32x2::from(other.group0()[3]),
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

impl AntiWedge<Translator> for RoundPoint {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Translator) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
                g1: self.group1() * Simd32x2::from(other.group0()[3]),
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

impl AntiWedge<Rotor> for Scalar {
    type Output = Scalar;

    fn anti_wedge(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiWedge<Translator> for Scalar {
    type Output = Scalar;

    fn anti_wedge(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
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

impl AntiWedge<Infinity> for Sphere {
    type Output = Scalar;

    fn anti_wedge(self, other: Infinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0(),
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

impl AntiWedge<Origin> for Sphere {
    type Output = RoundPoint;

    fn anti_wedge(self, other: Origin) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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
    type Output = RoundPoint;

    fn anti_wedge(self, other: Point) -> RoundPoint {
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

impl AntiWedge<PointAtInfinity> for Sphere {
    type Output = RoundPoint;

    fn anti_wedge(self, other: PointAtInfinity) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl AntiWedge<Rotor> for Sphere {
    type Output = MultiVector;

    fn anti_wedge(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
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

impl AntiWedge<Translator> for Sphere {
    type Output = MultiVector;

    fn anti_wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0() * Simd32x3::from(other.group0()[3]),
                g10: self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl AntiWedge<AntiScalar> for Translator {
    type Output = Translator;

    fn anti_wedge(self, other: AntiScalar) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl AntiWedge<Circle> for Translator {
    type Output = MultiVector;

    fn anti_wedge(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
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

impl AntiWedge<Dipole> for Translator {
    type Output = MultiVector;

    fn anti_wedge(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
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

impl AntiWedge<Flector> for Translator {
    type Output = Flector;

    fn anti_wedge(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group1()[2], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], 0.0, -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group0()[3]) * other.group0(),
                g1: Simd32x4::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Horizon> for Translator {
    type Output = Horizon;

    fn anti_wedge(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Infinity> for Translator {
    type Output = Infinity;

    fn anti_wedge(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Line> for Translator {
    type Output = MultiVector;

    fn anti_wedge(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
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

impl AntiWedge<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn anti_wedge(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn anti_wedge(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[3]) * other.group0(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Magnitude> for Translator {
    type Output = MultiVector;

    fn anti_wedge(self, other: Magnitude) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<Motor> for Translator {
    type Output = MultiVector;

    fn anti_wedge(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group3()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group3()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group3()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0])
                    + Simd32x3::from(self.group0()[3]) * other.group1(),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group7()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group2(),
                g3: Simd32x3::from(self.group0()[3]) * other.group3(),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group10()[0]) + Simd32x3::from(self.group0()[3]) * other.group4(),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group9()[2], other.group9()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group9()[2], 0.0, -other.group9()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group9()[1], other.group9()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group0()[3]) * other.group5(),
                g6: Simd32x4::from(self.group0()[3]) * other.group6(),
                g7: Simd32x3::from(self.group0()[3]) * other.group7(),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]) + Simd32x3::from(self.group0()[3]) * other.group8(),
                g9: Simd32x3::from(self.group0()[3]) * other.group9(),
                g10: Simd32x2::from(self.group0()[3]) * other.group10(),
            },
        }
    }
}

impl AntiWedge<Origin> for Translator {
    type Output = Origin;

    fn anti_wedge(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Plane> for Translator {
    type Output = Flector;

    fn anti_wedge(self, other: Plane) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<PlaneAtOrigin> for Translator {
    type Output = Flector;

    fn anti_wedge(self, other: PlaneAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AntiWedge<Point> for Translator {
    type Output = Point;

    fn anti_wedge(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<PointAtInfinity> for Translator {
    type Output = PointAtInfinity;

    fn anti_wedge(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl AntiWedge<Rotor> for Translator {
    type Output = MultiVector;

    fn anti_wedge(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiWedge<RoundPoint> for Translator {
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

impl AntiWedge<Scalar> for Translator {
    type Output = Scalar;

    fn anti_wedge(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiWedge<Sphere> for Translator {
    type Output = MultiVector;

    fn anti_wedge(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[3]) * other.group0(),
                g10: Simd32x2::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl AntiWedge<Translator> for Translator {
    type Output = Translator;

    fn anti_wedge(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0])
                    + Simd32x4::from(self.group0()[3]) * other.group0(),
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

impl Join<Flector> for Circle {
    type Output = AntiScalar;

    fn join(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Join<Infinity> for Circle {
    type Output = Plane;

    fn join(self, other: Infinity) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
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

impl Join<RoundPoint> for Circle {
    type Output = Sphere;

    fn join(self, other: RoundPoint) -> Sphere {
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

impl Join<Flector> for Dipole {
    type Output = Plane;

    fn join(self, other: Flector) -> Plane {
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

impl Join<Infinity> for Dipole {
    type Output = Line;

    fn join(self, other: Infinity) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
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

impl Join<Motor> for Dipole {
    type Output = AntiScalar;

    fn join(self, other: Motor) -> AntiScalar {
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

impl Join<Rotor> for Dipole {
    type Output = AntiScalar;

    fn join(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<RoundPoint> for Dipole {
    type Output = Circle;

    fn join(self, other: RoundPoint) -> Circle {
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

impl Join<Translator> for Dipole {
    type Output = AntiScalar;

    fn join(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<Circle> for Flector {
    type Output = AntiScalar;

    fn join(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Join<Dipole> for Flector {
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

impl Join<Magnitude> for Flector {
    type Output = Flector;

    fn join(self, other: Magnitude) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
                g1: self.group1() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Flector {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<RoundPoint> for Flector {
    type Output = Motor;

    fn join(self, other: RoundPoint) -> Motor {
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

impl Join<Scalar> for Flector {
    type Output = Flector;

    fn join(self, other: Scalar) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x4::from(other.group0()),
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

impl Join<RoundPoint> for Horizon {
    type Output = AntiScalar;

    fn join(self, other: RoundPoint) -> AntiScalar {
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

impl Join<Circle> for Infinity {
    type Output = Plane;

    fn join(self, other: Circle) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<Dipole> for Infinity {
    type Output = Line;

    fn join(self, other: Dipole) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Join<Magnitude> for Infinity {
    type Output = Infinity;

    fn join(self, other: Magnitude) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Join<MultiVector> for Infinity {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group10()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[0]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[0]]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()) * other.group3(),
                g8: Simd32x3::from(self.group0()) * other.group4(),
                g9: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]),
                g10: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group6()[3]]),
            },
        }
    }
}

impl Join<RoundPoint> for Infinity {
    type Output = Point;

    fn join(self, other: RoundPoint) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]),
            },
        }
    }
}

impl Join<Scalar> for Infinity {
    type Output = Infinity;

    fn join(self, other: Scalar) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Join<Sphere> for Infinity {
    type Output = AntiScalar;

    fn join(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group1()[0],
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

impl Join<RoundPoint> for Line {
    type Output = Plane;

    fn join(self, other: RoundPoint) -> Plane {
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

impl Join<RoundPoint> for LineAtInfinity {
    type Output = Plane;

    fn join(self, other: RoundPoint) -> Plane {
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

impl Join<RoundPoint> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn join(self, other: RoundPoint) -> PlaneAtOrigin {
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

impl Join<Flector> for Magnitude {
    type Output = Flector;

    fn join(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
                g1: Simd32x4::from(self.group0()[0]) * other.group1(),
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

impl Join<Infinity> for Magnitude {
    type Output = Infinity;

    fn join(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl Join<Motor> for Magnitude {
    type Output = Motor;

    fn join(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * other.group1(),
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

impl Join<Rotor> for Magnitude {
    type Output = Rotor;

    fn join(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Join<RoundPoint> for Magnitude {
    type Output = RoundPoint;

    fn join(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Join<Translator> for Magnitude {
    type Output = Translator;

    fn join(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Join<Dipole> for Motor {
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

impl Join<Magnitude> for Motor {
    type Output = Motor;

    fn join(self, other: Magnitude) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
                g1: self.group1() * Simd32x3::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Motor {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
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

impl Join<RoundPoint> for Motor {
    type Output = Plane;

    fn join(self, other: RoundPoint) -> Plane {
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

impl Join<Scalar> for Motor {
    type Output = Motor;

    fn join(self, other: Scalar) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
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

impl Join<Flector> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Flector) -> MultiVector {
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

impl Join<Infinity> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Infinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group10()[0]) * Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]])
                    * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g6: Simd32x4::from(0.0),
                g7: self.group3() * Simd32x3::from(other.group0()),
                g8: self.group4() * Simd32x3::from(other.group0()),
                g9: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group0()),
                g10: Simd32x2::from(self.group6()[3]) * Simd32x2::from([0.0, other.group0()]),
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

impl Join<Motor> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Motor) -> MultiVector {
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

impl Join<Rotor> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]])
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
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Join<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: RoundPoint) -> MultiVector {
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

impl Join<Translator> for MultiVector {
    type Output = MultiVector;

    fn join(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
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

impl Join<RoundPoint> for Origin {
    type Output = LineAtOrigin;

    fn join(self, other: RoundPoint) -> LineAtOrigin {
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

impl Join<RoundPoint> for Plane {
    type Output = AntiScalar;

    fn join(self, other: RoundPoint) -> AntiScalar {
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

impl Join<RoundPoint> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn join(self, other: RoundPoint) -> AntiScalar {
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

impl Join<RoundPoint> for Point {
    type Output = Line;

    fn join(self, other: RoundPoint) -> Line {
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

impl Join<RoundPoint> for PointAtInfinity {
    type Output = Line;

    fn join(self, other: RoundPoint) -> Line {
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

impl Join<Dipole> for Rotor {
    type Output = AntiScalar;

    fn join(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Join<Magnitude> for Rotor {
    type Output = Rotor;

    fn join(self, other: Magnitude) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Rotor {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Join<RoundPoint> for Rotor {
    type Output = PlaneAtOrigin;

    fn join(self, other: RoundPoint) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Join<Scalar> for Rotor {
    type Output = Rotor;

    fn join(self, other: Scalar) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Join<Circle> for RoundPoint {
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

impl Join<Dipole> for RoundPoint {
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

impl Join<Flector> for RoundPoint {
    type Output = Motor;

    fn join(self, other: Flector) -> Motor {
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

impl Join<Horizon> for RoundPoint {
    type Output = AntiScalar;

    fn join(self, other: Horizon) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl Join<Infinity> for RoundPoint {
    type Output = Point;

    fn join(self, other: Infinity) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Join<Line> for RoundPoint {
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

impl Join<LineAtInfinity> for RoundPoint {
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

impl Join<LineAtOrigin> for RoundPoint {
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

impl Join<Magnitude> for RoundPoint {
    type Output = RoundPoint;

    fn join(self, other: Magnitude) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x2::from(other.group0()[0]),
            },
        }
    }
}

impl Join<Motor> for RoundPoint {
    type Output = Plane;

    fn join(self, other: Motor) -> Plane {
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

impl Join<MultiVector> for RoundPoint {
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

impl Join<Origin> for RoundPoint {
    type Output = LineAtOrigin;

    fn join(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Join<Plane> for RoundPoint {
    type Output = AntiScalar;

    fn join(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl Join<PlaneAtOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn join(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<Point> for RoundPoint {
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

impl Join<PointAtInfinity> for RoundPoint {
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

impl Join<Rotor> for RoundPoint {
    type Output = PlaneAtOrigin;

    fn join(self, other: Rotor) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Join<RoundPoint> for RoundPoint {
    type Output = Dipole;

    fn join(self, other: RoundPoint) -> Dipole {
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

impl Join<Scalar> for RoundPoint {
    type Output = RoundPoint;

    fn join(self, other: Scalar) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Join<Sphere> for RoundPoint {
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

impl Join<Translator> for RoundPoint {
    type Output = Plane;

    fn join(self, other: Translator) -> Plane {
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

impl Join<Flector> for Scalar {
    type Output = Flector;

    fn join(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x4::from(self.group0()) * other.group1(),
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

impl Join<Infinity> for Scalar {
    type Output = Infinity;

    fn join(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl Join<Motor> for Scalar {
    type Output = Motor;

    fn join(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
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

impl Join<Rotor> for Scalar {
    type Output = Rotor;

    fn join(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<RoundPoint> for Scalar {
    type Output = RoundPoint;

    fn join(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Join<Translator> for Scalar {
    type Output = Translator;

    fn join(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Join<Infinity> for Sphere {
    type Output = AntiScalar;

    fn join(self, other: Infinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0(),
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

impl Join<RoundPoint> for Sphere {
    type Output = AntiScalar;

    fn join(self, other: RoundPoint) -> AntiScalar {
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

impl Join<Dipole> for Translator {
    type Output = AntiScalar;

    fn join(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Join<Magnitude> for Translator {
    type Output = Translator;

    fn join(self, other: Magnitude) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Join<MultiVector> for Translator {
    type Output = MultiVector;

    fn join(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]),
                g9: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group2()[0]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]]),
            },
        }
    }
}

impl Join<RoundPoint> for Translator {
    type Output = Plane;

    fn join(self, other: RoundPoint) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[0], 0.0, 0.0, other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, -other.group1()[0], 0.0, other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, -other.group1()[0], other.group0()[2]]),
            },
        }
    }
}

impl Join<Scalar> for Translator {
    type Output = Translator;

    fn join(self, other: Scalar) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
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

impl Meet<Flector> for AntiScalar {
    type Output = Flector;

    fn meet(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x4::from(self.group0()) * other.group1(),
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

impl Meet<Infinity> for AntiScalar {
    type Output = Infinity;

    fn meet(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl Meet<Motor> for AntiScalar {
    type Output = Motor;

    fn meet(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
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

impl Meet<Rotor> for AntiScalar {
    type Output = Rotor;

    fn meet(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Meet<RoundPoint> for AntiScalar {
    type Output = RoundPoint;

    fn meet(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Meet<Translator> for AntiScalar {
    type Output = Translator;

    fn meet(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
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
    type Output = RoundPoint;

    fn meet(self, other: Circle) -> RoundPoint {
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

impl Meet<Flector> for Circle {
    type Output = MultiVector;

    fn meet(self, other: Flector) -> MultiVector {
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
    type Output = RoundPoint;

    fn meet(self, other: Line) -> RoundPoint {
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

impl Meet<LineAtInfinity> for Circle {
    type Output = RoundPoint;

    fn meet(self, other: LineAtInfinity) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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
    type Output = RoundPoint;

    fn meet(self, other: LineAtOrigin) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Meet<Motor> for Circle {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
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

impl Meet<Rotor> for Circle {
    type Output = MultiVector;

    fn meet(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
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

impl Meet<Translator> for Circle {
    type Output = MultiVector;

    fn meet(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
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

impl Meet<Flector> for Dipole {
    type Output = RoundPoint;

    fn meet(self, other: Flector) -> RoundPoint {
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

impl Meet<Horizon> for Dipole {
    type Output = RoundPoint;

    fn meet(self, other: Horizon) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Meet<Motor> for Dipole {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
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
    type Output = RoundPoint;

    fn meet(self, other: Plane) -> RoundPoint {
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

impl Meet<PlaneAtOrigin> for Dipole {
    type Output = RoundPoint;

    fn meet(self, other: PlaneAtOrigin) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Meet<Rotor> for Dipole {
    type Output = MultiVector;

    fn meet(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
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

impl Meet<Sphere> for Dipole {
    type Output = RoundPoint;

    fn meet(self, other: Sphere) -> RoundPoint {
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

impl Meet<Translator> for Dipole {
    type Output = MultiVector;

    fn meet(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
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

impl Meet<AntiScalar> for Flector {
    type Output = Flector;

    fn meet(self, other: AntiScalar) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Flector {
    type Output = MultiVector;

    fn meet(self, other: Circle) -> MultiVector {
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

impl Meet<Dipole> for Flector {
    type Output = RoundPoint;

    fn meet(self, other: Dipole) -> RoundPoint {
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

impl Meet<Flector> for Flector {
    type Output = MultiVector;

    fn meet(self, other: Flector) -> MultiVector {
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

impl Meet<Horizon> for Flector {
    type Output = MultiVector;

    fn meet(self, other: Horizon) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Line> for Flector {
    type Output = Point;

    fn meet(self, other: Line) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Meet<LineAtInfinity> for Flector {
    type Output = PointAtInfinity;

    fn meet(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Flector {
    type Output = Point;

    fn meet(self, other: LineAtOrigin) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Magnitude> for Flector {
    type Output = Flector;

    fn meet(self, other: Magnitude) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[1]),
                g1: self.group1() * Simd32x4::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<Motor> for Flector {
    type Output = Flector;

    fn meet(self, other: Motor) -> Flector {
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

impl Meet<MultiVector> for Flector {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Origin> for Flector {
    type Output = Infinity;

    fn meet(self, other: Origin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group1()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Plane> for Flector {
    type Output = MultiVector;

    fn meet(self, other: Plane) -> MultiVector {
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

impl Meet<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn meet(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g8: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * other.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Point> for Flector {
    type Output = Infinity;

    fn meet(self, other: Point) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] - self.group1()[3] * other.group0()[3],
            },
        }
    }
}

impl Meet<PointAtInfinity> for Flector {
    type Output = Infinity;

    fn meet(self, other: PointAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Rotor> for Flector {
    type Output = Flector;

    fn meet(self, other: Rotor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
                g1: self.group1() * Simd32x4::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<RoundPoint> for Flector {
    type Output = Scalar;

    fn meet(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] + self.group1()[3] * other.group1()[0],
            },
        }
    }
}

impl Meet<Sphere> for Flector {
    type Output = MultiVector;

    fn meet(self, other: Sphere) -> MultiVector {
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

impl Meet<Translator> for Flector {
    type Output = Flector;

    fn meet(self, other: Translator) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g1: self.group1() * Simd32x4::from(other.group0()[3]),
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
    type Output = RoundPoint;

    fn meet(self, other: Dipole) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group2()[3]]),
            },
        }
    }
}

impl Meet<Flector> for Horizon {
    type Output = MultiVector;

    fn meet(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group0()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Meet<Motor> for Horizon {
    type Output = Flector;

    fn meet(self, other: Motor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
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
    type Output = Infinity;

    fn meet(self, other: Origin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0() * other.group0(),
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
    type Output = Infinity;

    fn meet(self, other: Point) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl Meet<Rotor> for Horizon {
    type Output = Flector;

    fn meet(self, other: Rotor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            },
        }
    }
}

impl Meet<RoundPoint> for Horizon {
    type Output = Scalar;

    fn meet(self, other: RoundPoint) -> Scalar {
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

impl Meet<Translator> for Horizon {
    type Output = Horizon;

    fn meet(self, other: Translator) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Meet<AntiScalar> for Infinity {
    type Output = Infinity;

    fn meet(self, other: AntiScalar) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Meet<Magnitude> for Infinity {
    type Output = Infinity;

    fn meet(self, other: Magnitude) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl Meet<Motor> for Infinity {
    type Output = Infinity;

    fn meet(self, other: Motor) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Meet<MultiVector> for Infinity {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group10()[0], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[1]]),
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

impl Meet<Rotor> for Infinity {
    type Output = Infinity;

    fn meet(self, other: Rotor) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Meet<Sphere> for Infinity {
    type Output = Scalar;

    fn meet(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group1()[0],
            },
        }
    }
}

impl Meet<Translator> for Infinity {
    type Output = Infinity;

    fn meet(self, other: Translator) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[3],
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
    type Output = RoundPoint;

    fn meet(self, other: Circle) -> RoundPoint {
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

impl Meet<Flector> for Line {
    type Output = Point;

    fn meet(self, other: Flector) -> Point {
        Point {
            groups: PointGroups {
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
    type Output = Infinity;

    fn meet(self, other: Line) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl Meet<LineAtInfinity> for Line {
    type Output = Infinity;

    fn meet(self, other: LineAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<LineAtOrigin> for Line {
    type Output = Infinity;

    fn meet(self, other: LineAtOrigin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
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

impl Meet<Motor> for Line {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
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

impl Meet<Rotor> for Line {
    type Output = MultiVector;

    fn meet(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
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

impl Meet<Translator> for Line {
    type Output = MultiVector;

    fn meet(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
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
    type Output = RoundPoint;

    fn meet(self, other: Circle) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Meet<Flector> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn meet(self, other: Flector) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]),
            },
        }
    }
}

impl Meet<Line> for LineAtInfinity {
    type Output = Infinity;

    fn meet(self, other: Line) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<LineAtOrigin> for LineAtInfinity {
    type Output = Infinity;

    fn meet(self, other: LineAtOrigin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl Meet<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Meet<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn meet(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Meet<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn meet(self, other: Translator) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
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
    type Output = RoundPoint;

    fn meet(self, other: Circle) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Meet<Flector> for LineAtOrigin {
    type Output = Point;

    fn meet(self, other: Flector) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[3], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[3], -other.group1()[2]]),
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
    type Output = Infinity;

    fn meet(self, other: Line) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Meet<LineAtInfinity> for LineAtOrigin {
    type Output = Infinity;

    fn meet(self, other: LineAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl Meet<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Meet<Rotor> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn meet(self, other: Rotor) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
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

impl Meet<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn meet(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0() * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Meet<Flector> for Magnitude {
    type Output = Flector;

    fn meet(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[1]) * other.group0(),
                g1: Simd32x4::from(self.group0()[1]) * other.group1(),
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

impl Meet<Infinity> for Magnitude {
    type Output = Infinity;

    fn meet(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl Meet<Motor> for Magnitude {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
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

impl Meet<Rotor> for Magnitude {
    type Output = MultiVector;

    fn meet(self, other: Rotor) -> MultiVector {
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
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<RoundPoint> for Magnitude {
    type Output = RoundPoint;

    fn meet(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Meet<Translator> for Magnitude {
    type Output = MultiVector;

    fn meet(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()[3]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<AntiScalar> for Motor {
    type Output = Motor;

    fn meet(self, other: AntiScalar) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Motor {
    type Output = MultiVector;

    fn meet(self, other: Circle) -> MultiVector {
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

impl Meet<Dipole> for Motor {
    type Output = MultiVector;

    fn meet(self, other: Dipole) -> MultiVector {
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

impl Meet<Flector> for Motor {
    type Output = Flector;

    fn meet(self, other: Flector) -> Flector {
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

impl Meet<Horizon> for Motor {
    type Output = Flector;

    fn meet(self, other: Horizon) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Meet<Infinity> for Motor {
    type Output = Infinity;

    fn meet(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Line> for Motor {
    type Output = MultiVector;

    fn meet(self, other: Line) -> MultiVector {
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

impl Meet<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn meet(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[3]) * other.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn meet(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[3]) * other.group0(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Magnitude> for Motor {
    type Output = MultiVector;

    fn meet(self, other: Magnitude) -> MultiVector {
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

impl Meet<Motor> for Motor {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
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

impl Meet<MultiVector> for Motor {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
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

impl Meet<Origin> for Motor {
    type Output = Origin;

    fn meet(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Plane> for Motor {
    type Output = Flector;

    fn meet(self, other: Plane) -> Flector {
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

impl Meet<PlaneAtOrigin> for Motor {
    type Output = Flector;

    fn meet(self, other: PlaneAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Meet<Point> for Motor {
    type Output = Point;

    fn meet(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<PointAtInfinity> for Motor {
    type Output = PointAtInfinity;

    fn meet(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<Rotor> for Motor {
    type Output = MultiVector;

    fn meet(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: self.group1() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<RoundPoint> for Motor {
    type Output = RoundPoint;

    fn meet(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
                g1: Simd32x2::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl Meet<Scalar> for Motor {
    type Output = Scalar;

    fn meet(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Sphere> for Motor {
    type Output = MultiVector;

    fn meet(self, other: Sphere) -> MultiVector {
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

impl Meet<Translator> for Motor {
    type Output = MultiVector;

    fn meet(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    + self.group1() * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Meet<Flector> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Flector) -> MultiVector {
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

impl Meet<Infinity> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Infinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group10()[0]) * Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()]),
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

impl Meet<Motor> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
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

impl Meet<Rotor> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: self.group1() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group6()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g2: self.group2() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group6()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group6()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group6()[2]) * Simd32x2::from([-other.group0()[2], 0.0])
                    + Simd32x2::from(self.group8()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group8()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group8()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: self.group3() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group10()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g4: self.group4() * Simd32x3::from(other.group0()[3]),
                g5: self.group5() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
                    + Simd32x4::from(self.group10()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: self.group6() * Simd32x4::from(other.group0()[3]),
                g7: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    + self.group7() * Simd32x3::from(other.group0()[3]),
                g8: self.group8() * Simd32x3::from(other.group0()[3]),
                g9: self.group9() * Simd32x3::from(other.group0()[3]),
                g10: self.group10() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: RoundPoint) -> MultiVector {
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

impl Meet<Translator> for MultiVector {
    type Output = MultiVector;

    fn meet(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
                g1: self.group1() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group6()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group6()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g2: self.group2() * Simd32x2::from(other.group0()[3])
                    + Simd32x2::from(self.group7()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group7()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group7()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: self.group3() * Simd32x3::from(other.group0()[3]),
                g4: self.group4() * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group10()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g5: self.group5() * Simd32x4::from(other.group0()[3])
                    + Simd32x4::from(self.group9()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group9()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g6: self.group6() * Simd32x4::from(other.group0()[3]),
                g7: self.group7() * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    + self.group8() * Simd32x3::from(other.group0()[3]),
                g9: self.group9() * Simd32x3::from(other.group0()[3]),
                g10: self.group10() * Simd32x2::from(other.group0()[3]),
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

impl Meet<Flector> for Origin {
    type Output = Infinity;

    fn meet(self, other: Flector) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group1()[3],
            },
        }
    }
}

impl Meet<Horizon> for Origin {
    type Output = Infinity;

    fn meet(self, other: Horizon) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0(),
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

impl Meet<Motor> for Origin {
    type Output = Origin;

    fn meet(self, other: Motor) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[3],
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
    type Output = Infinity;

    fn meet(self, other: Plane) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Meet<Rotor> for Origin {
    type Output = Origin;

    fn meet(self, other: Rotor) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Meet<Sphere> for Origin {
    type Output = RoundPoint;

    fn meet(self, other: Sphere) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(self.group0()) * other.group1() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl Meet<Translator> for Origin {
    type Output = Origin;

    fn meet(self, other: Translator) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0()[3],
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
    type Output = RoundPoint;

    fn meet(self, other: Dipole) -> RoundPoint {
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

impl Meet<Flector> for Plane {
    type Output = MultiVector;

    fn meet(self, other: Flector) -> MultiVector {
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

impl Meet<Motor> for Plane {
    type Output = Flector;

    fn meet(self, other: Motor) -> Flector {
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
    type Output = Infinity;

    fn meet(self, other: Origin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
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
    type Output = Infinity;

    fn meet(self, other: Point) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Meet<PointAtInfinity> for Plane {
    type Output = Infinity;

    fn meet(self, other: PointAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Rotor> for Plane {
    type Output = Flector;

    fn meet(self, other: Rotor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
                g1: self.group0() * Simd32x4::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<RoundPoint> for Plane {
    type Output = Scalar;

    fn meet(self, other: RoundPoint) -> Scalar {
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

impl Meet<Translator> for Plane {
    type Output = Flector;

    fn meet(self, other: Translator) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g1: self.group0() * Simd32x4::from(other.group0()[3]),
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
    type Output = RoundPoint;

    fn meet(self, other: Dipole) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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

impl Meet<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn meet(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g8: self.group0() * Simd32x3::from(other.group1()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Meet<Motor> for PlaneAtOrigin {
    type Output = Flector;

    fn meet(self, other: Motor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]),
                g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]),
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
    type Output = Infinity;

    fn meet(self, other: Point) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<PointAtInfinity> for PlaneAtOrigin {
    type Output = Infinity;

    fn meet(self, other: PointAtInfinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Rotor> for PlaneAtOrigin {
    type Output = Flector;

    fn meet(self, other: Rotor) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]),
                g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]),
            },
        }
    }
}

impl Meet<RoundPoint> for PlaneAtOrigin {
    type Output = Scalar;

    fn meet(self, other: RoundPoint) -> Scalar {
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

impl Meet<Translator> for PlaneAtOrigin {
    type Output = Flector;

    fn meet(self, other: Translator) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]),
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

impl Meet<Flector> for Point {
    type Output = Infinity;

    fn meet(self, other: Flector) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] + self.group0()[3] * other.group1()[3],
            },
        }
    }
}

impl Meet<Horizon> for Point {
    type Output = Infinity;

    fn meet(self, other: Horizon) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[3] * other.group0(),
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

impl Meet<Motor> for Point {
    type Output = Point;

    fn meet(self, other: Motor) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3]),
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
    type Output = Infinity;

    fn meet(self, other: Plane) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for Point {
    type Output = Infinity;

    fn meet(self, other: PlaneAtOrigin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Rotor> for Point {
    type Output = Point;

    fn meet(self, other: Rotor) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<Sphere> for Point {
    type Output = RoundPoint;

    fn meet(self, other: Sphere) -> RoundPoint {
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

impl Meet<Translator> for Point {
    type Output = Point;

    fn meet(self, other: Translator) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[3]),
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

impl Meet<Flector> for PointAtInfinity {
    type Output = Infinity;

    fn meet(self, other: Flector) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
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

impl Meet<Motor> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn meet(self, other: Motor) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
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
    type Output = Infinity;

    fn meet(self, other: Plane) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for PointAtInfinity {
    type Output = Infinity;

    fn meet(self, other: PlaneAtOrigin) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Rotor> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn meet(self, other: Rotor) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<Sphere> for PointAtInfinity {
    type Output = RoundPoint;

    fn meet(self, other: Sphere) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group1()[0]),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]),
            },
        }
    }
}

impl Meet<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn meet(self, other: Translator) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<AntiScalar> for Rotor {
    type Output = Rotor;

    fn meet(self, other: AntiScalar) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Rotor {
    type Output = MultiVector;

    fn meet(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]),
                g2: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]),
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

impl Meet<Dipole> for Rotor {
    type Output = MultiVector;

    fn meet(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], 0.0]),
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

impl Meet<Flector> for Rotor {
    type Output = Flector;

    fn meet(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], 0.0, 0.0, -other.group1()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[3], 0.0, -other.group1()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[3], -other.group1()[2]])
                    + Simd32x4::from(self.group0()[3]) * other.group0(),
                g1: Simd32x4::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl Meet<Horizon> for Rotor {
    type Output = Flector;

    fn meet(self, other: Horizon) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Meet<Infinity> for Rotor {
    type Output = Infinity;

    fn meet(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Line> for Rotor {
    type Output = MultiVector;

    fn meet(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
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

impl Meet<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn meet(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[3]) * other.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn meet(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<Magnitude> for Rotor {
    type Output = MultiVector;

    fn meet(self, other: Magnitude) -> MultiVector {
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
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Motor> for Rotor {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3])
                    + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(self.group0()[3]) * other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<MultiVector> for Rotor {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group4()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group4()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group4()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * other.group0(),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group6()[3]) + Simd32x3::from(self.group0()[3]) * other.group1(),
                g2: Simd32x2::from(0.0)
                    - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group6()[0], other.group8()[0]])
                    - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group6()[1], other.group8()[1]])
                    - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group6()[2], other.group8()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group2(),
                g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group10()[0]) + Simd32x3::from(self.group0()[3]) * other.group3(),
                g4: Simd32x3::from(self.group0()[3]) * other.group4(),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group10()[1], 0.0, 0.0, -other.group9()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group10()[1], 0.0, -other.group9()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group10()[1], -other.group9()[2]])
                    + Simd32x4::from(self.group0()[3]) * other.group5(),
                g6: Simd32x4::from(self.group0()[3]) * other.group6(),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]) + Simd32x3::from(self.group0()[3]) * other.group7(),
                g8: Simd32x3::from(self.group0()[3]) * other.group8(),
                g9: Simd32x3::from(self.group0()[3]) * other.group9(),
                g10: Simd32x2::from(self.group0()[3]) * other.group10(),
            },
        }
    }
}

impl Meet<Origin> for Rotor {
    type Output = Origin;

    fn meet(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Plane> for Rotor {
    type Output = Flector;

    fn meet(self, other: Plane) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]),
                g1: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for Rotor {
    type Output = Flector;

    fn meet(self, other: PlaneAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Meet<Point> for Rotor {
    type Output = Point;

    fn meet(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<PointAtInfinity> for Rotor {
    type Output = PointAtInfinity;

    fn meet(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<Rotor> for Rotor {
    type Output = Rotor;

    fn meet(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0])
                    + Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<RoundPoint> for Rotor {
    type Output = RoundPoint;

    fn meet(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
                g1: Simd32x2::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl Meet<Scalar> for Rotor {
    type Output = Scalar;

    fn meet(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Sphere> for Rotor {
    type Output = MultiVector;

    fn meet(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[1], 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group1()[1], 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group1()[1], -other.group0()[2]]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[3]) * other.group0(),
                g10: Simd32x2::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl Meet<Translator> for Rotor {
    type Output = MultiVector;

    fn meet(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]),
                g8: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<AntiScalar> for RoundPoint {
    type Output = RoundPoint;

    fn meet(self, other: AntiScalar) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x2::from(other.group0()),
            },
        }
    }
}

impl Meet<Flector> for RoundPoint {
    type Output = Scalar;

    fn meet(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] + self.group1()[0] * other.group1()[3],
            },
        }
    }
}

impl Meet<Horizon> for RoundPoint {
    type Output = Scalar;

    fn meet(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl Meet<Magnitude> for RoundPoint {
    type Output = RoundPoint;

    fn meet(self, other: Magnitude) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[1]),
                g1: self.group1() * Simd32x2::from(other.group0()[1]),
            },
        }
    }
}

impl Meet<Motor> for RoundPoint {
    type Output = RoundPoint;

    fn meet(self, other: Motor) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
                g1: self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<MultiVector> for RoundPoint {
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

impl Meet<Plane> for RoundPoint {
    type Output = Scalar;

    fn meet(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for RoundPoint {
    type Output = Scalar;

    fn meet(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Meet<Rotor> for RoundPoint {
    type Output = RoundPoint;

    fn meet(self, other: Rotor) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
                g1: self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<Sphere> for RoundPoint {
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

impl Meet<Translator> for RoundPoint {
    type Output = RoundPoint;

    fn meet(self, other: Translator) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[3]),
                g1: self.group1() * Simd32x2::from(other.group0()[3]),
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

impl Meet<Motor> for Scalar {
    type Output = Scalar;

    fn meet(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
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

impl Meet<Rotor> for Scalar {
    type Output = Scalar;

    fn meet(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Meet<Translator> for Scalar {
    type Output = Scalar;

    fn meet(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
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
    type Output = RoundPoint;

    fn meet(self, other: Dipole) -> RoundPoint {
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

impl Meet<Flector> for Sphere {
    type Output = MultiVector;

    fn meet(self, other: Flector) -> MultiVector {
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

impl Meet<Infinity> for Sphere {
    type Output = Scalar;

    fn meet(self, other: Infinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0(),
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

impl Meet<Motor> for Sphere {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
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
    type Output = RoundPoint;

    fn meet(self, other: Origin) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
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
    type Output = RoundPoint;

    fn meet(self, other: Point) -> RoundPoint {
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

impl Meet<PointAtInfinity> for Sphere {
    type Output = RoundPoint;

    fn meet(self, other: PointAtInfinity) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group1()[0]) * other.group0(),
                g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
            },
        }
    }
}

impl Meet<Rotor> for Sphere {
    type Output = MultiVector;

    fn meet(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]])
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

impl Meet<RoundPoint> for Sphere {
    type Output = Scalar;

    fn meet(self, other: RoundPoint) -> Scalar {
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

impl Meet<Translator> for Sphere {
    type Output = MultiVector;

    fn meet(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0() * Simd32x3::from(other.group0()[3]),
                g10: self.group1() * Simd32x2::from(other.group0()[3]),
            },
        }
    }
}

impl Meet<AntiScalar> for Translator {
    type Output = Translator;

    fn meet(self, other: AntiScalar) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}

impl Meet<Circle> for Translator {
    type Output = MultiVector;

    fn meet(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]),
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

impl Meet<Dipole> for Translator {
    type Output = MultiVector;

    fn meet(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]),
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

impl Meet<Flector> for Translator {
    type Output = Flector;

    fn meet(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group1()[2], other.group1()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], 0.0, -other.group1()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group0()[3]) * other.group0(),
                g1: Simd32x4::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl Meet<Horizon> for Translator {
    type Output = Horizon;

    fn meet(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Infinity> for Translator {
    type Output = Infinity;

    fn meet(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Line> for Translator {
    type Output = MultiVector;

    fn meet(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
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

impl Meet<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn meet(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn meet(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[3]) * other.group0(),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Magnitude> for Translator {
    type Output = MultiVector;

    fn meet(self, other: Magnitude) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<Motor> for Translator {
    type Output = MultiVector;

    fn meet(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<MultiVector> for Translator {
    type Output = MultiVector;

    fn meet(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group3()[0], 0.0])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group3()[1], 0.0])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group3()[2], 0.0])
                    + Simd32x2::from(self.group0()[3]) * other.group0(),
                g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group6()[2], other.group6()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group6()[2], 0.0, -other.group6()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group6()[1], other.group6()[0], 0.0])
                    + Simd32x3::from(self.group0()[3]) * other.group1(),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group7()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group7()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group7()[2]])
                    + Simd32x2::from(self.group0()[3]) * other.group2(),
                g3: Simd32x3::from(self.group0()[3]) * other.group3(),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group10()[0]) + Simd32x3::from(self.group0()[3]) * other.group4(),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group9()[2], other.group9()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group9()[2], 0.0, -other.group9()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group9()[1], other.group9()[0], 0.0, 0.0])
                    + Simd32x4::from(self.group0()[3]) * other.group5(),
                g6: Simd32x4::from(self.group0()[3]) * other.group6(),
                g7: Simd32x3::from(self.group0()[3]) * other.group7(),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]) + Simd32x3::from(self.group0()[3]) * other.group8(),
                g9: Simd32x3::from(self.group0()[3]) * other.group9(),
                g10: Simd32x2::from(self.group0()[3]) * other.group10(),
            },
        }
    }
}

impl Meet<Origin> for Translator {
    type Output = Origin;

    fn meet(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Plane> for Translator {
    type Output = Flector;

    fn meet(self, other: Plane) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<PlaneAtOrigin> for Translator {
    type Output = Flector;

    fn meet(self, other: PlaneAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Meet<Point> for Translator {
    type Output = Point;

    fn meet(self, other: Point) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<PointAtInfinity> for Translator {
    type Output = PointAtInfinity;

    fn meet(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
            },
        }
    }
}

impl Meet<Rotor> for Translator {
    type Output = MultiVector;

    fn meet(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Meet<RoundPoint> for Translator {
    type Output = RoundPoint;

    fn meet(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(self.group0()[3]) * other.group0(),
                g1: Simd32x2::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl Meet<Scalar> for Translator {
    type Output = Scalar;

    fn meet(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Meet<Sphere> for Translator {
    type Output = MultiVector;

    fn meet(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]),
                g5: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[3]) * other.group0(),
                g10: Simd32x2::from(self.group0()[3]) * other.group1(),
            },
        }
    }
}

impl Meet<Translator> for Translator {
    type Output = Translator;

    fn meet(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0])
                    + Simd32x4::from(self.group0()[3]) * other.group0(),
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

impl Wedge<Infinity> for Circle {
    type Output = Plane;

    fn wedge(self, other: Infinity) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
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

impl Wedge<Infinity> for Dipole {
    type Output = Line;

    fn wedge(self, other: Infinity) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(other.group0()),
                g1: self.group1() * Simd32x3::from(other.group0()),
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

impl Wedge<Rotor> for Dipole {
    type Output = AntiScalar;

    fn wedge(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
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

impl Wedge<Translator> for Dipole {
    type Output = AntiScalar;

    fn wedge(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl Wedge<Magnitude> for Flector {
    type Output = Flector;

    fn wedge(self, other: Magnitude) -> Flector {
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

impl Wedge<RoundPoint> for Horizon {
    type Output = AntiScalar;

    fn wedge(self, other: RoundPoint) -> AntiScalar {
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

impl Wedge<Circle> for Infinity {
    type Output = Plane;

    fn wedge(self, other: Circle) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<Dipole> for Infinity {
    type Output = Line;

    fn wedge(self, other: Dipole) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(self.group0()) * other.group0(),
                g1: Simd32x3::from(self.group0()) * other.group1(),
            },
        }
    }
}

impl Wedge<Magnitude> for Infinity {
    type Output = Infinity;

    fn wedge(self, other: Magnitude) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Wedge<MultiVector> for Infinity {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group10()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[0]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[0]]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(self.group0()) * other.group3(),
                g8: Simd32x3::from(self.group0()) * other.group4(),
                g9: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]),
                g10: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, -other.group6()[3]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for Infinity {
    type Output = Point;

    fn wedge(self, other: RoundPoint) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]),
            },
        }
    }
}

impl Wedge<Scalar> for Infinity {
    type Output = Infinity;

    fn wedge(self, other: Scalar) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Wedge<Sphere> for Infinity {
    type Output = AntiScalar;

    fn wedge(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group1()[0],
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

impl Wedge<RoundPoint> for LineAtInfinity {
    type Output = Plane;

    fn wedge(self, other: RoundPoint) -> Plane {
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

impl Wedge<RoundPoint> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: RoundPoint) -> PlaneAtOrigin {
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

impl Wedge<Flector> for Magnitude {
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

impl Wedge<Infinity> for Magnitude {
    type Output = Infinity;

    fn wedge(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl Wedge<Motor> for Magnitude {
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

impl Wedge<Rotor> for Magnitude {
    type Output = Rotor;

    fn wedge(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
            },
        }
    }
}

impl Wedge<RoundPoint> for Magnitude {
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

impl Wedge<Translator> for Magnitude {
    type Output = Translator;

    fn wedge(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from(self.group0()[0]) * other.group0(),
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

impl Wedge<Magnitude> for Motor {
    type Output = Motor;

    fn wedge(self, other: Magnitude) -> Motor {
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

impl Wedge<Infinity> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Infinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group10()[0]) * Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]])
                    * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g6: Simd32x4::from(0.0),
                g7: self.group3() * Simd32x3::from(other.group0()),
                g8: self.group4() * Simd32x3::from(other.group0()),
                g9: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from(other.group0()),
                g10: Simd32x2::from(self.group6()[3]) * Simd32x2::from([0.0, other.group0()]),
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

impl Wedge<Rotor> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]])
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
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
                g10: Simd32x2::from(0.0),
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

impl Wedge<Translator> for MultiVector {
    type Output = MultiVector;

    fn wedge(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]])
                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]),
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

impl Wedge<RoundPoint> for Origin {
    type Output = LineAtOrigin;

    fn wedge(self, other: RoundPoint) -> LineAtOrigin {
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

impl Wedge<RoundPoint> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn wedge(self, other: RoundPoint) -> AntiScalar {
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

impl Wedge<RoundPoint> for Point {
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

impl Wedge<RoundPoint> for PointAtInfinity {
    type Output = Line;

    fn wedge(self, other: RoundPoint) -> Line {
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

impl Wedge<Dipole> for Rotor {
    type Output = AntiScalar;

    fn wedge(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Wedge<Magnitude> for Rotor {
    type Output = Rotor;

    fn wedge(self, other: Magnitude) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for Rotor {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group4()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group4()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group4()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Wedge<RoundPoint> for Rotor {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: RoundPoint) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
            },
        }
    }
}

impl Wedge<Scalar> for Rotor {
    type Output = Rotor;

    fn wedge(self, other: Scalar) -> Rotor {
        Rotor {
            groups: RotorGroups {
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

impl Wedge<Horizon> for RoundPoint {
    type Output = AntiScalar;

    fn wedge(self, other: Horizon) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl Wedge<Infinity> for RoundPoint {
    type Output = Point;

    fn wedge(self, other: Infinity) -> Point {
        Point {
            groups: PointGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]])
                    * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0])
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
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

impl Wedge<LineAtInfinity> for RoundPoint {
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

impl Wedge<LineAtOrigin> for RoundPoint {
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

impl Wedge<Magnitude> for RoundPoint {
    type Output = RoundPoint;

    fn wedge(self, other: Magnitude) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(other.group0()[0]),
                g1: self.group1() * Simd32x2::from(other.group0()[0]),
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

impl Wedge<Origin> for RoundPoint {
    type Output = LineAtOrigin;

    fn wedge(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()),
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

impl Wedge<PlaneAtOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn wedge(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<Point> for RoundPoint {
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

impl Wedge<PointAtInfinity> for RoundPoint {
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

impl Wedge<Rotor> for RoundPoint {
    type Output = PlaneAtOrigin;

    fn wedge(self, other: Rotor) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]])
                    + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]])
                    + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]),
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

impl Wedge<Translator> for RoundPoint {
    type Output = Plane;

    fn wedge(self, other: Translator) -> Plane {
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

impl Wedge<Infinity> for Scalar {
    type Output = Infinity;

    fn wedge(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
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

impl Wedge<Rotor> for Scalar {
    type Output = Rotor;

    fn wedge(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
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

impl Wedge<Translator> for Scalar {
    type Output = Translator;

    fn wedge(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from(self.group0()) * other.group0(),
            },
        }
    }
}

impl Wedge<Infinity> for Sphere {
    type Output = AntiScalar;

    fn wedge(self, other: Infinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0(),
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

impl Wedge<Dipole> for Translator {
    type Output = AntiScalar;

    fn wedge(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Wedge<Magnitude> for Translator {
    type Output = Translator;

    fn wedge(self, other: Magnitude) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()[0]),
            },
        }
    }
}

impl Wedge<MultiVector> for Translator {
    type Output = MultiVector;

    fn wedge(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]])
                    + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]),
                g9: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group2()[0]),
                g10: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group1()[0]])
                    + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group1()[1]])
                    + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group1()[2]]),
            },
        }
    }
}

impl Wedge<RoundPoint> for Translator {
    type Output = Plane;

    fn wedge(self, other: RoundPoint) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[0], 0.0, 0.0, other.group0()[0]])
                    + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, -other.group1()[0], 0.0, other.group0()[1]])
                    + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, -other.group1()[0], other.group0()[2]]),
            },
        }
    }
}

impl Wedge<Scalar> for Translator {
    type Output = Translator;

    fn wedge(self, other: Scalar) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from(other.group0()),
            },
        }
    }
}
