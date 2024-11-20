use crate::traits::CenterNormSquared;
use crate::traits::SquareRoot;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         3       6       0
//   Median:         6       8       0
//  Average:        11      13       0
//  Maximum:        63      65       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         3       6       0
//   Median:         6       8       0
//  Average:        11      13       0
//  Maximum:        63      65       0
impl std::ops::Div<center_norm> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: center_norm) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        7        0
    fn center_norm(self) -> Scalar {
        return self.center_norm_squared().square_root();
    }
}
impl std::ops::Div<center_norm> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: center_norm) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn center_norm(self) -> Scalar {
        return self.center_norm_squared().square_root();
    }
}
impl std::ops::Div<center_norm> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: center_norm) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn center_norm(self) -> Scalar {
        return self.center_norm_squared().square_root();
    }
}
impl std::ops::Div<center_norm> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: center_norm) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        7        0
    fn center_norm(self) -> Scalar {
        return self.center_norm_squared().square_root();
    }
}
impl std::ops::Div<center_norm> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: center_norm) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn center_norm(self) -> Scalar {
        return self.center_norm_squared().square_root();
    }
}
impl std::ops::Div<center_norm> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: center_norm) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn center_norm(self) -> Scalar {
        return self.center_norm_squared().square_root();
    }
}
impl std::ops::Div<center_norm> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: center_norm) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       63       65        0
    fn center_norm(self) -> Scalar {
        return self.center_norm_squared().square_root();
    }
}
impl std::ops::Div<center_norm> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: center_norm) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn center_norm(self) -> Scalar {
        return self.center_norm_squared().square_root();
    }
}
impl std::ops::Div<center_norm> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: center_norm) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn center_norm(self) -> Scalar {
        return self.center_norm_squared().square_root();
    }
}
