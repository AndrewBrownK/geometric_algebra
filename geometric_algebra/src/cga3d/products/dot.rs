#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;

/// Dot Product
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products
pub trait Dot<T> {
    type Output;
    fn dot(self, other: T) -> Self::Output;
}

/// Anti-Dot Product
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products
pub trait AntiDot<T> {
    type Output;
    fn anti_dot(self, other: T) -> Self::Output;
}

impl AntiDot<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<Magnitude> for AntiScalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl AntiDot<MultiVector> for AntiScalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Circle> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group2()[0]
                    - self.group0()[1] * other.group2()[1]
                    - self.group0()[2] * other.group2()[2]
                    + self.group0()[3] * other.group0()[3]
                    + self.group1()[0] * other.group1()[0]
                    + self.group1()[1] * other.group1()[1]
                    + self.group1()[2] * other.group1()[2]
                    - self.group2()[0] * other.group0()[0]
                    - self.group2()[1] * other.group0()[1]
                    - self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Line> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group1()[0]
                    - self.group0()[1] * other.group1()[1]
                    - self.group0()[2] * other.group1()[2]
                    + self.group1()[0] * other.group0()[0]
                    + self.group1()[1] * other.group0()[1]
                    + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtInfinity> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtOrigin> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0]
                    + self.group1()[1] * other.group0()[1]
                    + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group8()[0]
                    - self.group0()[1] * other.group8()[1]
                    - self.group0()[2] * other.group8()[2]
                    + self.group0()[3] * other.group6()[3]
                    + self.group1()[0] * other.group7()[0]
                    + self.group1()[1] * other.group7()[1]
                    + self.group1()[2] * other.group7()[2]
                    - self.group2()[0] * other.group6()[0]
                    - self.group2()[1] * other.group6()[1]
                    - self.group2()[2] * other.group6()[2],
            },
        }
    }
}

impl AntiDot<Dipole> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group2()[0]
                    + self.group0()[1] * other.group2()[1]
                    + self.group0()[2] * other.group2()[2]
                    + self.group1()[0] * other.group1()[0]
                    + self.group1()[1] * other.group1()[1]
                    + self.group1()[2] * other.group1()[2]
                    + self.group2()[0] * other.group0()[0]
                    + self.group2()[1] * other.group0()[1]
                    + self.group2()[2] * other.group0()[2]
                    + self.group2()[3] * other.group2()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group5()[0]
                    + self.group0()[1] * other.group5()[1]
                    + self.group0()[2] * other.group5()[2]
                    + self.group1()[0] * other.group4()[0]
                    + self.group1()[1] * other.group4()[1]
                    + self.group1()[2] * other.group4()[2]
                    + self.group2()[0] * other.group3()[0]
                    + self.group2()[1] * other.group3()[1]
                    + self.group2()[2] * other.group3()[2]
                    + self.group2()[3] * other.group5()[3],
            },
        }
    }
}

impl AntiDot<Origin> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group2()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Point> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    + self.group2()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<PointAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: PointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for Horizon {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Sphere> for Horizon {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group1()[0],
            },
        }
    }
}

impl AntiDot<Circle> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0]
                    + self.group0()[1] * other.group1()[1]
                    + self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group0()[0]
                    - self.group1()[1] * other.group0()[1]
                    - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Line> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group7()[0]
                    + self.group0()[1] * other.group7()[1]
                    + self.group0()[2] * other.group7()[2]
                    - self.group1()[0] * other.group6()[0]
                    - self.group1()[1] * other.group6()[1]
                    - self.group1()[2] * other.group6()[2],
            },
        }
    }
}

impl AntiDot<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for LineAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group6()[0]
                    - self.group0()[1] * other.group6()[1]
                    - self.group0()[2] * other.group6()[2],
            },
        }
    }
}

