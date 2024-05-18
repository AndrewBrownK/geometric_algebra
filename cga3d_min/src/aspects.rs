//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::products::geometric::*;
use crate::*;

/// Round Bulk is a special type of bulk in CGA
/// All components of the RoundBulk lack factors of Origin (e4 in cga3d) and Infinity (e5 in cga3d).
/// It is equivalent to the bulk of the carrier geometry for round objects.
/// In other words, it is the distance from the origin to the carrier.
/// A round object with zero RoundBulk is aligned with the Origin (the carrier contains the Origin).
/// Some examples of objects without a RoundBulk are DipoleAligningOrigin, CircleAligningOrigin, or flat objects.
/// https://conformalgeometricalgebra.com/wiki/index.php?title=Main_Page
pub trait RoundBulk {
    type Output;
    fn round_bulk(self) -> Self::Output;
}

/// Round Weight is a special type of weight in CGA
/// All components of the RoundWeight include the factor Origin (e4 in cga3d), but not Infinity (e5 in cga3d).
/// It is equivalent to the weight of the carrier geometry for round objects.
/// In other words, it is the orientation of the carrier.
/// A round object with zero RoundWeight is at Infinity (the carrier is contained by the Horizon).
/// Some examples of objects without a RoundWeight are DipoleAtInfinity, CircleAtInfinity, or flat objects.
/// https://conformalgeometricalgebra.com/wiki/index.php?title=Main_Page
pub trait RoundWeight {
    type Output;
    fn round_weight(self) -> Self::Output;
}

/// FlatBulk is a type of bulk in CGA.
/// All components of the FlatBulk include the factor Infinity (e5 in cga3d), but not Origin (e4 in cga3d).
/// For flat objects, the meaning is the same as `Bulk` in rigid geometric algebra.
///
/// The Bulk of an object usually describes the object's relationship with the origin.
/// An object with a Bulk of zero contains the origin.
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
pub trait FlatBulk {
    type Output;
    fn flat_bulk(self) -> Self::Output;
}

/// FlatWeight is a type of weight in CGA.
/// All components of the FlatWeight contain factors of Origin (e4 in cga3d) and Infinity (e5 in cga3d).
/// For flat objects, the meaning is the same as `Weight` in rigid geometric algebra.
///
/// The Weight of an object usually describes the object's attitude and orientation.
/// An object with zero weight is contained by the horizon.
/// Also known as the attitude operator.
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
pub trait FlatWeight {
    type Output;
    fn flat_weight(self) -> Self::Output;
}

impl FlatBulk for Circle {
    type Output = Circle;

    fn flat_bulk(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: self.group2(),
            },
        }
    }
}

impl FlatBulk for Dipole {
    type Output = Dipole;

    fn flat_bulk(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: self.group2() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl FlatBulk for FlatPoint {
    type Output = FlatPoint;

    fn flat_bulk(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl FlatBulk for Flector {
    type Output = Flector;

    fn flat_bulk(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
                g1: self.group1() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}

impl FlatBulk for Line {
    type Output = Line;

    fn flat_bulk(self) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1(),
            },
        }
    }
}

impl FlatBulk for Motor {
    type Output = Motor;

    fn flat_bulk(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(0.0),
                g1: self.group1(),
            },
        }
    }
}

impl FlatBulk for MultiVector {
    type Output = MultiVector;

    fn flat_bulk(self) -> MultiVector {
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

impl FlatBulk for Plane {
    type Output = Plane;

    fn flat_bulk(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}

impl FlatBulk for RoundPoint {
    type Output = RoundPoint;

    fn flat_bulk(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1() * Simd32x2::from([0.0, 1.0]),
            },
        }
    }
}

impl FlatBulk for Sphere {
    type Output = Sphere;

    fn flat_bulk(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group1() * Simd32x2::from([0.0, 1.0]),
            },
        }
    }
}

impl FlatWeight for AntiScalar {
    type Output = AntiScalar;

    fn flat_weight(self) -> AntiScalar {
        self
    }
}

impl FlatWeight for Circle {
    type Output = Circle;

    fn flat_weight(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0),
                g1: self.group1(),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl FlatWeight for Dipole {
    type Output = Dipole;

    fn flat_weight(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: self.group2() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}

impl FlatWeight for DualNum {
    type Output = DualNum;

    fn flat_weight(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * Simd32x2::from([0.0, 1.0]),
            },
        }
    }
}

impl FlatWeight for FlatPoint {
    type Output = FlatPoint;

    fn flat_weight(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        }
    }
}

impl FlatWeight for Flector {
    type Output = Flector;

    fn flat_weight(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
                g1: self.group1() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl FlatWeight for Line {
    type Output = Line;

    fn flat_weight(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
            },
        }
    }
}

impl FlatWeight for Motor {
    type Output = Motor;

    fn flat_weight(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
            },
        }
    }
}

impl FlatWeight for MultiVector {
    type Output = MultiVector;

    fn flat_weight(self) -> MultiVector {
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

impl FlatWeight for Plane {
    type Output = Plane;

    fn flat_weight(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            },
        }
    }
}

impl FlatWeight for Sphere {
    type Output = Sphere;

    fn flat_weight(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: Simd32x2::from(0.0),
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
