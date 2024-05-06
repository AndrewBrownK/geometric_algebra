//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::*;

/// Dot Product
///
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products
pub trait Dot<T> {
    type Output;
    fn dot(self, other: T) -> Self::Output;
}

/// Anti-Dot Product
///
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

impl AntiDot<DualNum> for AntiScalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: DualNum) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Motor> for AntiScalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[3],
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
                g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2] + self.group0()[3] * other.group0()[3]
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

impl AntiDot<Line> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
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

impl AntiDot<Motor> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
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

impl AntiDot<MultiVector> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group8()[0] - self.group0()[1] * other.group8()[1] - self.group0()[2] * other.group8()[2] + self.group0()[3] * other.group6()[3]
                    - self.group1()[0] * other.group7()[0]
                    - self.group1()[1] * other.group7()[1]
                    - self.group1()[2] * other.group7()[2]
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
                    - self.group2()[3] * other.group2()[3],
            },
        }
    }
}

impl AntiDot<FlatPoint> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group2()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Flector> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group2()[3] * other.group0()[3],
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
                    - self.group2()[3] * other.group5()[3],
            },
        }
    }
}

impl AntiDot<AntiScalar> for DualNum {
    type Output = AntiScalar;

    fn anti_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl AntiDot<DualNum> for DualNum {
    type Output = AntiScalar;

    fn anti_dot(self, other: DualNum) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Motor> for DualNum {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for DualNum {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Scalar> for DualNum {
    type Output = AntiScalar;

    fn anti_dot(self, other: Scalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0(),
            },
        }
    }
}

impl AntiDot<Dipole> for FlatPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group2()[3],
            },
        }
    }
}

impl AntiDot<FlatPoint> for FlatPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Flector> for FlatPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for FlatPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group3()[0] + self.group0()[1] * other.group3()[1] + self.group0()[2] * other.group3()[2] - self.group0()[3] * other.group5()[3],
            },
        }
    }
}

impl AntiDot<Dipole> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group2()[3],
            },
        }
    }
}

impl AntiDot<FlatPoint> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Flector> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3] + self.group1()[0] * other.group1()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group3()[0] + self.group0()[1] * other.group3()[1] + self.group0()[2] * other.group3()[2] - self.group0()[3] * other.group5()[3]
                    + self.group1()[0] * other.group9()[0]
                    + self.group1()[1] * other.group9()[1]
                    + self.group1()[2] * other.group9()[2]
                    - self.group1()[3] * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Plane> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Sphere> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] - self.group1()[3] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<Circle> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
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

impl AntiDot<Line> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Motor> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group7()[0]
                    - self.group0()[1] * other.group7()[1]
                    - self.group0()[2] * other.group7()[2]
                    - self.group1()[0] * other.group6()[0]
                    - self.group1()[1] * other.group6()[1]
                    - self.group1()[2] * other.group6()[2],
            },
        }
    }
}

impl AntiDot<AntiScalar> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Circle> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
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

impl AntiDot<DualNum> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: DualNum) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Line> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Motor> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group7()[0] - self.group0()[1] * other.group7()[1] - self.group0()[2] * other.group7()[2] + self.group0()[3] * other.group0()[1]
                    - self.group1()[0] * other.group6()[0]
                    - self.group1()[1] * other.group6()[1]
                    - self.group1()[2] * other.group6()[2],
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
                g0: 0.0 - self.group6()[0] * other.group2()[0] - self.group6()[1] * other.group2()[1] - self.group6()[2] * other.group2()[2] + self.group6()[3] * other.group0()[3]
                    - self.group7()[0] * other.group1()[0]
                    - self.group7()[1] * other.group1()[1]
                    - self.group7()[2] * other.group1()[2]
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
                    - self.group5()[3] * other.group2()[3],
            },
        }
    }
}

