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

impl AntiDot<CircleBulk> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<CircleCarrierAspect> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[3] - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<CircleWeight> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2],
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

impl AntiDot<LineAtInfinity> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtOrigin> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
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

impl AntiDot<Rotor> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Translator> for Circle {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Circle> for CircleBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<CircleBulk> for CircleBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<CircleCarrierAspect> for CircleBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for CircleBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group6()[3],
            },
        }
    }
}

impl AntiDot<Circle> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2] + self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<CircleBulk> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Line> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<LineAtInfinity> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Motor> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group8()[0] - self.group0()[1] * other.group8()[1] - self.group0()[2] * other.group8()[2] + self.group0()[3] * other.group6()[3],
            },
        }
    }
}

impl AntiDot<Translator> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Circle> for CircleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2],
            },
        }
    }
}

impl AntiDot<Line> for CircleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Line) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<LineAtInfinity> for CircleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Motor> for CircleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Motor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for CircleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group8()[0] - self.group0()[1] * other.group8()[1] - self.group0()[2] * other.group8()[2],
            },
        }
    }
}

impl AntiDot<Translator> for CircleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl AntiDot<DipoleBulk> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleCarrierAspect> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group1()[0]
                    + self.group1()[1] * other.group1()[1]
                    + self.group1()[2] * other.group1()[2]
                    + self.group2()[0] * other.group0()[0]
                    + self.group2()[1] * other.group0()[1]
                    + self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleWeight> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group2()[0] * other.group0()[0] + self.group2()[1] * other.group0()[1] + self.group2()[2] * other.group0()[2],
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

impl AntiDot<FlatPointAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<FlatPointAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group2()[3] * other.group0(),
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

impl AntiDot<FlectorAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlectorAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
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

impl AntiDot<Transflector> for Dipole {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Dipole> for DipoleBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<DipoleBulk> for DipoleBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleCarrierAspect> for DipoleBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for DipoleBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group4()[0] + self.group0()[1] * other.group4()[1] + self.group0()[2] * other.group4()[2],
            },
        }
    }
}

impl AntiDot<Dipole> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group2()[0]
                    + self.group0()[1] * other.group2()[1]
                    + self.group0()[2] * other.group2()[2]
                    + self.group1()[0] * other.group1()[0]
                    + self.group1()[1] * other.group1()[1]
                    + self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<DipoleBulk> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group1()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<FlatPoint> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Flector> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlectorAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group5()[0]
                    + self.group0()[1] * other.group5()[1]
                    + self.group0()[2] * other.group5()[2]
                    + self.group1()[0] * other.group4()[0]
                    + self.group1()[1] * other.group4()[1]
                    + self.group1()[2] * other.group4()[2],
            },
        }
    }
}

impl AntiDot<Transflector> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Dipole> for DipoleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group2()[0] + self.group0()[1] * other.group2()[1] + self.group0()[2] * other.group2()[2],
            },
        }
    }
}

impl AntiDot<FlatPoint> for DipoleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<FlatPointAtInfinity> for DipoleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Flector> for DipoleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<FlectorAtInfinity> for DipoleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlectorAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for DipoleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group5()[0] + self.group0()[1] * other.group5()[1] + self.group0()[2] * other.group5()[2],
            },
        }
    }
}

impl AntiDot<Transflector> for DipoleWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
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

impl AntiDot<DipoleCarrierAspect> for FlatPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleWeight> for FlatPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
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

impl AntiDot<FlatPointAtOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
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

impl AntiDot<Dipole> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleWeight> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group3()[0] + self.group0()[1] * other.group3()[1] + self.group0()[2] * other.group3()[2],
            },
        }
    }
}

impl AntiDot<Dipole> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group2()[3],
            },
        }
    }
}

impl AntiDot<FlatPoint> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<Flector> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group5()[3],
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

impl AntiDot<DipoleCarrierAspect> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleWeight> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
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

impl AntiDot<FlatPointAtOrigin> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
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

impl AntiDot<SphereWeight> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: SphereWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Transflector> for Flector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group1()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<Dipole> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleWeight> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group3()[0] + self.group0()[1] * other.group3()[1] + self.group0()[2] * other.group3()[2] - self.group0()[3] * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Sphere> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<SphereWeight> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: SphereWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<MultiVector> for Horizon {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Sphere> for Horizon {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group1()[0],
            },
        }
    }
}

impl AntiDot<SphereWeight> for Horizon {
    type Output = AntiScalar;

