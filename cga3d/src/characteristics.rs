//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::dot::AntiDot;
use crate::products::dot::Dot;
use crate::products::exterior::AntiWedge;
use crate::products::exterior::Wedge;
use crate::products::geometric::GeometricAntiProduct;
use crate::products::geometric::GeometricProduct;
use crate::*;

/// Square Root
pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

/// Grade
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
pub trait Grade {
    type Output;
    fn grade() -> Self::Output;
}

/// Anti-Grade
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
pub trait AntiGrade {
    type Output;
    fn anti_grade() -> Self::Output;
}

/// Attitude
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Attitude
pub trait Attitude {
    type Output;
    fn attitude(self) -> Self::Output;
}

/// Inverse, as in `x^-1` (with respect to geometric product).
/// Useful to define the geometric quotient.
/// Not to be confused with the "Point Inversion" or "Sphere Inversion" operations.
pub trait Inverse {
    type Output;
    fn inverse(self) -> Self::Output;
}

/// Inverse, as in `x^-1` (with respect to geometric anti-product).
/// Useful to define the geometric anti-quotient.
/// Not to be confused with the "Point Inversion" or "Sphere Inversion" operations.
pub trait AntiInverse {
    type Output;
    fn anti_inverse(self) -> Self::Output;
}

/// Carrier
/// The Carrier of a round object is the lowest dimensional flat object that contains it.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Carriers
pub trait Carrier {
    type Output;
    fn carrier(self) -> Self::Output;
}

/// CoCarrier
/// The CoCarrier of a round object is the Carrier of its antidual.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Carriers
pub trait CoCarrier {
    type Output;
    fn co_carrier(self) -> Self::Output;
}

/// Container
/// The Container of a round object is the smallest Sphere that contains it.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Containers
pub trait Container {
    type Output;
    fn container(self) -> Self::Output;
}

/// Center
/// The Center of a round object is the RoundPoint having the same center and radius.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Centers
pub trait Center {
    type Output;
    fn center(self) -> Self::Output;
}

/// Partner
/// The Partner of a round object is the round object having the same center, same carrier,
/// and same absolute size, but having a squared radius of the opposite sign.
/// The dot product between a round object and its partner is always zero. They are orthogonal.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Partners
pub trait Partner {
    type Output;
    fn partner(self) -> Self::Output;
}

impl AntiGrade for AntiScalar {
    type Output = isize;

    fn anti_grade() -> isize {
        0
    }
}

impl AntiGrade for Circle {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for CircleBulkAspect {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for CircleCarrierAspect {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for CircleWeightAspect {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for Dipole {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for DipoleBulkAspect {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for DipoleCarrierAspect {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for DipoleWeightAspect {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for FlatPoint {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for FlatPointAtInfinity {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for FlatPointAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for Horizon {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for Infinity {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for Line {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for LineAtInfinity {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for LineAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for Origin {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for Plane {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for PlaneAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for RoundPoint {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for RoundPointAtInfinity {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for RoundPointAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for RoundPointBulkAspect {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for RoundPointCarrierAspect {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for Scalar {
    type Output = isize;

    fn anti_grade() -> isize {
        5
    }
}

impl AntiGrade for Sphere {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for SphereWeightAspect {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl Grade for AntiScalar {
    type Output = isize;

    fn grade() -> isize {
        5
    }
}

impl Grade for Circle {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for CircleBulkAspect {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for CircleCarrierAspect {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for CircleWeightAspect {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for Dipole {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for DipoleBulkAspect {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for DipoleCarrierAspect {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for DipoleWeightAspect {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for FlatPoint {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for FlatPointAtInfinity {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for FlatPointAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for Horizon {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for Infinity {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for Line {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for LineAtInfinity {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for LineAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for Origin {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for Plane {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for PlaneAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for RoundPoint {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for RoundPointAtInfinity {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for RoundPointAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for RoundPointBulkAspect {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for RoundPointCarrierAspect {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for Scalar {
    type Output = isize;

    fn grade() -> isize {
        0
    }
}

impl Grade for Sphere {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for SphereWeightAspect {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Attitude for AntiScalar {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Circle {
    type Output = Dipole;