impl AntiDot<DualNum> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: DualNum) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<FlatPoint> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group3()[0] * other.group0()[0] + self.group3()[1] * other.group0()[1] + self.group3()[2] * other.group0()[2] - self.group5()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Flector> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group3()[0] * other.group0()[0] + self.group3()[1] * other.group0()[1] + self.group3()[2] * other.group0()[2] - self.group5()[3] * other.group0()[3]
                    + self.group9()[0] * other.group1()[0]
                    + self.group9()[1] * other.group1()[1]
                    + self.group9()[2] * other.group1()[2]
                    - self.group10()[0] * other.group1()[3],
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
                    - self.group7()[0] * other.group0()[0]
                    - self.group7()[1] * other.group0()[1]
                    - self.group7()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Motor> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3]
                    - self.group6()[0] * other.group1()[0]
                    - self.group6()[1] * other.group1()[1]
                    - self.group6()[2] * other.group1()[2]
                    - self.group7()[0] * other.group0()[0]
                    - self.group7()[1] * other.group0()[1]
                    - self.group7()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1]
                    - self.group1()[0] * other.group1()[0]
                    - self.group1()[1] * other.group1()[1]
                    - self.group1()[2] * other.group1()[2]
                    + self.group2()[0] * other.group2()[1]
                    + self.group2()[1] * other.group2()[0]
                    + self.group3()[0] * other.group5()[0]
                    + self.group3()[1] * other.group5()[1]
                    + self.group3()[2] * other.group5()[2]
                    + self.group4()[0] * other.group4()[0]
                    + self.group4()[1] * other.group4()[1]
                    + self.group4()[2] * other.group4()[2]
                    + self.group5()[0] * other.group3()[0]
                    + self.group5()[1] * other.group3()[1]
                    + self.group5()[2] * other.group3()[2]
                    - self.group5()[3] * other.group5()[3]
                    - self.group6()[0] * other.group8()[0]
                    - self.group6()[1] * other.group8()[1]
                    - self.group6()[2] * other.group8()[2]
                    + self.group6()[3] * other.group6()[3]
                    - self.group7()[0] * other.group7()[0]
                    - self.group7()[1] * other.group7()[1]
                    - self.group7()[2] * other.group7()[2]
                    - self.group8()[0] * other.group6()[0]
                    - self.group8()[1] * other.group6()[1]
                    - self.group8()[2] * other.group6()[2]
                    + self.group9()[0] * other.group9()[0]
                    + self.group9()[1] * other.group9()[1]
                    + self.group9()[2] * other.group9()[2]
                    - self.group10()[0] * other.group10()[1]
                    - self.group10()[1] * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Plane> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group9()[0] * other.group0()[0] + self.group9()[1] * other.group0()[1] + self.group9()[2] * other.group0()[2] - self.group10()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<RoundPoint> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2]
                    + self.group2()[0] * other.group1()[1]
                    + self.group2()[1] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<Scalar> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Scalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0(),
            },
        }
    }
}

impl AntiDot<Sphere> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group9()[0] * other.group0()[0] + self.group9()[1] * other.group0()[1] + self.group9()[2] * other.group0()[2]
                    - self.group10()[0] * other.group1()[1]
                    - self.group10()[1] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<Flector> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group9()[0] + self.group0()[1] * other.group9()[1] + self.group0()[2] * other.group9()[2] - self.group0()[3] * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Plane> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Sphere> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<MultiVector> for RoundPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2]
                    + self.group1()[0] * other.group2()[1]
                    + self.group1()[1] * other.group2()[0],
            },
        }
    }
}

impl AntiDot<RoundPoint> for RoundPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
                    + self.group1()[0] * other.group1()[1]
                    + self.group1()[1] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<DualNum> for Scalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: DualNum) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[0],
            },
        }
    }
}

impl AntiDot<MultiVector> for Scalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[0],
            },
        }
    }
}

impl AntiDot<Scalar> for Scalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: Scalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<Flector> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group1()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group9()[0] + self.group0()[1] * other.group9()[1] + self.group0()[2] * other.group9()[2]
                    - self.group1()[0] * other.group10()[1]
                    - self.group1()[1] * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Plane> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Sphere> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
                    - self.group1()[0] * other.group1()[1]
                    - self.group1()[1] * other.group1()[0],
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

impl Dot<DualNum> for AntiScalar {
    type Output = Scalar;

    fn dot(self, other: DualNum) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[1],
            },
        }
    }
}

impl Dot<Motor> for AntiScalar {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
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
                g0: self.group0()[0] * other.group2()[0] + self.group0()[1] * other.group2()[1] + self.group0()[2] * other.group2()[2] - self.group0()[3] * other.group0()[3]
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

impl Dot<Motor> for Circle {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
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

impl Dot<MultiVector> for Circle {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group8()[0] + self.group0()[1] * other.group8()[1] + self.group0()[2] * other.group8()[2] - self.group0()[3] * other.group6()[3]
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

impl Dot<FlatPoint> for Dipole {
    type Output = Scalar;

    fn dot(self, other: FlatPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group2()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Flector> for Dipole {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group2()[3] * other.group0()[3],
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

impl Dot<AntiScalar> for DualNum {
    type Output = Scalar;

    fn dot(self, other: AntiScalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0(),
            },
        }
    }
}

impl Dot<DualNum> for DualNum {
    type Output = Scalar;