impl AntiDot<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0]
                    + self.group0()[1] * other.group1()[1]
                    + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group7()[0]
                    + self.group0()[1] * other.group7()[1]
                    + self.group0()[2] * other.group7()[2],
            },
        }
    }
}

impl AntiDot<AntiScalar> for Magnitude {
    type Output = AntiScalar;

    fn anti_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl AntiDot<Magnitude> for Magnitude {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<MultiVector> for Magnitude {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Scalar> for Magnitude {
    type Output = AntiScalar;

    fn anti_dot(self, other: Scalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl AntiDot<AntiScalar> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl AntiDot<Circle> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group6()[0] * other.group2()[0]
                    - self.group6()[1] * other.group2()[1]
                    - self.group6()[2] * other.group2()[2]
                    + self.group6()[3] * other.group0()[3]
                    + self.group7()[0] * other.group1()[0]
                    + self.group7()[1] * other.group1()[1]
                    + self.group7()[2] * other.group1()[2]
                    - self.group8()[0] * other.group0()[0]
                    - self.group8()[1] * other.group0()[1]
                    - self.group8()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Dipole> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group3()[0] * other.group2()[0]
                    + self.group3()[1] * other.group2()[1]
                    + self.group3()[2] * other.group2()[2]
                    + self.group4()[0] * other.group1()[0]
                    + self.group4()[1] * other.group1()[1]
                    + self.group4()[2] * other.group1()[2]
                    + self.group5()[0] * other.group0()[0]
                    + self.group5()[1] * other.group0()[1]
                    + self.group5()[2] * other.group0()[2]
                    + self.group5()[3] * other.group2()[3],
            },
        }
    }
}

impl AntiDot<Horizon> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Horizon) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group10()[0] * other.group0(),
            },
        }
    }
}

impl AntiDot<Line> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group6()[0] * other.group1()[0]
                    - self.group6()[1] * other.group1()[1]
                    - self.group6()[2] * other.group1()[2]
                    + self.group7()[0] * other.group0()[0]
                    + self.group7()[1] * other.group0()[1]
                    + self.group7()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtInfinity> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group6()[0] * other.group0()[0]
                    - self.group6()[1] * other.group0()[1]
                    - self.group6()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtOrigin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group7()[0] * other.group0()[0]
                    + self.group7()[1] * other.group0()[1]
                    + self.group7()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Magnitude> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<MultiVector> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group1()[0] * other.group1()[0]
                    + self.group1()[1] * other.group1()[1]
                    + self.group1()[2] * other.group1()[2]
                    - self.group2()[0] * other.group2()[1]
                    - self.group2()[1] * other.group2()[0]
                    + self.group3()[0] * other.group5()[0]
                    + self.group3()[1] * other.group5()[1]
                    + self.group3()[2] * other.group5()[2]
                    + self.group4()[0] * other.group4()[0]
                    + self.group4()[1] * other.group4()[1]
                    + self.group4()[2] * other.group4()[2]
                    + self.group5()[0] * other.group3()[0]
                    + self.group5()[1] * other.group3()[1]
                    + self.group5()[2] * other.group3()[2]
                    + self.group5()[3] * other.group5()[3]
                    - self.group6()[0] * other.group8()[0]
                    - self.group6()[1] * other.group8()[1]
                    - self.group6()[2] * other.group8()[2]
                    + self.group6()[3] * other.group6()[3]
                    + self.group7()[0] * other.group7()[0]
                    + self.group7()[1] * other.group7()[1]
                    + self.group7()[2] * other.group7()[2]
                    - self.group8()[0] * other.group6()[0]
                    - self.group8()[1] * other.group6()[1]
                    - self.group8()[2] * other.group6()[2]
                    + self.group9()[0] * other.group9()[0]
                    + self.group9()[1] * other.group9()[1]
                    + self.group9()[2] * other.group9()[2]
                    + self.group10()[0] * other.group10()[1]
                    + self.group10()[1] * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Origin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group5()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Plane> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group9()[0] * other.group0()[0]
                    + self.group9()[1] * other.group0()[1]
                    + self.group9()[2] * other.group0()[2]
                    + self.group10()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<PlaneAtOrigin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group9()[0] * other.group0()[0]
                    + self.group9()[1] * other.group0()[1]
                    + self.group9()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Point> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group3()[0] * other.group0()[0]
                    + self.group3()[1] * other.group0()[1]
                    + self.group3()[2] * other.group0()[2]
                    + self.group5()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<PointAtInfinity> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: PointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group3()[0] * other.group0()[0]
                    + self.group3()[1] * other.group0()[1]
                    + self.group3()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Radial> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Radial) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0]
                    + self.group1()[1] * other.group0()[1]
                    + self.group1()[2] * other.group0()[2]
                    - self.group2()[0] * other.group1()[1]
                    - self.group2()[1] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<Scalar> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Scalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl AntiDot<Sphere> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group9()[0] * other.group0()[0]
                    + self.group9()[1] * other.group0()[1]
                    + self.group9()[2] * other.group0()[2]
                    + self.group10()[0] * other.group1()[1]
                    + self.group10()[1] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<Dipole> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group2()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group5()[3],
            },
        }
    }
}

