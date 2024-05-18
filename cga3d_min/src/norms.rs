//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspects::*;
use crate::characteristics::*;
use crate::involutions::*;
use crate::products::dot::*;
use crate::products::exterior::*;
use crate::*;

/// CenterNorm
/// Note that this does not measure unitized distance unless you
/// combine it with the RoundWeightNorm. You can do this by unitizing
/// the object before taking this CenterNorm, or adding the RoundWeightNorm
/// and unitizing the resulting DualNum, or just invoking UnitizedCenterNorm instead.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait CenterNorm {
    type Output;
    fn center_norm(self) -> Self::Output;
}

/// CenterNormSquared
/// Note that this does not measure unitized distance squared unless you
/// combine it with the RoundWeightNormSquared. You can do this by unitizing
/// the object before taking this CenterNormSquared, or adding the RoundWeightNormSquared
/// and unitizing the resulting DualNum, or just invoking UnitizedCenterNormSquared instead.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait CenterNormSquared {
    type Output;
    fn center_norm_squared(self) -> Self::Output;
}

/// FlatBulkNorm
/// Note that this does not measure unitized distance unless you combine
/// it with the corresponding weight norm. You can do this by unitizing the object
/// before taking this FlatBulkNorm, or adding the corresponding weight norm and
/// unitizing the resulting DualNum.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait FlatBulkNorm {
    type Output;
    fn flat_bulk_norm(self) -> Self::Output;
}

/// FlatBulkNormSquared
/// Note that this does not measure unitized distance squared unless you combine
/// it with the corresponding weight norm. You can do this by unitizing the object
/// before taking this FlatBulkNormSquared, or adding the corresponding weight norm and
/// unitizing the resulting DualNum.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait FlatBulkNormSquared {
    type Output;
    fn flat_bulk_norm_squared(self) -> Self::Output;
}

/// FlatNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait FlatNorm {
    type Output;
    fn flat_norm(self) -> Self::Output;
}

/// FlatNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait FlatNormSquared {
    type Output;
    fn flat_norm_squared(self) -> Self::Output;
}

/// FlatWeightNorm
/// Note that this does not provide a unitized orientation unless your object
/// is unitized first. Sometimes you want the weight norm before unitization
/// so you can perform unitization later.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait FlatWeightNorm {
    type Output;
    fn flat_weight_norm(self) -> Self::Output;
}

/// FlatWeightNormSquared
/// Note that this does not provide a unitized orientation unless your object
/// is unitized first. Sometimes you want the weight norm before unitization
/// so you can perform unitization later.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait FlatWeightNormSquared {
    type Output;
    fn flat_weight_norm_squared(self) -> Self::Output;
}

/// RadiusNorm
/// Note that this does not measure unitized distance unless you
/// combine it with the RoundWeightNorm. You can do this by unitizing
/// the object before taking this RadiusNorm, or adding the RoundWeightNorm
/// and unitizing the resulting DualNum, or just invoking UnitizedRadiusNorm instead.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait RadiusNorm {
    type Output;
    fn radius_norm(self) -> Self::Output;
}

/// RadiusNormSquared
/// Note that this does not measure unitized distance squared unless you
/// combine it with the RoundWeightNormSquared. You can do this by unitizing
/// the object before taking this RadiusNormSquared, or adding the RoundWeightNormSquared
/// and unitizing the resulting DualNum, or just invoking UnitizedRadiusNormSquared instead.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait RadiusNormSquared {
    type Output;
    fn radius_norm_squared(self) -> Self::Output;
}

/// RoundBulkNorm
/// Note that this does not measure unitized distance unless you combine
/// it with the corresponding weight norm. You can do this by unitizing the object
/// before taking this RoundBulkNorm, or adding the corresponding weight norm and
/// unitizing the resulting DualNum.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait RoundBulkNorm {
    type Output;
    fn round_bulk_norm(self) -> Self::Output;
}

/// RoundBulkNormSquared
/// Note that this does not measure unitized distance squared unless you combine
/// it with the corresponding weight norm. You can do this by unitizing the object
/// before taking this RoundBulkNormSquared, or adding the corresponding weight norm and
/// unitizing the resulting DualNum.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait RoundBulkNormSquared {
    type Output;
    fn round_bulk_norm_squared(self) -> Self::Output;
}

/// RoundNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait RoundNorm {
    type Output;
    fn round_norm(self) -> Self::Output;
}

/// RoundNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait RoundNormSquared {
    type Output;
    fn round_norm_squared(self) -> Self::Output;
}

/// RoundWeightNorm
/// Note that this does not provide a unitized orientation unless your object
/// is unitized first. Sometimes you want the weight norm before unitization
/// so you can perform unitization later.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait RoundWeightNorm {
    type Output;
    fn round_weight_norm(self) -> Self::Output;
}

/// RoundWeightNormSquared
/// Note that this does not provide a unitized orientation unless your object
/// is unitized first. Sometimes you want the weight norm before unitization
/// so you can perform unitization later.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait RoundWeightNormSquared {
    type Output;
    fn round_weight_norm_squared(self) -> Self::Output;
}

/// UnitizedCenterNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedCenterNorm {
    type Output;
    fn unitized_center_norm(self) -> Self::Output;
}

/// UnitizedCenterNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedCenterNormSquared {
    type Output;
    fn unitized_center_norm_squared(self) -> Self::Output;
}

/// UnitizedFlatNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedFlatNorm {
    type Output;
    fn unitized_flat_norm(self) -> Self::Output;
}

/// UnitizedFlatNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedFlatNormSquared {
    type Output;
    fn unitized_flat_norm_squared(self) -> Self::Output;
}

/// UnitizedRadiusNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedRadiusNorm {
    type Output;
    fn unitized_radius_norm(self) -> Self::Output;
}

/// UnitizedRadiusNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedRadiusNormSquared {
    type Output;
    fn unitized_radius_norm_squared(self) -> Self::Output;
}

/// UnitizedRoundNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedRoundNorm {
    type Output;
    fn unitized_round_norm(self) -> Self::Output;
}

/// UnitizedRoundNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedRoundNormSquared {
    type Output;
    fn unitized_round_norm_squared(self) -> Self::Output;
}

impl RoundBulkNormSquared for Circle {
    type Output = Scalar;

    fn round_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk_carrier: Circle = self.round_bulk();
        round_bulk_carrier.dot(round_bulk_carrier)
    }
}

impl RoundBulkNormSquared for Dipole {
    type Output = Scalar;

    fn round_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk_carrier: Dipole = self.round_bulk();
        round_bulk_carrier.dot(round_bulk_carrier)
    }
}

impl RoundBulkNormSquared for DualNum {
    type Output = Scalar;

    fn round_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk_carrier: DualNum = self.round_bulk();
        round_bulk_carrier.dot(round_bulk_carrier)
    }
}

impl RoundBulkNormSquared for MultiVector {
    type Output = Scalar;

    fn round_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk_carrier: MultiVector = self.round_bulk();
        round_bulk_carrier.dot(round_bulk_carrier)
    }
}

impl RoundBulkNormSquared for RoundPoint {
    type Output = Scalar;

    fn round_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk_carrier: RoundPoint = self.round_bulk();
        round_bulk_carrier.dot(round_bulk_carrier)
    }
}

impl RoundBulkNormSquared for Scalar {
    type Output = Scalar;

    fn round_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk_carrier: Scalar = self.round_bulk();
        round_bulk_carrier.dot(round_bulk_carrier)
    }
}

impl RoundBulkNorm for Circle {
    type Output = Scalar;

    fn round_bulk_norm(self) -> Scalar {
        self.round_bulk_norm_squared().sqrt()
    }
}

impl RoundBulkNorm for Dipole {
    type Output = Scalar;

    fn round_bulk_norm(self) -> Scalar {
        self.round_bulk_norm_squared().sqrt()
    }
}

impl RoundBulkNorm for DualNum {
    type Output = Scalar;

    fn round_bulk_norm(self) -> Scalar {
        self.round_bulk_norm_squared().sqrt()
    }
}

impl RoundBulkNorm for MultiVector {
    type Output = Scalar;

    fn round_bulk_norm(self) -> Scalar {
        self.round_bulk_norm_squared().sqrt()
    }
}

impl RoundBulkNorm for RoundPoint {
    type Output = Scalar;

    fn round_bulk_norm(self) -> Scalar {
        self.round_bulk_norm_squared().sqrt()
    }
}

impl RoundBulkNorm for Scalar {
    type Output = Scalar;

    fn round_bulk_norm(self) -> Scalar {
        self.round_bulk_norm_squared().sqrt()
    }
}