    fn anti_dot(self, other: SphereWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<MultiVector> for Infinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group2()[0],
            },
        }
    }
}

impl AntiDot<Origin> for Infinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<RoundPoint> for Infinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group1()[0],
            },
        }
    }
}

impl AntiDot<RoundPointAtOrigin> for Infinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[0],
            },
        }
    }
}

impl AntiDot<RoundPointCarrierAspect> for Infinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[3],
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

impl AntiDot<CircleCarrierAspect> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<CircleWeight> for Line {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
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

impl AntiDot<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<CircleCarrierAspect> for LineAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<CircleWeight> for LineAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for LineAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group6()[0] - self.group0()[1] * other.group6()[1] - self.group0()[2] * other.group6()[2],
            },
        }
    }
}

impl AntiDot<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
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
                g0: 0.0 - self.group0()[0] * other.group7()[0] - self.group0()[1] * other.group7()[1] - self.group0()[2] * other.group7()[2],
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
                g0: 0.0 - self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1],
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
                g0: 0.0 - self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1],
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

impl AntiDot<Scalar> for Magnitude {
    type Output = AntiScalar;

    fn anti_dot(self, other: Scalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0(),
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

impl AntiDot<CircleCarrierAspect> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<CircleWeight> for Motor {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
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
                g0: 0.0 - self.group0()[0] * other.group7()[0] - self.group0()[1] * other.group7()[1] - self.group0()[2] * other.group7()[2] + self.group0()[3] * other.group0()[1]
                    - self.group1()[0] * other.group6()[0]
                    - self.group1()[1] * other.group6()[1]
                    - self.group1()[2] * other.group6()[2],
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

impl AntiDot<CircleBulk> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group6()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<CircleCarrierAspect> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group6()[3] * other.group0()[3] - self.group8()[0] * other.group0()[0] - self.group8()[1] * other.group0()[1] - self.group8()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<CircleWeight> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group8()[0] * other.group0()[0] - self.group8()[1] * other.group0()[1] - self.group8()[2] * other.group0()[2],
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

impl AntiDot<DipoleBulk> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group4()[0] * other.group0()[0] + self.group4()[1] * other.group0()[1] + self.group4()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleCarrierAspect> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group4()[0] * other.group1()[0]
                    + self.group4()[1] * other.group1()[1]
                    + self.group4()[2] * other.group1()[2]
                    + self.group5()[0] * other.group0()[0]
                    + self.group5()[1] * other.group0()[1]
                    + self.group5()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleWeight> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group5()[0] * other.group0()[0] + self.group5()[1] * other.group0()[1] + self.group5()[2] * other.group0()[2],
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

impl AntiDot<FlatPointAtInfinity> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group3()[0] * other.group0()[0] + self.group3()[1] * other.group0()[1] + self.group3()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<FlatPointAtOrigin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlatPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group5()[3] * other.group0(),
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

impl AntiDot<FlectorAtInfinity> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlectorAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group3()[0] * other.group0()[0] + self.group3()[1] * other.group0()[1] + self.group3()[2] * other.group0()[2] - self.group10()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Horizon> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Horizon) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group10()[0] * other.group0(),
            },
        }
    }
}

impl AntiDot<Infinity> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Infinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group2()[0] * other.group0(),
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

impl AntiDot<LineAtInfinity> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group6()[0] * other.group0()[0] - self.group6()[1] * other.group0()[1] - self.group6()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<LineAtOrigin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group7()[0] * other.group0()[0] - self.group7()[1] * other.group0()[1] - self.group7()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Magnitude> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1],
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

impl AntiDot<Origin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group2()[1] * other.group0(),
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

impl AntiDot<PlaneAtOrigin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group9()[0] * other.group0()[0] + self.group9()[1] * other.group0()[1] + self.group9()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Rotor> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Rotor) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3] - self.group7()[0] * other.group0()[0] - self.group7()[1] * other.group0()[1] - self.group7()[2] * other.group0()[2],
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

impl AntiDot<RoundPointAtInfinity> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] + self.group2()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<RoundPointAtOrigin> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group2()[0] * other.group0()[1] + self.group2()[1] * other.group0()[0],
            },
        }
    }
}

impl AntiDot<RoundPointBulk> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<RoundPointCarrierAspect> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] + self.group2()[1] * other.group0()[3],
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

impl AntiDot<SphereWeight> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: SphereWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group10()[1] * other.group0(),
            },
        }
    }
}

