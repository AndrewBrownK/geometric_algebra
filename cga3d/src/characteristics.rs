//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::aspect_duals::*;
use crate::products::exterior::AntiWedge;
use crate::products::exterior::Wedge;
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
    fn grade(self) -> Self::Output;
}

/// Anti-Grade
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
pub trait AntiGrade {
    type Output;
    fn anti_grade(self) -> Self::Output;
}

/// Attitude
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Attitude
pub trait Attitude {
    type Output;
    fn attitude(self) -> Self::Output;
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
/// The Center of a round object is the Radial (RoundPoint) having the same center and radius.
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

    fn anti_grade(self) -> isize {
        0
    }
}

impl AntiGrade for Circle {
    type Output = isize;

    fn anti_grade(self) -> isize {
        2
    }
}

impl AntiGrade for Dipole {
    type Output = isize;

    fn anti_grade(self) -> isize {
        3
    }
}

impl AntiGrade for Horizon {
    type Output = isize;

    fn anti_grade(self) -> isize {
        1
    }
}

impl AntiGrade for Infinity {
    type Output = isize;

    fn anti_grade(self) -> isize {
        4
    }
}

impl AntiGrade for Line {
    type Output = isize;

    fn anti_grade(self) -> isize {
        2
    }
}

impl AntiGrade for LineAtInfinity {
    type Output = isize;

    fn anti_grade(self) -> isize {
        2
    }
}

impl AntiGrade for LineAtOrigin {
    type Output = isize;

    fn anti_grade(self) -> isize {
        2
    }
}

impl AntiGrade for Origin {
    type Output = isize;

    fn anti_grade(self) -> isize {
        3
    }
}

impl AntiGrade for Plane {
    type Output = isize;

    fn anti_grade(self) -> isize {
        1
    }
}

impl AntiGrade for PlaneAtOrigin {
    type Output = isize;

    fn anti_grade(self) -> isize {
        1
    }
}

impl AntiGrade for Point {
    type Output = isize;

    fn anti_grade(self) -> isize {
        3
    }
}

impl AntiGrade for PointAtInfinity {
    type Output = isize;

    fn anti_grade(self) -> isize {
        3
    }
}

impl AntiGrade for RoundPoint {
    type Output = isize;

    fn anti_grade(self) -> isize {
        4
    }
}

impl AntiGrade for Scalar {
    type Output = isize;

    fn anti_grade(self) -> isize {
        5
    }
}

impl AntiGrade for Sphere {
    type Output = isize;

    fn anti_grade(self) -> isize {
        1
    }
}

impl Grade for AntiScalar {
    type Output = isize;

    fn grade(self) -> isize {
        5
    }
}

impl Grade for Circle {
    type Output = isize;

    fn grade(self) -> isize {
        3
    }
}

impl Grade for Dipole {
    type Output = isize;

    fn grade(self) -> isize {
        2
    }
}

impl Grade for Horizon {
    type Output = isize;

    fn grade(self) -> isize {
        4
    }
}

impl Grade for Infinity {
    type Output = isize;

    fn grade(self) -> isize {
        1
    }
}

impl Grade for Line {
    type Output = isize;

    fn grade(self) -> isize {
        3
    }
}

impl Grade for LineAtInfinity {
    type Output = isize;

    fn grade(self) -> isize {
        3
    }
}

impl Grade for LineAtOrigin {
    type Output = isize;

    fn grade(self) -> isize {
        3
    }
}

impl Grade for Origin {
    type Output = isize;

    fn grade(self) -> isize {
        2
    }
}

impl Grade for Plane {
    type Output = isize;

    fn grade(self) -> isize {
        4
    }
}

impl Grade for PlaneAtOrigin {
    type Output = isize;

    fn grade(self) -> isize {
        4
    }
}

impl Grade for Point {
    type Output = isize;

    fn grade(self) -> isize {
        2
    }
}

impl Grade for PointAtInfinity {
    type Output = isize;

    fn grade(self) -> isize {
        2
    }
}

impl Grade for RoundPoint {
    type Output = isize;

    fn grade(self) -> isize {
        1
    }
}

impl Grade for Scalar {
    type Output = isize;

    fn grade(self) -> isize {
        0
    }
}

impl Grade for Sphere {
    type Output = isize;

    fn grade(self) -> isize {
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

impl Attitude for Dipole {
    type Output = RoundPoint;

    fn attitude(self) -> RoundPoint {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Line {
    type Output = PointAtInfinity;

    fn attitude(self) -> PointAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for LineAtOrigin {
    type Output = PointAtInfinity;

    fn attitude(self) -> PointAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Magnitude {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
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
    type Output = Infinity;

    fn attitude(self) -> Infinity {
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

impl Attitude for Point {
    type Output = Infinity;

    fn attitude(self) -> Infinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for RoundPoint {
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

impl Carrier for Circle {
    type Output = Plane;

    fn carrier(self) -> Plane {
        self.wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl Carrier for Dipole {
    type Output = Line;

    fn carrier(self) -> Line {
        self.wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl Carrier for Magnitude {
    type Output = Infinity;

    fn carrier(self) -> Infinity {
        self.wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl Carrier for MultiVector {
    type Output = MultiVector;

    fn carrier(self) -> MultiVector {
        self.wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl Carrier for RoundPoint {
    type Output = Point;

    fn carrier(self) -> Point {
        self.wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl Carrier for Scalar {
    type Output = Infinity;

    fn carrier(self) -> Infinity {
        self.wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl Carrier for Sphere {
    type Output = AntiScalar;

    fn carrier(self) -> AntiScalar {
        self.wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl Center for Circle {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Dipole {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for MultiVector {
    type Output = MultiVector;

    fn center(self) -> MultiVector {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for RoundPoint {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Sphere {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl CoCarrier for Circle {
    type Output = Line;

    fn co_carrier(self) -> Line {
        self.right_round_weight_dual().wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl CoCarrier for Dipole {
    type Output = Plane;

    fn co_carrier(self) -> Plane {
        self.right_round_weight_dual().wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl CoCarrier for MultiVector {
    type Output = MultiVector;

    fn co_carrier(self) -> MultiVector {
        self.right_round_weight_dual().wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl CoCarrier for RoundPoint {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.right_round_weight_dual().wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl CoCarrier for Sphere {
    type Output = Point;

    fn co_carrier(self) -> Point {
        self.right_round_weight_dual().wedge(Infinity {
            groups: InfinityGroups { g0: 1.0 },
        })
    }
}

impl Container for Circle {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().right_weight_dual())
    }
}

impl Container for Dipole {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().right_weight_dual())
    }
}

impl Container for MultiVector {
    type Output = MultiVector;

    fn container(self) -> MultiVector {
        self.wedge(self.carrier().right_weight_dual())
    }
}

impl Container for RoundPoint {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().right_weight_dual())
    }
}

impl Container for Sphere {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().right_weight_dual())
    }
}

impl Partner for Circle {
    type Output = Circle;

    fn partner(self) -> Circle {
        self.right_bulk_dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Dipole {
    type Output = Dipole;

    fn partner(self) -> Dipole {
        self.right_bulk_dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for MultiVector {
    type Output = MultiVector;

    fn partner(self) -> MultiVector {
        self.right_bulk_dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for RoundPoint {
    type Output = RoundPoint;

    fn partner(self) -> RoundPoint {
        self.right_bulk_dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Sphere {
    type Output = Sphere;

    fn partner(self) -> Sphere {
        self.right_bulk_dual().container().neg().anti_wedge(self.carrier())
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