impl RoundWeightNormSquared for Circle {
    type Output = AntiScalar;

    fn round_weight_norm_squared(self) -> AntiScalar {
        let mut round_weight_carrier: Sphere = self.round_weight().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        });
        round_weight_carrier.anti_dot(round_weight_carrier)
    }
}

impl RoundWeightNormSquared for Dipole {
    type Output = AntiScalar;

    fn round_weight_norm_squared(self) -> AntiScalar {
        let mut round_weight_carrier: Circle = self.round_weight().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        });
        round_weight_carrier.anti_dot(round_weight_carrier)
    }
}

impl RoundWeightNormSquared for MultiVector {
    type Output = AntiScalar;

    fn round_weight_norm_squared(self) -> AntiScalar {
        let mut round_weight_carrier: MultiVector = self.round_weight().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        });
        round_weight_carrier.anti_dot(round_weight_carrier)
    }
}

impl RoundWeightNormSquared for RoundPoint {
    type Output = AntiScalar;

    fn round_weight_norm_squared(self) -> AntiScalar {
        let mut round_weight_carrier: Dipole = self.round_weight().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        });
        round_weight_carrier.anti_dot(round_weight_carrier)
    }
}

impl RoundWeightNormSquared for Sphere {
    type Output = AntiScalar;

    fn round_weight_norm_squared(self) -> AntiScalar {
        let mut round_weight_carrier: AntiScalar = self.round_weight().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        });
        round_weight_carrier.anti_dot(round_weight_carrier)
    }
}

impl RoundWeightNorm for Circle {
    type Output = AntiScalar;

    fn round_weight_norm(self) -> AntiScalar {
        self.round_weight_norm_squared().anti_sqrt()
    }
}

impl RoundWeightNorm for Dipole {
    type Output = AntiScalar;

    fn round_weight_norm(self) -> AntiScalar {
        self.round_weight_norm_squared().anti_sqrt()
    }
}

impl RoundWeightNorm for MultiVector {
    type Output = AntiScalar;

    fn round_weight_norm(self) -> AntiScalar {
        self.round_weight_norm_squared().anti_sqrt()
    }
}

impl RoundWeightNorm for RoundPoint {
    type Output = AntiScalar;

    fn round_weight_norm(self) -> AntiScalar {
        self.round_weight_norm_squared().anti_sqrt()
    }
}

impl RoundWeightNorm for Sphere {
    type Output = AntiScalar;

    fn round_weight_norm(self) -> AntiScalar {
        self.round_weight_norm_squared().anti_sqrt()
    }
}

impl RoundNormSquared for Circle {
    type Output = DualNum;

    fn round_norm_squared(self) -> DualNum {
        self.round_bulk_norm_squared().add(self.round_weight_norm_squared())
    }
}

impl RoundNormSquared for Dipole {
    type Output = DualNum;

    fn round_norm_squared(self) -> DualNum {
        self.round_bulk_norm_squared().add(self.round_weight_norm_squared())
    }
}

impl RoundNormSquared for MultiVector {
    type Output = DualNum;

    fn round_norm_squared(self) -> DualNum {
        self.round_bulk_norm_squared().add(self.round_weight_norm_squared())
    }
}

impl RoundNormSquared for RoundPoint {
    type Output = DualNum;

    fn round_norm_squared(self) -> DualNum {
        self.round_bulk_norm_squared().add(self.round_weight_norm_squared())
    }
}

impl RoundNorm for Circle {
    type Output = DualNum;

    fn round_norm(self) -> DualNum {
        self.round_bulk_norm().add(self.round_weight_norm())
    }
}

impl RoundNorm for Dipole {
    type Output = DualNum;

    fn round_norm(self) -> DualNum {
        self.round_bulk_norm().add(self.round_weight_norm())
    }
}

impl RoundNorm for MultiVector {
    type Output = DualNum;

    fn round_norm(self) -> DualNum {
        self.round_bulk_norm().add(self.round_weight_norm())
    }
}

impl RoundNorm for RoundPoint {
    type Output = DualNum;

    fn round_norm(self) -> DualNum {
        self.round_bulk_norm().add(self.round_weight_norm())
    }
}

impl UnitizedRoundNormSquared for Circle {
    type Output = f32;