impl AntiDot<Transflector> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group3()[0] * other.group0()[0]
                    + self.group3()[1] * other.group0()[1]
                    + self.group3()[2] * other.group0()[2]
                    + self.group9()[0] * other.group1()[0]
                    + self.group9()[1] * other.group1()[1]
                    + self.group9()[2] * other.group1()[2]
                    - self.group10()[0] * other.group1()[3],
            },
        }
    }
}

impl AntiDot<Translator> for MultiVector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Translator) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3] - self.group6()[0] * other.group0()[0] - self.group6()[1] * other.group0()[1] - self.group6()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Infinity> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Infinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<MultiVector> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group2()[1],
            },
        }
    }
}

impl AntiDot<RoundPoint> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group1()[1],
            },
        }
    }
}

impl AntiDot<RoundPointAtInfinity> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<RoundPointAtOrigin> for Origin {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0()[1],
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

impl AntiDot<SphereWeight> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: SphereWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Transflector> for Plane {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
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
                g0: self.group0()[0] * other.group9()[0] + self.group0()[1] * other.group9()[1] + self.group0()[2] * other.group9()[2],
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

impl AntiDot<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Transflector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
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

impl AntiDot<Circle> for Rotor {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
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
                g0: 0.0 - self.group0()[0] * other.group7()[0] - self.group0()[1] * other.group7()[1] - self.group0()[2] * other.group7()[2] + self.group0()[3] * other.group0()[1],
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

impl AntiDot<Infinity> for RoundPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: Infinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0(),
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

impl AntiDot<Origin> for RoundPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[1] * other.group0(),
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

impl AntiDot<RoundPointAtInfinity> for RoundPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<RoundPointAtOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[1] + self.group1()[1] * other.group0()[0],
            },
        }
    }
}

impl AntiDot<RoundPointBulk> for RoundPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<RoundPointCarrierAspect> for RoundPoint {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group1()[1] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] + self.group0()[3] * other.group2()[0],
            },
        }
    }
}

impl AntiDot<Origin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<RoundPoint> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[0],
            },
        }
    }
}

impl AntiDot<RoundPointBulk> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Infinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Infinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0(),
            },
        }
    }
}

impl AntiDot<MultiVector> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group2()[1] + self.group0()[1] * other.group2()[0],
            },
        }
    }
}

impl AntiDot<Origin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: Origin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0(),
            },
        }
    }
}

impl AntiDot<RoundPoint> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[1] + self.group0()[1] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[1] + self.group0()[1] * other.group0()[0],
            },
        }
    }
}

impl AntiDot<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[1] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<MultiVector> for RoundPointBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<RoundPoint> for RoundPointBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<RoundPointAtInfinity> for RoundPointBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<RoundPointBulk> for RoundPointBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Infinity> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: Infinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<MultiVector> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] + self.group0()[3] * other.group2()[1],
            },
        }
    }
}

impl AntiDot<RoundPoint> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPoint) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group1()[1],
            },
        }
    }
}

impl AntiDot<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl AntiDot<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointBulk) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn anti_dot(self, other: RoundPointCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Magnitude> for Scalar {
    type Output = AntiScalar;

    fn anti_dot(self, other: Magnitude) -> AntiScalar {
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

impl AntiDot<FlectorAtInfinity> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlectorAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Horizon> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: Horizon) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0(),
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

impl AntiDot<PlaneAtOrigin> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
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

impl AntiDot<SphereWeight> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: SphereWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[1] * other.group0(),
            },
        }
    }
}

impl AntiDot<Transflector> for Sphere {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group1()[3],
            },
        }
    }
}

impl AntiDot<Flector> for SphereWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group1()[3],
            },
        }
    }
}

impl AntiDot<FlectorAtInfinity> for SphereWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: FlectorAtInfinity) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Horizon> for SphereWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Horizon) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl AntiDot<MultiVector> for SphereWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group10()[1],
            },
        }
    }
}

impl AntiDot<Plane> for SphereWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl AntiDot<Sphere> for SphereWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group1()[1],
            },
        }
    }
}

impl AntiDot<Transflector> for SphereWeight {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0() * other.group1()[3],
            },
        }
    }
}