    fn dot(self, other: DualNum) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl Dot<Motor> for DualNum {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for DualNum {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl Dot<Scalar> for DualNum {
    type Output = Scalar;

    fn dot(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Dot<Dipole> for FlatPoint {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group2()[3],
            },
        }
    }
}

impl Dot<FlatPoint> for FlatPoint {
    type Output = Scalar;

    fn dot(self, other: FlatPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Flector> for FlatPoint {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for FlatPoint {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group3()[0] - self.group0()[1] * other.group3()[1] - self.group0()[2] * other.group3()[2] + self.group0()[3] * other.group5()[3],
            },
        }
    }
}

impl Dot<Dipole> for Flector {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group2()[3],
            },
        }
    }
}

impl Dot<FlatPoint> for Flector {
    type Output = Scalar;

    fn dot(self, other: FlatPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Flector> for Flector {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0()[3] - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Flector {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group3()[0] - self.group0()[1] * other.group3()[1] - self.group0()[2] * other.group3()[2] + self.group0()[3] * other.group5()[3]
                    - self.group1()[0] * other.group9()[0]
                    - self.group1()[1] * other.group9()[1]
                    - self.group1()[2] * other.group9()[2]
                    + self.group1()[3] * other.group10()[0],
            },
        }
    }
}

impl Dot<Plane> for Flector {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Sphere> for Flector {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] + self.group1()[3] * other.group1()[0],
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
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Motor> for Line {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
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

impl Dot<AntiScalar> for Motor {
    type Output = Scalar;

    fn dot(self, other: AntiScalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Circle> for Motor {
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

impl Dot<DualNum> for Motor {
    type Output = Scalar;

    fn dot(self, other: DualNum) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl Dot<Line> for Motor {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Motor> for Motor {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for Motor {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group7()[0] + self.group0()[1] * other.group7()[1] + self.group0()[2] * other.group7()[2] - self.group0()[3] * other.group0()[1]
                    + self.group1()[0] * other.group6()[0]
                    + self.group1()[1] * other.group6()[1]
                    + self.group1()[2] * other.group6()[2],
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
                g0: self.group6()[0] * other.group2()[0] + self.group6()[1] * other.group2()[1] + self.group6()[2] * other.group2()[2] - self.group6()[3] * other.group0()[3]
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

impl Dot<DualNum> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: DualNum) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl Dot<FlatPoint> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: FlatPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group3()[0] * other.group0()[0] - self.group3()[1] * other.group0()[1] - self.group3()[2] * other.group0()[2] + self.group5()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Flector> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group3()[0] * other.group0()[0] - self.group3()[1] * other.group0()[1] - self.group3()[2] * other.group0()[2] + self.group5()[3] * other.group0()[3]
                    - self.group9()[0] * other.group1()[0]
                    - self.group9()[1] * other.group1()[1]
                    - self.group9()[2] * other.group1()[2]
                    + self.group10()[0] * other.group1()[3],
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

impl Dot<Motor> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0()[3]
                    + self.group6()[0] * other.group1()[0]
                    + self.group6()[1] * other.group1()[1]
                    + self.group6()[2] * other.group1()[2]
                    + self.group7()[0] * other.group0()[0]
                    + self.group7()[1] * other.group0()[1]
                    + self.group7()[2] * other.group0()[2],
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

impl Dot<Plane> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group9()[0] * other.group0()[0] - self.group9()[1] * other.group0()[1] - self.group9()[2] * other.group0()[2]
                    + self.group10()[0] * other.group0()[3],
            },
        }
    }
}

impl Dot<RoundPoint> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2]
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
                g0: 0.0 - self.group9()[0] * other.group0()[0] - self.group9()[1] * other.group0()[1] - self.group9()[2] * other.group0()[2]
                    + self.group10()[0] * other.group1()[1]
                    + self.group10()[1] * other.group1()[0],
            },
        }
    }
}

impl Dot<Flector> for Plane {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Plane {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group9()[0] - self.group0()[1] * other.group9()[1] - self.group0()[2] * other.group9()[2]
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
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Sphere> for Plane {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl Dot<MultiVector> for RoundPoint {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2]
                    - self.group1()[0] * other.group2()[1]
                    - self.group1()[1] * other.group2()[0],
            },
        }
    }
}

impl Dot<RoundPoint> for RoundPoint {
    type Output = Scalar;

    fn dot(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
                    - self.group1()[0] * other.group1()[1]
                    - self.group1()[1] * other.group1()[0],
            },
        }
    }
}

impl Dot<DualNum> for Scalar {
    type Output = Scalar;

    fn dot(self, other: DualNum) -> Scalar {
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

impl Dot<Flector> for Sphere {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] + self.group1()[0] * other.group1()[3],
            },
        }
    }
}

impl Dot<MultiVector> for Sphere {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group9()[0] - self.group0()[1] * other.group9()[1] - self.group0()[2] * other.group9()[2]
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
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl Dot<Sphere> for Sphere {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
                    + self.group1()[0] * other.group1()[1]
                    + self.group1()[1] * other.group1()[0],
            },
        }
    }
}