impl AntiDot<Origin> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<Point> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group9()[0]
                    + self.group0()[1] * other.group9()[1]
                    + self.group0()[2] * other.group9()[2]
                    + self.group0()[3] * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Plane> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Sphere> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    + self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<MultiVector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group9()[0]
                    + self.group0()[1] * other.group9()[1]
                    + self.group0()[2] * other.group9()[2],
            },
        }
    }
}

impl AntiDot<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Dipole> for Point {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    + self.group0()[3] * other.group2()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Point {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group3()[0]
                    + self.group0()[1] * other.group3()[1]
                    + self.group0()[2] * other.group3()[2]
                    + self.group0()[3] * other.group5()[3],
            },
        }
    }
}

impl AntiDot<Origin> for Point {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Point> for Point {
    type Output = AntiScalar;

    fn anti_dot(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Dipole> for PointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for PointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group3()[0]
                    + self.group0()[1] * other.group3()[1]
                    + self.group0()[2] * other.group3()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for Radial {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0]
                    + self.group0()[1] * other.group1()[1]
                    + self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group2()[1]
                    - self.group1()[1] * other.group2()[0],
            },
        }
    }
}

impl AntiDot<Radial> for Radial {
    type Output = AntiScalar;

    fn anti_dot(self, other: Radial) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    - self.group1()[0] * other.group1()[1]
                    - self.group1()[1] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<Magnitude> for Scalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl AntiDot<MultiVector> for Scalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl AntiDot<Scalar> for Scalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: Scalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<Horizon> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: Horizon) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl AntiDot<MultiVector> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group9()[0]
                    + self.group0()[1] * other.group9()[1]
                    + self.group0()[2] * other.group9()[2]
                    + self.group1()[0] * other.group10()[1]
                    + self.group1()[1] * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Plane> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<PlaneAtOrigin> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Sphere> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
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

impl Dot<AntiScalar> for AntiScalar {
    type Output = Scalar;

    fn dot(self, other: AntiScalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<Magnitude> for AntiScalar {
    type Output = Scalar;

    fn dot(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[1],
            },
        }
    }
}

impl Dot<MultiVector> for AntiScalar {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[1],
            },
        }
    }
}