impl AntiDot<Dipole> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Dipole) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleCarrierAspect> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<DipoleWeight> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: DipoleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Flector> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Flector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group1()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl AntiDot<MultiVector> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: MultiVector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0()[0] * other.group3()[0]
                    + self.group0()[1] * other.group3()[1]
                    + self.group0()[2] * other.group3()[2]
                    + self.group1()[0] * other.group9()[0]
                    + self.group1()[1] * other.group9()[1]
                    + self.group1()[2] * other.group9()[2]
                    - self.group1()[3] * other.group10()[0],
            },
        }
    }
}

impl AntiDot<Plane> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Plane) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<PlaneAtOrigin> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<Sphere> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Sphere) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] - self.group1()[3] * other.group1()[0],
            },
        }
    }
}

impl AntiDot<SphereWeight> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: SphereWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group1()[3] * other.group0(),
            },
        }
    }
}

impl AntiDot<Transflector> for Transflector {
    type Output = AntiScalar;

    fn anti_dot(self, other: Transflector) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group1()[0] * other.group1()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2],
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

impl AntiDot<Circle> for Translator {
    type Output = AntiScalar;

    fn anti_dot(self, other: Circle) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<CircleCarrierAspect> for Translator {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleCarrierAspect) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl AntiDot<CircleWeight> for Translator {
    type Output = AntiScalar;

    fn anti_dot(self, other: CircleWeight) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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
                g0: 0.0 - self.group0()[0] * other.group6()[0] - self.group0()[1] * other.group6()[1] - self.group0()[2] * other.group6()[2] + self.group0()[3] * other.group0()[1],
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

impl Dot<Rotor> for AntiScalar {
    type Output = Scalar;

    fn dot(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<Translator> for AntiScalar {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
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

impl Dot<CircleBulk> for Circle {
    type Output = Scalar;

    fn dot(self, other: CircleBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<CircleCarrierAspect> for Circle {
    type Output = Scalar;

    fn dot(self, other: CircleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3] + self.group2()[0] * other.group0()[0] + self.group2()[1] * other.group0()[1] + self.group2()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<CircleWeight> for Circle {
    type Output = Scalar;

    fn dot(self, other: CircleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group2()[0] * other.group0()[0] + self.group2()[1] * other.group0()[1] + self.group2()[2] * other.group0()[2],
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
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtOrigin> for Circle {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
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

impl Dot<Rotor> for Circle {
    type Output = Scalar;

    fn dot(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Translator> for Circle {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Circle> for CircleBulk {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<CircleBulk> for CircleBulk {
    type Output = Scalar;

    fn dot(self, other: CircleBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<CircleCarrierAspect> for CircleBulk {
    type Output = Scalar;

    fn dot(self, other: CircleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for CircleBulk {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group6()[3],
            },
        }
    }
}

impl Dot<Circle> for CircleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group2()[0] + self.group0()[1] * other.group2()[1] + self.group0()[2] * other.group2()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<CircleBulk> for CircleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: CircleBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: CircleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Line> for CircleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<LineAtInfinity> for CircleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Motor> for CircleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for CircleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group8()[0] + self.group0()[1] * other.group8()[1] + self.group0()[2] * other.group8()[2] - self.group0()[3] * other.group6()[3],
            },
        }
    }
}

impl Dot<Translator> for CircleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Circle> for CircleWeight {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group2()[0] + self.group0()[1] * other.group2()[1] + self.group0()[2] * other.group2()[2],
            },
        }
    }
}

impl Dot<Line> for CircleWeight {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<LineAtInfinity> for CircleWeight {
    type Output = Scalar;

    fn dot(self, other: LineAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Motor> for CircleWeight {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for CircleWeight {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group8()[0] + self.group0()[1] * other.group8()[1] + self.group0()[2] * other.group8()[2],
            },
        }
    }
}

impl Dot<Translator> for CircleWeight {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
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

impl Dot<DipoleBulk> for Dipole {
    type Output = Scalar;

    fn dot(self, other: DipoleBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleCarrierAspect> for Dipole {
    type Output = Scalar;

    fn dot(self, other: DipoleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
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

impl Dot<DipoleWeight> for Dipole {
    type Output = Scalar;

    fn dot(self, other: DipoleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2],
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

impl Dot<FlatPointAtInfinity> for Dipole {
    type Output = Scalar;

    fn dot(self, other: FlatPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<FlatPointAtOrigin> for Dipole {
    type Output = Scalar;

    fn dot(self, other: FlatPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group2()[3] * other.group0(),
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

impl Dot<FlectorAtInfinity> for Dipole {
    type Output = Scalar;

    fn dot(self, other: FlectorAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl Dot<Transflector> for Dipole {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Dipole> for DipoleBulk {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<DipoleBulk> for DipoleBulk {
    type Output = Scalar;

    fn dot(self, other: DipoleBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleCarrierAspect> for DipoleBulk {
    type Output = Scalar;

    fn dot(self, other: DipoleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for DipoleBulk {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group4()[0] - self.group0()[1] * other.group4()[1] - self.group0()[2] * other.group4()[2],
            },
        }
    }
}

impl Dot<Dipole> for DipoleCarrierAspect {
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
                    - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<DipoleBulk> for DipoleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: DipoleBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: DipoleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<FlatPoint> for DipoleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: FlatPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: FlatPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Flector> for DipoleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: FlectorAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for DipoleCarrierAspect {
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
                    - self.group1()[2] * other.group4()[2],
            },
        }
    }
}

impl Dot<Transflector> for DipoleCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Dipole> for DipoleWeight {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2],
            },
        }
    }
}

impl Dot<FlatPoint> for DipoleWeight {
    type Output = Scalar;

    fn dot(self, other: FlatPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<FlatPointAtInfinity> for DipoleWeight {
    type Output = Scalar;

    fn dot(self, other: FlatPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Flector> for DipoleWeight {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<FlectorAtInfinity> for DipoleWeight {
    type Output = Scalar;

    fn dot(self, other: FlectorAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for DipoleWeight {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group5()[0] - self.group0()[1] * other.group5()[1] - self.group0()[2] * other.group5()[2],
            },
        }
    }
}

impl Dot<Transflector> for DipoleWeight {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl Dot<DipoleCarrierAspect> for FlatPoint {
    type Output = Scalar;

    fn dot(self, other: DipoleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleWeight> for FlatPoint {
    type Output = Scalar;

    fn dot(self, other: DipoleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl Dot<FlatPointAtOrigin> for FlatPoint {
    type Output = Scalar;

    fn dot(self, other: FlatPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
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

impl Dot<Dipole> for FlatPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: DipoleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleWeight> for FlatPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: DipoleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for FlatPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group3()[0] - self.group0()[1] * other.group3()[1] - self.group0()[2] * other.group3()[2],
            },
        }
    }
}

impl Dot<Dipole> for FlatPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group2()[3],
            },
        }
    }
}

impl Dot<FlatPoint> for FlatPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: FlatPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: FlatPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<Flector> for FlatPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for FlatPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group5()[3],
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

impl Dot<DipoleCarrierAspect> for Flector {
    type Output = Scalar;

    fn dot(self, other: DipoleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleWeight> for Flector {
    type Output = Scalar;

    fn dot(self, other: DipoleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl Dot<FlatPointAtOrigin> for Flector {
    type Output = Scalar;

    fn dot(self, other: FlatPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
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

impl Dot<PlaneAtOrigin> for Flector {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
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

impl Dot<SphereWeight> for Flector {
    type Output = Scalar;

    fn dot(self, other: SphereWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Transflector> for Flector {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<Dipole> for FlectorAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = Scalar;

    fn dot(self, other: DipoleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleWeight> for FlectorAtInfinity {
    type Output = Scalar;

    fn dot(self, other: DipoleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for FlectorAtInfinity {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group3()[0] - self.group0()[1] * other.group3()[1] - self.group0()[2] * other.group3()[2]
                    + self.group0()[3] * other.group10()[0],
            },
        }
    }
}

impl Dot<Sphere> for FlectorAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl Dot<SphereWeight> for FlectorAtInfinity {
    type Output = Scalar;

    fn dot(self, other: SphereWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
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

impl Dot<SphereWeight> for Horizon {
    type Output = Scalar;

    fn dot(self, other: SphereWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<MultiVector> for Infinity {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group2()[0],
            },
        }
    }
}

impl Dot<Origin> for Infinity {
    type Output = Scalar;

    fn dot(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<RoundPoint> for Infinity {
    type Output = Scalar;

    fn dot(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group1()[0],
            },
        }
    }
}

impl Dot<RoundPointAtOrigin> for Infinity {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[0],
            },
        }
    }
}

impl Dot<RoundPointCarrierAspect> for Infinity {
    type Output = Scalar;

    fn dot(self, other: RoundPointCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
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

impl Dot<CircleCarrierAspect> for Line {
    type Output = Scalar;

    fn dot(self, other: CircleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<CircleWeight> for Line {
    type Output = Scalar;

    fn dot(self, other: CircleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
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

impl Dot<LineAtOrigin> for Line {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
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

impl Dot<Rotor> for Line {
    type Output = Scalar;

    fn dot(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Circle> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<CircleCarrierAspect> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: CircleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<CircleWeight> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: CircleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for LineAtInfinity {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group6()[0] + self.group0()[1] * other.group6()[1] + self.group0()[2] * other.group6()[2],
            },
        }
    }
}

impl Dot<Circle> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<Line> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtOrigin> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Motor> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<MultiVector> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group7()[0] + self.group0()[1] * other.group7()[1] + self.group0()[2] * other.group7()[2],
            },
        }
    }
}

impl Dot<Rotor> for LineAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
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

impl Dot<Motor> for Magnitude {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0()[3],
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

impl Dot<Rotor> for Magnitude {
    type Output = Scalar;

    fn dot(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0()[3],
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

impl Dot<Translator> for Magnitude {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0()[3],
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

impl Dot<CircleCarrierAspect> for Motor {
    type Output = Scalar;

    fn dot(self, other: CircleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<CircleWeight> for Motor {
    type Output = Scalar;

    fn dot(self, other: CircleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
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

impl Dot<LineAtOrigin> for Motor {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Magnitude> for Motor {
    type Output = Scalar;

    fn dot(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[1],
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

impl Dot<Rotor> for Motor {
    type Output = Scalar;

    fn dot(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Translator> for Motor {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
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

impl Dot<CircleBulk> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: CircleBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group6()[3] * other.group0(),
            },
        }
    }
}

impl Dot<CircleCarrierAspect> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: CircleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group6()[3] * other.group0()[3] + self.group8()[0] * other.group0()[0] + self.group8()[1] * other.group0()[1] + self.group8()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<CircleWeight> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: CircleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group8()[0] * other.group0()[0] + self.group8()[1] * other.group0()[1] + self.group8()[2] * other.group0()[2],
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

impl Dot<DipoleBulk> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: DipoleBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group4()[0] * other.group0()[0] - self.group4()[1] * other.group0()[1] - self.group4()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleCarrierAspect> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: DipoleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group4()[0] * other.group1()[0]
                    - self.group4()[1] * other.group1()[1]
                    - self.group4()[2] * other.group1()[2]
                    - self.group5()[0] * other.group0()[0]
                    - self.group5()[1] * other.group0()[1]
                    - self.group5()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleWeight> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: DipoleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group5()[0] * other.group0()[0] - self.group5()[1] * other.group0()[1] - self.group5()[2] * other.group0()[2],
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

impl Dot<FlatPointAtInfinity> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: FlatPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group3()[0] * other.group0()[0] - self.group3()[1] * other.group0()[1] - self.group3()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<FlatPointAtOrigin> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: FlatPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group5()[3] * other.group0(),
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

impl Dot<FlectorAtInfinity> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: FlectorAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group3()[0] * other.group0()[0] - self.group3()[1] * other.group0()[1] - self.group3()[2] * other.group0()[2]
                    + self.group10()[0] * other.group0()[3],
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

impl Dot<Infinity> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Infinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group2()[0] * other.group0(),
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
                g0: self.group6()[0] * other.group0()[0] + self.group6()[1] * other.group0()[1] + self.group6()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtOrigin> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group7()[0] * other.group0()[0] + self.group7()[1] * other.group0()[1] + self.group7()[2] * other.group0()[2],
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

impl Dot<Origin> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group2()[1] * other.group0(),
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

impl Dot<PlaneAtOrigin> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group9()[0] * other.group0()[0] - self.group9()[1] * other.group0()[1] - self.group9()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Rotor> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0()[3] + self.group7()[0] * other.group0()[0] + self.group7()[1] * other.group0()[1] + self.group7()[2] * other.group0()[2],
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

impl Dot<RoundPointAtInfinity> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] - self.group2()[0] * other.group0()[3],
            },
        }
    }
}

impl Dot<RoundPointAtOrigin> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group2()[0] * other.group0()[1] - self.group2()[1] * other.group0()[0],
            },
        }
    }
}

impl Dot<RoundPointBulk> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: RoundPointBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<RoundPointCarrierAspect> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: RoundPointCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] - self.group2()[1] * other.group0()[3],
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

impl Dot<SphereWeight> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: SphereWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group10()[1] * other.group0(),
            },
        }
    }
}

impl Dot<Transflector> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group3()[0] * other.group0()[0]
                    - self.group3()[1] * other.group0()[1]
                    - self.group3()[2] * other.group0()[2]
                    - self.group9()[0] * other.group1()[0]
                    - self.group9()[1] * other.group1()[1]
                    - self.group9()[2] * other.group1()[2]
                    + self.group10()[0] * other.group1()[3],
            },
        }
    }
}

impl Dot<Translator> for MultiVector {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0()[3] + self.group6()[0] * other.group0()[0] + self.group6()[1] * other.group0()[1] + self.group6()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Infinity> for Origin {
    type Output = Scalar;

    fn dot(self, other: Infinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<MultiVector> for Origin {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group2()[1],
            },
        }
    }
}

impl Dot<RoundPoint> for Origin {
    type Output = Scalar;

    fn dot(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group1()[1],
            },
        }
    }
}

impl Dot<RoundPointAtInfinity> for Origin {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<RoundPointAtOrigin> for Origin {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0() * other.group0()[1],
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

impl Dot<PlaneAtOrigin> for Plane {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
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

impl Dot<SphereWeight> for Plane {
    type Output = Scalar;

    fn dot(self, other: SphereWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Transflector> for Plane {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<Flector> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group9()[0] - self.group0()[1] * other.group9()[1] - self.group0()[2] * other.group9()[2],
            },
        }
    }
}

impl Dot<Plane> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Sphere> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Transflector> for PlaneAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<AntiScalar> for Rotor {
    type Output = Scalar;

    fn dot(self, other: AntiScalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Circle> for Rotor {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<Line> for Rotor {
    type Output = Scalar;

    fn dot(self, other: Line) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<LineAtOrigin> for Rotor {
    type Output = Scalar;

    fn dot(self, other: LineAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Magnitude> for Rotor {
    type Output = Scalar;

    fn dot(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl Dot<Motor> for Rotor {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for Rotor {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group7()[0] + self.group0()[1] * other.group7()[1] + self.group0()[2] * other.group7()[2] - self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl Dot<Rotor> for Rotor {
    type Output = Scalar;

    fn dot(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Translator> for Rotor {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Infinity> for RoundPoint {
    type Output = Scalar;

    fn dot(self, other: Infinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0(),
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

impl Dot<Origin> for RoundPoint {
    type Output = Scalar;

    fn dot(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[1] * other.group0(),
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

impl Dot<RoundPointAtInfinity> for RoundPoint {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group1()[0] * other.group0()[3],
            },
        }
    }
}

impl Dot<RoundPointAtOrigin> for RoundPoint {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[1] - self.group1()[1] * other.group0()[0],
            },
        }
    }
}

impl Dot<RoundPointBulk> for RoundPoint {
    type Output = Scalar;

    fn dot(self, other: RoundPointBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<RoundPointCarrierAspect> for RoundPoint {
    type Output = Scalar;

    fn dot(self, other: RoundPointCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group1()[1] * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for RoundPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] - self.group0()[3] * other.group2()[0],
            },
        }
    }
}

impl Dot<Origin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<RoundPoint> for RoundPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group1()[0],
            },
        }
    }
}

impl Dot<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[0],
            },
        }
    }
}

impl Dot<RoundPointBulk> for RoundPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: RoundPointBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = Scalar;

    fn dot(self, other: RoundPointCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Infinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Infinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0(),
            },
        }
    }
}

impl Dot<MultiVector> for RoundPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group2()[1] - self.group0()[1] * other.group2()[0],
            },
        }
    }
}

impl Dot<Origin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: Origin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0(),
            },
        }
    }
}

impl Dot<RoundPoint> for RoundPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[1] - self.group0()[1] * other.group1()[0],
            },
        }
    }
}

impl Dot<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[3],
            },
        }
    }
}

impl Dot<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group0()[0],
            },
        }
    }
}

impl Dot<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = Scalar;

    fn dot(self, other: RoundPointCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[1] * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for RoundPointBulk {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<RoundPoint> for RoundPointBulk {
    type Output = Scalar;

    fn dot(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<RoundPointAtInfinity> for RoundPointBulk {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<RoundPointBulk> for RoundPointBulk {
    type Output = Scalar;

    fn dot(self, other: RoundPointBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = Scalar;

    fn dot(self, other: RoundPointCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Infinity> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: Infinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<MultiVector> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] - self.group0()[3] * other.group2()[1],
            },
        }
    }
}

impl Dot<RoundPoint> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: RoundPoint) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group1()[1],
            },
        }
    }
}

impl Dot<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: RoundPointAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl Dot<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: RoundPointBulk) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn dot(self, other: RoundPointCarrierAspect) -> Scalar {
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

impl Dot<FlectorAtInfinity> for Sphere {
    type Output = Scalar;

