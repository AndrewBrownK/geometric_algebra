//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::*;

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

impl AntiDot<Rotor> for AntiScalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Translator> for AntiScalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[3],
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
                g0: 0.0 - self.group0()[3] * other.group1()[3] + self.group1()[0] * other.group4()[0] + self.group1()[1] * other.group4()[1] + self.group1()[2] * other.group4()[2],
            },
        }
    }
}

impl AntiDot<Origin> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
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

impl AntiDot<PlaneAtOrigin> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Point> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
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

impl AntiDot<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
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
                g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2],
            },
        }
    }
}

impl AntiDot<Rotor> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Motor> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2],
            },
        }
    }
}

impl AntiDot<Rotor> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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
                g0: self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Motor> for Magnitude {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Magnitude {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Rotor> for Magnitude {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Translator> for Magnitude {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3],
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

impl AntiDot<LineAtOrigin> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Magnitude> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[1],
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
                g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2] + self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Rotor> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Translator> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
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

impl AntiDot<Flector> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[3] * other.group0()[3] + self.group4()[0] * other.group1()[0] + self.group4()[1] * other.group1()[1] + self.group4()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<Line> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtOrigin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Magnitude> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Motor> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3] - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[1]
                    - self.group1()[3] * other.group1()[3]
                    - self.group2()[0] * other.group2()[0]
                    - self.group2()[1] * other.group2()[1]
                    - self.group2()[2] * other.group2()[2]
                    + self.group4()[0] * other.group4()[0]
                    + self.group4()[1] * other.group4()[1]
                    + self.group4()[2] * other.group4()[2],
            },
        }
    }
}

impl AntiDot<Origin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Plane> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group4()[0] * other.group0()[0] + self.group4()[1] * other.group0()[1] + self.group4()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<PlaneAtOrigin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group4()[0] * other.group0()[0] + self.group4()[1] * other.group0()[1] + self.group4()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Point> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Rotor> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3] - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Translator> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Flector> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group1()[3],
            },
        }
    }
}

impl AntiDot<Origin> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<Point> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
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
                g0: self.group0()[0] * other.group4()[0] + self.group0()[1] * other.group4()[1] + self.group0()[2] * other.group4()[2],
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

impl AntiDot<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Flector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group4()[0] + self.group0()[1] * other.group4()[1] + self.group0()[2] * other.group4()[2],
            },
        }
    }
}

impl AntiDot<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Flector> for Point {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Point {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group1()[3],
            },
        }
    }
}

impl AntiDot<Origin> for Point {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Point> for Point {
    type Output = AntiScalar;

    fn anti_dot(self, other: Point) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<AntiScalar> for Rotor {
    type Output = AntiScalar;

    fn anti_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Line> for Rotor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Magnitude> for Rotor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Motor> for Rotor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Rotor {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2] + self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Rotor> for Rotor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Translator> for Rotor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<AntiScalar> for Translator {
    type Output = AntiScalar;

    fn anti_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Magnitude> for Translator {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Motor> for Translator {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for Translator {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<Rotor> for Translator {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Translator> for Translator {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
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
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group1()[3] * other.group1()[3],
            },
        }
    }
}

impl Dot<Horizon> for Flector {
    type Output = Scalar;

    fn dot(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[3] * other.group0(),
            },
        }
    }
}

impl Dot<MultiVector> for Flector {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] - self.group1()[3] * other.group4()[3],
            },
        }
    }
}

impl Dot<Plane> for Flector {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Point> for Flector {
    type Output = Scalar;

    fn dot(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<PointAtInfinity> for Flector {
    type Output = Scalar;

    fn dot(self, other: PointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Flector> for Horizon {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group1()[3],
            },
        }
    }
}

impl Dot<Horizon> for Horizon {
    type Output = Scalar;

    fn dot(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<MultiVector> for Horizon {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group4()[3],
            },
        }
    }
}

impl Dot<Plane> for Horizon {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<Line> for Line {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<LineAtInfinity> for Line {
    type Output = Scalar;

    fn dot(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Motor> for Line {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Line {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group3()[0] - self.group1()[1] * other.group3()[1] - self.group1()[2] * other.group3()[2],
            },
        }
    }
}

impl Dot<Translator> for Line {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Line> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<LineAtInfinity> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Motor> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group3()[0] - self.group0()[1] * other.group3()[1] - self.group0()[2] * other.group3()[2],
            },
        }
    }
}

impl Dot<Translator> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Magnitude> for Magnitude {
    type Output = Scalar;

    fn dot(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0],
            },
        }
    }
}

impl Dot<MultiVector> for Magnitude {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0],
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

impl Dot<Line> for Motor {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<LineAtInfinity> for Motor {
    type Output = Scalar;

    fn dot(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Motor> for Motor {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Motor {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group3()[0] - self.group1()[1] * other.group3()[1] - self.group1()[2] * other.group3()[2],
            },
        }
    }
}

impl Dot<Translator> for Motor {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Flector> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] - self.group4()[3] * other.group1()[3],
            },
        }
    }
}

impl Dot<Horizon> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group4()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Line> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group3()[0] * other.group1()[0] - self.group3()[1] * other.group1()[1] - self.group3()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<LineAtInfinity> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group3()[0] * other.group0()[0] - self.group3()[1] * other.group0()[1] - self.group3()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Magnitude> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0],
            },
        }
    }
}

impl Dot<Motor> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group3()[0] * other.group1()[0] - self.group3()[1] * other.group1()[1] - self.group3()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group1()[0] * other.group1()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2]
                    - self.group3()[0] * other.group3()[0]
                    - self.group3()[1] * other.group3()[1]
                    - self.group3()[2] * other.group3()[2]
                    - self.group4()[3] * other.group4()[3],
            },
        }
    }
}

impl Dot<Plane> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group4()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Point> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<PointAtInfinity> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: PointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
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

impl Dot<Translator> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group3()[0] * other.group0()[0] - self.group3()[1] * other.group0()[1] - self.group3()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Flector> for Plane {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group1()[3],
            },
        }
    }
}

impl Dot<Horizon> for Plane {
    type Output = Scalar;

    fn dot(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<MultiVector> for Plane {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group4()[3],
            },
        }
    }
}

impl Dot<Plane> for Plane {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Flector> for Point {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Point {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<Point> for Point {
    type Output = Scalar;

    fn dot(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<PointAtInfinity> for Point {
    type Output = Scalar;

    fn dot(self, other: PointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Flector> for PointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for PointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<Point> for PointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Point) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<PointAtInfinity> for PointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: PointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
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

impl Dot<Line> for Translator {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<LineAtInfinity> for Translator {
    type Output = Scalar;

    fn dot(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Motor> for Translator {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Translator {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group3()[0] - self.group0()[1] * other.group3()[1] - self.group0()[2] * other.group3()[2],
            },
        }
    }
}

impl Dot<Translator> for Translator {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}