impl Dot<Circle> for Circle {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group2()[0]
                    + self.group0()[1] * other.group2()[1]
                    + self.group0()[2] * other.group2()[2]
                    - self.group0()[3] * other.group0()[3]
                    + self.group1()[0] * other.group1()[0]
                    + self.group1()[1] * other.group1()[1]
                    + self.group1()[2] * other.group1()[2]
                    + self.group2()[0] * other.group0()[0]
                    + self.group2()[1] * other.group0()[1]
                    + self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Line> for Circle {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0]
                    + self.group0()[1] * other.group1()[1]
                    + self.group0()[2] * other.group1()[2]
                    + self.group1()[0] * other.group0()[0]
                    + self.group1()[1] * other.group0()[1]
                    + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtInfinity> for Circle {
    type Output = Scalar;

    fn dot(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtOrigin> for Circle {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0]
                    + self.group1()[1] * other.group0()[1]
                    + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Circle {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group8()[0]
                    + self.group0()[1] * other.group8()[1]
                    + self.group0()[2] * other.group8()[2]
                    - self.group0()[3] * other.group6()[3]
                    + self.group1()[0] * other.group7()[0]
                    + self.group1()[1] * other.group7()[1]
                    + self.group1()[2] * other.group7()[2]
                    + self.group2()[0] * other.group6()[0]
                    + self.group2()[1] * other.group6()[1]
                    + self.group2()[2] * other.group6()[2],
            },
        }
    }
}

impl Dot<Dipole> for Dipole {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
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
                    + self.group2()[3] * other.group2()[3],
            },
        }
    }
}

impl Dot<MultiVector> for Dipole {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group5()[0]
                    - self.group0()[1] * other.group5()[1]
                    - self.group0()[2] * other.group5()[2]
                    - self.group1()[0] * other.group4()[0]
                    - self.group1()[1] * other.group4()[1]
                    - self.group1()[2] * other.group4()[2]
                    - self.group2()[0] * other.group3()[0]
                    - self.group2()[1] * other.group3()[1]
                    - self.group2()[2] * other.group3()[2]
                    + self.group2()[3] * other.group5()[3],
            },
        }
    }
}

impl Dot<Origin> for Dipole {
    type Output = Scalar;

    fn dot(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group2()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Point> for Dipole {
    type Output = Scalar;

    fn dot(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2]
                    + self.group2()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<PointAtInfinity> for Dipole {
    type Output = Scalar;

    fn dot(self, other: PointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Horizon {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group10()[0],
            },
        }
    }
}

impl Dot<Sphere> for Horizon {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group1()[0],
            },
        }
    }
}

impl Dot<Circle> for Line {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0]
                    + self.group0()[1] * other.group1()[1]
                    + self.group0()[2] * other.group1()[2]
                    + self.group1()[0] * other.group0()[0]
                    + self.group1()[1] * other.group0()[1]
                    + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Line> for Line {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtOrigin> for Line {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Line {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group7()[0]
                    + self.group0()[1] * other.group7()[1]
                    + self.group0()[2] * other.group7()[2]
                    + self.group1()[0] * other.group6()[0]
                    + self.group1()[1] * other.group6()[1]
                    + self.group1()[2] * other.group6()[2],
            },
        }
    }
}

impl Dot<Circle> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group6()[0]
                    + self.group0()[1] * other.group6()[1]
                    + self.group0()[2] * other.group6()[2],
            },
        }
    }
}

impl Dot<Circle> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0]
                    + self.group0()[1] * other.group1()[1]
                    + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<Line> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtOrigin> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group7()[0]
                    + self.group0()[1] * other.group7()[1]
                    + self.group0()[2] * other.group7()[2],
            },
        }
    }
}

impl Dot<AntiScalar> for Magnitude {
    type Output = Scalar;

    fn dot(self, other: AntiScalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0(),
            },
        }
    }
}

impl Dot<Magnitude> for Magnitude {
    type Output = Scalar;