    fn dot(self, other: FlectorAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[0] * other.group0()[3],
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

impl Dot<PlaneAtOrigin> for Sphere {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
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

impl Dot<SphereWeight> for Sphere {
    type Output = Scalar;

    fn dot(self, other: SphereWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[1] * other.group0(),
            },
        }
    }
}

impl Dot<Transflector> for Sphere {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] + self.group1()[0] * other.group1()[3],
            },
        }
    }
}

impl Dot<Flector> for SphereWeight {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group1()[3],
            },
        }
    }
}

impl Dot<FlectorAtInfinity> for SphereWeight {
    type Output = Scalar;

    fn dot(self, other: FlectorAtInfinity) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<Horizon> for SphereWeight {
    type Output = Scalar;

    fn dot(self, other: Horizon) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl Dot<MultiVector> for SphereWeight {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group10()[1],
            },
        }
    }
}

impl Dot<Plane> for SphereWeight {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0()[3],
            },
        }
    }
}

impl Dot<Sphere> for SphereWeight {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group1()[1],
            },
        }
    }
}

impl Dot<Transflector> for SphereWeight {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group1()[3],
            },
        }
    }
}

impl Dot<Dipole> for Transflector {
    type Output = Scalar;

    fn dot(self, other: Dipole) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleCarrierAspect> for Transflector {
    type Output = Scalar;

    fn dot(self, other: DipoleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<DipoleWeight> for Transflector {
    type Output = Scalar;

    fn dot(self, other: DipoleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Flector> for Transflector {
    type Output = Scalar;

    fn dot(self, other: Flector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<MultiVector> for Transflector {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0
                    - self.group0()[0] * other.group3()[0]
                    - self.group0()[1] * other.group3()[1]
                    - self.group0()[2] * other.group3()[2]
                    - self.group1()[0] * other.group9()[0]
                    - self.group1()[1] * other.group9()[1]
                    - self.group1()[2] * other.group9()[2]
                    + self.group1()[3] * other.group10()[0],
            },
        }
    }
}

impl Dot<Plane> for Transflector {
    type Output = Scalar;

    fn dot(self, other: Plane) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<PlaneAtOrigin> for Transflector {
    type Output = Scalar;

    fn dot(self, other: PlaneAtOrigin) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Sphere> for Transflector {
    type Output = Scalar;

    fn dot(self, other: Sphere) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] + self.group1()[3] * other.group1()[0],
            },
        }
    }
}

impl Dot<SphereWeight> for Transflector {
    type Output = Scalar;

    fn dot(self, other: SphereWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group1()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Transflector> for Transflector {
    type Output = Scalar;

    fn dot(self, other: Transflector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2],
            },
        }
    }
}

impl Dot<AntiScalar> for Translator {
    type Output = Scalar;

