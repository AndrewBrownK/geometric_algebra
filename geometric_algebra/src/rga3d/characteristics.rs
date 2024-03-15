#![allow(clippy::assign_op_pattern)]
use crate::rga3d::products::exterior::AntiWedge;
use crate::rga3d::*;

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

impl AntiGrade for AntiScalar {
    type Output = isize;

    fn anti_grade(self) -> isize {
        0
    }
}

impl AntiGrade for Horizon {
    type Output = isize;

    fn anti_grade(self) -> isize {
        1
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

impl AntiGrade for Scalar {
    type Output = isize;

    fn anti_grade(self) -> isize {
        4
    }
}

impl Grade for AntiScalar {
    type Output = isize;

    fn grade(self) -> isize {
        4
    }
}

impl Grade for Horizon {
    type Output = isize;

    fn grade(self) -> isize {
        3
    }
}

impl Grade for Line {
    type Output = isize;

    fn grade(self) -> isize {
        2
    }
}

impl Grade for LineAtInfinity {
    type Output = isize;

    fn grade(self) -> isize {
        2
    }
}

impl Grade for LineAtOrigin {
    type Output = isize;

    fn grade(self) -> isize {
        2
    }
}

impl Grade for Origin {
    type Output = isize;

    fn grade(self) -> isize {
        1
    }
}

impl Grade for Plane {
    type Output = isize;

    fn grade(self) -> isize {
        3
    }
}

impl Grade for PlaneAtOrigin {
    type Output = isize;

    fn grade(self) -> isize {
        3
    }
}

impl Grade for Point {
    type Output = isize;

    fn grade(self) -> isize {
        1
    }
}

impl Grade for PointAtInfinity {
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

impl Attitude for AntiScalar {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
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

impl Attitude for Motor {
    type Output = Flector;

    fn attitude(self) -> Flector {
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

impl Attitude for Point {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Rotor {
    type Output = Flector;

    fn attitude(self) -> Flector {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Translator {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::one())
    }
}

impl Sqrt for AntiScalar {
    type Output = AntiScalar;

    fn sqrt(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0().sqrt(),
            },
        }
    }
}

impl Sqrt for Scalar {
    type Output = Scalar;

    fn sqrt(self) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0().sqrt(),
            },
        }
    }
}
