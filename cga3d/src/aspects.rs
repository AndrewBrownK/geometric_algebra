//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::products::geometric::GeometricProduct;
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
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group2() },
        }
    }
}

impl Bulk for Dipole {
    type Output = FlatPointAtInfinity;

    fn bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            },
        }
    }
}

impl Bulk for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Bulk for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn bulk(self) -> FlatPointAtInfinity {
        self
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

impl Bulk for Horizon {
    type Output = Horizon;

    fn bulk(self) -> Horizon {
        self
    }
}

impl Bulk for Infinity {
    type Output = Infinity;

    fn bulk(self) -> Infinity {
        self
    }
}

impl Bulk for Line {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group1() },
        }
    }
}

impl Bulk for LineAtInfinity {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        self
    }
}

impl Bulk for Motor {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group1() },
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
    type Output = Horizon;

    fn bulk(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0()[3] },
        }
    }
}

impl Bulk for RoundPoint {
    type Output = Infinity;

    fn bulk(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group1()[1] },
        }
    }
}

impl Bulk for RoundPointAtInfinity {
    type Output = Infinity;

    fn bulk(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0()[3] },
        }
    }
}

impl Bulk for RoundPointAtOrigin {
    type Output = Infinity;

    fn bulk(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0()[1] },
        }
    }
}

impl Bulk for Sphere {
    type Output = Horizon;

    fn bulk(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group1()[1] },
        }
    }
}

impl Bulk for Translator {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl RoundBulk for Circle {
    type Output = CircleBulk;

    fn round_bulk(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: self.group0()[3] },
        }
    }
}

impl RoundBulk for CircleBulk {
    type Output = CircleBulk;

    fn round_bulk(self) -> CircleBulk {
        self
    }
}

impl RoundBulk for CircleCarrierAspect {
    type Output = CircleBulk;

    fn round_bulk(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: self.group0()[3] },
        }
    }
}

impl RoundBulk for Dipole {
    type Output = DipoleBulk;

    fn round_bulk(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups { g0: self.group1() },
        }
    }
}

impl RoundBulk for DipoleBulk {
    type Output = DipoleBulk;

    fn round_bulk(self) -> DipoleBulk {
        self
    }
}

impl RoundBulk for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn round_bulk(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups { g0: self.group1() },
        }
    }
}

impl RoundBulk for Magnitude {
    type Output = Scalar;

    fn round_bulk(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0()[0] },
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
    type Output = RoundPointBulk;

    fn round_bulk(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups { g0: self.group0() },
        }
    }
}

impl RoundBulk for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn round_bulk(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl RoundBulk for RoundPointBulk {
    type Output = RoundPointBulk;

    fn round_bulk(self) -> RoundPointBulk {
        self
    }
}

impl RoundBulk for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn round_bulk(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
    type Output = CircleWeight;

    fn round_weight(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl RoundWeight for CircleCarrierAspect {
    type Output = CircleWeight;

    fn round_weight(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl RoundWeight for CircleWeight {
    type Output = CircleWeight;

    fn round_weight(self) -> CircleWeight {
        self
    }
}

impl RoundWeight for Dipole {
    type Output = DipoleWeight;

    fn round_weight(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for DipoleCarrierAspect {
    type Output = DipoleWeight;

    fn round_weight(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for DipoleWeight {
    type Output = DipoleWeight;

    fn round_weight(self) -> DipoleWeight {
        self
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

impl RoundWeight for Origin {
    type Output = Origin;

    fn round_weight(self) -> Origin {
        self
    }
}

impl RoundWeight for RoundPoint {
    type Output = Origin;

    fn round_weight(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group1()[0] },
        }
    }
}

impl RoundWeight for RoundPointAtOrigin {
    type Output = Origin;

    fn round_weight(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0()[0] },
        }
    }
}

impl RoundWeight for RoundPointCarrierAspect {
    type Output = Origin;

    fn round_weight(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0()[3] },
        }
    }
}

impl RoundWeight for Sphere {
    type Output = SphereWeight;

    fn round_weight(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: self.group1()[0] },
        }
    }
}

impl RoundWeight for SphereWeight {
    type Output = SphereWeight;

    fn round_weight(self) -> SphereWeight {
        self
    }
}

impl Weight for AntiScalar {
    type Output = AntiScalar;

    fn weight(self) -> AntiScalar {
        self
    }
}

impl Weight for Circle {
    type Output = LineAtOrigin;

    fn weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl Weight for Dipole {
    type Output = FlatPointAtOrigin;

    fn weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group2()[3] },
        }
    }
}

impl Weight for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group0()[3] },
        }
    }
}

impl Weight for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn weight(self) -> FlatPointAtOrigin {
        self
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
    type Output = LineAtOrigin;

    fn weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Weight for LineAtOrigin {
    type Output = LineAtOrigin;

    fn weight(self) -> LineAtOrigin {
        self
    }
}

impl Weight for Magnitude {
    type Output = AntiScalar;

    fn weight(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[1] },
        }
    }
}

impl Weight for Motor {
    type Output = Rotor;

    fn weight(self) -> Rotor {
        Rotor {
            groups: RotorGroups { g0: self.group0() },
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
    type Output = PlaneAtOrigin;

    fn weight(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Weight for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight(self) -> PlaneAtOrigin {
        self
    }
}

impl Weight for Rotor {
    type Output = Rotor;

    fn weight(self) -> Rotor {
        self
    }
}

impl Weight for Sphere {
    type Output = PlaneAtOrigin;

    fn weight(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Weight for Translator {
    type Output = AntiScalar;

    fn weight(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[3] },
        }
    }
}