    fn dot(self, other: AntiScalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0(),
            },
        }
    }
}

impl Dot<Circle> for Translator {
    type Output = Scalar;

    fn dot(self, other: Circle) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<CircleCarrierAspect> for Translator {
    type Output = Scalar;

    fn dot(self, other: CircleCarrierAspect) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<CircleWeight> for Translator {
    type Output = Scalar;

    fn dot(self, other: CircleWeight) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2],
            },
        }
    }
}

impl Dot<Magnitude> for Translator {
    type Output = Scalar;

    fn dot(self, other: Magnitude) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl Dot<Motor> for Translator {
    type Output = Scalar;

    fn dot(self, other: Motor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<MultiVector> for Translator {
    type Output = Scalar;

    fn dot(self, other: MultiVector) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0()[0] * other.group6()[0] + self.group0()[1] * other.group6()[1] + self.group0()[2] * other.group6()[2] - self.group0()[3] * other.group0()[1],
            },
        }
    }
}

impl Dot<Rotor> for Translator {
    type Output = Scalar;

    fn dot(self, other: Rotor) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}

impl Dot<Translator> for Translator {
    type Output = Scalar;

    fn dot(self, other: Translator) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: 0.0 - self.group0()[3] * other.group0()[3],
            },
        }
    }
}