    fn attitude(self) -> Dipole {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for CircleCarrierAspect {
    type Output = DipoleBulkAspect;

    fn attitude(self) -> DipoleBulkAspect {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for CircleWeightAspect {
    type Output = DipoleBulkAspect;

    fn attitude(self) -> DipoleBulkAspect {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Dipole {
    type Output = RoundPointAtInfinity;

    fn attitude(self) -> RoundPointAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for DipoleCarrierAspect {
    type Output = RoundPointBulkAspect;

    fn attitude(self) -> RoundPointBulkAspect {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for DipoleWeightAspect {
    type Output = RoundPointBulkAspect;

    fn attitude(self) -> RoundPointBulkAspect {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for FlatPoint {
    type Output = Infinity;

    fn attitude(self) -> Infinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for FlatPointAtOrigin {
    type Output = Infinity;

    fn attitude(self) -> Infinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Flector {
    type Output = MultiVector;

    fn attitude(self) -> MultiVector {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Line {
    type Output = FlatPointAtInfinity;

    fn attitude(self) -> FlatPointAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for LineAtOrigin {
    type Output = FlatPointAtInfinity;

    fn attitude(self) -> FlatPointAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Magnitude {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Motor {
    type Output = FlectorAtInfinity;

    fn attitude(self) -> FlectorAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for MultiVector {
    type Output = MultiVector;

    fn attitude(self) -> MultiVector {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Origin {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Plane {
    type Output = LineAtInfinity;

    fn attitude(self) -> LineAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn attitude(self) -> LineAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Rotor {
    type Output = FlectorAtInfinity;

    fn attitude(self) -> FlectorAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for RoundPoint {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for RoundPointAtOrigin {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for RoundPointCarrierAspect {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Sphere {
    type Output = Circle;

    fn attitude(self) -> Circle {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for SphereWeightAspect {
    type Output = CircleBulkAspect;

    fn attitude(self) -> CircleBulkAspect {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Transflector {
    type Output = LineAtInfinity;

    fn attitude(self) -> LineAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Translator {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::one())
    }
}

impl Carrier for Circle {
    type Output = Plane;

    fn carrier(self) -> Plane {
        self.wedge(Infinity::one())
    }
}

impl Carrier for CircleBulkAspect {
    type Output = Horizon;

    fn carrier(self) -> Horizon {
        self.wedge(Infinity::one())
    }
}

impl Carrier for CircleCarrierAspect {
    type Output = Plane;

    fn carrier(self) -> Plane {
        self.wedge(Infinity::one())
    }
}

impl Carrier for CircleWeightAspect {
    type Output = PlaneAtOrigin;

    fn carrier(self) -> PlaneAtOrigin {
        self.wedge(Infinity::one())
    }
}

impl Carrier for Dipole {
    type Output = Line;

    fn carrier(self) -> Line {
        self.wedge(Infinity::one())
    }
}

impl Carrier for DipoleBulkAspect {
    type Output = LineAtInfinity;

    fn carrier(self) -> LineAtInfinity {
        self.wedge(Infinity::one())
    }
}

impl Carrier for DipoleCarrierAspect {
    type Output = Line;

    fn carrier(self) -> Line {
        self.wedge(Infinity::one())
    }
}

impl Carrier for DipoleWeightAspect {
    type Output = LineAtOrigin;

    fn carrier(self) -> LineAtOrigin {
        self.wedge(Infinity::one())
    }
}

impl Carrier for Magnitude {
    type Output = Infinity;

    fn carrier(self) -> Infinity {
        self.wedge(Infinity::one())
    }
}

impl Carrier for MultiVector {
    type Output = MultiVector;

    fn carrier(self) -> MultiVector {
        self.wedge(Infinity::one())
    }
}

impl Carrier for Origin {
    type Output = FlatPointAtOrigin;

    fn carrier(self) -> FlatPointAtOrigin {
        self.wedge(Infinity::one())
    }
}

impl Carrier for RoundPoint {
    type Output = FlatPoint;

    fn carrier(self) -> FlatPoint {
        self.wedge(Infinity::one())
    }
}

impl Carrier for RoundPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn carrier(self) -> FlatPointAtInfinity {
        self.wedge(Infinity::one())
    }
}

impl Carrier for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn carrier(self) -> FlatPointAtOrigin {
        self.wedge(Infinity::one())
    }
}

impl Carrier for RoundPointBulkAspect {
    type Output = FlatPointAtInfinity;

    fn carrier(self) -> FlatPointAtInfinity {
        self.wedge(Infinity::one())
    }
}

impl Carrier for RoundPointCarrierAspect {
    type Output = FlatPoint;

    fn carrier(self) -> FlatPoint {
        self.wedge(Infinity::one())
    }
}

impl Carrier for Scalar {
    type Output = Infinity;

    fn carrier(self) -> Infinity {
        self.wedge(Infinity::one())
    }
}

impl Carrier for Sphere {
    type Output = AntiScalar;

    fn carrier(self) -> AntiScalar {
        self.wedge(Infinity::one())
    }
}

impl Carrier for SphereWeightAspect {
    type Output = AntiScalar;

    fn carrier(self) -> AntiScalar {
        self.wedge(Infinity::one())
    }
}

impl CoCarrier for Circle {
    type Output = Line;

    fn co_carrier(self) -> Line {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for CircleCarrierAspect {
    type Output = Line;

    fn co_carrier(self) -> Line {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for CircleWeightAspect {
    type Output = LineAtOrigin;

    fn co_carrier(self) -> LineAtOrigin {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for Dipole {
    type Output = Plane;

    fn co_carrier(self) -> Plane {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for DipoleCarrierAspect {
    type Output = Plane;

    fn co_carrier(self) -> Plane {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for DipoleWeightAspect {
    type Output = PlaneAtOrigin;

    fn co_carrier(self) -> PlaneAtOrigin {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for Magnitude {
    type Output = Infinity;

    fn co_carrier(self) -> Infinity {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for MultiVector {
    type Output = MultiVector;

    fn co_carrier(self) -> MultiVector {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for Origin {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for RoundPoint {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for Sphere {
    type Output = FlatPoint;

    fn co_carrier(self) -> FlatPoint {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl CoCarrier for SphereWeightAspect {
    type Output = FlatPointAtOrigin;

    fn co_carrier(self) -> FlatPointAtOrigin {
        self.anti_dual().wedge(Infinity::one())
    }
}

impl Sqrt for AntiScalar {
    type Output = AntiScalar;

    fn sqrt(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0().sqrt() },
        }
    }
}

impl Sqrt for Scalar {
    type Output = Scalar;

    fn sqrt(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0().sqrt() },
        }
    }
}

impl Center for Circle {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn center(self) -> RoundPointCarrierAspect {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for CircleWeightAspect {
    type Output = Origin;

    fn center(self) -> Origin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Dipole {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn center(self) -> RoundPointCarrierAspect {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for DipoleWeightAspect {
    type Output = Origin;

    fn center(self) -> Origin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Magnitude {
    type Output = Infinity;

    fn center(self) -> Infinity {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for MultiVector {
    type Output = MultiVector;

    fn center(self) -> MultiVector {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Origin {
    type Output = Origin;

    fn center(self) -> Origin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for RoundPoint {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn center(self) -> RoundPointAtOrigin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn center(self) -> RoundPointCarrierAspect {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Sphere {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for SphereWeightAspect {
    type Output = Origin;

    fn center(self) -> Origin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Container for Circle {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for CircleBulkAspect {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for CircleCarrierAspect {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for CircleWeightAspect {
    type Output = SphereWeightAspect;

    fn container(self) -> SphereWeightAspect {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Dipole {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for DipoleBulkAspect {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for DipoleCarrierAspect {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for DipoleWeightAspect {
    type Output = SphereWeightAspect;

    fn container(self) -> SphereWeightAspect {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Magnitude {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for MultiVector {
    type Output = MultiVector;

    fn container(self) -> MultiVector {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Origin {
    type Output = SphereWeightAspect;

    fn container(self) -> SphereWeightAspect {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for RoundPoint {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for RoundPointAtInfinity {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for RoundPointAtOrigin {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for RoundPointBulkAspect {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for RoundPointCarrierAspect {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Scalar {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Sphere {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn container(self) -> SphereWeightAspect {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Partner for Circle {
    type Output = Circle;

    fn partner(self) -> Circle {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for CircleCarrierAspect {
    type Output = Circle;

    fn partner(self) -> Circle {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn partner(self) -> CircleWeightAspect {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Dipole {
    type Output = Dipole;

    fn partner(self) -> Dipole {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for DipoleCarrierAspect {
    type Output = Dipole;

    fn partner(self) -> Dipole {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn partner(self) -> DipoleWeightAspect {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for MultiVector {
    type Output = MultiVector;

    fn partner(self) -> MultiVector {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Origin {
    type Output = Origin;

    fn partner(self) -> Origin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for RoundPoint {
    type Output = RoundPoint;

    fn partner(self) -> RoundPoint {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn partner(self) -> RoundPointAtOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn partner(self) -> RoundPoint {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Sphere {
    type Output = Sphere;

    fn partner(self) -> Sphere {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn partner(self) -> SphereWeightAspect {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl AntiInverse for AntiScalar {
    type Output = AntiScalar;

    fn anti_inverse(self) -> AntiScalar {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Circle {
    type Output = Circle;

    fn anti_inverse(self) -> Circle {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn anti_inverse(self) -> CircleBulkAspect {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_inverse(self) -> CircleCarrierAspect {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Dipole {
    type Output = Dipole;

    fn anti_inverse(self) -> Dipole {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn anti_inverse(self) -> DipoleBulkAspect {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_inverse(self) -> DipoleCarrierAspect {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for FlatPoint {
    type Output = FlatPoint;

    fn anti_inverse(self) -> FlatPoint {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_inverse(self) -> FlatPointAtOrigin {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Flector {
    type Output = Flector;

    fn anti_inverse(self) -> Flector {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Line {
    type Output = Line;

    fn anti_inverse(self) -> Line {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_inverse(self) -> LineAtOrigin {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Magnitude {
    type Output = Magnitude;

    fn anti_inverse(self) -> Magnitude {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Motor {
    type Output = Motor;

    fn anti_inverse(self) -> Motor {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for MultiVector {
    type Output = MultiVector;

    fn anti_inverse(self) -> MultiVector {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Plane {
    type Output = Plane;

    fn anti_inverse(self) -> Plane {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_inverse(self) -> PlaneAtOrigin {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Rotor {
    type Output = Rotor;

    fn anti_inverse(self) -> Rotor {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for RoundPoint {
    type Output = RoundPoint;

    fn anti_inverse(self) -> RoundPoint {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn anti_inverse(self) -> RoundPointAtInfinity {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_inverse(self) -> RoundPointAtOrigin {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn anti_inverse(self) -> RoundPointBulkAspect {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_inverse(self) -> RoundPointCarrierAspect {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Scalar {
    type Output = Scalar;

    fn anti_inverse(self) -> Scalar {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Sphere {
    type Output = Sphere;

    fn anti_inverse(self) -> Sphere {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Transflector {
    type Output = Transflector;

    fn anti_inverse(self) -> Transflector {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Translator {
    type Output = Translator;

    fn anti_inverse(self) -> Translator {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl Inverse for AntiScalar {
    type Output = AntiScalar;

    fn inverse(self) -> AntiScalar {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Circle {
    type Output = Circle;

    fn inverse(self) -> Circle {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn inverse(self) -> CircleBulkAspect {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn inverse(self) -> CircleCarrierAspect {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Dipole {
    type Output = Dipole;

    fn inverse(self) -> Dipole {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn inverse(self) -> DipoleBulkAspect {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn inverse(self) -> DipoleCarrierAspect {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for FlatPoint {
    type Output = FlatPoint;

    fn inverse(self) -> FlatPoint {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn inverse(self) -> FlatPointAtOrigin {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Flector {
    type Output = Flector;

    fn inverse(self) -> Flector {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Line {
    type Output = Line;

    fn inverse(self) -> Line {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for LineAtOrigin {
    type Output = LineAtOrigin;

    fn inverse(self) -> LineAtOrigin {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Magnitude {
    type Output = Magnitude;

    fn inverse(self) -> Magnitude {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Motor {
    type Output = Motor;

    fn inverse(self) -> Motor {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for MultiVector {
    type Output = MultiVector;

    fn inverse(self) -> MultiVector {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Plane {
    type Output = Plane;

    fn inverse(self) -> Plane {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn inverse(self) -> PlaneAtOrigin {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Rotor {
    type Output = Rotor;

    fn inverse(self) -> Rotor {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for RoundPoint {
    type Output = RoundPoint;

    fn inverse(self) -> RoundPoint {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn inverse(self) -> RoundPointAtInfinity {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn inverse(self) -> RoundPointAtOrigin {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn inverse(self) -> RoundPointBulkAspect {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn inverse(self) -> RoundPointCarrierAspect {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Scalar {
    type Output = Scalar;

    fn inverse(self) -> Scalar {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Sphere {
    type Output = Sphere;

    fn inverse(self) -> Sphere {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Transflector {
    type Output = Transflector;

    fn inverse(self) -> Transflector {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Translator {
    type Output = Translator;

    fn inverse(self) -> Translator {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}
