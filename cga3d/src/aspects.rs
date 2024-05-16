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

impl Bulk for AntiPlane {
    type Output = Infinity;

    fn bulk(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0()[3] },
        }
    }
}

impl Bulk for Circle {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            },
        }
    }
}

impl Bulk for CircleAligningOrigin {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group2() },
        }
    }
}

impl Bulk for CircleAtInfinity {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
        }
    }
}

impl Bulk for CircleAtOrigin {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group1() },
        }
    }
}

impl Bulk for CircleOrthogonalOrigin {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
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

impl Bulk for DipoleAligningOrigin {
    type Output = FlatPointAtInfinity;

    fn bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
        }
    }
}

impl Bulk for DipoleAtInfinity {
    type Output = FlatPointAtInfinity;

    fn bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
        }
    }
}

impl Bulk for DipoleAtOrigin {
    type Output = FlatPointAtInfinity;

    fn bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups { g0: self.group1() },
        }
    }
}

impl Bulk for DipoleOrthogonalOrigin {
    type Output = FlatPointAtInfinity;

    fn bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups { g0: self.group2() },
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
    type Output = FlectorAtInfinity;

    fn bulk(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            },
        }
    }
}

impl Bulk for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn bulk(self) -> FlectorAtInfinity {
        self
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group5(),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group8() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
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

impl Bulk for SphereAtOrigin {
    type Output = Horizon;

    fn bulk(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0()[1] },
        }
    }
}

impl Bulk for Transflector {
    type Output = FlectorAtInfinity;

    fn bulk(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            },
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

impl RoundBulk for AntiCircleOnOrigin {
    type Output = AntiLineAtOrigin;

    fn round_bulk(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl RoundBulk for AntiDipoleOnOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn round_bulk(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: self.group0()[3] },
        }
    }
}

impl RoundBulk for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn round_bulk(self) -> AntiFlatPointAtOrigin {
        self
    }
}

impl RoundBulk for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn round_bulk(self) -> AntiLineAtOrigin {
        self
    }
}

impl RoundBulk for AntiPlane {
    type Output = AntiPlaneAtOrigin;

    fn round_bulk(self) -> AntiPlaneAtOrigin {
        AntiPlaneAtOrigin {
            groups: AntiPlaneAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl RoundBulk for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn round_bulk(self) -> AntiPlaneAtOrigin {
        self
    }
}

impl RoundBulk for AntiSphereOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn round_bulk(self) -> AntiPlaneAtOrigin {
        AntiPlaneAtOrigin {
            groups: AntiPlaneAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl RoundBulk for Circle {
    type Output = AntiFlatPointAtOrigin;

    fn round_bulk(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: self.group2()[3] },
        }
    }
}

impl RoundBulk for CircleAtInfinity {
    type Output = AntiFlatPointAtOrigin;

    fn round_bulk(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: self.group1()[3] },
        }
    }
}

impl RoundBulk for CircleOrthogonalOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn round_bulk(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: self.group1()[3] },
        }
    }
}

impl RoundBulk for Dipole {
    type Output = AntiLineAtOrigin;

    fn round_bulk(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl RoundBulk for DipoleAtInfinity {
    type Output = AntiLineAtOrigin;

    fn round_bulk(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundBulk for DipoleOrthogonalOrigin {
    type Output = AntiLineAtOrigin;

    fn round_bulk(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl RoundBulk for DualNum {
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
                g3: Simd32x4::from(0.0),
                g4: self.group4(),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group8() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl RoundBulk for RoundPoint {
    type Output = AntiPlaneAtOrigin;

    fn round_bulk(self) -> AntiPlaneAtOrigin {
        AntiPlaneAtOrigin {
            groups: AntiPlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundBulk for Scalar {
    type Output = Scalar;

    fn round_bulk(self) -> Scalar {
        self
    }
}

impl RoundWeight for AntiCircleOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for AntiDipoleOnOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl RoundWeight for AntiSphereOnOrigin {
    type Output = Origin;

    fn round_weight(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0()[3] },
        }
    }
}

impl RoundWeight for Circle {
    type Output = NullCircleAtOrigin;

    fn round_weight(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for CircleAligningOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for CircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for CircleOnOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for CircleOrthogonalOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for Dipole {
    type Output = NullDipoleAtOrigin;

    fn round_weight(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for DipoleAligningOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for DipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RoundWeight for DipoleOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl RoundWeight for DipoleOrthogonalOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups { g0: self.group0() },
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
                g3: self.group3() * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: self.group6(),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: self.group10() * Simd32x2::from([1.0, 0.0]),
            },
        }
    }
}

impl RoundWeight for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight(self) -> NullCircleAtOrigin {
        self
    }
}

impl RoundWeight for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight(self) -> NullDipoleAtOrigin {
        self
    }
}

impl RoundWeight for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn round_weight(self) -> NullSphereAtOrigin {
        self
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

impl RoundWeight for Sphere {
    type Output = NullSphereAtOrigin;

    fn round_weight(self) -> NullSphereAtOrigin {
        NullSphereAtOrigin {
            groups: NullSphereAtOriginGroups { g0: self.group1()[0] },
        }
    }
}

impl RoundWeight for SphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn round_weight(self) -> NullSphereAtOrigin {
        NullSphereAtOrigin {
            groups: NullSphereAtOriginGroups { g0: self.group0()[0] },
        }
    }
}

impl RoundWeight for SphereOnOrigin {
    type Output = NullSphereAtOrigin;

    fn round_weight(self) -> NullSphereAtOrigin {
        NullSphereAtOrigin {
            groups: NullSphereAtOriginGroups { g0: self.group0()[3] },
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
    type Output = LineAtOrigin;

    fn weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl Weight for CircleAligningOrigin {
    type Output = LineAtOrigin;

    fn weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl Weight for CircleAtInfinity {
    type Output = LineAtOrigin;

    fn weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Weight for CircleOnOrigin {
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

impl Weight for DipoleAligningOrigin {
    type Output = FlatPointAtOrigin;

    fn weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group1()[3] },
        }
    }
}

impl Weight for DipoleAtInfinity {
    type Output = FlatPointAtOrigin;

    fn weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group1()[3] },
        }
    }
}

impl Weight for DipoleOnOrigin {
    type Output = FlatPointAtOrigin;

    fn weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group0()[3] },
        }
    }
}

impl Weight for DualNum {
    type Output = AntiScalar;

    fn weight(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[1] },
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
                g3: self.group3() * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: self.group7(),
                g8: Simd32x4::from(0.0),
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

impl Weight for SphereOnOrigin {
    type Output = PlaneAtOrigin;

    fn weight(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Weight for Transflector {
    type Output = PlaneAtOrigin;

    fn weight(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
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