    fn unitized_round_norm_squared(self) -> f32 {
        self.round_bulk_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedRoundNormSquared for Dipole {
    type Output = f32;

    fn unitized_round_norm_squared(self) -> f32 {
        self.round_bulk_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedRoundNormSquared for MultiVector {
    type Output = f32;

    fn unitized_round_norm_squared(self) -> f32 {
        self.round_bulk_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedRoundNormSquared for RoundPoint {
    type Output = f32;

    fn unitized_round_norm_squared(self) -> f32 {
        self.round_bulk_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedRoundNorm for Circle {
    type Output = f32;

    fn unitized_round_norm(self) -> f32 {
        self.unitized_round_norm_squared().sqrt()
    }
}

impl UnitizedRoundNorm for Dipole {
    type Output = f32;

    fn unitized_round_norm(self) -> f32 {
        self.unitized_round_norm_squared().sqrt()
    }
}

impl UnitizedRoundNorm for MultiVector {
    type Output = f32;

    fn unitized_round_norm(self) -> f32 {
        self.unitized_round_norm_squared().sqrt()
    }
}

impl UnitizedRoundNorm for RoundPoint {
    type Output = f32;

    fn unitized_round_norm(self) -> f32 {
        self.unitized_round_norm_squared().sqrt()
    }
}

impl FlatBulkNormSquared for Circle {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Sphere = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNormSquared for Dipole {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Circle = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNormSquared for FlatPoint {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Line = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNormSquared for Flector {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Motor = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNormSquared for Line {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Plane = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNormSquared for Motor {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Plane = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNormSquared for MultiVector {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: MultiVector = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNormSquared for Plane {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: AntiScalar = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNormSquared for RoundPoint {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Dipole = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNormSquared for Sphere {
    type Output = Scalar;

    fn flat_bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: AntiScalar = self.flat_bulk().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([1.0, 0.0]),
            },
        });
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl FlatBulkNorm for Circle {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatBulkNorm for Dipole {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatBulkNorm for FlatPoint {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatBulkNorm for Flector {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatBulkNorm for Line {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatBulkNorm for Motor {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatBulkNorm for MultiVector {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatBulkNorm for Plane {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatBulkNorm for RoundPoint {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatBulkNorm for Sphere {
    type Output = Scalar;

    fn flat_bulk_norm(self) -> Scalar {
        self.flat_bulk_norm_squared().sqrt()
    }
}

impl FlatWeightNormSquared for AntiScalar {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: AntiScalar = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for Circle {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Circle = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for Dipole {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Dipole = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for DualNum {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: DualNum = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for FlatPoint {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: FlatPoint = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for Flector {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Flector = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for Line {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Line = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for Motor {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Motor = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for MultiVector {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: MultiVector = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for Plane {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Plane = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNormSquared for Sphere {
    type Output = AntiScalar;

    fn flat_weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Sphere = self.flat_weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl FlatWeightNorm for AntiScalar {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for Circle {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for Dipole {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for DualNum {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for FlatPoint {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for Flector {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for Line {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for Motor {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for MultiVector {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for Plane {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatWeightNorm for Sphere {
    type Output = AntiScalar;

    fn flat_weight_norm(self) -> AntiScalar {
        self.flat_weight_norm_squared().anti_sqrt()
    }
}

impl FlatNormSquared for Circle {
    type Output = DualNum;

    fn flat_norm_squared(self) -> DualNum {
        self.flat_bulk_norm_squared().add(self.flat_weight_norm_squared())
    }
}

impl FlatNormSquared for Dipole {
    type Output = DualNum;

    fn flat_norm_squared(self) -> DualNum {
        self.flat_bulk_norm_squared().add(self.flat_weight_norm_squared())
    }
}

impl FlatNormSquared for FlatPoint {
    type Output = DualNum;

    fn flat_norm_squared(self) -> DualNum {
        self.flat_bulk_norm_squared().add(self.flat_weight_norm_squared())
    }
}

impl FlatNormSquared for Flector {
    type Output = DualNum;

    fn flat_norm_squared(self) -> DualNum {
        self.flat_bulk_norm_squared().add(self.flat_weight_norm_squared())
    }
}

impl FlatNormSquared for Line {
    type Output = DualNum;

    fn flat_norm_squared(self) -> DualNum {
        self.flat_bulk_norm_squared().add(self.flat_weight_norm_squared())
    }
}

impl FlatNormSquared for Motor {
    type Output = DualNum;

    fn flat_norm_squared(self) -> DualNum {
        self.flat_bulk_norm_squared().add(self.flat_weight_norm_squared())
    }
}

impl FlatNormSquared for MultiVector {
    type Output = DualNum;

    fn flat_norm_squared(self) -> DualNum {
        self.flat_bulk_norm_squared().add(self.flat_weight_norm_squared())
    }
}

impl FlatNormSquared for Plane {
    type Output = DualNum;

    fn flat_norm_squared(self) -> DualNum {
        self.flat_bulk_norm_squared().add(self.flat_weight_norm_squared())
    }
}

impl FlatNormSquared for Sphere {
    type Output = DualNum;

    fn flat_norm_squared(self) -> DualNum {
        self.flat_bulk_norm_squared().add(self.flat_weight_norm_squared())
    }
}

impl FlatNorm for Circle {
    type Output = DualNum;

    fn flat_norm(self) -> DualNum {
        self.flat_bulk_norm().add(self.flat_weight_norm())
    }
}

impl FlatNorm for Dipole {
    type Output = DualNum;

    fn flat_norm(self) -> DualNum {
        self.flat_bulk_norm().add(self.flat_weight_norm())
    }
}

impl FlatNorm for FlatPoint {
    type Output = DualNum;

    fn flat_norm(self) -> DualNum {
        self.flat_bulk_norm().add(self.flat_weight_norm())
    }
}

impl FlatNorm for Flector {
    type Output = DualNum;

    fn flat_norm(self) -> DualNum {
        self.flat_bulk_norm().add(self.flat_weight_norm())
    }
}

impl FlatNorm for Line {
    type Output = DualNum;

    fn flat_norm(self) -> DualNum {
        self.flat_bulk_norm().add(self.flat_weight_norm())
    }
}

impl FlatNorm for Motor {
    type Output = DualNum;

    fn flat_norm(self) -> DualNum {
        self.flat_bulk_norm().add(self.flat_weight_norm())
    }
}

impl FlatNorm for MultiVector {
    type Output = DualNum;

    fn flat_norm(self) -> DualNum {
        self.flat_bulk_norm().add(self.flat_weight_norm())
    }
}

impl FlatNorm for Plane {
    type Output = DualNum;

    fn flat_norm(self) -> DualNum {
        self.flat_bulk_norm().add(self.flat_weight_norm())
    }
}

impl FlatNorm for Sphere {
    type Output = DualNum;

    fn flat_norm(self) -> DualNum {
        self.flat_bulk_norm().add(self.flat_weight_norm())
    }
}

impl UnitizedFlatNormSquared for Circle {
    type Output = f32;

    fn unitized_flat_norm_squared(self) -> f32 {
        self.flat_bulk_norm_squared().group0() / self.flat_weight_norm_squared().group0()
    }
}

impl UnitizedFlatNormSquared for Dipole {
    type Output = f32;

    fn unitized_flat_norm_squared(self) -> f32 {
        self.flat_bulk_norm_squared().group0() / self.flat_weight_norm_squared().group0()
    }
}

impl UnitizedFlatNormSquared for FlatPoint {
    type Output = f32;

    fn unitized_flat_norm_squared(self) -> f32 {
        self.flat_bulk_norm_squared().group0() / self.flat_weight_norm_squared().group0()
    }
}

impl UnitizedFlatNormSquared for Flector {
    type Output = f32;

    fn unitized_flat_norm_squared(self) -> f32 {
        self.flat_bulk_norm_squared().group0() / self.flat_weight_norm_squared().group0()
    }
}

impl UnitizedFlatNormSquared for Line {
    type Output = f32;

    fn unitized_flat_norm_squared(self) -> f32 {
        self.flat_bulk_norm_squared().group0() / self.flat_weight_norm_squared().group0()
    }
}

impl UnitizedFlatNormSquared for Motor {
    type Output = f32;

    fn unitized_flat_norm_squared(self) -> f32 {
        self.flat_bulk_norm_squared().group0() / self.flat_weight_norm_squared().group0()
    }
}

impl UnitizedFlatNormSquared for MultiVector {
    type Output = f32;

    fn unitized_flat_norm_squared(self) -> f32 {
        self.flat_bulk_norm_squared().group0() / self.flat_weight_norm_squared().group0()
    }
}

impl UnitizedFlatNormSquared for Plane {
    type Output = f32;

    fn unitized_flat_norm_squared(self) -> f32 {
        self.flat_bulk_norm_squared().group0() / self.flat_weight_norm_squared().group0()
    }
}

impl UnitizedFlatNormSquared for Sphere {
    type Output = f32;

    fn unitized_flat_norm_squared(self) -> f32 {
        self.flat_bulk_norm_squared().group0() / self.flat_weight_norm_squared().group0()
    }
}

impl UnitizedFlatNorm for Circle {
    type Output = f32;

    fn unitized_flat_norm(self) -> f32 {
        self.unitized_flat_norm_squared().sqrt()
    }
}

impl UnitizedFlatNorm for Dipole {
    type Output = f32;

    fn unitized_flat_norm(self) -> f32 {
        self.unitized_flat_norm_squared().sqrt()
    }
}

impl UnitizedFlatNorm for FlatPoint {
    type Output = f32;

    fn unitized_flat_norm(self) -> f32 {
        self.unitized_flat_norm_squared().sqrt()
    }
}

impl UnitizedFlatNorm for Flector {
    type Output = f32;

    fn unitized_flat_norm(self) -> f32 {
        self.unitized_flat_norm_squared().sqrt()
    }
}

impl UnitizedFlatNorm for Line {
    type Output = f32;

    fn unitized_flat_norm(self) -> f32 {
        self.unitized_flat_norm_squared().sqrt()
    }
}

impl UnitizedFlatNorm for Motor {
    type Output = f32;

    fn unitized_flat_norm(self) -> f32 {
        self.unitized_flat_norm_squared().sqrt()
    }
}

impl UnitizedFlatNorm for MultiVector {
    type Output = f32;

    fn unitized_flat_norm(self) -> f32 {
        self.unitized_flat_norm_squared().sqrt()
    }
}

impl UnitizedFlatNorm for Plane {
    type Output = f32;

    fn unitized_flat_norm(self) -> f32 {
        self.unitized_flat_norm_squared().sqrt()
    }
}

impl UnitizedFlatNorm for Sphere {
    type Output = f32;

    fn unitized_flat_norm(self) -> f32 {
        self.unitized_flat_norm_squared().sqrt()
    }
}

impl CenterNormSquared for Circle {
    type Output = Scalar;

    fn center_norm_squared(self) -> Scalar {
        self.round_bulk_norm_squared().add(self.flat_weight_norm_squared().anti_dual())
    }
}

impl CenterNormSquared for Dipole {
    type Output = Scalar;

    fn center_norm_squared(self) -> Scalar {
        self.round_bulk_norm_squared().add(self.flat_weight_norm_squared().anti_dual())
    }
}

impl CenterNormSquared for DualNum {
    type Output = Scalar;

    fn center_norm_squared(self) -> Scalar {
        self.round_bulk_norm_squared().add(self.flat_weight_norm_squared().anti_dual())
    }
}

impl CenterNormSquared for MultiVector {
    type Output = Scalar;

    fn center_norm_squared(self) -> Scalar {
        self.round_bulk_norm_squared().add(self.flat_weight_norm_squared().anti_dual())
    }
}

impl CenterNorm for Circle {
    type Output = Scalar;

    fn center_norm(self) -> Scalar {
        self.center_norm_squared().sqrt()
    }
}

impl CenterNorm for Dipole {
    type Output = Scalar;

    fn center_norm(self) -> Scalar {
        self.center_norm_squared().sqrt()
    }
}

impl CenterNorm for DualNum {
    type Output = Scalar;

    fn center_norm(self) -> Scalar {
        self.center_norm_squared().sqrt()
    }
}

impl CenterNorm for MultiVector {
    type Output = Scalar;

    fn center_norm(self) -> Scalar {
        self.center_norm_squared().sqrt()
    }
}

impl UnitizedCenterNormSquared for Circle {
    type Output = f32;

    fn unitized_center_norm_squared(self) -> f32 {
        self.center_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedCenterNormSquared for Dipole {
    type Output = f32;

    fn unitized_center_norm_squared(self) -> f32 {
        self.center_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedCenterNormSquared for MultiVector {
    type Output = f32;

    fn unitized_center_norm_squared(self) -> f32 {
        self.center_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedCenterNorm for Circle {
    type Output = f32;

    fn unitized_center_norm(self) -> f32 {
        self.unitized_center_norm_squared().sqrt()
    }
}

impl UnitizedCenterNorm for Dipole {
    type Output = f32;

    fn unitized_center_norm(self) -> f32 {
        self.unitized_center_norm_squared().sqrt()
    }
}

impl UnitizedCenterNorm for MultiVector {
    type Output = f32;

    fn unitized_center_norm(self) -> f32 {
        self.unitized_center_norm_squared().sqrt()
    }
}

impl RadiusNormSquared for Circle {
    type Output = Scalar;

    fn radius_norm_squared(self) -> Scalar {
        self.anti_dot(self).anti_dual()
    }
}

impl RadiusNormSquared for Dipole {
    type Output = Scalar;

    fn radius_norm_squared(self) -> Scalar {
        self.anti_dot(self).anti_dual()
    }
}

impl RadiusNormSquared for DualNum {
    type Output = Scalar;

    fn radius_norm_squared(self) -> Scalar {
        self.anti_dot(self).anti_dual()
    }
}

impl RadiusNormSquared for MultiVector {
    type Output = Scalar;

    fn radius_norm_squared(self) -> Scalar {
        self.anti_dot(self).anti_dual()
    }
}

impl RadiusNormSquared for RoundPoint {
    type Output = Scalar;

    fn radius_norm_squared(self) -> Scalar {
        self.anti_dot(self).anti_dual()
    }
}

impl RadiusNormSquared for Sphere {
    type Output = Scalar;

    fn radius_norm_squared(self) -> Scalar {
        self.anti_dot(self).anti_dual()
    }
}

impl RadiusNorm for Circle {
    type Output = Scalar;

    fn radius_norm(self) -> Scalar {
        self.radius_norm_squared().sqrt()
    }
}

impl RadiusNorm for Dipole {
    type Output = Scalar;

    fn radius_norm(self) -> Scalar {
        self.radius_norm_squared().sqrt()
    }
}

impl RadiusNorm for DualNum {
    type Output = Scalar;

    fn radius_norm(self) -> Scalar {
        self.radius_norm_squared().sqrt()
    }
}

impl RadiusNorm for MultiVector {
    type Output = Scalar;

    fn radius_norm(self) -> Scalar {
        self.radius_norm_squared().sqrt()
    }
}

impl RadiusNorm for RoundPoint {
    type Output = Scalar;

    fn radius_norm(self) -> Scalar {
        self.radius_norm_squared().sqrt()
    }
}

impl RadiusNorm for Sphere {
    type Output = Scalar;

    fn radius_norm(self) -> Scalar {
        self.radius_norm_squared().sqrt()
    }
}

impl UnitizedRadiusNormSquared for Circle {
    type Output = f32;

    fn unitized_radius_norm_squared(self) -> f32 {
        self.radius_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedRadiusNormSquared for Dipole {
    type Output = f32;

    fn unitized_radius_norm_squared(self) -> f32 {
        self.radius_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedRadiusNormSquared for MultiVector {
    type Output = f32;

    fn unitized_radius_norm_squared(self) -> f32 {
        self.radius_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedRadiusNormSquared for RoundPoint {
    type Output = f32;

    fn unitized_radius_norm_squared(self) -> f32 {
        self.radius_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedRadiusNormSquared for Sphere {
    type Output = f32;

    fn unitized_radius_norm_squared(self) -> f32 {
        self.radius_norm_squared().group0() / self.round_weight_norm_squared().group0()
    }
}

impl UnitizedRadiusNorm for Circle {
    type Output = f32;

    fn unitized_radius_norm(self) -> f32 {
        self.unitized_radius_norm_squared().sqrt()
    }
}

impl UnitizedRadiusNorm for Dipole {
    type Output = f32;

    fn unitized_radius_norm(self) -> f32 {
        self.unitized_radius_norm_squared().sqrt()
    }
}

impl UnitizedRadiusNorm for MultiVector {
    type Output = f32;

    fn unitized_radius_norm(self) -> f32 {
        self.unitized_radius_norm_squared().sqrt()
    }
}

impl UnitizedRadiusNorm for RoundPoint {
    type Output = f32;

    fn unitized_radius_norm(self) -> f32 {
        self.unitized_radius_norm_squared().sqrt()
    }
}

impl UnitizedRadiusNorm for Sphere {
    type Output = f32;

    fn unitized_radius_norm(self) -> f32 {
        self.unitized_radius_norm_squared().sqrt()
    }
}
