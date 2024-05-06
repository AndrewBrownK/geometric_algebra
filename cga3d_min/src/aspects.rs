//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::products::geometric::*;
use crate::*;

/// The Bulk of an object usually describes the object's relationship with the origin.
/// An object with a Bulk of zero contains the origin.
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
pub trait Bulk {
    type Output;
    fn bulk(self) -> Self::Output;
}

/// The Weight of an object usually describes the object's attitude and orientation.
/// An object with zero weight is contained by the horizon.
/// Also known as the attitude operator.
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
pub trait Weight {
    type Output;
    fn weight(self) -> Self::Output;
}

/// Round Bulk is a special type of bulk in CGA
/// https://conformalgeometricalgebra.com/wiki/index.php?title=Main_Page
pub trait RoundBulk {
    type Output;
    fn round_bulk(self) -> Self::Output;
}

/// Round Weight is a special type of weight in CGA
/// https://conformalgeometricalgebra.com/wiki/index.php?title=Main_Page
pub trait RoundWeight {
    type Output;
    fn round_weight(self) -> Self::Output;
}

impl Bulk for Circle {
    type Output = Circle;

    fn bulk(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: self.group2(),
            },
        }
    }
}

impl Bulk for Dipole {
    type Output = Dipole;

    fn bulk(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: self.group2() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl Bulk for FlatPoint {
    type Output = FlatPoint;

    fn bulk(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl Bulk for Flector {
    type Output = Flector;

    fn bulk(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
                g1: self.group1() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}

impl Bulk for Line {
    type Output = Line;

    fn bulk(self) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1(),
            },
        }
    }
}

impl Bulk for Motor {
    type Output = Motor;

    fn bulk(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(0.0),
                g1: self.group1(),
            },
        }
    }
}

impl Bulk for MultiVector {
    type Output = MultiVector;

    fn bulk(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: self.group2() * Simd32x2::from([0.0, 1.0]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group5() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group8(),
                g9: Simd32x3::from(0.0),
                g10: self.group10() * Simd32x2::from([0.0, 1.0]),
            },
        }
    }
}

impl Bulk for Plane {
    type Output = Plane;

    fn bulk(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}

impl Bulk for RoundPoint {
    type Output = RoundPoint;

    fn bulk(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1() * Simd32x2::from([0.0, 1.0]),
            },
        }
    }
}

impl Bulk for Sphere {
    type Output = Sphere;

    fn bulk(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1() * Simd32x2::from([0.0, 1.0]),
            },
        }
    }
}

impl Bulk for Transflector {
    type Output = Transflector;

    fn bulk(self) -> Transflector {
        Transflector {
            groups: TransflectorGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}

impl Bulk for Translator {
    type Output = Translator;

    fn bulk(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl RoundBulk for Circle {
    type Output = Circle;

    fn round_bulk(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl RoundBulk for Dipole {
    type Output = Dipole;

    fn round_bulk(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1(),
                g2: Simd32x4::from(0.0),
            },
        }
    }
}

impl RoundBulk for DualNum {
    type Output = DualNum;

    fn round_bulk(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * Simd32x2::from([1.0, 0.0]),
            },
        }
    }
}

impl RoundBulk for MultiVector {
    type Output = MultiVector;

    fn round_bulk(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from([1.0, 0.0]),
                g1: self.group1(),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: self.group4(),
                g5: Simd32x4::from(0.0),
                g6: self.group6() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl RoundBulk for RoundPoint {
    type Output = RoundPoint;

    fn round_bulk(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: Simd32x2::from(0.0),
            },
        }
    }
}

impl RoundBulk for Scalar {
    type Output = Scalar;

    fn round_bulk(self) -> Scalar {
        self
    }
}

impl RoundWeight for Circle {
    type Output = Circle;

    fn round_weight(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl RoundWeight for Dipole {
    type Output = Dipole;

    fn round_weight(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from(0.0),
            },
        }
    }
}

impl RoundWeight for MultiVector {
    type Output = MultiVector;

    fn round_weight(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: self.group2() * Simd32x2::from([1.0, 0.0]),
                g3: self.group3(),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group6() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: self.group10() * Simd32x2::from([1.0, 0.0]),
            },
        }
    }
}

impl RoundWeight for RoundPoint {
    type Output = RoundPoint;

    fn round_weight(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1() * Simd32x2::from([1.0, 0.0]),
            },
        }
    }
}

impl RoundWeight for Sphere {
    type Output = Sphere;

    fn round_weight(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1() * Simd32x2::from([1.0, 0.0]),
            },
        }
    }
}

impl Weight for AntiScalar {
    type Output = AntiScalar;

    fn weight(self) -> AntiScalar {
        self
    }
}

impl Weight for Circle {
    type Output = Circle;

    fn weight(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0),
                g1: self.group1(),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl Weight for Dipole {
    type Output = Dipole;

    fn weight(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: self.group2() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}

impl Weight for DualNum {
    type Output = DualNum;

    fn weight(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * Simd32x2::from([0.0, 1.0]),
            },
        }
    }
}

impl Weight for FlatPoint {
    type Output = FlatPoint;

    fn weight(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}

impl Weight for Flector {
    type Output = Flector;

    fn weight(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
                g1: self.group1() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl Weight for Line {
    type Output = Line;

    fn weight(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
            },
        }
    }
}

impl Weight for Motor {
    type Output = Motor;

    fn weight(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
            },
        }
    }
}

impl Weight for MultiVector {
    type Output = MultiVector;

    fn weight(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from([0.0, 1.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group5() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
                g6: Simd32x4::from(0.0),
                g7: self.group7(),
                g8: Simd32x3::from(0.0),
                g9: self.group9(),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Weight for Plane {
    type Output = Plane;

    fn weight(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl Weight for Rotor {
    type Output = Rotor;

    fn weight(self) -> Rotor {
        self
    }
}

impl Weight for Sphere {
    type Output = Sphere;

    fn weight(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: Simd32x2::from(0.0),
            },
        }
    }
}

impl Weight for Transflector {
    type Output = Transflector;

    fn weight(self) -> Transflector {
        Transflector {
            groups: TransflectorGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl Weight for Translator {
    type Output = Translator;

    fn weight(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}
