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

impl FlatBulk for AntiPlane {
    type Output = Infinity;

    fn flat_bulk(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0()[3] },
        }
    }
}

impl FlatBulk for Circle {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            },
        }
    }
}

impl FlatBulk for CircleAligningOrigin {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group2() },
        }
    }
}

impl FlatBulk for CircleAtInfinity {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
        }
    }
}

impl FlatBulk for CircleAtOrigin {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group1() },
        }
    }
}

impl FlatBulk for CircleOrthogonalOrigin {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
        }
    }
}

impl FlatBulk for Dilator {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]),
            },
        }
    }
}

impl FlatBulk for Dipole {
    type Output = FlatPointAtInfinity;

    fn flat_bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            },
        }
    }
}

impl FlatBulk for DipoleAligningOrigin {
    type Output = FlatPointAtInfinity;

    fn flat_bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
        }
    }
}

impl FlatBulk for DipoleAtInfinity {
    type Output = FlatPointAtInfinity;

    fn flat_bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
        }
    }
}

impl FlatBulk for DipoleAtOrigin {
    type Output = FlatPointAtInfinity;

    fn flat_bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups { g0: self.group1() },
        }
    }
}

impl FlatBulk for DipoleOrthogonalOrigin {
    type Output = FlatPointAtInfinity;

    fn flat_bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups { g0: self.group2() },
        }
    }
}

impl FlatBulk for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn flat_bulk(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl FlatBulk for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn flat_bulk(self) -> FlatPointAtInfinity {
        self
    }
}

impl FlatBulk for Flector {
    type Output = FlectorAtInfinity;

    fn flat_bulk(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            },
        }
    }
}

impl FlatBulk for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn flat_bulk(self) -> FlectorAtInfinity {
        self
    }
}

impl FlatBulk for Horizon {
    type Output = Horizon;

    fn flat_bulk(self) -> Horizon {
        self
    }
}

impl FlatBulk for Infinity {
    type Output = Infinity;

    fn flat_bulk(self) -> Infinity {
        self
    }
}

impl FlatBulk for Line {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group1() },
        }
    }
}

impl FlatBulk for LineAtInfinity {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        self
    }
}

impl FlatBulk for Motor {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group1() },
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

impl FlatBulk for Plane {
    type Output = Horizon;

    fn flat_bulk(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0()[3] },
        }
    }
}

impl FlatBulk for RoundPoint {
    type Output = Infinity;

    fn flat_bulk(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group1()[1] },
        }
    }
}

impl FlatBulk for RoundPointAtOrigin {
    type Output = Infinity;

    fn flat_bulk(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0()[1] },
        }
    }
}

impl FlatBulk for Sphere {
    type Output = Horizon;

    fn flat_bulk(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group1()[1] },
        }
    }
}

impl FlatBulk for SphereAtOrigin {
    type Output = Horizon;

    fn flat_bulk(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0()[1] },
        }
    }
}

impl FlatBulk for Transflector {
    type Output = FlectorAtInfinity;

    fn flat_bulk(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            },
        }
    }
}

impl FlatBulk for Translator {
    type Output = LineAtInfinity;

    fn flat_bulk(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
    type Output = LineAtOrigin;

    fn flat_weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl FlatWeight for CircleAligningOrigin {
    type Output = LineAtOrigin;

    fn flat_weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl FlatWeight for CircleAtInfinity {
    type Output = LineAtOrigin;

    fn flat_weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group0() },
        }
    }
}

impl FlatWeight for CircleOnOrigin {
    type Output = LineAtOrigin;

    fn flat_weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl FlatWeight for Dilator {
    type Output = Rotor;

    fn flat_weight(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()]),
            },
        }
    }
}

impl FlatWeight for Dipole {
    type Output = FlatPointAtOrigin;

    fn flat_weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group2()[3] },
        }
    }
}

impl FlatWeight for DipoleAligningOrigin {
    type Output = FlatPointAtOrigin;

    fn flat_weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group1()[3] },
        }
    }
}

impl FlatWeight for DipoleAtInfinity {
    type Output = FlatPointAtOrigin;

    fn flat_weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group1()[3] },
        }
    }
}

impl FlatWeight for DipoleOnOrigin {
    type Output = FlatPointAtOrigin;

    fn flat_weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group0()[3] },
        }
    }
}

impl FlatWeight for DualNum {
    type Output = AntiScalar;

    fn flat_weight(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[1] },
        }
    }
}

impl FlatWeight for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn flat_weight(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group0()[3] },
        }
    }
}

impl FlatWeight for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn flat_weight(self) -> FlatPointAtOrigin {
        self
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
    type Output = LineAtOrigin;

    fn flat_weight(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group0() },
        }
    }
}

impl FlatWeight for LineAtOrigin {
    type Output = LineAtOrigin;

    fn flat_weight(self) -> LineAtOrigin {
        self
    }
}

impl FlatWeight for Motor {
    type Output = Rotor;

    fn flat_weight(self) -> Rotor {
        Rotor {
            groups: RotorGroups { g0: self.group0() },
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

impl FlatWeight for Plane {
    type Output = PlaneAtOrigin;

    fn flat_weight(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl FlatWeight for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn flat_weight(self) -> PlaneAtOrigin {
        self
    }
}

impl FlatWeight for Rotor {
    type Output = Rotor;

    fn flat_weight(self) -> Rotor {
        self
    }
}

impl FlatWeight for Sphere {
    type Output = PlaneAtOrigin;

    fn flat_weight(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl FlatWeight for SphereOnOrigin {
    type Output = PlaneAtOrigin;

    fn flat_weight(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl FlatWeight for Transflector {
    type Output = PlaneAtOrigin;

    fn flat_weight(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
        }
    }
}

impl FlatWeight for Translator {
    type Output = AntiScalar;

    fn flat_weight(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[3] },
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

impl RoundBulk for Dilator {
    type Output = AntiFlatPointAtOrigin;

    fn round_bulk(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: self.group3()[3] },
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

impl RoundWeight for Dilator {
    type Output = NullCircleAtOrigin;

    fn round_weight(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups { g0: self.group1() },
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