    fn dot(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl Dot<MultiVector> for Magnitude {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl Dot<Scalar> for Magnitude {
    type Output = Scalar;

    fn dot(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Dot<AntiScalar> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: AntiScalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0(),
            },
        }
    }
}

impl Dot<Circle> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group6()[0] * other.group2()[0]
                    + self.group6()[1] * other.group2()[1]
                    + self.group6()[2] * other.group2()[2]
                    - self.group6()[3] * other.group0()[3]
                    + self.group7()[0] * other.group1()[0]
                    + self.group7()[1] * other.group1()[1]
                    + self.group7()[2] * other.group1()[2]
                    + self.group8()[0] * other.group0()[0]
                    + self.group8()[1] * other.group0()[1]
                    + self.group8()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Dipole> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group3()[0] * other.group2()[0]
                    - self.group3()[1] * other.group2()[1]
                    - self.group3()[2] * other.group2()[2]
                    - self.group4()[0] * other.group1()[0]
                    - self.group4()[1] * other.group1()[1]
                    - self.group4()[2] * other.group1()[2]
                    - self.group5()[0] * other.group0()[0]
                    - self.group5()[1] * other.group0()[1]
                    - self.group5()[2] * other.group0()[2]
                    + self.group5()[3] * other.group2()[3],
            },
        }
    }
}

impl Dot<Horizon> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group10()[0] * other.group0(),
            },
        }
    }
}

impl Dot<Line> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group6()[0] * other.group1()[0]
                    + self.group6()[1] * other.group1()[1]
                    + self.group6()[2] * other.group1()[2]
                    + self.group7()[0] * other.group0()[0]
                    + self.group7()[1] * other.group0()[1]
                    + self.group7()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtInfinity> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group6()[0] * other.group0()[0]
                    + self.group6()[1] * other.group0()[1]
                    + self.group6()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtOrigin> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group7()[0] * other.group0()[0]
                    + self.group7()[1] * other.group0()[1]
                    + self.group7()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Magnitude> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl Dot<MultiVector> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
                    + self.group1()[0] * other.group1()[0]
                    + self.group1()[1] * other.group1()[1]
                    + self.group1()[2] * other.group1()[2]
                    - self.group2()[0] * other.group2()[1]
                    - self.group2()[1] * other.group2()[0]
                    - self.group3()[0] * other.group5()[0]
                    - self.group3()[1] * other.group5()[1]
                    - self.group3()[2] * other.group5()[2]
                    - self.group4()[0] * other.group4()[0]
                    - self.group4()[1] * other.group4()[1]
                    - self.group4()[2] * other.group4()[2]
                    - self.group5()[0] * other.group3()[0]
                    - self.group5()[1] * other.group3()[1]
                    - self.group5()[2] * other.group3()[2]
                    + self.group5()[3] * other.group5()[3]
                    + self.group6()[0] * other.group8()[0]
                    + self.group6()[1] * other.group8()[1]
                    + self.group6()[2] * other.group8()[2]
                    - self.group6()[3] * other.group6()[3]
                    + self.group7()[0] * other.group7()[0]
                    + self.group7()[1] * other.group7()[1]
                    + self.group7()[2] * other.group7()[2]
                    + self.group8()[0] * other.group6()[0]
                    + self.group8()[1] * other.group6()[1]
                    + self.group8()[2] * other.group6()[2]
                    - self.group9()[0] * other.group9()[0]
                    - self.group9()[1] * other.group9()[1]
                    - self.group9()[2] * other.group9()[2]
                    + self.group10()[0] * other.group10()[1]
                    + self.group10()[1] * other.group10()[0],
            },
        }
    }
}

impl Dot<Origin> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group5()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Plane> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group9()[0] * other.group0()[0]
                    - self.group9()[1] * other.group0()[1]
                    - self.group9()[2] * other.group0()[2]
                    + self.group10()[0] * other.group0()[3],
            },
        }
    }
}

impl Dot<PlaneAtOrigin> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group9()[0] * other.group0()[0]
                    - self.group9()[1] * other.group0()[1]
                    - self.group9()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Point> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group3()[0] * other.group0()[0]
                    - self.group3()[1] * other.group0()[1]
                    - self.group3()[2] * other.group0()[2]
                    + self.group5()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<PointAtInfinity> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: PointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group3()[0] * other.group0()[0]
                    - self.group3()[1] * other.group0()[1]
                    - self.group3()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Radial> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Radial) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0]
                    + self.group1()[1] * other.group0()[1]
                    + self.group1()[2] * other.group0()[2]
                    - self.group2()[0] * other.group1()[1]
                    - self.group2()[1] * other.group1()[0],
            },
        }
    }
}

impl Dot<Scalar> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Dot<Sphere> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group9()[0] * other.group0()[0]
                    - self.group9()[1] * other.group0()[1]
                    - self.group9()[2] * other.group0()[2]
                    + self.group10()[0] * other.group1()[1]
                    + self.group10()[1] * other.group1()[0],
            },
        }
    }
}

impl Dot<Dipole> for Origin {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group2()[3],
            },
        }
    }
}

impl Dot<MultiVector> for Origin {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group5()[3],
            },
        }
    }
}

impl Dot<Origin> for Origin {
    type Output = Scalar;

    fn dot(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<Point> for Origin {
    type Output = Scalar;

    fn dot(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for Plane {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group9()[0]
                    - self.group0()[1] * other.group9()[1]
                    - self.group0()[2] * other.group9()[2]
                    + self.group0()[3] * other.group10()[0],
            },
        }
    }
}

impl Dot<Plane> for Plane {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<PlaneAtOrigin> for Plane {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Sphere> for Plane {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2]
                    + self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl Dot<MultiVector> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group9()[0]
                    - self.group0()[1] * other.group9()[1]
                    - self.group0()[2] * other.group9()[2],
            },
        }
    }
}

impl Dot<Plane> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Sphere> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Dipole> for Point {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2]
                    + self.group0()[3] * other.group2()[3],
            },
        }
    }
}

impl Dot<MultiVector> for Point {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group3()[0]
                    - self.group0()[1] * other.group3()[1]
                    - self.group0()[2] * other.group3()[2]
                    + self.group0()[3] * other.group5()[3],
            },
        }
    }
}

impl Dot<Origin> for Point {
    type Output = Scalar;

    fn dot(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Point> for Point {
    type Output = Scalar;

    fn dot(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Dipole> for PointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for PointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group3()[0]
                    - self.group0()[1] * other.group3()[1]
                    - self.group0()[2] * other.group3()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Radial {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0]
                    + self.group0()[1] * other.group1()[1]
                    + self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group2()[1]
                    - self.group1()[1] * other.group2()[0],
            },
        }
    }
}

impl Dot<Radial> for Radial {
    type Output = Scalar;

    fn dot(self, other: Radial) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0]
                    + self.group0()[1] * other.group0()[1]
                    + self.group0()[2] * other.group0()[2]
                    - self.group1()[0] * other.group1()[1]
                    - self.group1()[1] * other.group1()[0],
            },
        }
    }
}

impl Dot<Magnitude> for Scalar {
    type Output = Scalar;

    fn dot(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Dot<MultiVector> for Scalar {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl Dot<Scalar> for Scalar {
    type Output = Scalar;

    fn dot(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<Horizon> for Sphere {
    type Output = Scalar;

    fn dot(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0(),
            },
        }
    }
}

impl Dot<MultiVector> for Sphere {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group9()[0]
                    - self.group0()[1] * other.group9()[1]
                    - self.group0()[2] * other.group9()[2]
                    + self.group1()[0] * other.group10()[1]
                    + self.group1()[1] * other.group10()[0],
            },
        }
    }
}

impl Dot<Plane> for Sphere {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2]
                    + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl Dot<PlaneAtOrigin> for Sphere {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Sphere> for Sphere {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group0()[0]
                    - self.group0()[1] * other.group0()[1]
                    - self.group0()[2] * other.group0()[2]
                    + self.group1()[0] * other.group1()[1]
                    + self.group1()[1] * other.group1()[0],
            },
        }
    }
}
